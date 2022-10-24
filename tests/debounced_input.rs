use embedded_controls::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
use embedded_time::{duration::Milliseconds, Clock, Instant};

mod common;

use crate::common::{MockClock, MockInputSwitch};

struct TestDebouncedInputConfig;

impl DebouncedInputConfig for TestDebouncedInputConfig {
    type D = Milliseconds;
    const DEBOUNCE_DURATION: Milliseconds = Milliseconds(30_u32);
}

type TestDebouncedInput<InputSwitch> =
    DebouncedInput<InputSwitch, Instant<MockClock>, TestDebouncedInputConfig>;

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

    let clock = MockClock;
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..4 {
        assert_eq!(
            debounced_input.update(clock.try_now().unwrap()),
            Ok(DebouncedInputEvent::Low)
        );
        assert_eq!(debounced_input.is_high(), false);
    }

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Rise)
    );
    assert_eq!(debounced_input.is_high(), true);

    for _ in 0..4 {
        assert_eq!(
            debounced_input.update(clock.try_now().unwrap()),
            Ok(DebouncedInputEvent::High)
        );
        assert_eq!(debounced_input.is_high(), true);
    }

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Fall)
    );
    assert_eq!(debounced_input.is_high(), false);

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
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

    let clock = MockClock;
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    for _ in 0..6 {
        assert_eq!(
            debounced_input.update(clock.try_now().unwrap()),
            Ok(DebouncedInputEvent::Low)
        );
        assert_eq!(debounced_input.is_high(), false);
        assert_eq!(debounced_input.is_low(), true);
    }

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Rise)
    );
    assert_eq!(debounced_input.is_high(), true);
    assert_eq!(debounced_input.is_low(), false);

    for _ in 0..6 {
        assert_eq!(
            debounced_input.update(clock.try_now().unwrap()),
            Ok(DebouncedInputEvent::High)
        );
        assert_eq!(debounced_input.is_high(), true);
        assert_eq!(debounced_input.is_low(), false);
    }

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Fall)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Low)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}

#[test]
fn debounced_input_error() {
    let state_results = [Err("Some error"), Ok(true)];

    let clock = MockClock;
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input = TestDebouncedInput::new(input_switch);

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Err("Some error")
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(DebouncedInputEvent::Low)
    );
    assert_eq!(debounced_input.is_high(), false);
    assert_eq!(debounced_input.is_low(), true);
}
