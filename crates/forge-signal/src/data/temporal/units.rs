use std::marker::PhantomData;
use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;

fn reject_zero(name: &str) -> SignalError {
    SignalError::invalid_input(format!("{name} must be greater than zero"))
}

/// Generic validated positive millisecond proof shared by temporal duration families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PositiveMilliseconds<Tag> {
    milliseconds: NonZeroU64,
    #[serde(skip)]
    _tag: PhantomData<Tag>,
}

impl<Tag> PositiveMilliseconds<Tag> {
    pub fn new(name: &'static str, milliseconds: u64) -> Result<Self, SignalError> {
        let milliseconds = NonZeroU64::new(milliseconds).ok_or_else(|| reject_zero(name))?;
        Ok(Self {
            milliseconds,
            _tag: PhantomData,
        })
    }

    pub fn get(self) -> u64 {
        self.milliseconds.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TemporalDurationTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IntervalPeriodTag;

/// Validated positive temporal duration used by first-class runtime policies.
pub type TemporalDuration = PositiveMilliseconds<TemporalDurationTag>;

impl PositiveMilliseconds<TemporalDurationTag> {
    pub fn temporal_duration(milliseconds: u64) -> Result<Self, SignalError> {
        Self::new("temporal duration", milliseconds)
    }
}

/// Validated positive period for recurring interval policies.
pub type IntervalPeriod = PositiveMilliseconds<IntervalPeriodTag>;

impl PositiveMilliseconds<IntervalPeriodTag> {
    pub fn interval_period(milliseconds: u64) -> Result<Self, SignalError> {
        Self::new("interval period", milliseconds)
    }
}
