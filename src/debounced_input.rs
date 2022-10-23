use crate::Control;

use core::{fmt::Debug, marker::PhantomData};
use embedded_time::{duration::Generic, Clock, Instant};
use switch_hal::InputSwitch;

pub struct DebouncedInput<InputSwitch, Timestamp, Config: ?Sized> {
    input_switch: InputSwitch,
    disturbance_timestamp: Option<Timestamp>,
    state: bool,
    config: PhantomData<Config>,
}

pub trait DebouncedInputConfig {
    type D;
    const DEBOUNCE_DURATION: Self::D;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebouncedInputEvent {
    Pressed,
    Released,
}

impl<InputSwitch, Timestamp, Config: ?Sized> DebouncedInput<InputSwitch, Timestamp, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        DebouncedInput {
            input_switch,
            disturbance_timestamp: None,
            state: false,
            config: PhantomData::<Config>,
        }
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        &self.input_switch
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.input_switch
    }
}

impl<Swt, Clk: ?Sized, Cfg: ?Sized> Control for DebouncedInput<Swt, Instant<Clk>, Cfg>
where
    Swt: InputSwitch,
    Clk: Clock,
    Cfg: DebouncedInputConfig,
    <Cfg as DebouncedInputConfig>::D: PartialOrd,
    <Cfg as DebouncedInputConfig>::D: TryFrom<Generic<<Clk as Clock>::T>>,
    <<Cfg as DebouncedInputConfig>::D as TryFrom<Generic<<Clk as Clock>::T>>>::Error: Debug,
{
    type Timestamp = Instant<Clk>;
    type Event = Option<DebouncedInputEvent>;
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let state = self.input_switch.is_active()?;

        if state != self.state {
            match &self.disturbance_timestamp {
                Some(disturbance_timestamp) => {
                    let elapsed: Cfg::D = disturbance_timestamp
                        .checked_duration_until(&now)
                        .unwrap()
                        .try_into()
                        .unwrap();

                    if elapsed >= Cfg::DEBOUNCE_DURATION {
                        self.state = state;
                        self.disturbance_timestamp = None;

                        return Ok(Some(if self.state {
                            DebouncedInputEvent::Pressed
                        } else {
                            DebouncedInputEvent::Released
                        }));
                    }
                }
                None => {
                    self.disturbance_timestamp = Some(now);
                }
            }
        } else {
            self.disturbance_timestamp = None;
        }

        Ok(None)
    }
}
