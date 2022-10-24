use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};

use core::{fmt::Debug, marker::PhantomData};
use embedded_time::{duration::Generic, Clock, Instant};
use switch_hal::InputSwitch;

pub struct Button<InputSwitch, Timestamp, Config: ?Sized> {
    debounced_input: DebouncedInput<InputSwitch, Timestamp, Config>,
    event_start_timestamp: Option<Timestamp>,
    hold: bool,
    config: PhantomData<Config>,
}

pub struct DoubleClickEventConfig<Duration> {
    pub max_press_duration: Duration,
    pub max_gap_duration: Duration,
}

pub struct HoldEventConfig<Duration> {
    pub capture_duration: Duration,
    pub period_duration: Duration,
}

pub trait ButtonConfig: DebouncedInputConfig {
    const DOUBLE_CLICK_EVENT: Option<DoubleClickEventConfig<Self::D>>;
    const HOLD_EVENT: Option<HoldEventConfig<Self::D>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    NoEvent,
    Pressed,
    Clicked,
    DoubleClicked,
    HoldStarted,
    Holding,
    HoldFinished,
}

impl<InputSwitch, Timestamp, Config: ?Sized> Button<InputSwitch, Timestamp, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        Button {
            debounced_input: DebouncedInput::new(input_switch),
            event_start_timestamp: None,
            hold: false,
            config: PhantomData::<Config>,
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.debounced_input.is_high()
    }

    pub fn is_released(&self) -> bool {
        self.debounced_input.is_low()
    }

    pub fn is_hold(&self) -> bool {
        self.hold
    }

    pub fn borrow_input_switch(&self) -> &InputSwitch {
        self.debounced_input.borrow_input_switch()
    }

    pub fn release_input_switch(self) -> InputSwitch {
        self.debounced_input.release_input_switch()
    }
}

impl<Swt, Clk: ?Sized, Cfg: ?Sized> Control for Button<Swt, Instant<Clk>, Cfg>
where
    Swt: InputSwitch,
    Clk: Clock,
    Cfg: ButtonConfig + DebouncedInputConfig,
    <Cfg as DebouncedInputConfig>::D: PartialOrd,
    <Cfg as DebouncedInputConfig>::D: TryFrom<Generic<<Clk as Clock>::T>>,
    <<Cfg as DebouncedInputConfig>::D as TryFrom<Generic<<Clk as Clock>::T>>>::Error: Debug,
{
    type Timestamp = Instant<Clk>;
    type Event = ButtonEvent;
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let debounced_event = self.debounced_input.update(now.clone())?;

        let elapsed: Option<<Cfg as DebouncedInputConfig>::D> = match &self.event_start_timestamp {
            Some(event_start_timestamp) => Some(
                event_start_timestamp
                    .checked_duration_until(&now)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
            None => None,
        };

        Ok(ButtonEvent::NoEvent)
    }
}
