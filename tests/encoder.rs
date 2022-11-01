mod common;

use crate::common::{MockClock, MockElapsedTimer, MockInputSwitch};

use embedded_controls::{
    encoder_config, Control, DebouncedInputConfig, Encoder, EncoderConfig, EncoderEvent,
};

encoder_config!(
    TestEncoderConfig,
    debounce_timer: MockElapsedTimer = MockElapsedTimer::new(1),
    counter_divider: i8 = 4
);

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
        // reverse direct
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
        // reverse direct
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
