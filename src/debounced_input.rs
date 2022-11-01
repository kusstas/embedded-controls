use crate::{Control, ElapsedTimer};

use core::marker::PhantomData;
use switch_hal::InputSwitch;

/// Represents a config for [`DebouncedInput`](crate::DebouncedInput).
pub trait DebouncedInputConfig {
    /// Elapsed timer type that used for [`DebouncedInput`](crate::DebouncedInput).
    type Timer: ElapsedTimer;

    /// Elapsed timer instance that used for [`DebouncedInput`](crate::DebouncedInput).
    /// This timer is used for debounce of input by timeout after disturbance start.
    const DEBOUNCE_TIMER: Self::Timer;
}

/// The state machine of [`DebouncedInput`](crate::DebouncedInput).
pub enum DebouncedInputState<T> {
    FixedLow,
    FixedHigh,
    RiseDisturbance(T),
    FallDisturbance(T),
}

/// Concrete implementation of debounced input.
///
/// # Type Params
/// `Switch` - [`InputSwitch`](switch_hal::InputSwitch) that provides input for debouncing.
///
/// `Config` - [`DebouncedInputConfig`](crate::DebouncedInputConfig) that provides configs for debouncing.
///
/// # Example
/// ```ignore
/// debounced_input_config!(
///     SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<Switch> = DebouncedInput<Switch, SomeDebouncedInputConfig>;
///
/// let mut clock = SysClock::new();
/// let mut debounced_input = MyDebouncedInput::new(pin.into_active_low_switch());
///
/// loop {
///     match debounced_input.update(clock.now()).unwrap() {
///         DebouncedInputEvent::Low => do_something_when_low(),
///         DebouncedInputEvent::High => do_something_when_high(),
///         DebouncedInputEvent::Rise => do_something_upon_rise(),
///         DebouncedInputEvent::Fall => do_something_upon_fall(),
///     }
/// }
/// ```
pub struct DebouncedInput<Switch: InputSwitch, Config: DebouncedInputConfig> {
    input_switch: Switch,
    state: DebouncedInputState<<Config::Timer as ElapsedTimer>::Timestamp>,
    config: PhantomData<Config>,
}

/// The event result of update [`DebouncedInput`](crate::DebouncedInput).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebouncedInputEvent {
    /// Stable low state, the input is inactive.
    Low,
    /// Stable high state, the input is active.
    High,
    /// Rise event, the input is rised from inactive to active state.
    Rise,
    /// Fall event, the input is rised from active to inactive state.
    Fall,
}

impl<Switch: InputSwitch, Config: DebouncedInputConfig> DebouncedInput<Switch, Config> {
    /// Creates a new [`DebouncedInput<Switch, Config>`] from a concrete `Switch`.
    ///
    /// `input_switch` - an concrete instance of `Switch`.
    pub fn new(input_switch: Switch) -> Self {
        DebouncedInput {
            input_switch,
            state: DebouncedInputState::FixedLow,
            config: PhantomData::<Config>,
        }
    }

    /// Returns the is stable high state.
    pub fn is_high(&self) -> bool {
        match self.state {
            DebouncedInputState::FixedLow | DebouncedInputState::RiseDisturbance(_) => false,
            DebouncedInputState::FixedHigh | DebouncedInputState::FallDisturbance(_) => true,
        }
    }

    /// Returns the is stable low state.
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Borrow `Switch`.
    pub fn borrow_input_switch(&self) -> &Switch {
        &self.input_switch
    }

    /// Consumes `self` and release `Switch`.
    pub fn release_input_switch(self) -> Switch {
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

/// Create a config for [`DebouncedInput`](crate::DebouncedInput).
///
/// # Example 1
/// ```ignore
/// debounced_input_config!(
///     SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
///
/// # Example 2
/// ```ignore
/// debounced_input_config!(
///     pub SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
///
/// # Example 3
/// ```ignore
/// pub struct SomeDebouncedInputConfig;
///
/// debounced_input_config!(
///     impl SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
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
