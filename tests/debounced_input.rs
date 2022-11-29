mod common;

use crate::common::{MockInputSwitch, MockTimestamp};

use embedded_controls::{
    debounced_input_config, Control, DebouncedInput, DebouncedInputEvent, Error,
};
use timestamp_source::Timer;

debounced_input_config!(
    TestDebouncedInputConfig,
    debounce_timer: Timer<MockTimestamp> = Timer::new(3)
);

type TestDebouncedInput<Switch> = DebouncedInput<Switch, TestDebouncedInputConfig>;

#[test]
fn debounced_input_success() {
    let state_results = [
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    ];

    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..3 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
        assert_eq!(debounced_input.is_high(), false);
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Rise));
    assert_eq!(debounced_input.is_high(), true);

    for _ in 0..4 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::High));
        assert_eq!(debounced_input.is_high(), true);
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Fall));
    assert_eq!(debounced_input.is_high(), false);

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert_eq!(debounced_input.is_high(), false);
}

#[test]
fn debounced_input_success_with_bounce() {
    let state_results = [
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    ];

    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..5 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
        assert_eq!(debounced_input.is_high(), false);
        assert_eq!(debounced_input.is_low(), true);
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Rise));
    assert_eq!(debounced_input.is_high(), true);
    assert_eq!(debounced_input.is_low(), false);

    for _ in 0..6 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::High));
        assert_eq!(debounced_input.is_high(), true);
        assert_eq!(debounced_input.is_low(), false);
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Fall));
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}

#[test]
fn debounced_input_error() {
    let state_results = [Ok(false), Err("Some error"), Ok(true)];

    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    assert_eq!(
        debounced_input.update(),
        Err(Error::InputSwitch("Some error"))
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}
