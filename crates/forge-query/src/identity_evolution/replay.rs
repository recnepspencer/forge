#![cfg_attr(not(test), allow(dead_code))]

use crate::identity::{BasisDigest, FailureDigest, LineageDigest, ResultDigest};

use super::{
    evidence::{
        IdentityEvolutionCertificationDenialEvidence, IdentityEvolutionCertificationResultEvidence,
    },
    families::IdentityEvolutionOutcomeFamily,
    metadata::BranchLocalityClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionReplayParityClass {
    ReplayEquivalent,
    ReplayDivergent,
}

impl IdentityEvolutionReplayParityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReplayEquivalent => "replay_equivalent",
            Self::ReplayDivergent => "replay_divergent",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionReplayArtifact {
    query_digest: String,
    basis_digest: BasisDigest,
    lineage_digest: LineageDigest,
    branch_locality_digest: ResultDigest,
    complexity_contract_digest: ResultDigest,
    result_or_failure_digest: String,
    counter_snapshot_digest: String,
    replay_digest: ResultDigest,
    parity_class: IdentityEvolutionReplayParityClass,
}

impl IdentityEvolutionReplayArtifact {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn branch_locality_digest(&self) -> &ResultDigest {
        &self.branch_locality_digest
    }

    pub fn complexity_contract_digest(&self) -> &ResultDigest {
        &self.complexity_contract_digest
    }

    pub fn result_or_failure_digest(&self) -> &str {
        &self.result_or_failure_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn replay_digest(&self) -> &ResultDigest {
        &self.replay_digest
    }

    pub fn parity_class(&self) -> IdentityEvolutionReplayParityClass {
        self.parity_class
    }

    pub(crate) fn from_result_evidence(
        evidence: &IdentityEvolutionCertificationResultEvidence,
    ) -> Self {
        let replay_digest = ResultDigest::from_parts(&[
            format!("query_digest:{}", evidence.query_digest().as_str()),
            format!("basis_digest:{}", evidence.basis_digest().as_str()),
            format!("lineage_digest:{}", evidence.lineage_digest().as_str()),
            format!(
                "branch_locality_digest:{}",
                evidence.branch_locality_digest().as_str()
            ),
            format!(
                "complexity_contract_digest:{}",
                evidence.complexity_contract_digest().as_str()
            ),
            format!("outcome_family:{}", evidence.outcome_family().as_str()),
            format!("result_digest:{}", evidence.result_digest()),
            format!(
                "counter_snapshot_digest:{}",
                evidence.counter_snapshot().counter_snapshot_digest().as_str()
            ),
        ]);
        Self {
            query_digest: evidence.query_digest().as_str().to_string(),
            basis_digest: evidence.basis_digest().clone(),
            lineage_digest: evidence.lineage_digest().clone(),
            branch_locality_digest: evidence.branch_locality_digest().clone(),
            complexity_contract_digest: evidence.complexity_contract_digest().clone(),
            result_or_failure_digest: evidence.result_digest().to_string(),
            counter_snapshot_digest: evidence
                .counter_snapshot()
                .counter_snapshot_digest()
                .as_str()
                .to_string(),
            replay_digest,
            parity_class: IdentityEvolutionReplayParityClass::ReplayEquivalent,
        }
    }

    pub(crate) fn from_denial_evidence(
        evidence: &IdentityEvolutionCertificationDenialEvidence,
    ) -> Self {
        let replay_digest = ResultDigest::from_parts(&[
            format!("query_digest:{}", evidence.query_digest().as_str()),
            format!("basis_digest:{}", evidence.basis_digest().as_str()),
            format!("lineage_digest:{}", evidence.lineage_digest().as_str()),
            format!(
                "branch_locality_digest:{}",
                evidence.branch_locality_digest().as_str()
            ),
            format!(
                "complexity_contract_digest:{}",
                evidence.complexity_contract_digest().as_str()
            ),
            format!("failure_digest:{}", evidence.failure_digest().as_str()),
            format!(
                "counter_snapshot_digest:{}",
                evidence.counter_snapshot().counter_snapshot_digest().as_str()
            ),
        ]);
        Self {
            query_digest: evidence.query_digest().as_str().to_string(),
            basis_digest: evidence.basis_digest().clone(),
            lineage_digest: evidence.lineage_digest().clone(),
            branch_locality_digest: evidence.branch_locality_digest().clone(),
            complexity_contract_digest: evidence.complexity_contract_digest().clone(),
            result_or_failure_digest: evidence.failure_digest().as_str().to_string(),
            counter_snapshot_digest: evidence
                .counter_snapshot()
                .counter_snapshot_digest()
                .as_str()
                .to_string(),
            replay_digest,
            parity_class: IdentityEvolutionReplayParityClass::ReplayEquivalent,
        }
    }
}

pub fn compare_identity_evolution_result_replay(
    control: &IdentityEvolutionCertificationResultEvidence,
    replay: &IdentityEvolutionCertificationResultEvidence,
) -> IdentityEvolutionReplayArtifact {
    let parity_class = if control.query_digest() == replay.query_digest()
        && control.basis_digest() == replay.basis_digest()
        && control.lineage_digest() == replay.lineage_digest()
        && control.branch_locality_digest() == replay.branch_locality_digest()
        && control.complexity_contract_digest() == replay.complexity_contract_digest()
        && control.outcome_family() == replay.outcome_family()
        && control.branch_locality_class() == replay.branch_locality_class()
        && control.complexity_status() == replay.complexity_status()
        && control.result_digest() == replay.result_digest()
        && control.counter_snapshot().counter_snapshot_digest()
            == replay.counter_snapshot().counter_snapshot_digest()
    {
        IdentityEvolutionReplayParityClass::ReplayEquivalent
    } else {
        IdentityEvolutionReplayParityClass::ReplayDivergent
    };
    let mut artifact = IdentityEvolutionReplayArtifact::from_result_evidence(replay);
    artifact.parity_class = parity_class;
    artifact
}

pub fn compare_identity_evolution_denial_replay(
    control: &IdentityEvolutionCertificationDenialEvidence,
    replay: &IdentityEvolutionCertificationDenialEvidence,
) -> IdentityEvolutionReplayArtifact {
    let parity_class = if control.query_digest() == replay.query_digest()
        && control.basis_digest() == replay.basis_digest()
        && control.lineage_digest() == replay.lineage_digest()
        && control.branch_locality_digest() == replay.branch_locality_digest()
        && control.complexity_contract_digest() == replay.complexity_contract_digest()
        && control.failure_digest() == replay.failure_digest()
        && control.counter_snapshot().counter_snapshot_digest()
            == replay.counter_snapshot().counter_snapshot_digest()
    {
        IdentityEvolutionReplayParityClass::ReplayEquivalent
    } else {
        IdentityEvolutionReplayParityClass::ReplayDivergent
    };
    let mut artifact = IdentityEvolutionReplayArtifact::from_denial_evidence(replay);
    artifact.parity_class = parity_class;
    artifact
}

pub fn compare_identity_evolution_result_classification(
    control_outcome_family: IdentityEvolutionOutcomeFamily,
    control_branch_locality: BranchLocalityClass,
    replay_outcome_family: IdentityEvolutionOutcomeFamily,
    replay_branch_locality: BranchLocalityClass,
    replay: &IdentityEvolutionCertificationResultEvidence,
) -> IdentityEvolutionReplayArtifact {
    let parity_class = if control_outcome_family == replay_outcome_family
        && control_branch_locality == replay_branch_locality
    {
        IdentityEvolutionReplayParityClass::ReplayEquivalent
    } else {
        IdentityEvolutionReplayParityClass::ReplayDivergent
    };
    let mut artifact = IdentityEvolutionReplayArtifact::from_result_evidence(replay);
    artifact.parity_class = parity_class;
    artifact
}

pub fn compare_identity_evolution_denial_classification(
    control_failure_digest: &FailureDigest,
    replay_failure_digest: &FailureDigest,
    replay: &IdentityEvolutionCertificationDenialEvidence,
) -> IdentityEvolutionReplayArtifact {
    let parity_class = if control_failure_digest == replay_failure_digest {
        IdentityEvolutionReplayParityClass::ReplayEquivalent
    } else {
        IdentityEvolutionReplayParityClass::ReplayDivergent
    };
    let mut artifact = IdentityEvolutionReplayArtifact::from_denial_evidence(replay);
    artifact.parity_class = parity_class;
    artifact
}
