/// Function signature for a reducer.
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
/// let reducer: Reducer<u8, Action> = |state: &u8, action: &Action| -> u8 {
///     match action {
///         Action::Increment => state + 1,
///         Action::Decrement => state - 1
///     }
/// };
/// ```
pub type Reducer<State, Action> = fn(&State, &Action) -> State;