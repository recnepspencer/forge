use std::collections::BTreeSet;

use super::super::PhysicalWorkMutantLocalization;
use super::PhysicalWorkHostileTruthCaseEvidence;
use crate::physical_runtime::record_serving::evidence::physical_work::hostile_validation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkHostileTruthCampaignEvidence {
    cases: Box<[PhysicalWorkHostileTruthCaseEvidence]>,
    mutants: Box<[PhysicalWorkMutantLocalization]>,
    verdict: PhysicalWorkHostileTruthVerdict,
}

impl PhysicalWorkHostileTruthCampaignEvidence {
    pub fn new(
        cases: impl IntoIterator<Item = PhysicalWorkHostileTruthCaseEvidence>,
        mutants: impl IntoIterator<Item = PhysicalWorkMutantLocalization>,
    ) -> Self {
        let cases = cases.into_iter().collect::<Vec<_>>().into_boxed_slice();
        let mutants = mutants.into_iter().collect::<Vec<_>>().into_boxed_slice();
        let findings = hostile_validation::validate_campaign(&cases, &mutants);
        Self {
            cases,
            mutants,
            verdict: PhysicalWorkHostileTruthVerdict::from_findings(findings),
        }
    }

    pub const fn cases(&self) -> &[PhysicalWorkHostileTruthCaseEvidence] {
        &self.cases
    }

    pub const fn mutants(&self) -> &[PhysicalWorkMutantLocalization] {
        &self.mutants
    }

    pub const fn verdict(&self) -> &PhysicalWorkHostileTruthVerdict {
        &self.verdict
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalWorkHostileTruthFinding {
    ProcessBindingMismatch,
    StoreIdentityMismatch,
    UnexpectedCurrentTruth,
    InvalidScenarioTransition,
    MissingArtifactManifest,
    DuplicateArtifactPath,
    MissingMutationCoordinationArtifact,
    MissingRecoveryObligation,
    UnexpectedRecoveryObligation,
    ReopenTruthMismatch,
    ReopenRecoveryMismatch,
    OracleRejected,
    MissingScenario,
    DuplicateScenario,
    DuplicateStoreIdentity,
    MixedSourceBinding,
    MixedBinaryBinding,
    MixedRunEnvironment,
    MixedFilesystemVolumeProfile,
    DuplicateFilesystemRootIdentity,
    RejectedScenario,
    MissingMutantLocalization,
    MutantSurvived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalWorkHostileTruthVerdict {
    Accepted,
    Rejected(Box<[PhysicalWorkHostileTruthFinding]>),
}

impl PhysicalWorkHostileTruthVerdict {
    pub const fn accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn findings(&self) -> &[PhysicalWorkHostileTruthFinding] {
        match self {
            Self::Accepted => &[],
            Self::Rejected(findings) => findings,
        }
    }

    pub(super) fn from_findings(
        findings: impl IntoIterator<Item = PhysicalWorkHostileTruthFinding>,
    ) -> Self {
        let findings = findings.into_iter().collect::<BTreeSet<_>>();
        if findings.is_empty() {
            Self::Accepted
        } else {
            Self::Rejected(findings.into_iter().collect())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkHostileTruthEvidenceDenial {
    DuplicateProcessIdentity,
    InvalidProcessRole,
    InvalidProcessFate,
    ZeroStoreIdentity,
    ZeroRuntimeIdentity,
    PrefixExceedsArtifact,
    ArtifactRoleMismatch,
    InconsistentReopenPosture,
}
