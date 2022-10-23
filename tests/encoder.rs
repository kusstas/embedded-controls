use embedded_controls::{
    Control, DebouncedInputConfig, Encoder, EncoderConfig, EncoderCounter, EncoderEvent,
};
use embedded_time::{duration::Milliseconds, Clock, Instant};

mod mocks;

use crate::mocks::{MockClock, MockInputSwitch};

struct TestEncoderConfig;

impl DebouncedInputConfig for TestEncoderConfig {
    type D = Milliseconds;
    const DEBOUNCE_DURATION: Milliseconds = Milliseconds(10_u32);
}

impl EncoderConfig for TestEncoderConfig {
    const COUNTER_DIVIDER: EncoderCounter = 4;
}

#[test]
fn encoder_success() {
    let state_results_a = [
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        // revers direct
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    ];

    let state_results_b = [
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        // revers direct
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
    ];

    let clock = MockClock;
    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder =
        Encoder::<_, _, Instant<MockClock>, TestEncoderConfig>::new(input_switch_a, input_switch_b);

    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Ok(Some(EncoderEvent::RightTurn))
    );

    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Ok(Some(EncoderEvent::RightTurn))
    );

    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Ok(Some(EncoderEvent::LeftTurn))
    );

    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Ok(Some(EncoderEvent::LeftTurn))
    );
}

#[test]
fn encoder_error() {
    let state_results_a = [Err("Some error 0"), Ok(true), Ok(true)];
    let state_results_b = [Err("Some error 1"), Ok(true)];

    let clock = MockClock;
    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder =
        Encoder::<_, _, Instant<MockClock>, TestEncoderConfig>::new(input_switch_a, input_switch_b);

    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Err("Some error 0")
    );
    assert_eq!(
        encoder.update(clock.try_now().unwrap()),
        Err("Some error 1")
    );
    assert_eq!(encoder.update(clock.try_now().unwrap()), Ok(None));
}
