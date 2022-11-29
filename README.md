# embedded-controls

![Rust](https://github.com/kusstas/embedded-controls/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/d/embedded-controls.svg)](https://crates.io/crates/embedded-controls)
[![crates.io](https://img.shields.io/crates/v/embedded-controls.svg)](https://crates.io/crates/embedded-controls)
[![docs.rs](https://docs.rs/embedded-controls/badge.svg)](https://docs.rs/embedded-controls)

Embedded controls is a `no_std` embedded Rust library for working with controls like buttons, encoders and etc.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
embedded-controls = "0.1.4"
```

Usage in code:

```rust
use embedded_controls::{
    debounced_input_config,
    encoder_config,
    Control,
    DebouncedInput,
    DebouncedInputEvent,
    Encoder,
    EncoderEvent,
};
use timestamp_source::Timer;

debounced_input_config!(
    MyDebouncedInputConfig,
    debounce_timer: Timer<SomeTimestamp> = Timer::new(30.millis())
);

encoder_config!(
    MyEncoderConfig,
    debounce_timer: Timer<SomeTimestamp> = Timer::new(2.millis())
    counts_div: i8 = 4
);

type MyDebouncedInput<Switch> = DebouncedInput<Switch, MyDebouncedInputConfig>;
type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, MyEncoderConfig>;

fn main() {
    let pin_debounced_input; // Some pin for debounced input
    let pin_encoder_a; // Some pin for channel A of encoder
    let pin_encoder_b; // Some pin for channel B of encoder

    let mut my_debounced_input = MyDebouncedInput::new(
        pin_debounced_input.into_active_low_switch()
    );

    let mut my_encoder = MyEncoder::new(
        pin_encoder_a.into_active_low_switch(),
        pin_encoder_b.into_active_low_switch(),
    );

    loop {
        match my_debounced_input.update().unwrap() {
            DebouncedInputEvent::Low => do_something_when_low(),
            DebouncedInputEvent::High => do_something_when_high(),
            DebouncedInputEvent::Rise => do_something_upon_rise(),
            DebouncedInputEvent::Fall => do_something_upon_fall(),
        }

        match encoder.update().unwrap() {
            EncoderEvent::NoTurn => do_something_when_no_turn(),
            EncoderEvent::ClockwiseTurn => do_something_upon_clockwise_turn(),
            EncoderEvent::CounterClockwiseTurn => do_something_upon_counter_clockwise_turn(),
        }
    }
}
```

## Documentation

https://docs.rs/crate/embedded-controls

or build a local copy

```sh
cargo docs
```

and open `target/doc/embedded_controls/index.html` in your browser.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
