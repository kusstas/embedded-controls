use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent, Duration};

use core::{fmt::Debug, marker::PhantomData};
use switch_hal::InputSwitch;

pub struct DoubleClickEventConfig<Duration> {
    pub max_press_duration: Duration,
    pub max_gap_duration: Duration,
}

pub struct HoldEventConfig<Duration> {
    pub capture_duration: Duration,
    pub period_duration: Duration,
}

pub trait ButtonConfig: DebouncedInputConfig {
    const DOUBLE_CLICK_EVENT: Option<DoubleClickEventConfig<Self::D>>;
    const HOLD_EVENT: Option<HoldEventConfig<Self::D>>;
}

pub struct Button<InputSwitch, Config: ButtonConfig> {
    debounced_input: DebouncedInput<InputSwitch, Config>,
    event_start_timestamp: Option<<<Config as DebouncedInputConfig>::D as Duration>::Instant>,
    double_click_candidate: bool,
    hold: bool,
    config: PhantomData<Config>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    Nothing,
    Pressed,
    Clicked,
    DoubleClicked,
    HoldStarted,
    Holding,
    HoldFinished,
}

impl<Duration> DoubleClickEventConfig<Duration> {
    pub const fn new(max_press_duration: Duration, max_gap_duration: Duration) -> Self {
        DoubleClickEventConfig {
            max_press_duration,
            max_gap_duration,
        }
    }
}

impl<Duration> HoldEventConfig<Duration> {
    pub const fn new(capture_duration: Duration, period_duration: Duration) -> Self {
        HoldEventConfig {
            capture_duration,
            period_duration,
        }
    }
}

impl<InputSwitch, Config: ButtonConfig> Button<InputSwitch, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        Button {
            debounced_input: DebouncedInput::new(input_switch),
            event_start_timestamp: None,
            double_click_candidate: false,
            hold: false,
            config: PhantomData::<Config>,
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.debounced_input.is_high()
    }

    pub fn is_released(&self) -> bool {
        self.debounced_input.is_low()
    }

    pub fn is_holding(&self) -> bool {
        self.hold
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        self.debounced_input.borrow_input_switch()
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.debounced_input.release_input_switch()
    }
}
