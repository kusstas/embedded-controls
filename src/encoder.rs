use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent, ElapsedTimer};

use core::{marker::PhantomData, ops::AddAssign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use switch_hal::InputSwitch;

/// Represents a config for [`Encoder`](crate::Encoder).
pub trait EncoderConfig: DebouncedInputConfig {
    /// The type of counts counter.
    type Counts: AddAssign + Integer + Signed + Copy;

    /// The number of counts to register one turn of the encoder.
    const COUNTS_DIV: Self::Counts;
}

/// Concrete implementation of encoder.
///
/// # Type Params
/// `SwitchA` - [`InputSwitch`](switch_hal::InputSwitch) that provides input A channel.
///
/// `SwitchB` - [`InputSwitch`](switch_hal::InputSwitch) that provides input B channel.
///
/// `Config` - [`EncoderConfig`](crate::EncoderConfig) that provides configs for encoder.
///
/// # Example
/// ```ignore
/// encoder_config!(
///     SomeEncoderConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(2.millis()),
///     counts_div: i8 = 4
/// );
///
/// type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, SomeEncoderConfig>;
///
/// let mut clock = SysClock::new();
/// let mut encoder = MyEncoder::new(
///     pin_a.into_active_low_switch(),
///     pin_b.into_active_low_switch(),
/// );
///
/// loop {
///     match encoder.update(clock.now()).unwrap() {
///         EncoderEvent::NoTurn => do_something_when_no_turn(),
///         EncoderEvent::ClockwiseTurn => do_something_upon_clockwise_turn(),
///         EncoderEvent::CounterClockwiseTurn => do_something_upon_counter_clockwise_turn(),
///     }
/// }
/// ```
pub struct Encoder<SwitchA: InputSwitch, SwitchB: InputSwitch, Config: EncoderConfig> {
    debounced_input_a: DebouncedInput<SwitchA, Config>,
    debounced_input_b: DebouncedInput<SwitchB, Config>,
    counts: Config::Counts,
    config: PhantomData<Config>,
}

/// The event result of update [`Encoder`](crate::Encoder).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncoderEvent {
    /// Encoder doesn't rotate.
    NoTurn,
    /// Encoder rotates clockwise.
    ClockwiseTurn,
    /// Encoder rotates counter clockwise.
    CounterClockwiseTurn,
}

impl<SwitchA: InputSwitch, SwitchB: InputSwitch, Config: EncoderConfig>
    Encoder<SwitchA, SwitchB, Config>
{
    /// Creates a new [`Encoder<SwitchA, SwitchB, Config>`] from concretes `SwitchA`, `SwitchB`.
    pub fn new(input_switch_a: SwitchA, input_switch_b: SwitchB) -> Self {
        Encoder {
            debounced_input_a: DebouncedInput::new(input_switch_a),
            debounced_input_b: DebouncedInput::new(input_switch_b),
            counts: Zero::zero(),
            config: PhantomData::<Config>,
        }
    }

    /// Consumses `self` and release `(SwitchA, SwitchB)`.
    pub fn release_input_switches(self) -> (SwitchA, SwitchB) {
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

        fn check_event<Counts: Signed>(
            event: DebouncedInputEvent,
            antogonist_state: bool,
            direct: Counts,
        ) -> Counts {
            match event {
                DebouncedInputEvent::Rise if antogonist_state => -direct,
                DebouncedInputEvent::Rise => direct,
                DebouncedInputEvent::Fall if antogonist_state => direct,
                DebouncedInputEvent::Fall => -direct,
                _ => Zero::zero(),
            }
        }

        let direct = One::one();

        self.counts += check_event(a_event, self.debounced_input_b.is_high(), direct);
        self.counts += check_event(b_event, self.debounced_input_a.is_high(), -direct);

        let result_event = if !self.counts.is_zero() && (self.counts % Config::COUNTS_DIV).is_zero()
        {
            let counts = self.counts;
            self.counts = Zero::zero();

            match counts.is_positive() {
                true => EncoderEvent::ClockwiseTurn,
                false => EncoderEvent::CounterClockwiseTurn,
            }
        } else {
            EncoderEvent::NoTurn
        };

        Ok(result_event)
    }
}

/// Create a config for [`Encoder`](crate::Encoder).
///
/// # Example 1
/// ```ignore
/// encoder_config!(
///     SomeEncoderConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(2.millis()),
///     counts_div: i8 = 4
/// );
///
/// type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, SomeEncoderConfig>;
/// ```
///
/// # Example 2
/// ```ignore
/// encoder_config!(
///     pub SomeEncoderConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(2.millis()),
///     counts_div: i8 = 4
/// );
///
/// type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, SomeEncoderConfig>;
/// ```
///
/// # Example 3
/// ```ignore
/// pub struct SomeEncoderConfig;
///
/// encoder_config!(
///     impl SomeEncoderConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(2.millis()),
///     counts_div: i8 = 4
/// );
///
/// type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, SomeEncoderConfig>;
/// ```
#[macro_export]
macro_rules! encoder_config {
    (
        impl $config_name:ty,
        debounce_timer: $timer_type:ty = $timer_value:expr,
        counts_div: $counts_type:ty = $counts_div_value:expr
    ) => {
        embedded_controls::debounced_input_config!(
            impl $config_name,
            debounce_timer: $timer_type = $timer_value
        );

        impl EncoderConfig for $config_name {
            type Counts = $counts_type;
            const COUNTS_DIV: $counts_type = $counts_div_value;
        }
    };
    (
        $vis:vis $config_name:ident,
        debounce_timer: $timer_type:ty = $timer_value:expr,
        counts_div: $counts_type:ty = $counts_div_value:expr
    ) => {
        $vis struct $config_name;

        encoder_config!(impl $config_name,
            debounce_timer: $timer_type = $timer_value,
            counts_div: $counts_type = $counts_div_value
        );
    };
}
