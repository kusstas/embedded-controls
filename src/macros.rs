/// Create a config for [`DebouncedInput`](crate::DebouncedInput).
///
/// # Example 1
/// ```ignore
/// debounced_input_config!(
///     SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
///
/// # Example 2
/// ```ignore
/// debounced_input_config!(
///     pub SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
///
/// # Example 3
/// ```ignore
/// pub struct SomeDebouncedInputConfig;
///
/// debounced_input_config!(
///     impl SomeDebouncedInputConfig,
///     debounce_timer: MyElapsedTimer = MyElapsedTimer::new(20.millis())
/// );
///
/// type MyDebouncedInput<InputSwitch> = DebouncedInput<InputSwitch, SomeDebouncedInputConfig>;
/// ```
#[macro_export]
macro_rules! debounced_input_config {
    (impl $config_name:ty, debounce_timer: $timer_type:ty = $timer_value:expr) => {
        impl embedded_controls::DebouncedInputConfig for $config_name {
            type Timer = $timer_type;
            const DEBOUNCE_TIMER: $timer_type = $timer_value;
        }
    };
    ($vis:vis $config_name:ident, debounce_timer: $timer_type:ty = $timer_value:expr) => {
        $vis struct $config_name;

        debounced_input_config!(
            impl $config_name,
            debounce_timer: $timer_type = $timer_value
        );
    };
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

        impl embedded_controls::EncoderConfig for $config_name {
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
