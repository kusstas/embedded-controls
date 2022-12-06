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
        assert!(!debounced_input.is_high());
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Rise));
    assert!(debounced_input.is_high());

    for _ in 0..4 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::High));
        assert!(debounced_input.is_high());
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Fall));
    assert!(!debounced_input.is_high());

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert!(!debounced_input.is_high());
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
        assert!(!debounced_input.is_high());
        assert!(debounced_input.is_low());
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Rise));
    assert!(debounced_input.is_high());
    assert!(!debounced_input.is_low());

    for _ in 0..6 {
        assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::High));
        assert!(debounced_input.is_high());
        assert!(!debounced_input.is_low());
    }

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Fall));
    assert!(!debounced_input.is_high());
    assert!(debounced_input.is_low());

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert!(!debounced_input.is_high());
    assert!(debounced_input.is_low());
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

    assert!(!debounced_input.is_high());
    assert!(debounced_input.is_low());

    assert_eq!(debounced_input.update(), Ok(DebouncedInputEvent::Low));
    assert!(!debounced_input.is_high());
    assert!(debounced_input.is_low());
}
