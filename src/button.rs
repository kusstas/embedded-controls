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

pub enum ButtonState<D> {
    Idle,
    Undefined(D),
    Holding(D),
    DoubleClickCandidate(D),
}

pub struct Button<InputSwitch, Config: ButtonConfig> {
    debounced_input: DebouncedInput<InputSwitch, Config>,
    state: ButtonState<<<Config as DebouncedInputConfig>::D as Duration>::Instant>,
    config: PhantomData<Config>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    Idle,
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
            state: ButtonState::Idle,
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
        match self.state {
            ButtonState::Holding(_) => true,
            _ => false,
        }
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        self.debounced_input.borrow_input_switch()
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.debounced_input.release_input_switch()
    }
}

impl<Swt: InputSwitch, Cfg: ButtonConfig> Control for Button<Swt, Cfg> {
    type Timestamp = <<Cfg as DebouncedInputConfig>::D as Duration>::Instant;
    type Event = ButtonEvent;
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let debounced_input_event = self.debounced_input.update(now.clone())?;

        match debounced_input_event {
            DebouncedInputEvent::Low => todo!(),
            DebouncedInputEvent::High => todo!(),
            DebouncedInputEvent::Rise => todo!(),
            DebouncedInputEvent::Fall => todo!(),
        }

        Ok(ButtonEvent::Idle)
    }
}
