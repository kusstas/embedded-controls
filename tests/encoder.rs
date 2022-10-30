use embedded_controls::{Control, DebouncedInputConfig, Encoder, EncoderConfig, EncoderEvent};

mod common;

use crate::common::{MockClock, MockDuration, MockInputSwitch};

struct TestEncoderConfig;

impl DebouncedInputConfig for TestEncoderConfig {
    type D = MockDuration;
    const DEBOUNCE_DURATION: MockDuration = MockDuration::new(1);
}

impl EncoderConfig for TestEncoderConfig {
    type Counter = i8;
    const COUNTER_DIVIDER: i8 = 4;
}

type TestEncoder<InputSwitchA, InputSwitchB> =
    Encoder<InputSwitchA, InputSwitchB, TestEncoderConfig>;

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

    let mut clock = MockClock::new();
    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder = TestEncoder::new(input_switch_a, input_switch_b);

    for _ in 0..4 {
        assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::RightTurn));

    for _ in 0..3 {
        assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::RightTurn));

    for _ in 0..6 {
        assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::LeftTurn));

    for _ in 0..3 {
        assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::LeftTurn));
}

#[test]
fn encoder_error() {
    let state_results_a = [Err("Some error 0"), Ok(true), Ok(true)];
    let state_results_b = [Err("Some error 1"), Ok(true)];

    let mut clock = MockClock::new();
    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder = TestEncoder::new(input_switch_a, input_switch_b);

    assert_eq!(encoder.update(clock.now()), Err("Some error 0"));
    assert_eq!(encoder.update(clock.now()), Err("Some error 1"));
    assert_eq!(encoder.update(clock.now()), Ok(EncoderEvent::NoTurn));
}
