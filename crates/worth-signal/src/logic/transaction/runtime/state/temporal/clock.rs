use crate::data::error::SignalError;
use crate::data::temporal::{
    ClockAdvanceRequest, RuntimeClockBasis, TemporalClockAdvanceSummary, ValidatedClockAdvance,
};

use super::super::runtime_state::SignalRuntime;
use super::TemporalRuntimeState;

impl TemporalRuntimeState {
    pub fn clock_basis(&self) -> RuntimeClockBasis {
        self.clock_basis
    }

    pub fn validate_clock_advance(
        &self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        self.clock_basis.validate_advance(request)
    }

    pub fn apply_clock_advance(&mut self, validated: ValidatedClockAdvance) {
        self.clock_basis.apply_validated_advance(validated);
    }

    pub fn bump_previous_value_capability_epoch(&mut self) {
        self.previous_value_capability_epoch =
            self.previous_value_capability_epoch.saturating_add(1);
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn clock_basis(&self) -> RuntimeClockBasis {
        self.temporal.clock_basis()
    }

    pub fn validate_clock_advance(
        &self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        self.temporal.validate_clock_advance(request)
    }

    pub fn advance_clock(
        &mut self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        let validated = self.validate_clock_advance(request)?;
        self.temporal.apply_clock_advance(validated);
        Ok(validated)
    }

    pub fn advance_clock_with_summary(
        &mut self,
        request: ClockAdvanceRequest,
    ) -> Result<TemporalClockAdvanceSummary, SignalError> {
        let frontier_before = self.temporal.frontier_snapshot();
        let validated = self.advance_clock(request)?;
        let frontier_after = self.temporal.frontier_snapshot();
        Ok(TemporalClockAdvanceSummary::new(
            validated,
            frontier_before,
            frontier_after,
        ))
    }
}
