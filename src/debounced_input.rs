use crate::{Control, Duration};

use core::marker::PhantomData;
use switch_hal::InputSwitch;

pub trait DebouncedInputConfig {
    type D: Duration;
    const DEBOUNCE_DURATION: Self::D;
}

pub struct DebouncedInput<InputSwitch, Config: DebouncedInputConfig> {
    input_switch: InputSwitch,
    disturbance_timestamp: Option<<<Config as DebouncedInputConfig>::D as Duration>::Instant>,
    state: bool,
    config: PhantomData<Config>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebouncedInputEvent {
    Low,
    High,
    Rise,
    Fall,
}

impl<InputSwitch, Config: DebouncedInputConfig> DebouncedInput<InputSwitch, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        DebouncedInput {
            input_switch,
            disturbance_timestamp: None,
            state: false,
            config: PhantomData::<Config>,
        }
    }

    pub fn is_high(&self) -> bool {
        self.state
    }

    pub fn is_low(&self) -> bool {
        !self.state
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        &self.input_switch
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.input_switch
    }
}

impl<Swt: InputSwitch, Cfg: DebouncedInputConfig> Control for DebouncedInput<Swt, Cfg> {
    type Timestamp = <<Cfg as DebouncedInputConfig>::D as Duration>::Instant;
    type Event = DebouncedInputEvent;
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let state = self.input_switch.is_active()?;

        if state != self.state {
            match &self.disturbance_timestamp {
                Some(disturbance_timestamp) => {
                    if Cfg::DEBOUNCE_DURATION
                        .is_elapsed(disturbance_timestamp, &now)
                        .unwrap()
                    {
                        self.state = state;
                        self.disturbance_timestamp = None;

                        return Ok(match self.state {
                            true => DebouncedInputEvent::Rise,
                            false => DebouncedInputEvent::Fall,
                        });
                    }
                }
                None => {
                    self.disturbance_timestamp = Some(now);
                }
            }
        } else {
            self.disturbance_timestamp = None;
        }

        Ok(match self.state {
            true => DebouncedInputEvent::High,
            false => DebouncedInputEvent::Low,
        })
    }
}
