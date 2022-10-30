use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent, Duration};

use core::{marker::PhantomData, ops::AddAssign};
use num::{Integer, One, Signed, Zero};
use switch_hal::InputSwitch;

pub trait EncoderConfig: DebouncedInputConfig {
    type Counter: AddAssign + Integer + Signed + Copy;
    const COUNTER_DIVIDER: Self::Counter;
}

pub struct Encoder<InputSwitchA, InputSwitchB, Config: EncoderConfig> {
    debounced_input_a: DebouncedInput<InputSwitchA, Config>,
    debounced_input_b: DebouncedInput<InputSwitchB, Config>,
    counter: <Config as EncoderConfig>::Counter,
    config: PhantomData<Config>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncoderEvent {
    NoTurn,
    LeftTurn,
    RightTurn,
}

impl<InputSwitchA, InputSwitchB, Config: EncoderConfig>
    Encoder<InputSwitchA, InputSwitchB, Config>
{
    pub fn new(input_switch_a: InputSwitchA, input_switch_b: InputSwitchB) -> Self {
        Encoder {
            debounced_input_a: DebouncedInput::new(input_switch_a),
            debounced_input_b: DebouncedInput::new(input_switch_b),
            counter: Zero::zero(),
            config: PhantomData::<Config>,
        }
    }

    pub fn release_input_switches(self) -> (InputSwitchA, InputSwitchB) {
        (
            self.debounced_input_a.release_input_switch(),
            self.debounced_input_b.release_input_switch(),
        )
    }
}

impl<SwtA, SwtB, Cfg: EncoderConfig> Control for Encoder<SwtA, SwtB, Cfg>
where
    SwtA: InputSwitch,
    SwtB: InputSwitch,
    <SwtA as InputSwitch>::Error: From<<SwtB as InputSwitch>::Error>,
{
    type Timestamp = <<Cfg as DebouncedInputConfig>::D as Duration>::Instant;
    type Event = EncoderEvent;
    type Error = <SwtA as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let a_event = self.debounced_input_a.update(now.clone())?;
        let b_event = self.debounced_input_b.update(now)?;

        fn check_event<Counter: Signed>(
            event: DebouncedInputEvent,
            antogonist_state: bool,
            counter_direct: Counter,
        ) -> Counter {
            match event {
                DebouncedInputEvent::Rise if antogonist_state => -counter_direct,
                DebouncedInputEvent::Rise => counter_direct,
                DebouncedInputEvent::Fall if antogonist_state => counter_direct,
                DebouncedInputEvent::Fall => -counter_direct,
                _ => Zero::zero(),
            }
        }

        let one = One::one();

        self.counter += check_event(a_event, self.debounced_input_b.is_high(), one);
        self.counter += check_event(b_event, self.debounced_input_a.is_high(), -one);

        Ok(
            if self.counter != Zero::zero() && self.counter % Cfg::COUNTER_DIVIDER == Zero::zero() {
                let turn = if self.counter.is_positive() {
                    EncoderEvent::RightTurn
                } else {
                    EncoderEvent::LeftTurn
                };

                self.counter = Zero::zero();
                turn
            } else {
                EncoderEvent::NoTurn
            },
        )
    }
}
