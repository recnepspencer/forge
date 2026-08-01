use std::fmt;
use std::time::{Duration, Instant};

use worth_ui::facade::intent::{
    UiIntentExecutionClockReading, UiIntentExecutionDeadlineBasis,
    UiIntentExecutionDeadlineOverflow,
};

const PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET: Duration = Duration::from_secs(10);

pub(in crate::native_frame) struct PlatformPulseIntentClock {
    origin: Instant,
    last_tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::native_frame) enum PlatformPulseIntentClockDenial {
    Regressed,
    TickExhausted,
    DeadlineOverflow(UiIntentExecutionDeadlineOverflow),
}

impl PlatformPulseIntentClock {
    pub(in crate::native_frame) fn new() -> Self {
        Self::starting_at(Instant::now())
    }

    pub(super) fn read(
        &mut self,
    ) -> Result<UiIntentExecutionClockReading, PlatformPulseIntentClockDenial> {
        self.read_at(Instant::now())
    }

    pub(super) fn new_attempt_deadline(
        &mut self,
    ) -> Result<UiIntentExecutionDeadlineBasis, PlatformPulseIntentClockDenial> {
        let reading = self.read()?;
        let allowance = duration_ticks(PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET)?;
        reading
            .deadline_after_ticks(allowance)
            .map_err(PlatformPulseIntentClockDenial::DeadlineOverflow)
    }

    fn starting_at(origin: Instant) -> Self {
        Self {
            origin,
            last_tick: 0,
        }
    }

    fn read_at(
        &mut self,
        now: Instant,
    ) -> Result<UiIntentExecutionClockReading, PlatformPulseIntentClockDenial> {
        let elapsed = now
            .checked_duration_since(self.origin)
            .ok_or(PlatformPulseIntentClockDenial::Regressed)?;
        let tick = duration_ticks(elapsed)?;
        if tick < self.last_tick {
            return Err(PlatformPulseIntentClockDenial::Regressed);
        }
        self.last_tick = tick;
        Ok(UiIntentExecutionClockReading::at_tick(tick))
    }
}

impl fmt::Display for PlatformPulseIntentClockDenial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regressed => formatter.write_str("monotonic execution clock regressed"),
            Self::TickExhausted => formatter.write_str("monotonic execution clock exhausted"),
            Self::DeadlineOverflow(denial) => write!(
                formatter,
                "execution deadline overflowed from tick {} with allowance {}",
                denial.reading().tick(),
                denial.allowance_ticks()
            ),
        }
    }
}

fn duration_ticks(duration: Duration) -> Result<u64, PlatformPulseIntentClockDenial> {
    u64::try_from(duration.as_millis()).map_err(|_| PlatformPulseIntentClockDenial::TickExhausted)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{PlatformPulseIntentClock, PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET};

    #[test]
    fn repaint_frequency_cannot_consume_an_execution_deadline() {
        let origin = Instant::now();
        let mut clock = PlatformPulseIntentClock::starting_at(origin);
        let deadline = clock
            .new_attempt_deadline()
            .expect("bounded Pulse deadline should be representable");

        for _ in 0..100_000 {
            assert_eq!(
                clock
                    .read_at(origin)
                    .expect("same instant is monotonic")
                    .tick(),
                0
            );
        }
        assert_eq!(
            clock
                .read_at(origin + PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET)
                .expect("deadline instant is monotonic")
                .tick(),
            deadline_tick(deadline)
        );
    }

    #[test]
    fn elapsed_time_and_regression_remain_distinct() {
        let origin = Instant::now();
        let mut clock = PlatformPulseIntentClock::starting_at(origin);
        let deadline = clock
            .new_attempt_deadline()
            .expect("bounded Pulse deadline should be representable");
        let expired = clock
            .read_at(origin + PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET + Duration::from_millis(1))
            .expect("successor instant is monotonic");

        assert!(expired.tick() > deadline_tick(deadline));
        assert!(clock.read_at(origin).is_err());
    }

    fn deadline_tick(deadline: worth_ui::facade::intent::UiIntentExecutionDeadlineBasis) -> u64 {
        // The public deadline basis intentionally withholds its raw tick. This
        // independent oracle uses the declared Pulse policy instead.
        let _ = deadline;
        PLATFORM_PULSE_INTENT_ATTEMPT_BUDGET.as_millis() as u64
    }
}
