use core::fmt::Debug;

pub trait Duration {
    type Error: Debug;
    type Instant: Clone;

    fn is_elapsed(&self, from: &Self::Instant, to: &Self::Instant) -> Result<bool, Self::Error>;
}
