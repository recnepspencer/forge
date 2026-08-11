use super::super::classification_error;
use super::super::{SubscriptionSupportArtifactId, SupportAffectedSetDigest};
use super::affected_set::SupportCompatibilityAffectedSet;
use super::decision::{
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
};
use super::manifest_admission::SupportManifestAdmissionWitness;
use super::receipt_witness::SupportCompatibilityReceiptWitness;
use crate::failure::StoreError;
use crate::{CompatibilityRejectionKind, CompatibilityRelation};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportCompatibilityOutcome {
    ExactMigrated(ExactCompatibleSupportMigration),
    Degraded(DegradedCompatibleSupportPosture),
    Rejected(SupportVersionSkewRejection),
}

impl SubscriptionSupportCompatibilityOutcome {
    pub fn outcome_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        match self {
            Self::ExactMigrated(_) => {
                SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration
            }
            Self::Degraded(_) => {
                SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility
            }
            Self::Rejected(rejection) => rejection.rejection_kind(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactCompatibleSupportMigration {
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: CompatibilityRelation,
    classifier_equivalence_digest: String,
    migrated_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl ExactCompatibleSupportMigration {
    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn classifier_equivalence_digest(&self) -> &str {
        &self.classifier_equivalence_digest
    }

    pub fn milestone12_relation(&self) -> CompatibilityRelation {
        self.milestone12_relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedCompatibleSupportPosture {
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_relation: CompatibilityRelation,
    drift_reason: String,
    degraded_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl DegradedCompatibleSupportPosture {
    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn drift_reason(&self) -> &str {
        &self.drift_reason
    }

    pub fn milestone12_relation(&self) -> CompatibilityRelation {
        self.milestone12_relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportVersionSkewRejection {
    rejection_kind: SubscriptionSupportCompatibilityDecisionKind,
    affected_set_digest: SupportAffectedSetDigest,
    manifest_digest: String,
    compatibility_digest: String,
    milestone12_receipt_digest: String,
    milestone12_rejection_kind: Option<CompatibilityRejectionKind>,
    rejection_reason: String,
    rejected_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl SupportVersionSkewRejection {
    pub fn rejection_kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        self.rejection_kind
    }

    pub fn rejection_reason(&self) -> &str {
        &self.rejection_reason
    }

    pub fn milestone12_rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.milestone12_rejection_kind
    }
}

pub(super) fn outcome_from_decision(
    affected_set: SupportCompatibilityAffectedSet,
    manifest_admission: SupportManifestAdmissionWitness,
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<SubscriptionSupportCompatibilityOutcome, StoreError> {
    match decision.kind() {
        SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
            materialize_exact_compatible_migration_outcome(
                &affected_set,
                &manifest_admission,
                decision,
            )
        }
        SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
            materialize_degraded_compatibility_outcome(&affected_set, &manifest_admission, decision)
        }
        SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
        | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
        | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
            materialize_version_skew_rejection_outcome(&affected_set, &manifest_admission, decision)
        }
    }
}

fn materialize_exact_compatible_migration_outcome(
    affected_set: &SupportCompatibilityAffectedSet,
    manifest_admission: &SupportManifestAdmissionWitness,
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<SubscriptionSupportCompatibilityOutcome, StoreError> {
    let receipt = manifest_admission.compatibility_receipt();
    let milestone12_relation = validate_exact_compatible_migration_relation(receipt)?;
    let classifier_equivalence_digest = validate_exact_compatible_migration_evidence(decision)?;
    Ok(SubscriptionSupportCompatibilityOutcome::ExactMigrated(
        ExactCompatibleSupportMigration {
            affected_set_digest: affected_set.affected_set_digest().clone(),
            manifest_digest: manifest_admission.manifest_digest().to_string(),
            compatibility_digest: manifest_admission.compatibility_digest().to_string(),
            milestone12_receipt_digest: receipt.receipt_digest().to_string(),
            milestone12_relation,
            classifier_equivalence_digest,
            migrated_artifact_ids: affected_set.affected_artifact_ids(),
        },
    ))
}

fn validate_exact_compatible_migration_relation(
    receipt: &SupportCompatibilityReceiptWitness,
) -> Result<CompatibilityRelation, StoreError> {
    receipt.relation().ok_or_else(|| {
        classification_error(
            "exact compatible support migration requires an accepted Milestone 12 relation",
        )
    })
}

fn validate_exact_compatible_migration_evidence(
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<String, StoreError> {
    decision
        .classifier_equivalence_digest()
        .ok_or_else(|| {
            classification_error(
                "exact compatible support migration requires classifier equivalence evidence",
            )
        })
        .map(str::to_string)
}

fn materialize_degraded_compatibility_outcome(
    affected_set: &SupportCompatibilityAffectedSet,
    manifest_admission: &SupportManifestAdmissionWitness,
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<SubscriptionSupportCompatibilityOutcome, StoreError> {
    let receipt = manifest_admission.compatibility_receipt();
    let milestone12_relation = validate_degraded_compatibility_relation(receipt)?;
    let drift_reason = validate_degraded_compatibility_evidence(decision)?;
    Ok(SubscriptionSupportCompatibilityOutcome::Degraded(
        DegradedCompatibleSupportPosture {
            affected_set_digest: affected_set.affected_set_digest().clone(),
            manifest_digest: manifest_admission.manifest_digest().to_string(),
            compatibility_digest: manifest_admission.compatibility_digest().to_string(),
            milestone12_receipt_digest: receipt.receipt_digest().to_string(),
            milestone12_relation,
            drift_reason,
            degraded_artifact_ids: affected_set.affected_artifact_ids(),
        },
    ))
}

fn validate_degraded_compatibility_relation(
    receipt: &SupportCompatibilityReceiptWitness,
) -> Result<CompatibilityRelation, StoreError> {
    receipt.relation().ok_or_else(|| {
        classification_error(
            "degraded compatible support posture requires an accepted Milestone 12 relation",
        )
    })
}

fn validate_degraded_compatibility_evidence(
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<String, StoreError> {
    decision
        .drift_reason()
        .ok_or_else(|| {
            classification_error("degraded compatible support posture requires drift evidence")
        })
        .map(str::to_string)
}

fn materialize_version_skew_rejection_outcome(
    affected_set: &SupportCompatibilityAffectedSet,
    manifest_admission: &SupportManifestAdmissionWitness,
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<SubscriptionSupportCompatibilityOutcome, StoreError> {
    let receipt = manifest_admission.compatibility_receipt();
    let rejection_kind = decision.kind();
    let rejection_reason = validate_version_skew_rejection_evidence(decision)?;
    Ok(SubscriptionSupportCompatibilityOutcome::Rejected(
        SupportVersionSkewRejection {
            rejection_kind,
            affected_set_digest: affected_set.affected_set_digest().clone(),
            manifest_digest: manifest_admission.manifest_digest().to_string(),
            compatibility_digest: manifest_admission.compatibility_digest().to_string(),
            milestone12_receipt_digest: receipt.receipt_digest().to_string(),
            milestone12_rejection_kind: receipt.rejection_kind(),
            rejection_reason,
            rejected_artifact_ids: affected_set.affected_artifact_ids(),
        },
    ))
}

fn validate_version_skew_rejection_evidence(
    decision: &SubscriptionSupportCompatibilityDecision,
) -> Result<String, StoreError> {
    decision
        .drift_reason()
        .ok_or_else(|| {
            classification_error("version-skew support rejection requires typed rejection evidence")
        })
        .map(str::to_string)
}
