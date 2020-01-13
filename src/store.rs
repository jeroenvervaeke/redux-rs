pub struct Store<S, A, D>
where
    D: Dispatch<A>,
{
    dispatch: D,
    listeners: Vec<Box<dyn Listener>>,
    state: S,
}

impl<S, A, D> Store<S, A, D> {
    pub fn new() -> Self
    where
        S: Default,
    {
        Self::new_with_state(S::default())
    }

    pub fn new_with_state(state: S) -> Self {
        Store {
            dispatch:,// todo
            listeners: Vec::with_capacity(1),
            state,
        }
    }

    fn state(&self) -> &S {
        &self.state
    }

    // TODO: add unsubscribe
    fn subscribe(&mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener)
    }

    // TODO: replace reducer
    // replaceReducer
}

/// A *listener function* is a function that gets called when the
pub trait Listener {
    fn call();
}

///  A *dispatching function* is a function that accepts an action
pub trait Dispatch<A> {
    fn dispatch(action: &A);
}
