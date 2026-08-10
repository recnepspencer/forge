use super::super::{
    classification_error, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SupportPathClass, SupportProgramPathPlan,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::decision::{
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionKind,
};
use crate::failure::StoreError;
use std::collections::BTreeSet;

pub(super) fn validate_decision_origin_and_path(
    decision: &SubscriptionSupportPortabilityDecision,
    affected_set: &SupportPortabilityAffectedSet,
    path_plan: &SupportProgramPathPlan,
) -> Result<(), StoreError> {
    match decision.kind() {
        SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
        | SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission => {
            validate_export_path(affected_set, path_plan)
        }
        SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
        | SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
        | SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected => {
            validate_import_path(affected_set, path_plan)
        }
    }
}

fn validate_export_path(
    affected_set: &SupportPortabilityAffectedSet,
    path_plan: &SupportProgramPathPlan,
) -> Result<(), StoreError> {
    if affected_set.action_origin() != SubscriptionSupportActionOrigin::ReplicationExport
        || path_plan.path_class() != SupportPathClass::ReplicationExport
    {
        return Err(classification_error(
            "subscription-support replication decisions require export-origin bases and replication-export paths",
        ));
    }
    Ok(())
}

fn validate_import_path(
    affected_set: &SupportPortabilityAffectedSet,
    path_plan: &SupportProgramPathPlan,
) -> Result<(), StoreError> {
    if affected_set.action_origin() != SubscriptionSupportActionOrigin::ReplicationImport
        || path_plan.path_class() != SupportPathClass::ImportAdmission
    {
        return Err(classification_error(
            "subscription-support import decisions require import-origin bases and import-admission paths",
        ));
    }
    Ok(())
}

pub(super) fn validate_basis_artifact_ids(
    affected_set: &SupportPortabilityAffectedSet,
    basis_artifact_ids: &[SubscriptionSupportArtifactId],
    omitted_artifact_ids: &[SubscriptionSupportArtifactId],
) -> Result<(), StoreError> {
    let omitted = omitted_artifact_ids.iter().collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    for artifact_id in basis_artifact_ids {
        if !seen.insert(artifact_id) {
            return Err(classification_error(
                "subscription-support basis evidence cannot repeat artifact ids",
            ));
        }
        if !affected_set.contains_artifact_id(artifact_id) {
            return Err(classification_error(
                "subscription-support basis evidence must name only artifacts in the admitted portability scope",
            ));
        }
        if omitted.contains(artifact_id) {
            return Err(classification_error(
                "subscription-support omitted artifacts cannot also claim basis evidence",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_omitted_artifact_ids(
    affected_set: &SupportPortabilityAffectedSet,
    omitted_artifact_ids: &[SubscriptionSupportArtifactId],
) -> Result<(), StoreError> {
    let mut seen = BTreeSet::new();
    for artifact_id in omitted_artifact_ids {
        if !seen.insert(artifact_id) {
            return Err(classification_error(
                "subscription-support omission reports cannot repeat omitted artifact ids",
            ));
        }
        if !affected_set.contains_artifact_id(artifact_id) {
            return Err(classification_error(
                "subscription-support omission reports must name only artifacts in the admitted portability scope",
            ));
        }
    }
    Ok(())
}

pub(super) fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support portability {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
