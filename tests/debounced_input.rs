mod common;

use crate::common::{MockClock, MockDuration, MockInputSwitch};

use embedded_controls::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};

struct TestDebouncedInputConfig;

impl DebouncedInputConfig for TestDebouncedInputConfig {
    type D = MockDuration;
    const DEBOUNCE_DURATION: MockDuration = MockDuration::new(3);
}

type TestDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, TestDebouncedInputConfig>;

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

    let mut clock = MockClock::new();
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..4 {
        assert_eq!(
            debounced_input.update(clock.now()),
            Ok(DebouncedInputEvent::Low)
        );
        assert_eq!(debounced_input.is_high(), false);
    }

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Rise)
    );
    assert_eq!(debounced_input.is_high(), true);

    for _ in 0..4 {
        assert_eq!(
            debounced_input.update(clock.now()),
            Ok(DebouncedInputEvent::High)
        );
        assert_eq!(debounced_input.is_high(), true);
    }

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Fall)
    );
    assert_eq!(debounced_input.is_high(), false);

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Low)
    );
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

    let mut clock = MockClock::new();
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..6 {
        assert_eq!(
            debounced_input.update(clock.now()),
            Ok(DebouncedInputEvent::Low)
        );
        assert_eq!(debounced_input.is_high(), false);
        assert_eq!(debounced_input.is_low(), true);
    }

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Rise)
    );
    assert_eq!(debounced_input.is_high(), true);
    assert_eq!(debounced_input.is_low(), false);

    for _ in 0..6 {
        assert_eq!(
            debounced_input.update(clock.now()),
            Ok(DebouncedInputEvent::High)
        );
        assert_eq!(debounced_input.is_high(), true);
        assert_eq!(debounced_input.is_low(), false);
    }

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Fall)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Low)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}

#[test]
fn debounced_input_error() {
    let state_results = [Err("Some error"), Ok(true)];

    let mut clock = MockClock::new();
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    assert_eq!(debounced_input.update(clock.now()), Err("Some error"));
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(
        debounced_input.update(clock.now()),
        Ok(DebouncedInputEvent::Low)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}
