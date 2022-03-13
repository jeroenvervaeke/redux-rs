use async_trait::async_trait;
use std::marker::PhantomData;

use crate::{
    middleware::{MiddleWare, StoreApi, StoreWithMiddleware},
    Reducer, Selector, Subscriber,
};

mod worker;
use worker::{Address, Dispatch, Select, StateWorker, Subscribe};

/// The store is the heart of any redux application, it contains the state of the application.
///
/// The state of the store can be modified by dispatching actions to it.
/// Updates to the state can be observed by subscribing to the store or by writing middleware.
/// Getting a part of the store or the full store is possible with the select and state_cloned methods.
pub struct Store<State, Action, RootReducer>
where
    State: Send,
    RootReducer: Send,
{
    worker_address: Address<State, Action, RootReducer>,
    _worker_handle: crate::async_spawner::SpawnResult,

    _types: PhantomData<RootReducer>,
}

impl<State, Action, RootReducer> Store<State, Action, RootReducer>
where
    Action: Send + 'static,
    RootReducer: Reducer<State, Action> + Send + 'static,
    State: Send + 'static,
{
    /// Create a new store with the given root reducer and default state
    pub fn new(root_reducer: RootReducer) -> Self
    where
        State: Default,
    {
        Self::new_with_state(root_reducer, Default::default())
    }

    /// Create a new store with the given root reducer and the provided state
    pub fn new_with_state(root_reducer: RootReducer, state: State) -> Self {
        let mut worker = StateWorker::new(root_reducer, state);
        let worker_address = worker.address();

        let _worker_handle = crate::async_spawner::spawn(async move {
            worker.run().await;
        });

        Store {
            worker_address,
            _worker_handle,

            _types: Default::default(),
        }
    }

    /// Dispatch a new action to the store
    ///
    /// Notice that this method takes &self and not &mut self,
    /// this enables us to dispatch actions from multiple places at once without requiring locks.
    pub async fn dispatch(&self, action: Action) {
        self.worker_address.send(Dispatch::new(action)).await;
    }

    /// Select a part of the state, this is more efficient than copying the entire state all the time.
    /// In case you still need a full copy of the state, use the state_cloned method.
    pub async fn select<S: Selector<State, Result = Result>, Result>(&self, selector: S) -> Result
    where
        S: Selector<State, Result = Result> + Send + 'static,
        Result: Send + 'static,
    {
        self.worker_address.send(Select::new(selector)).await
    }

    /// Returns a cloned version of the state.
    /// This is not efficient, if you only need a part of the state use select instead
    pub async fn state_cloned(&self) -> State
    where
        State: Clone,
    {
        self.select(|state: &State| state.clone()).await
    }

    /// Subscribe to state changes.
    /// Every time an action is dispatched the subscriber will be notified after the state is updated
    pub async fn subscribe<S: Subscriber<State> + Send + 'static>(&self, subscriber: S) {
        self.worker_address.send(Subscribe::new(Box::new(subscriber))).await
    }

    /// Wrap the store with middleware, see middleware module for more examples
    pub async fn wrap<M, OuterAction>(self, middleware: M) -> StoreWithMiddleware<Self, M, State, Action, OuterAction>
    where
        M: MiddleWare<State, OuterAction, Self, Action> + Send + Sync,
        OuterAction: Send + Sync + 'static,
        State: Sync,
        Action: Sync,
        RootReducer: Sync,
    {
        StoreWithMiddleware::new(self, middleware).await
    }
}

#[async_trait]
impl<State, Action, RootReducer> StoreApi<State, Action> for Store<State, Action, RootReducer>
where
    Action: Send + Sync + 'static,
    RootReducer: Reducer<State, Action> + Send + Sync + 'static,
    State: Send + Sync + 'static,
{
    async fn dispatch<A: Into<Action> + Send>(&self, action: A) {
        Store::dispatch(self, action.into()).await
    }

    async fn select<S: Selector<State, Result = Result>, Result>(&self, selector: S) -> Result
    where
        S: Selector<State, Result = Result> + Send + 'static,
        Result: Send + 'static,
    {
        Store::select(self, selector).await
    }

    async fn state_cloned(&self) -> State
    where
        State: Clone,
    {
        Store::state_cloned(self).await
    }

    async fn subscribe<S: Subscriber<State> + Send + 'static>(&self, subscriber: S) {
        Store::subscribe(self, subscriber).await
    }
}

#[cfg(all(test, feature = "test_async_tokio"))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Arc;

    #[derive(Clone, Debug, PartialEq)]
    struct Counter {
        value: i32,
    }

    impl Counter {
        pub fn new(value: i32) -> Self {
            Counter { value }
        }
    }

    impl Default for Counter {
        fn default() -> Self {
            Self { value: 42 }
        }
    }

    struct ValueSelector;
    impl Selector<Counter> for ValueSelector {
        type Result = i32;

        fn select(&self, state: &Counter) -> Self::Result {
            state.value
        }
    }

    enum CounterAction {
        Increment,
        Decrement,
    }

    fn counter_reducer(state: Counter, action: CounterAction) -> Counter {
        match action {
            CounterAction::Increment => Counter { value: state.value + 1 },
            CounterAction::Decrement => Counter { value: state.value - 1 },
        }
    }

    #[tokio::test]
    async fn counter_default_state() {
        let store = Store::new(counter_reducer);
        assert_eq!(Counter::default(), store.state_cloned().await);
    }

    #[tokio::test]
    async fn counter_supplied_state() {
        let store = Store::new_with_state(counter_reducer, Counter::new(5));
        assert_eq!(Counter::new(5), store.state_cloned().await);
    }

    #[tokio::test]
    async fn counter_actions_cloned_state() {
        let store = Store::new(counter_reducer);
        assert_eq!(Counter::new(42), store.state_cloned().await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(Counter::new(43), store.state_cloned().await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(Counter::new(44), store.state_cloned().await);

        store.dispatch(CounterAction::Decrement).await;
        assert_eq!(Counter::new(43), store.state_cloned().await);
    }

    #[tokio::test]
    async fn counter_actions_selector_struct() {
        let store = Store::new(counter_reducer);
        assert_eq!(42, store.select(ValueSelector).await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(43, store.select(ValueSelector).await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(44, store.select(ValueSelector).await);

        store.dispatch(CounterAction::Decrement).await;
        assert_eq!(43, store.select(ValueSelector).await);
    }

    #[tokio::test]
    async fn counter_actions_selector_lambda() {
        let store = Store::new(counter_reducer);
        assert_eq!(42, store.select(|state: &Counter| state.value).await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(43, store.select(|state: &Counter| state.value).await);

        store.dispatch(CounterAction::Increment).await;
        assert_eq!(44, store.select(|state: &Counter| state.value).await);

        store.dispatch(CounterAction::Decrement).await;
        assert_eq!(43, store.select(|state: &Counter| state.value).await);
    }

    #[tokio::test]
    async fn counter_subscribe() {
        let store = Store::new(counter_reducer);
        assert_eq!(42, store.select(|state: &Counter| state.value).await);

        let sum = Arc::new(AtomicI32::new(0));

        // Count the total value of all changes
        let captured_sum = sum.clone();
        store
            .subscribe(move |state: &Counter| {
                captured_sum.fetch_add(state.value, Ordering::Relaxed);
            })
            .await;

        store.dispatch(CounterAction::Increment).await;
        store.dispatch(CounterAction::Increment).await;
        store.dispatch(CounterAction::Decrement).await;

        // Sum should be: 43 + 44 + 43 = 130
        assert_eq!(sum.load(Ordering::Relaxed), 130);
    }
}
