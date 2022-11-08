use crate::{DebouncedInput, DebouncedInputConfig, DebouncedInputEvent, ElapsedTimer};

use switch_hal::InputSwitch;

pub trait ButtonConfig: DebouncedInputConfig {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    Idle,
    Pressed,
    Clicked,
    DoubleClicked,
    HoldStarted,
    HoldIdle,
    HoldTick,
    HoldFinished,
}

pub enum ButtonState<T> {
    Idle,
    Pressed(T),
    Hold(T),
    DoubleClick(T),
}

/// Concrete implementation of button.
///
/// # Type Params
/// `Switch` - [`InputSwitch`](switch_hal::InputSwitch) that provides input for button.
///
/// `Config` - [`ButtonConfig`](crate::ButtonConfig) that provides configs for button.
///
pub struct Button<Switch: InputSwitch, Config: DebouncedInputConfig> {
    debounced_input: DebouncedInput<Switch, Config>,
    state: ButtonState<<Config::Timer as ElapsedTimer>::Timestamp>,
}

impl<Switch: InputSwitch, Config: DebouncedInputConfig> Button<Switch, Config> {
    /// Creates a new [`Button<Switch, Config>`] from a concrete `Switch`.
    ///
    /// `input_switch` - an concrete instance of `Switch`.
    pub fn new(input_switch: Switch) -> Self {
        Button {
            debounced_input: DebouncedInput::new(input_switch),
            state: ButtonState::Idle,
        }
    }

    /// Returns the is pressed of this [`Button<Switch, Config>`].
    pub fn is_pressed(&self) -> bool {
        self.debounced_input.is_high()
    }

    /// Returns the is released of this [`Button<Switch, Config>`].
    pub fn is_released(&self) -> bool {
        self.debounced_input.is_low()
    }

    /// Returns the is holding of this [`Button<Switch, Config>`].
    pub fn is_holding(&self) -> bool {
        match self.state {
            ButtonState::Hold(_) => true,
            _ => false,
        }
    }

    /// Borrow `Switch`.
    pub fn borrow_input_switch(&self) -> &Switch {
        self.debounced_input.borrow_input_switch()
    }

    /// Consumes `self` and release `Switch`.
    pub fn release_input_switch(self) -> Switch {
        self.debounced_input.release_input_switch()
    }
}
