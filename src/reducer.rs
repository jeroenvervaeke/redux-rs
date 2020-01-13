/// Function signature for a reducer.
///
///
///  A *reducer* is a function that calculates a new state given the previous
///  state and an action. They must be *pure functions*—functions that return
///  the exact same output for given inputs. These functions should not cause
///  any side-effects.
///
///
///
///
/// # Example
///
/// ```
/// # use redux_rs::Reducer;
/// #
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// let reducer: dyn Reducer<u8, Action> = |state: &u8, action: &Action| -> u8 {
///     match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1
///     }
/// };
/// ```
pub trait Reducer<S, A> {
    fn reduce(&self, state: S, action: &A) -> S;
}

impl<S, A, F> Reducer<S, A> for F
where
    F: Fn(S, &A) -> S,
{
    fn reduce(&self, state: S, action: &A) -> S {
        self(state, action)
    }
}
