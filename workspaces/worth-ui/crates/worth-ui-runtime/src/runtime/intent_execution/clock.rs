/// One caller-observed reading in the monotonic clock domain used only by
/// framework-managed intent execution.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiIntentExecutionClockReading {
    tick: u64,
}

/// A requested absolute execution deadline derived from an execution-clock
/// reading rather than from another framework clock domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionDeadlineBasis {
    tick: u64,
}

/// Typed refusal to construct an execution deadline beyond the clock domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionDeadlineOverflow {
    reading: UiIntentExecutionClockReading,
    allowance_ticks: u64,
}

impl UiIntentExecutionClockReading {
    pub const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }

    pub const fn deadline_after_ticks(
        self,
        allowance_ticks: u64,
    ) -> Result<UiIntentExecutionDeadlineBasis, UiIntentExecutionDeadlineOverflow> {
        match self.tick.checked_add(allowance_ticks) {
            Some(tick) => Ok(UiIntentExecutionDeadlineBasis { tick }),
            None => Err(UiIntentExecutionDeadlineOverflow {
                reading: self,
                allowance_ticks,
            }),
        }
    }
}

impl UiIntentExecutionDeadlineBasis {
    pub(crate) const fn tick(self) -> u64 {
        self.tick
    }
}

impl UiIntentExecutionDeadlineOverflow {
    pub const fn reading(self) -> UiIntentExecutionClockReading {
        self.reading
    }

    pub const fn allowance_ticks(self) -> u64 {
        self.allowance_ticks
    }
}

#[cfg(test)]
mod tests {
    use super::{UiIntentExecutionClockReading, UiIntentExecutionDeadlineOverflow};

    #[test]
    fn deadline_basis_is_derived_from_the_execution_clock_domain() {
        let reading = UiIntentExecutionClockReading::at_tick(41);
        let deadline = reading
            .deadline_after_ticks(1)
            .expect("bounded allowance should produce a deadline");

        assert_eq!(deadline.tick(), 42);
    }

    #[test]
    fn deadline_overflow_preserves_its_exact_basis() {
        let reading = UiIntentExecutionClockReading::at_tick(u64::MAX);

        assert_eq!(
            reading.deadline_after_ticks(1),
            Err(UiIntentExecutionDeadlineOverflow {
                reading,
                allowance_ticks: 1,
            })
        );
    }
}
