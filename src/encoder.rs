use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent, ElapsedTimer};

use core::{marker::PhantomData, ops::AddAssign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use switch_hal::InputSwitch;

pub trait EncoderConfig: DebouncedInputConfig {
    type Counter: AddAssign + Integer + Signed + Copy;
    const COUNTER_DIVIDER: Self::Counter;
}

pub struct Encoder<InputSwitchA, InputSwitchB, Config: EncoderConfig> {
    debounced_input_a: DebouncedInput<InputSwitchA, Config>,
    debounced_input_b: DebouncedInput<InputSwitchB, Config>,
    counter: Config::Counter,
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

impl<SwitchA: InputSwitch, SwitchB: InputSwitch, Config: EncoderConfig> Control
    for Encoder<SwitchA, SwitchB, Config>
where
    SwitchA::Error: From<SwitchB::Error>,
{
    type Timestamp = <Config::Timer as ElapsedTimer>::Timestamp;
    type Event = EncoderEvent;
    type Error = SwitchA::Error;

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

        let counter_direct = One::one();

        self.counter += check_event(a_event, self.debounced_input_b.is_high(), counter_direct);
        self.counter += check_event(b_event, self.debounced_input_a.is_high(), -counter_direct);

        let result_event =
            if !self.counter.is_zero() && (self.counter % Config::COUNTER_DIVIDER).is_zero() {
                let counter = self.counter;
                self.counter = Zero::zero();

                match counter.is_positive() {
                    true => EncoderEvent::RightTurn,
                    false => EncoderEvent::LeftTurn,
                }
            } else {
                EncoderEvent::NoTurn
            };

        Ok(result_event)
    }
}

#[macro_export]
macro_rules! encoder_config {
    (
        impl $config_name:ty,
        debounce_timer: $timer_type:ty = $timer_value:expr,
        counter_divider: $counter_type:ty = $divider_value:expr
    ) => {
        embedded_controls::debounced_input_config!(
            impl $config_name,
            debounce_timer: $timer_type = $timer_value
        );

        impl EncoderConfig for $config_name {
            type Counter = $counter_type;
            const COUNTER_DIVIDER: $counter_type = $divider_value;
        }
    };
    (
        $vis:vis $config_name:ident,
        debounce_timer: $timer_type:ty = $timer_value:expr,
        counter_divider: $counter_type:ty = $divider_value:expr
    ) => {
        $vis struct $config_name;

        encoder_config!(impl $config_name,
            debounce_timer: $timer_type = $timer_value,
            counter_divider: $counter_type = $divider_value
        );
    };
}
