//! # redux-rs - A Rust implementation of Redux.
//!
//! Redux-rs is a predictable state container for Rust applications.
//!
//! The goal of this project is to provide _similar_ functionality as its Javascript counterpart.
//! However, due to the differences between Javascript and Rust, the API is not exactly the same.
//!
//! This project offers the following functionality:
//! - A lock-free store, where you can dispatch actions to, with only a shared reference (`&Store`)
//! - Flexible middleware that can intercept/modify/launch actions at any time
//!
//! ## Concepts
//!
//! Data in the redux store is immutable. The only way to update the data in the store is by dispatching actions to the store.
//! The data is altered using a provided reducer.
//!
//! Middleware can be used to introduce side effects when dispatching actions.
//! An example of a side effect is making an API call.
//!
//! ### State
//!
//! A state is the form of data that Redux manages.
//! Theoretically it could be anything, but as an example, let's consider a simple counter.
//! The counter can only increment and decrement.
//! The state would look like this:
//!
//! ```
//! #[derive(Default)]
//! struct State {
//!     counter: i8
//! }
//! ```
//!
//! ### Actions
//!
//! In order to change the state, we need to dispatch actions. In Rust, the different actions would usually be represented by an enum.
//! In the case of our counter example, we want to be able to increment and decrement the counter value.
//!
//! ```
//! enum Action {
//!     Increment,
//!     Decrement
//! }
//! ```
//!
//! ### Reducer
//!
//! To actually change the state (read: create a new one), we need what is called a **reducer**.
//! A reducer is a pure function which takes in the current state plus the action to perform and returns a new state.
//!
//! >Note: A reducer is a pure function: it should not introduce any side-effects.
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn async_test() {
//! # use redux_rs::Store;
//! #
//! # #[derive(Default)]
//! # struct State {
//! #     counter: i8
//! # }
//! #
//! # enum Action {
//! #     Increment,
//! #     Decrement
//! # }
//! #
//! fn reducer(state: State, action: Action) -> State {
//!     match action {
//!         Action::Increment => State {
//!             counter: state.counter + 1
//!         },
//!         Action::Decrement => State {
//!             counter: state.counter - 1
//!         }
//!     }
//! }
//! # let _ = Store::new(reducer);
//! # }
//! ```
//!
//! Note how the reducer uses the old data to create a new state.
//!
//! ### Store
//!
//! To put it all together, we use a store that keeps track of a state and provides an easy to use API for dispatching actions.
//! The store takes the reducer and an initial state.
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn async_test() {
//! # use redux_rs::Store;
//! # #[derive(Default)]
//! # struct State {
//! #     counter: i8
//! # }
//! #
//! # enum Action {
//! #     Increment,
//! #     Decrement
//! # }
//! #
//! # fn reducer(state: State, action: Action) -> State {
//! #     match action {
//! #         Action::Increment => State {
//! #             counter: state.counter + 1
//! #         },
//! #         Action::Decrement => State {
//! #             counter: state.counter - 1
//! #         }
//! #     }
//! # }
//! #
//! // The store needs to be mutable as it will change its inner state when dispatching actions.
//! let mut store = Store::new(reducer);
//!
//! // Let it do its highly complex math.
//! store.dispatch(Action::Increment).await;
//! store.dispatch(Action::Decrement).await;
//!
//! // Print the current count.
//! println!("{}", store.select(|state: &State| state.counter).await);
//! # };
//! ```
//!
//! ### Subscriptions
//!
//! Sometimes one might want to listen to changes happening. This is where subscriptions come in.
//! Subscriptions are callbacks with the current state that get called whenever an action gets dispatched.
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn async_test() {
//! # #[derive(Default)]
//! # struct State {
//! #     counter: i8
//! # }
//! #
//! # enum Action {
//! #     Increment,
//! #     Decrement
//! # }
//! #
//! # fn reducer(state: State, action: Action) -> State {
//! #     match action {
//! #         Action::Increment => State {
//! #             counter: state.counter + 1
//! #         },
//! #         Action::Decrement => State {
//! #             counter: state.counter - 1
//! #         }
//! #     }
//! # }
//! #
//! # let mut store = redux_rs::Store::new(reducer);
//! #
//! store.subscribe(|state: &State| {
//!      println!("Something changed! Current value: {}", state.counter);
//! }).await;
//! # }
//! ```

mod async_spawner;
mod middleware;
pub mod middlewares;
mod reducer;
mod selector;
mod store;
mod subscriber;

pub use middleware::{MiddleWare, StoreApi, StoreWithMiddleware};
pub use reducer::Reducer;
pub use selector::Selector;
pub use store::Store;
pub use subscriber::Subscriber;
