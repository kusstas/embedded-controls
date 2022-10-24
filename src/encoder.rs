use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};

use core::{fmt::Debug, marker::PhantomData};
use embedded_time::{duration::Generic, Clock, Instant};
use switch_hal::InputSwitch;

pub type EncoderCounter = i32;

pub struct Encoder<InputSwitchA, InputSwitchB, Time, Config: ?Sized> {
    debounced_input_a: DebouncedInput<InputSwitchA, Time, Config>,
    debounced_input_b: DebouncedInput<InputSwitchB, Time, Config>,
    counter: EncoderCounter,
    config: PhantomData<Config>,
}

pub trait EncoderConfig: DebouncedInputConfig {
    const COUNTER_DIVIDER: EncoderCounter;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncoderEvent {
    NoTurn,
    LeftTurn,
    RightTurn,
}

impl<InputSwitchA, InputSwitchB, Time, Config: ?Sized>
    Encoder<InputSwitchA, InputSwitchB, Time, Config>
{
    pub fn new(input_switch_a: InputSwitchA, input_switch_b: InputSwitchB) -> Self {
        Encoder {
            debounced_input_a: DebouncedInput::new(input_switch_a),
            debounced_input_b: DebouncedInput::new(input_switch_b),
            counter: Default::default(),
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

impl<SwtA, SwtB, Clk: ?Sized, Cfg: ?Sized> Control for Encoder<SwtA, SwtB, Instant<Clk>, Cfg>
where
    SwtA: InputSwitch,
    SwtB: InputSwitch,
    Clk: Clock,
    Cfg: EncoderConfig + DebouncedInputConfig,
    <Cfg as DebouncedInputConfig>::D: PartialOrd,
    <Cfg as DebouncedInputConfig>::D: TryFrom<Generic<<Clk as Clock>::T>>,
    <<Cfg as DebouncedInputConfig>::D as TryFrom<Generic<<Clk as Clock>::T>>>::Error: Debug,
    <SwtA as InputSwitch>::Error: From<<SwtB as InputSwitch>::Error>,
{
    type Timestamp = Instant<Clk>;
    type Event = EncoderEvent;
    type Error = <SwtA as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let a_event = self.debounced_input_a.update(now.clone())?;
        let b_event = self.debounced_input_b.update(now)?;

        fn check_event(
            event: DebouncedInputEvent,
            antogonist_state: bool,
            counter_direct: EncoderCounter,
        ) -> EncoderCounter {
            match event {
                DebouncedInputEvent::Rise if antogonist_state => -counter_direct,
                DebouncedInputEvent::Rise => counter_direct,
                DebouncedInputEvent::Fall if antogonist_state => counter_direct,
                DebouncedInputEvent::Fall => -counter_direct,
                _ => Default::default(),
            }
        }

        self.counter += check_event(a_event, self.debounced_input_b.is_high(), 1);
        self.counter += check_event(b_event, self.debounced_input_a.is_high(), -1);

        Ok(
            if self.counter != 0 && self.counter % Cfg::COUNTER_DIVIDER == 0 {
                let turn = if self.counter.is_positive() {
                    EncoderEvent::RightTurn
                } else {
                    EncoderEvent::LeftTurn
                };

                self.counter = 0;
                turn
            } else {
                EncoderEvent::NoTurn
            },
        )
    }
}
