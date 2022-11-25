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
embedded-controls = "0.1.3"
```

Usage in code:

```rust
use embedded_controls::{
    debounced_input_config,
    encoder_config,
    Control,
    DebouncedInput,
    DebouncedInputEvent,
    ElapsedTimer,
    Encoder,
    EncoderEvent,
};

pub struct MyElapsedTimer {
    duration: u32,
}

// You can implement this trait for embedded-time, cortex-m-rtic, and other time types
impl ElapsedTimer for MyElapsedTimer {
    type Error = ();
    type Timestamp = u32;

    fn is_timeout(
        &self,
        from: &Self::Timestamp,
        to: &Self::Timestamp,
    ) -> Result<bool, Self::Error> {
        if to >= from {
            Ok((to - from) >= self.duration)
        } else {
            Err(())
        }
    }
}

debounced_input_config!(
    MyDebouncedInputConfig,
    debounce_timer: MyElapsedTimer = MyElapsedTimer::new(30)
);

encoder_config!(
    MyEncoderConfig,
    debounce_timer: MyElapsedTimer = MyElapsedTimer::new(2),
    counts_div: i8 = 4
);

type MyDebouncedInput<Switch> = DebouncedInput<Switch, MyDebouncedInputConfig>;
type MyEncoder<SwitchA, SwitchB> = Encoder<SwitchA, SwitchB, MyEncoderConfig>;

fn main() {
    let pin_debounced_input; // Some pin for debounced input
    let pin_encoder_a; // Some pin for channel A of encoder
    let pin_encoder_b; // Some pin for channel B of encoder
    let clock; // Some clock instance

    let mut my_debounced_input = MyDebouncedInput::new(
        pin_debounced_input.into_active_low_switch()
    );

    let mut my_encoder = MyEncoder::new(
        pin_encoder_a.into_active_low_switch(),
        pin_encoder_b.into_active_low_switch(),
    );

    loop {
        match my_debounced_input.update(clock.now()).unwrap() {
            DebouncedInputEvent::Low => do_something_when_low(),
            DebouncedInputEvent::High => do_something_when_high(),
            DebouncedInputEvent::Rise => do_something_upon_rise(),
            DebouncedInputEvent::Fall => do_something_upon_fall(),
        }

        match encoder.update(clock.now()).unwrap() {
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
