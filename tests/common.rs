use std::{cell::RefCell, ops::AddAssign};

use switch_hal::InputSwitch;
use timestamp_source::Timestamp;

pub struct MockTimestamp {
    ticks: u32,
}

pub struct MockInputSwitch<'a> {
    state_results: &'a [Result<bool, &'static str>],
    index: RefCell<usize>,
}

impl Timestamp for MockTimestamp {
    type Duration = u32;
    type Error = ();

    fn now() -> Self {
        static mut TICKS: u32 = 0;

        unsafe {
            TICKS += 1;
            MockTimestamp { ticks: TICKS }
        }
    }

    fn duration_since_epoch(self) -> Self::Duration {
        self.ticks
    }

    fn duration_since(&self, other: &Self) -> Result<Self::Duration, Self::Error> {
        Ok(self.ticks - other.ticks)
    }
}

impl<'a> MockInputSwitch<'a> {
    pub fn new(state_results: &'a [Result<bool, &'static str>]) -> Self {
        MockInputSwitch {
            state_results,
            index: RefCell::new(Default::default()),
        }
    }

    pub fn next(&self) -> Result<bool, &'static str> {
        let state_result = self.state_results[*self.index.borrow() as usize].clone();

        self.index.try_borrow_mut().unwrap().add_assign(1);

        state_result
    }
}

impl<'a> InputSwitch for MockInputSwitch<'a> {
    type Error = &'static str;

    fn is_active(&self) -> Result<bool, Self::Error> {
        self.next()
    }
}
