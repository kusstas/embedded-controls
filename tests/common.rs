use std::{cell::RefCell, ops::AddAssign};

use embedded_time::{clock::Error, fraction::Fraction, Clock, Instant};
use switch_hal::InputSwitch;

#[derive(Debug)]
pub struct MockClock;

impl Clock for MockClock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(10, 1_000);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        static mut TICKS: u32 = 0;
        unsafe {
            TICKS += 1;
        }
        Ok(Instant::new(unsafe { TICKS }))
    }
}

pub struct MockInputSwitch<'a> {
    state_results: &'a [Result<bool, &'static str>],
    index: RefCell<usize>,
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
