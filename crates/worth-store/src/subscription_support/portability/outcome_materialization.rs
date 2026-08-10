use super::super::{classification_error, stable_digest, SubscriptionSupportArtifactId};
use super::affected_set::SupportPortabilityAffectedSet;
use super::capsule_manifest::CapsuleSupportManifest;
use super::decision::{
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionEvidence,
};
use super::import_admission::SupportImportAdmissionWitness;
use super::import_not_resumable::ImportedSupportNotResumableReport;
use super::imported_semantic_access::ImportedSupportSemanticAccess;
use super::outcome::SubscriptionSupportPortabilityOutcome;
use super::partial_omission::PartialSupportOmissionReport;
use super::rejection::SupportPortabilityRejection;
use super::replicated_bundle::ReplicatedSupportBundle;
use crate::failure::StoreError;
use std::collections::BTreeSet;

pub(super) fn outcome_from_decision(
    affected_set: &SupportPortabilityAffectedSet,
    manifest: &CapsuleSupportManifest,
    decision: &SubscriptionSupportPortabilityDecision,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    match decision.evidence() {
        SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication {
            source_identity_digest,
            target_identity_digest,
        } => materialize_full_scope_replication(
            affected_set,
            manifest,
            source_identity_digest,
            target_identity_digest,
        ),
        SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
            omitted_artifact_ids,
            omission_reason,
        } => materialize_partial_scope_omission(manifest, omitted_artifact_ids, omission_reason),
        SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted {
            target_admission_digest,
            source_identity_preservation_digest,
            imported_semantic_digest,
        } => materialize_admitted_import(
            manifest,
            target_admission_digest,
            source_identity_preservation_digest,
            imported_semantic_digest,
        ),
        SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
            target_admission_digest,
            basis_artifact_ids,
            denial_reason,
        } => materialize_not_resumable_import(
            affected_set,
            manifest,
            target_admission_digest,
            basis_artifact_ids,
            denial_reason,
        ),
        SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
            rejection_reason,
        } => materialize_rejection(manifest, decision, rejection_reason),
    }
}

fn materialize_full_scope_replication(
    affected_set: &SupportPortabilityAffectedSet,
    manifest: &CapsuleSupportManifest,
    source_identity_digest: &String,
    target_identity_digest: &String,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    let identity_preservation_digest = stable_digest(&(
        manifest.manifest_digest(),
        source_identity_digest,
        target_identity_digest,
        affected_set.affected_set_digest(),
    ))?;
    Ok(SubscriptionSupportPortabilityOutcome::FullScopeReplicated(
        ReplicatedSupportBundle::new(
            manifest.manifest_digest().to_string(),
            source_identity_digest.to_string(),
            target_identity_digest.to_string(),
            affected_set.affected_artifact_ids(),
            identity_preservation_digest,
        ),
    ))
}

fn materialize_partial_scope_omission(
    manifest: &CapsuleSupportManifest,
    omitted_artifact_ids: &[SubscriptionSupportArtifactId],
    omission_reason: &String,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    if omitted_artifact_ids.len() as u64 != manifest.omitted_support_count() {
        return Err(classification_error(
            "subscription-support partial omission report must match manifest omitted count",
        ));
    }
    Ok(SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(
        PartialSupportOmissionReport::new(
            manifest.manifest_digest().to_string(),
            omission_reason.to_string(),
            omitted_artifact_ids.to_vec(),
        ),
    ))
}

fn materialize_admitted_import(
    manifest: &CapsuleSupportManifest,
    target_admission_digest: &String,
    source_identity_preservation_digest: &String,
    imported_semantic_digest: &String,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    let import_admission = SupportImportAdmissionWitness::exact(
        manifest,
        target_admission_digest.clone(),
        source_identity_preservation_digest.clone(),
    )?;
    let semantic_access = ImportedSupportSemanticAccess::from_import_admission(
        import_admission,
        imported_semantic_digest.clone(),
    )?;
    Ok(SubscriptionSupportPortabilityOutcome::Imported(
        semantic_access,
    ))
}

fn materialize_not_resumable_import(
    affected_set: &SupportPortabilityAffectedSet,
    manifest: &CapsuleSupportManifest,
    target_admission_digest: &String,
    basis_artifact_ids: &[SubscriptionSupportArtifactId],
    denial_reason: &String,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    let import_admission =
        SupportImportAdmissionWitness::new(manifest, target_admission_digest.clone())?;
    let admitted_basis = basis_artifact_ids.iter().collect::<BTreeSet<_>>();
    let missing_basis_artifact_ids = affected_set
        .affected_artifact_ids()
        .into_iter()
        .filter(|artifact_id| !admitted_basis.contains(artifact_id))
        .collect();
    Ok(SubscriptionSupportPortabilityOutcome::ImportedNotResumable(
        ImportedSupportNotResumableReport::new(
            import_admission,
            denial_reason.to_string(),
            missing_basis_artifact_ids,
        ),
    ))
}

fn materialize_rejection(
    manifest: &CapsuleSupportManifest,
    decision: &SubscriptionSupportPortabilityDecision,
    rejection_reason: &String,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    Ok(SubscriptionSupportPortabilityOutcome::Rejected(
        SupportPortabilityRejection::new(
            decision.kind(),
            manifest.manifest_digest().to_string(),
            rejection_reason.to_string(),
        ),
    ))
}
