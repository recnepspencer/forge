use crate::evidence::{HarnessEvidenceBundle, HarnessEvidenceRequirement};
use crate::scenario::HarnessExpectedObservation;

use super::HarnessHonestyDenial;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HarnessHonestyPolicy;

impl HarnessHonestyPolicy {
    pub fn validate_step_evidence(
        self,
        evidence: &HarnessEvidenceBundle,
        requirements: &[HarnessEvidenceRequirement],
        expectations: &[HarnessExpectedObservation],
    ) -> Result<(), HarnessHonestyDenial> {
        evidence.validate(requirements)?;
        evidence.validate_expected_observations(expectations)?;
        Ok(())
    }

    pub fn reject_app_local_shell_state_injection(self) -> HarnessHonestyDenial {
        HarnessHonestyDenial::AppLocalShellStateInjection
    }
}
