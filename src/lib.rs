//! Embedded controls library is based on [switch-hal](https://crates.io/crates/switch-hal)
//! that allows to handle primitive controls like [`DebounceInput`](crate::DebouncedInput),
//! [`Encoder`](crate::Encoder).
//! [`Controls`](crate::Control) are updating by [`timestamps`](crate::Control::Timestamp)
//! that passed to [`update`](crate::Control::update) and return [`event`](crate::Control::Event)
//! or [`error`](crate::Control::Error).

#![no_std]

mod button;
mod debounced_input;
mod encoder;

pub use debounced_input::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
pub use encoder::{Encoder, EncoderConfig, EncoderEvent};

use core::fmt::Debug;

/// Represents an elapsed timer that used for configs.
/// # Example
/// ```
/// # use embedded_controls::ElapsedTimer;
/// pub struct MyElapsedTimer {
///     duration: u32,
/// }
///
/// impl ElapsedTimer for MyElapsedTimer {
///     type Error = ();
///     type Timestamp = u32;
///
///     fn is_timeout(
///         &self,
///         from: &Self::Timestamp,
///         to: &Self::Timestamp,
///     ) -> Result<bool, Self::Error> {
///         if to >= from {
///             Ok((to - from) >= self.duration)
///         } else {
///             Err(())
///         }
///     }
/// }
/// ```
pub trait ElapsedTimer {
    type Error: Debug;
    type Timestamp: Clone;

    /// Returns true if a timer duration is more(equal) then duration between from-to timestamps,
    /// otherwise false.
    /// # Errors
    /// This function will return an error if duration between from-to timestamps is negative.
    fn is_timeout(&self, from: &Self::Timestamp, to: &Self::Timestamp)
        -> Result<bool, Self::Error>;
}

/// Represents a control, such as debounced input, button, encoder and etc.
pub trait Control {
    type Timestamp;
    type Event;
    type Error;

    /// Update a control and return an current event or error after update.
    ///
    /// `now` - the current timestamp upon `update` invoke.
    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error>;
}
