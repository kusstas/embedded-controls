//! Embedded controls library is based on [switch-hal](https://crates.io/crates/switch-hal)
//! that allows to handle primitive controls like [`DebounceInput`](crate::DebouncedInput),
//! [`Encoder`](crate::Encoder).
//! [`Controls`](crate::Control) are updating by [`timestamps`](crate::Control::Timestamp)
//! that passed to [`update`](crate::Control::update) and return [`event`](crate::Control::Event)
//! or [`error`](crate::Control::Error).

#![no_std]

mod debounced_input;
mod encoder;
mod error;

pub mod macros;

pub use debounced_input::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
pub use encoder::{Encoder, EncoderConfig, EncoderEvent};
pub use error::Error;

/// Represents a control, such as debounced input, button, encoder and etc.
pub trait Control {
    type Event;
    type Error;

    /// Update a control and return an current event or error after update.
    fn update(&mut self) -> Result<Self::Event, Self::Error>;
}
