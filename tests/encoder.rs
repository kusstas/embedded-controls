mod common;

use crate::common::{MockInputSwitch, MockTimestamp};

use embedded_controls::{encoder_config, Control, Encoder, EncoderEvent, Error};
use timestamp_source::Timer;

encoder_config!(
    TestEncoderConfig,
    debounce_timer: Timer<MockTimestamp> = Timer::new(1),
    counts_div: i8 = 4
);

type TestEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, TestEncoderConfig>;

#[test]
fn encoder_success() {
    let state_results_a = [
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

    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder = TestEncoder::new(input_switch_a, input_switch_b);

    for _ in 0..4 {
        assert_eq!(encoder.update(), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(), Ok(EncoderEvent::ClockwiseTurn));

    for _ in 0..3 {
        assert_eq!(encoder.update(), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(), Ok(EncoderEvent::ClockwiseTurn));

    for _ in 0..6 {
        assert_eq!(encoder.update(), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(), Ok(EncoderEvent::CounterClockwiseTurn));

    for _ in 0..3 {
        assert_eq!(encoder.update(), Ok(EncoderEvent::NoTurn));
    }

    assert_eq!(encoder.update(), Ok(EncoderEvent::CounterClockwiseTurn));
}

#[test]
fn encoder_error() {
    let state_results_a = [Ok(false), Err("Some error 0"), Ok(true), Ok(true)];
    let state_results_b = [Ok(true), Err("Some error 1"), Ok(true)];

    let input_switch_a = MockInputSwitch::new(&state_results_a);
    let input_switch_b = MockInputSwitch::new(&state_results_b);
    let mut encoder = TestEncoder::new(input_switch_a, input_switch_b);

    assert_eq!(encoder.update(), Err(Error::InputSwitch("Some error 0")));
    assert_eq!(encoder.update(), Err(Error::InputSwitch("Some error 1")));
    assert_eq!(encoder.update(), Ok(EncoderEvent::NoTurn));
}
