use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::explanations::HadwigerReusableNegativeEvidence;

use super::graph_memory::{FailureBasisFingerprint, GraphResidentFailure};
use super::hypotheses::{InvariantHypothesis, ReactivationCondition};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeadEndSignature {
    core: HadwigerArtifactCore,
    signature_token: String,
}

impl DeadEndSignature {
    pub fn from_graph_resident_failure(
        failure: &GraphResidentFailure,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let signature_token = format!(
            "{}:{}",
            failure.failure_scope().stable_token(),
            failure
                .failure_basis_fingerprint()
                .artifact_digest()
                .stable_token()
        );
        let core = artifact_core(
            HadwigerArtifactKind::DeadEndSignature,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "dead_end_signature".to_string(),
            },
            vec![
                failure.reference(),
                failure.failure_basis_fingerprint().reference(),
            ],
            vec![HadwigerArtifactPayloadEntry::text(
                "signature_token",
                signature_token.clone(),
            )],
        )?;
        Ok(Self {
            core,
            signature_token,
        })
    }

    pub(crate) fn from_reusable_negative_evidence(
        evidence: &HadwigerReusableNegativeEvidence,
        failure_basis_fingerprint: &FailureBasisFingerprint,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let signature_token = format!(
            "{}:{}",
            failure_basis_fingerprint.scope_token(),
            failure_basis_fingerprint.artifact_digest().stable_token()
        );
        let core = artifact_core(
            HadwigerArtifactKind::DeadEndSignature,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "dead_end_signature_from_retained_negative_evidence".to_string(),
            },
            vec![evidence.reference(), failure_basis_fingerprint.reference()],
            vec![HadwigerArtifactPayloadEntry::text(
                "signature_token",
                signature_token.clone(),
            )],
        )?;
        Ok(Self {
            core,
            signature_token,
        })
    }

    pub fn stable_token(&self) -> &str {
        &self.signature_token
    }
}

impl_hadwiger_artifact!(DeadEndSignature, core);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SuppressionRelation {
    EquivalentDeadEnd,
    ReactivatedByNewEvidence,
}

impl SuppressionRelation {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::EquivalentDeadEnd => "equivalent_dead_end",
            Self::ReactivatedByNewEvidence => "reactivated_by_new_evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentSuppressionProof {
    core: HadwigerArtifactCore,
    dead_end_signature: DeadEndSignature,
    relation: SuppressionRelation,
}

impl ExperimentSuppressionProof {
    pub fn from_dead_end_signature(
        dead_end_signature: DeadEndSignature,
        failure_basis_fingerprint: &FailureBasisFingerprint,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut parents = vec![
            dead_end_signature.reference(),
            failure_basis_fingerprint.reference(),
        ];
        parents.extend(failure_basis_fingerprint.parent_artifacts().iter().cloned());
        let core = artifact_core(
            HadwigerArtifactKind::ExperimentSuppressionProof,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "experiment_suppression_proof".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "dead_end_signature",
                    dead_end_signature.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "failure_basis_fingerprint",
                    failure_basis_fingerprint.artifact_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "relation",
                    SuppressionRelation::EquivalentDeadEnd.as_str(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            dead_end_signature,
            relation: SuppressionRelation::EquivalentDeadEnd,
        })
    }

