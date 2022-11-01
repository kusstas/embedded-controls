#![no_std]

mod button;
mod debounced_input;
mod encoder;

pub use debounced_input::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
pub use encoder::{Encoder, EncoderConfig, EncoderEvent};

use core::fmt::Debug;

pub trait ElapsedTimer {
    type Error: Debug;
    type Timestamp: Clone;

    fn timeout(&self, from: &Self::Timestamp, to: &Self::Timestamp) -> Result<bool, Self::Error>;
}

pub trait Control {
    type Timestamp;
    type Event;
    type Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error>;
}
