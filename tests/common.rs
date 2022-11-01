use std::{cell::RefCell, ops::AddAssign};

use embedded_controls::ElapsedTimer;
use switch_hal::InputSwitch;

pub struct MockClock {
    counter: u32,
}

pub struct MockElapsedTimer {
    duration: u32,
}

pub struct MockInputSwitch<'a> {
    state_results: &'a [Result<bool, &'static str>],
    index: RefCell<usize>,
}

impl MockClock {
    pub fn new() -> Self {
        MockClock {
            counter: Default::default(),
        }
    }

    pub fn now(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

impl MockElapsedTimer {
    pub const fn new(duration: u32) -> Self {
        MockElapsedTimer { duration }
    }
}

impl ElapsedTimer for MockElapsedTimer {
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
