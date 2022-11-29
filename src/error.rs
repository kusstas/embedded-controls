/// Controls errors container.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error<T, S> {
    // Error of [`ElapsedTimer`](timestamp_source::ElapsedTimer)
    ElapsedTimer(T),
    // Error of [`InputSwitch`](switch_hal::InputSwitch)
    InputSwitch(S),
}

/// Propagate error from components error to [`Error`](crate::Error)
#[macro_export]
macro_rules! prop_error {
    ($expr:expr, $err_type:path) => {
        match $expr {
            Ok(ok_value) => ok_value,
            Err(err_value) => return Err($err_type(err_value)),
        }
    };
}
