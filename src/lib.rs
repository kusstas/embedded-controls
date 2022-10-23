#![no_std]

mod button;
mod debounced_input;
mod encoder;

pub use button::{
    Button, ButtonConfig, ButtonEvent, ClickCount, ClickEventConfig, HoldEventConfig,
};
pub use debounced_input::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
pub use encoder::{Encoder, EncoderConfig, EncoderCounter, EncoderEvent};

pub trait Control {
    type Timestamp;
    type Event;
    type Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error>;
}
