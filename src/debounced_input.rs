use crate::{Control, Duration};

use core::marker::PhantomData;
use switch_hal::InputSwitch;

pub trait DebouncedInputConfig {
    type D: Duration;
    const DEBOUNCE_DURATION: Self::D;
}

pub enum DebouncedInputState<D> {
    FixedLow,
    FixedHigh,
    RiseDisturbance(D),
    FallDisturbance(D),
}

pub struct DebouncedInput<InputSwitch, Config: DebouncedInputConfig> {
    input_switch: InputSwitch,
    state: DebouncedInputState<<<Config as DebouncedInputConfig>::D as Duration>::Instant>,
    config: PhantomData<Config>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebouncedInputEvent {
    Low,
    High,
    Rise,
    Fall,
}

impl<InputSwitch, Config: DebouncedInputConfig> DebouncedInput<InputSwitch, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        DebouncedInput {
            input_switch,
            state: DebouncedInputState::FixedLow,
            config: PhantomData::<Config>,
        }
    }

    pub fn is_high(&self) -> bool {
        match self.state {
            DebouncedInputState::FixedLow | DebouncedInputState::RiseDisturbance(_) => false,
            DebouncedInputState::FixedHigh | DebouncedInputState::FallDisturbance(_) => true,
        }
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        &self.input_switch
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.input_switch
    }
}

impl<Swt: InputSwitch, Cfg: DebouncedInputConfig> Control for DebouncedInput<Swt, Cfg> {
    type Timestamp = <<Cfg as DebouncedInputConfig>::D as Duration>::Instant;
    type Event = DebouncedInputEvent;
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let input_switch_state = self.input_switch.is_active()?;

        Ok(match &self.state {
            DebouncedInputState::FixedLow => {
                if input_switch_state {
                    self.state = DebouncedInputState::RiseDisturbance(now)
                }
                DebouncedInputEvent::Low
            }
            DebouncedInputState::FixedHigh => {
                if !input_switch_state {
                    self.state = DebouncedInputState::FallDisturbance(now)
                }
                DebouncedInputEvent::High
            }
            DebouncedInputState::RiseDisturbance(start) => {
                if !input_switch_state {
                    self.state = DebouncedInputState::FixedLow;
                    DebouncedInputEvent::Low
                } else if Cfg::DEBOUNCE_DURATION.is_elapsed(start, &now).unwrap() {
                    self.state = DebouncedInputState::FixedHigh;
                    DebouncedInputEvent::Rise
                } else {
                    DebouncedInputEvent::Low
                }
            }
            DebouncedInputState::FallDisturbance(start) => {
                if input_switch_state {
                    self.state = DebouncedInputState::FixedHigh;
                    DebouncedInputEvent::High
                } else if Cfg::DEBOUNCE_DURATION.is_elapsed(start, &now).unwrap() {
                    self.state = DebouncedInputState::FixedLow;
                    DebouncedInputEvent::Fall
                } else {
                    DebouncedInputEvent::High
                }
            }
        })
    }
}
