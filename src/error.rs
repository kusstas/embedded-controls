/// Controls errors container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<T, S> {
    // Error of [`ElapsedTimer`](timestamp_source::ElapsedTimer)
    ElapsedTimer(T),
    // Error of [`InputSwitch`](switch_hal::InputSwitch)
    InputSwitch(S),
}
