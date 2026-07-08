use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmAdvisoryFilterLaw {
    advisory_filter_present: bool,
}

impl S8LsmAdvisoryFilterLaw {
    pub(crate) const fn baseline_absent() -> Self {
        Self {
            advisory_filter_present: false,
        }
    }

    pub const fn advisory_filter_present(self) -> bool {
        self.advisory_filter_present
    }

    pub const fn verify_filter_posture(self, filter_claimed: bool) -> Result<(), S8StrategyDenial> {
        if filter_claimed == self.advisory_filter_present {
            return Ok(());
        }
        Err(S8StrategyDenial::AdvisoryFilterPostureViolation)
    }
}