    pub fn reactivated_by(
        self,
        reactivation_condition: &ReactivationCondition,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::ExperimentSuppressionProof,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "experiment_reactivation_proof".to_string(),
            },
            vec![self.reference(), reactivation_condition.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "dead_end_signature",
                    self.dead_end_signature.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "qualifying_evidence",
                    reactivation_condition.qualifying_evidence().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "relation",
                    SuppressionRelation::ReactivatedByNewEvidence.as_str(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            dead_end_signature: self.dead_end_signature,
            relation: SuppressionRelation::ReactivatedByNewEvidence,
        })
    }

    pub fn blocks_equivalent_experiment(&self) -> bool {
        self.relation == SuppressionRelation::EquivalentDeadEnd
    }

    pub fn relation(&self) -> SuppressionRelation {
        self.relation
    }

    pub fn dead_end_signature(&self) -> &DeadEndSignature {
        &self.dead_end_signature
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ExperimentSuppressionProof, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentPlan {
    core: HadwigerArtifactCore,
    hypothesis_reference: HadwigerArtifactReference,
    suppression_proof: Option<ExperimentSuppressionProof>,
}

impl ExperimentPlan {
    pub(crate) fn from_hypothesis(
        hypothesis: &InvariantHypothesis,
        suppression_proof: Option<ExperimentSuppressionProof>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut parents = vec![hypothesis.reference()];
        if let Some(proof) = suppression_proof.as_ref() {
            parents.push(proof.reference());
        }
        let core = artifact_core(
            HadwigerArtifactKind::ExperimentPlan,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "experiment_plan".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "hypothesis",
                    hypothesis.artifact_digest().stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "suppressed",
                    suppression_proof.is_some().to_string(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            hypothesis_reference: hypothesis.reference(),
            suppression_proof,
        })
    }

    pub fn hypothesis_reference(&self) -> &HadwigerArtifactReference {
        &self.hypothesis_reference
    }

    pub fn suppression_proof(&self) -> Option<&ExperimentSuppressionProof> {
        self.suppression_proof.as_ref()
    }

    pub fn is_suppressed(&self) -> bool {
        self.suppression_proof.is_some()
    }
}

impl_hadwiger_artifact!(ExperimentPlan, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentBatch {
    core: HadwigerArtifactCore,
    experiment_plans: Vec<ExperimentPlan>,
    suppression_proofs: Vec<ExperimentSuppressionProof>,
    query_readiness_checks: usize,
}

impl ExperimentBatch {
    pub(crate) fn new(
        mut experiment_plans: Vec<ExperimentPlan>,
        mut suppression_proofs: Vec<ExperimentSuppressionProof>,
        query_readiness_checks: usize,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        experiment_plans.sort_by_key(|plan| plan.reference().stable_token());
        suppression_proofs.sort_by_key(|proof| proof.reference().stable_token());
        let mut parents: Vec<_> = experiment_plans
            .iter()
            .map(ExperimentPlan::reference)
            .collect();
        parents.extend(
            suppression_proofs
                .iter()
                .map(ExperimentSuppressionProof::reference),
        );
        let core = artifact_core(
            HadwigerArtifactKind::ExperimentBatch,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "experiment_batch".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::unsigned(
                    "plan_count",
                    experiment_plans.len() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "suppression_count",
                    suppression_proofs.len() as u128,
                ),
                HadwigerArtifactPayloadEntry::unsigned(
                    "query_readiness_checks",
                    query_readiness_checks as u128,
                ),
            ],
        )?;
        Ok(Self {
            core,
            experiment_plans,
            suppression_proofs,
            query_readiness_checks,
        })
    }

    pub fn experiment_plans(&self) -> &[ExperimentPlan] {
        &self.experiment_plans
    }

    pub fn suppression_proofs(&self) -> &[ExperimentSuppressionProof] {
        &self.suppression_proofs
    }

    pub fn query_readiness_checks(&self) -> usize {
        self.query_readiness_checks
    }
}

impl_hadwiger_artifact!(ExperimentBatch, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentResult {
    core: HadwigerArtifactCore,
}

impl ExperimentResult {
    pub fn skipped_unsupported(plan: &ExperimentPlan) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::ExperimentResult,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "experiment_result_skipped_unsupported".to_string(),
            },
            vec![plan.reference()],
            vec![HadwigerArtifactPayloadEntry::text(
                "result",
                "skipped_unsupported",
            )],
        )?;
        Ok(Self { core })
    }
}

impl_hadwiger_artifact!(ExperimentResult, core);
