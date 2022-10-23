use embedded_controls::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};
use embedded_time::{duration::Milliseconds, Clock, Instant};

mod mocks;

use crate::mocks::{MockClock, MockInputSwitch};

struct TestDebouncedInputConfig;

impl DebouncedInputConfig for TestDebouncedInputConfig {
    type D = Milliseconds;
    const DEBOUNCE_DURATION: Milliseconds = Milliseconds(30_u32);
}

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
    let mut debounced_input =
        DebouncedInput::<_, Instant<MockClock>, TestDebouncedInputConfig>::new(input_switch);

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(Some(DebouncedInputEvent::Pressed))
    );

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(Some(DebouncedInputEvent::Released))
    );

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
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
    let mut debounced_input =
        DebouncedInput::<_, Instant<MockClock>, TestDebouncedInputConfig>::new(input_switch);

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(Some(DebouncedInputEvent::Pressed))
    );

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Ok(Some(DebouncedInputEvent::Released))
    );

    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
}

#[test]
fn debounced_input_error() {
    let state_results = [Err("Some error"), Ok(true)];

    let clock = MockClock;
    let input_switch = MockInputSwitch::new(&state_results);
    let mut debounced_input =
        DebouncedInput::<_, Instant<MockClock>, TestDebouncedInputConfig>::new(input_switch);

    assert_eq!(
        debounced_input.update(clock.try_now().unwrap()),
        Err("Some error")
    );
    assert_eq!(debounced_input.update(clock.try_now().unwrap()), Ok(None));
}
