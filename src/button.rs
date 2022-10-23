use crate::{Control, DebouncedInput, DebouncedInputConfig, DebouncedInputEvent};

use core::{fmt::Debug, marker::PhantomData};
use embedded_time::{duration::Generic, Clock, Instant};
use switch_hal::InputSwitch;

pub type ClickCount = u8;

pub struct Button<InputSwitch, Timestamp, Config: ?Sized> {
    debounced_input: DebouncedInput<InputSwitch, Timestamp, Config>,
    event_start_timestamp: Option<Timestamp>,
    press_count: ClickCount,
    hold: bool,
    config: PhantomData<Config>,
}

pub struct ClickEventConfig<Duration> {
    pub max_press_duration: Duration,
    pub max_gap_duration: Duration,
    pub max_count: ClickCount,
}

pub struct HoldEventConfig<Duration> {
    pub capture_duration: Duration,
    pub period_duration: Duration,
}

pub trait ButtonConfig: DebouncedInputConfig {
    const CLICK_EVENT: Option<ClickEventConfig<Self::D>>;
    const HOLD_EVENT: Option<HoldEventConfig<Self::D>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonEvent {
    Clicked(ClickCount),
    HoldStarted,
    Holding,
    HoldFinished,
}

impl<InputSwitch, Timestamp, Config: ?Sized> Button<InputSwitch, Timestamp, Config> {
    pub fn new(input_switch: InputSwitch) -> Self {
        Button {
            debounced_input: DebouncedInput::new(input_switch),
            event_start_timestamp: None,
            press_count: Default::default(),
            hold: false,
            config: PhantomData::<Config>,
        }
    }

    pub fn get_pressed(&self) -> bool {
        self.debounced_input.get_state()
    }

    pub fn get_released(&self) -> bool {
        !self.get_pressed()
    }

    pub fn get_hold(&self) -> bool {
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
    type Event = (Option<DebouncedInputEvent>, Option<ButtonEvent>);
    type Error = <Swt as InputSwitch>::Error;

    fn update(&mut self, now: Self::Timestamp) -> Result<Self::Event, Self::Error> {
        let debounced_event = self.debounced_input.update(now.clone())?;

        match &debounced_event {
            Some(event) => match event {
                DebouncedInputEvent::Pressed => {}
                DebouncedInputEvent::Released => {}
            },
            None => {}
        }

        Ok((debounced_event, None))
    }
}
