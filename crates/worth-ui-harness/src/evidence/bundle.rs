use std::collections::BTreeSet;

use super::{HarnessEvidenceBasis, HarnessEvidenceFamily, HarnessEvidenceRequirement};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HarnessEvidenceBundle {
    families: BTreeSet<HarnessEvidenceFamily>,
    basis: Option<HarnessEvidenceBasis>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessEvidenceValidationDenial {
    MissingRequiredEvidence {
        family: HarnessEvidenceFamily,
    },
    ExpectedEvidenceMissing {
        family: HarnessEvidenceFamily,
    },
    DigestExpectation(super::HarnessDigestExpectationDenial),
    RuntimeEvidenceWithoutBasis,
    StaleEvidenceBasis {
        expected: HarnessEvidenceBasis,
        provided: HarnessEvidenceBasis,
    },
}

impl HarnessEvidenceBundle {
    pub fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn observe_visible_frame(&mut self) {
        self.families
            .insert(HarnessEvidenceFamily::VisibleFrameObservation);
    }

    pub(crate) fn observe_runtime_launch(&mut self, basis: HarnessEvidenceBasis) {
        self.families.insert(HarnessEvidenceFamily::RuntimeReceipt);
        self.families.insert(HarnessEvidenceFamily::ArtifactDigest);
        self.families
            .insert(HarnessEvidenceFamily::ActivePlanObservation);
        self.families
            .insert(HarnessEvidenceFamily::ActivePlanDigest);
        self.families.insert(HarnessEvidenceFamily::SnapshotDigest);
        self.families.insert(HarnessEvidenceFamily::FrameEpoch);
        self.basis = Some(basis);
    }

    pub(crate) fn record_operation_receipt(&mut self) {
        self.families
            .insert(HarnessEvidenceFamily::OperationReceipt);
    }

    pub fn contains(&self, family: HarnessEvidenceFamily) -> bool {
        self.families.contains(&family)
    }

    pub fn families(&self) -> &BTreeSet<HarnessEvidenceFamily> {
        &self.families
    }

    pub fn basis(&self) -> Option<HarnessEvidenceBasis> {
        self.basis
    }

    pub(crate) fn merge_step_evidence(&mut self, step_evidence: HarnessEvidenceBundle) {
        self.families.extend(step_evidence.families);
        if step_evidence.basis.is_some() {
            self.basis = step_evidence.basis;
        }
    }

    pub fn validate(
        &self,
        requirements: &[HarnessEvidenceRequirement],
    ) -> Result<(), HarnessEvidenceValidationDenial> {
        for requirement in requirements {
            reject_missing_required_family(*requirement, self)?;
        }
        reject_runtime_evidence_without_basis(self)?;
        Ok(())
    }

    pub fn validate_expected_observations(
        &self,
        expectations: &[crate::scenario::HarnessExpectedObservation],
    ) -> Result<(), HarnessEvidenceValidationDenial> {
        for expectation in expectations {
            match *expectation {
                crate::scenario::HarnessExpectedObservation::EvidenceFamily(family) => {
                    reject_expected_family(family, self)?;
                }
                crate::scenario::HarnessExpectedObservation::Digest(expectation) => expectation
                    .validate(self.basis)
                    .map_err(HarnessEvidenceValidationDenial::DigestExpectation)?,
            }
        }
        Ok(())
    }

    pub fn validate_against_basis(
        &self,
        expected: HarnessEvidenceBasis,
    ) -> Result<(), HarnessEvidenceValidationDenial> {
        match self.basis {
            Some(provided) if provided == expected => Ok(()),
            Some(provided) => {
                Err(HarnessEvidenceValidationDenial::StaleEvidenceBasis { expected, provided })
            }
            None => Err(HarnessEvidenceValidationDenial::RuntimeEvidenceWithoutBasis),
        }
    }
}

fn reject_expected_family(
    family: HarnessEvidenceFamily,
    bundle: &HarnessEvidenceBundle,
) -> Result<(), HarnessEvidenceValidationDenial> {
    if bundle.contains(family) {
        Ok(())
    } else {
        Err(HarnessEvidenceValidationDenial::ExpectedEvidenceMissing { family })
    }
}

fn reject_missing_required_family(
    requirement: HarnessEvidenceRequirement,
    bundle: &HarnessEvidenceBundle,
) -> Result<(), HarnessEvidenceValidationDenial> {
    if bundle.contains(requirement.family()) {
        Ok(())
    } else {
        Err(HarnessEvidenceValidationDenial::MissingRequiredEvidence {
            family: requirement.family(),
        })
    }
}

fn reject_runtime_evidence_without_basis(
    bundle: &HarnessEvidenceBundle,
) -> Result<(), HarnessEvidenceValidationDenial> {
    let has_runtime_evidence = bundle.contains(HarnessEvidenceFamily::RuntimeReceipt)
        || bundle.contains(HarnessEvidenceFamily::ActivePlanObservation);
    if has_runtime_evidence && bundle.basis.is_none() {
        Err(HarnessEvidenceValidationDenial::RuntimeEvidenceWithoutBasis)
    } else {
        Ok(())
    }
}
