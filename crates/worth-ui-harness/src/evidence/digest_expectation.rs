use super::{HarnessEvidenceBasis, HarnessEvidenceFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessDigestDerivationBasis {
    CurrentRun,
    FixedForDiagnostics,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HarnessDigestExpectation {
    family: HarnessEvidenceFamily,
    basis: HarnessDigestDerivationBasis,
    fixed_value: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessDigestExpectationDenial {
    MissingRunBasis {
        family: HarnessEvidenceFamily,
    },
    FixedDigestRejected {
        family: HarnessEvidenceFamily,
        expected: u64,
        actual: u64,
    },
    MissingDigestFamily {
        family: HarnessEvidenceFamily,
    },
}

impl HarnessDigestExpectation {
    pub fn active_plan_derived_from_run() -> Self {
        Self::derived_from_run(HarnessEvidenceFamily::ActivePlanDigest)
    }

    pub fn artifact_derived_from_run() -> Self {
        Self::derived_from_run(HarnessEvidenceFamily::ArtifactDigest)
    }

    pub fn active_plan_fixed_for_diagnostics(expected: u64) -> Self {
        Self::fixed_for_diagnostics(HarnessEvidenceFamily::ActivePlanDigest, expected)
    }

    pub fn family(self) -> HarnessEvidenceFamily {
        self.family
    }

    pub(crate) fn validate(
        self,
        basis: Option<HarnessEvidenceBasis>,
    ) -> Result<(), HarnessDigestExpectationDenial> {
        let Some(basis) = basis else {
            return Err(HarnessDigestExpectationDenial::MissingRunBasis {
                family: self.family,
            });
        };
        let Some(actual) = digest_for_family(self.family, basis) else {
            return Err(HarnessDigestExpectationDenial::MissingDigestFamily {
                family: self.family,
            });
        };
        match self.basis {
            HarnessDigestDerivationBasis::CurrentRun => Ok(()),
            HarnessDigestDerivationBasis::FixedForDiagnostics => {
                let expected = self.fixed_value.expect("fixed expectation value");
                Err(HarnessDigestExpectationDenial::FixedDigestRejected {
                    family: self.family,
                    expected,
                    actual,
                })
            }
        }
    }

    fn derived_from_run(family: HarnessEvidenceFamily) -> Self {
        Self {
            family,
            basis: HarnessDigestDerivationBasis::CurrentRun,
            fixed_value: None,
        }
    }

    fn fixed_for_diagnostics(family: HarnessEvidenceFamily, expected: u64) -> Self {
        Self {
            family,
            basis: HarnessDigestDerivationBasis::FixedForDiagnostics,
            fixed_value: Some(expected),
        }
    }
}

fn digest_for_family(family: HarnessEvidenceFamily, basis: HarnessEvidenceBasis) -> Option<u64> {
    match family {
        HarnessEvidenceFamily::ArtifactDigest => Some(basis.artifact_digest()),
        HarnessEvidenceFamily::ActivePlanDigest => Some(basis.active_plan_digest()),
        HarnessEvidenceFamily::SnapshotDigest => Some(basis.snapshot_digest()),
        HarnessEvidenceFamily::FrameEpoch => Some(basis.frame_epoch()),
        _ => None,
    }
}
