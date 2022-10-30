#![no_std]

mod button;
mod debounced_input;
mod duration;
mod encoder;

pub use button::{Button, ButtonConfig, ButtonEvent, DoubleClickEventConfig, HoldEventConfig};
pub use debounced_input::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
pub use duration::Duration;
pub use encoder::{Encoder, EncoderConfig, EncoderEvent};

pub trait Control {
    type Timestamp;
    type Event;
    type Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error>;
}
