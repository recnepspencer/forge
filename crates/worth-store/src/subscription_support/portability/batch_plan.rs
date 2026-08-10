use super::super::{
    classification_error, SupportActionId, SupportProgramDensityClass, SupportProgramPathPlan,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::capsule_manifest::CapsuleSupportManifest;
use super::decision::{
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionKind,
};
use super::evidence_validation::{
    validate_decision_origin_and_path, validate_omitted_artifact_ids,
};
use super::scope_footprint::SupportPortabilityScopeFootprint;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportPortabilityAffectedSet,
    path_plan: SupportProgramPathPlan,
    footprint: SupportPortabilityScopeFootprint,
    manifest: CapsuleSupportManifest,
    decision: SubscriptionSupportPortabilityDecision,
}

impl SupportPortabilityBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportPortabilityAffectedSet,
        path_plan: SupportProgramPathPlan,
        footprint: SupportPortabilityScopeFootprint,
        manifest: CapsuleSupportManifest,
        decision: SubscriptionSupportPortabilityDecision,
    ) -> Result<Self, StoreError> {
        validate_density(&path_plan)?;
        validate_batch_width(&affected_set, &path_plan)?;
        validate_decision_origin_and_path(&decision, &affected_set, &path_plan)?;
        validate_manifest_binding(&affected_set, &manifest)?;
        validate_decision_coverage(&decision, &affected_set, &footprint, &manifest)?;
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            footprint,
            manifest,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportPortabilityAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn footprint(&self) -> &SupportPortabilityScopeFootprint {
        &self.footprint
    }

    pub fn manifest(&self) -> &CapsuleSupportManifest {
        &self.manifest
    }

    pub fn decision(&self) -> &SubscriptionSupportPortabilityDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportPortabilityAffectedSet,
        SupportProgramPathPlan,
        SupportPortabilityScopeFootprint,
        CapsuleSupportManifest,
        SubscriptionSupportPortabilityDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.footprint,
            self.manifest,
            self.decision,
        )
    }
}

fn validate_density(path_plan: &SupportProgramPathPlan) -> Result<(), StoreError> {
    if path_plan.density_class() != SupportProgramDensityClass::PortabilityScopeBatch {
        return Err(classification_error(
            "subscription-support portability plans require portability-scope batch density",
        ));
    }
    Ok(())
}

fn validate_batch_width(
    affected_set: &SupportPortabilityAffectedSet,
    path_plan: &SupportProgramPathPlan,
) -> Result<(), StoreError> {
    if path_plan.batch_width() != affected_set.affected_count() {
        return Err(classification_error(
            "subscription-support portability plan width must match affected-set breadth",
        ));
    }
    Ok(())
}

fn validate_manifest_binding(
    affected_set: &SupportPortabilityAffectedSet,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    if manifest.affected_set_digest() != affected_set.affected_set_digest() {
        return Err(classification_error(
            "subscription-support capsule manifest must bind the admitted affected set",
        ));
    }
    Ok(())
}

fn validate_decision_coverage(
    decision: &SubscriptionSupportPortabilityDecision,
    affected_set: &SupportPortabilityAffectedSet,
    footprint: &SupportPortabilityScopeFootprint,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    match decision.kind() {
        SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
        | SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted => {
            validate_exact_coverage(affected_set, footprint, manifest)
        }
        SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable => {
            validate_not_resumable_coverage(affected_set, footprint, manifest)
        }
        SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission => {
            validate_partial_coverage(decision, affected_set, footprint, manifest)
        }
        SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected => {
            validate_rejection_coverage(footprint, manifest)
        }
    }
}

fn validate_exact_coverage(
    affected_set: &SupportPortabilityAffectedSet,
    footprint: &SupportPortabilityScopeFootprint,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    if footprint.omitted_support_count() != 0
        || manifest.omitted_support_count() != 0
        || manifest.manifest_entry_count() != affected_set.affected_count()
        || manifest.required_basis_count() != affected_set.affected_count()
    {
        return Err(classification_error(
            "exact subscription-support portability requires full-scope manifest coverage",
        ));
    }
    Ok(())
}

fn validate_not_resumable_coverage(
    affected_set: &SupportPortabilityAffectedSet,
    footprint: &SupportPortabilityScopeFootprint,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    if footprint.omitted_support_count() != 0
        || manifest.omitted_support_count() != 0
        || manifest.manifest_entry_count() != affected_set.affected_count()
    {
        return Err(classification_error(
            "not-resumable support import still requires full-scope support manifest coverage",
        ));
    }
    if manifest.required_basis_count() >= manifest.manifest_entry_count() {
        return Err(classification_error(
            "missing-basis not-resumable support import requires missing basis evidence",
        ));
    }
    Ok(())
}

fn validate_partial_coverage(
    decision: &SubscriptionSupportPortabilityDecision,
    affected_set: &SupportPortabilityAffectedSet,
    footprint: &SupportPortabilityScopeFootprint,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    let omitted_artifact_ids = decision.omitted_artifact_ids_for_scope(affected_set);
    validate_omitted_artifact_ids(affected_set, &omitted_artifact_ids)?;
    if footprint.omitted_support_count() == 0 || manifest.omitted_support_count() == 0 {
        return Err(classification_error(
            "partial subscription-support portability requires a non-empty omission footprint",
        ));
    }
    Ok(())
}

fn validate_rejection_coverage(
    footprint: &SupportPortabilityScopeFootprint,
    manifest: &CapsuleSupportManifest,
) -> Result<(), StoreError> {
    if footprint.included_support_count() != 0
        || manifest.manifest_entry_count() != 0
        || manifest.required_basis_count() != 0
    {
        return Err(classification_error(
            "unsupported subscription-support portability rejection cannot include support or basis evidence",
        ));
    }
    Ok(())
}
