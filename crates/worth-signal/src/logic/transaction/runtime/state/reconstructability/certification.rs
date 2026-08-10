use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;

use super::certification_summary::temporal_certification_bundle;
use super::temporal_evidence::{TemporalReconstructabilityArtifact, TemporalReplayParityReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalCertificationFamily {
    TemporalEligibilityReplayParity,
    TemporalBranchRestoreEquivalence,
    TemporalWakeBoundedness,
    PreviousValueTimeGatedEquivalence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemporalCertificationRecord {
    pub family: TemporalCertificationFamily,
    pub artifact: TemporalReconstructabilityArtifact,
    pub parity: Option<TemporalReplayParityReport>,
    pub passed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TemporalCertificationBuilder {
    temporal_eligibility_replay_parity: Option<TemporalCertificationRecord>,
    temporal_branch_restore_equivalence: Option<TemporalCertificationRecord>,
    temporal_wake_boundedness: Option<TemporalCertificationRecord>,
    previous_value_time_gated_equivalence: Option<TemporalCertificationRecord>,
}

pub const REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES: [TemporalCertificationFamily; 4] = [
    TemporalCertificationFamily::TemporalEligibilityReplayParity,
    TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
    TemporalCertificationFamily::TemporalWakeBoundedness,
    TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemporalCertificationFailure {
    MissingRequiredFamily {
        family: TemporalCertificationFamily,
    },
    DuplicateFamily {
        family: TemporalCertificationFamily,
        count: u32,
    },
    FailedFamily {
        family: TemporalCertificationFamily,
    },
    ParityMismatch {
        family: TemporalCertificationFamily,
        mismatch_classes: Vec<super::temporal_evidence::TemporalReplayMismatchClass>,
    },
    EmptyCertificationDigest {
        family: TemporalCertificationFamily,
    },
}

impl TemporalCertificationFailure {
    pub fn family(&self) -> TemporalCertificationFamily {
        match self {
            Self::MissingRequiredFamily { family }
            | Self::DuplicateFamily { family, .. }
            | Self::FailedFamily { family }
            | Self::ParityMismatch { family, .. }
            | Self::EmptyCertificationDigest { family } => *family,
        }
    }
}

pub fn temporal_certification_record(
    family: TemporalCertificationFamily,
    artifact: TemporalReconstructabilityArtifact,
    parity: Option<TemporalReplayParityReport>,
) -> TemporalCertificationRecord {
    let passed = parity.as_ref().map(|report| report.parity).unwrap_or(true);
    TemporalCertificationRecord {
        family,
        artifact,
        parity,
        passed,
    }
}

pub fn temporal_certification_builder() -> TemporalCertificationBuilder {
    TemporalCertificationBuilder::new()
}

impl TemporalCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_temporal_eligibility_replay_parity(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
        parity: TemporalReplayParityReport,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_eligibility_replay_parity,
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
        )?;
        Self::ensure_parity_evidence(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            &artifact,
            &parity,
        )?;
        if artifact.eligibility_fact_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalEligibilityReplayParity,
                "requires at least one lowered temporal eligibility fact",
            ));
        }
        self.temporal_eligibility_replay_parity = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact,
            Some(parity),
        ));
        Ok(self)
    }

    pub fn with_temporal_branch_restore_equivalence(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
        parity: TemporalReplayParityReport,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_branch_restore_equivalence,
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
        )?;
        Self::ensure_parity_evidence(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            &artifact,
            &parity,
        )?;
        if artifact.wake_summary.scheduled_count() == 0
            && artifact.wake_summary.ready_count() == 0
            && artifact.wake_summary.retired_count() == 0
        {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
                "requires restored branch-local temporal wake state",
            ));
        }
        self.temporal_branch_restore_equivalence = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            artifact,
            Some(parity),
        ));
        Ok(self)
    }

    pub fn with_temporal_wake_boundedness(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.temporal_wake_boundedness,
            TemporalCertificationFamily::TemporalWakeBoundedness,
        )?;
        Self::ensure_non_default_artifact(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            &artifact,
        )?;
        if artifact.interval_regeneration_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::TemporalWakeBoundedness,
                "requires interval regeneration evidence",
            ));
        }
        self.temporal_wake_boundedness = Some(temporal_certification_record(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            artifact,
            None,
        ));
        Ok(self)
    }

    pub fn with_previous_value_time_gated_equivalence(
        mut self,
        artifact: TemporalReconstructabilityArtifact,
    ) -> Result<Self, SignalError> {
        Self::ensure_empty(
            &self.previous_value_time_gated_equivalence,
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
        )?;
        Self::ensure_non_default_artifact(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            &artifact,
        )?;
        if artifact.previous_value_reference_count == 0 {
            return Err(Self::invalid_family_evidence(
                TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
                "requires at least one temporal previous-value reference",
            ));
        }
        self.previous_value_time_gated_equivalence = Some(temporal_certification_record(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            artifact,
            None,
        ));
        Ok(self)
    }

    pub fn build(
        self,
    ) -> Result<super::certification_summary::TemporalCertificationBundle, SignalError> {
        let records = [
            self.temporal_eligibility_replay_parity,
            self.temporal_branch_restore_equivalence,
            self.temporal_wake_boundedness,
            self.previous_value_time_gated_equivalence,
        ];
        let mut complete = Vec::with_capacity(REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(Self::invalid_family_evidence(
                    family,
                    "required certification family was not supplied",
                ));
            };
            complete.push(record);
        }

        let bundle = temporal_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn ensure_empty(
        existing: &Option<TemporalCertificationRecord>,
        family: TemporalCertificationFamily,
    ) -> Result<(), SignalError> {
        if existing.is_none() {
            return Ok(());
        }
        Err(Self::invalid_family_evidence(
            family,
            "duplicate certification family evidence",
        ))
    }

    fn ensure_parity_evidence(
        family: TemporalCertificationFamily,
        artifact: &TemporalReconstructabilityArtifact,
        parity: &TemporalReplayParityReport,
    ) -> Result<(), SignalError> {
        Self::ensure_non_default_artifact(family, artifact)?;
        if !parity.parity {
            return Err(Self::invalid_family_evidence(
                family,
                "requires a passing temporal replay parity report",
            ));
        }
        if &parity.replayed != artifact {
            return Err(Self::invalid_family_evidence(
                family,
                "record artifact must be the replayed temporal artifact from the parity report",
            ));
        }
        Ok(())
    }

    fn ensure_non_default_artifact(
        family: TemporalCertificationFamily,
        artifact: &TemporalReconstructabilityArtifact,
    ) -> Result<(), SignalError> {
        if artifact.certification_digest.is_empty() {
            return Err(Self::invalid_family_evidence(
                family,
                "artifact certification digest is empty",
            ));
        }
        if artifact == &TemporalReconstructabilityArtifact::default() {
            return Err(Self::invalid_family_evidence(
                family,
                "default temporal artifact is not certification evidence",
            ));
        }
        Ok(())
    }

    fn invalid_family_evidence(
        family: TemporalCertificationFamily,
        reason: &'static str,
    ) -> SignalError {
        SignalError::invalid_input(format!(
            "invalid temporal certification evidence for {family:?}: {reason}"
        ))
    }
}
