use crate::{Control, ElapsedTimer};

use core::marker::PhantomData;
use switch_hal::InputSwitch;

/// Represents a config for [DebouncedInput](crate::DebouncedInput).
pub trait DebouncedInputConfig {
    /// Elapsed timer type that used for [DebouncedInput](crate::DebouncedInput).
    type Timer: ElapsedTimer;
    /// Elapsed timer instance that used for [DebouncedInput](crate::DebouncedInput).
    /// This timer is used for debounce input by timeout after disturbance start.
    const DEBOUNCE_TIMER: Self::Timer;
}

pub enum DebouncedInputState<T> {
    FixedLow,
    FixedHigh,
    RiseDisturbance(T),
    FallDisturbance(T),
}

pub struct DebouncedInput<InputSwitch, Config: DebouncedInputConfig> {
    input_switch: InputSwitch,
    state: DebouncedInputState<<Config::Timer as ElapsedTimer>::Timestamp>,
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

impl<Switch: InputSwitch, Config: DebouncedInputConfig> Control for DebouncedInput<Switch, Config> {
    type Timestamp = <Config::Timer as ElapsedTimer>::Timestamp;
    type Event = DebouncedInputEvent;
    type Error = <Switch as InputSwitch>::Error;

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
                } else if Config::DEBOUNCE_TIMER.is_timeout(start, &now).unwrap() {
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
                } else if Config::DEBOUNCE_TIMER.is_timeout(start, &now).unwrap() {
                    self.state = DebouncedInputState::FixedLow;
                    DebouncedInputEvent::Fall
                } else {
                    DebouncedInputEvent::High
                }
            }
        })
    }
}

#[macro_export]
macro_rules! debounced_input_config {
    (impl $config_name:ty, debounce_timer: $timer_type:ty = $timer_value:expr) => {
        impl DebouncedInputConfig for $config_name {
            type Timer = $timer_type;
            const DEBOUNCE_TIMER: $timer_type = $timer_value;
        }
    };
    ($vis:vis $config_name:ident, debounce_timer: $timer_type:ty = $timer_value:expr) => {
        $vis struct $config_name;

        debounced_input_config!(
            impl $config_name,
            debounce_timer: $timer_type = $timer_value
        );
    };
}
