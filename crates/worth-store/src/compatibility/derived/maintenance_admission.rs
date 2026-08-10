use super::super::admission::{
    CompatibilityAdmissionCounters, CompatibilityRejection, CompatibilityRejectionKind,
};

use super::super::decoding::QuarantinedDecodedArtifact;

use super::super::manifests::{ArtifactCompatibilityWindow, ArtifactFamilyId};

use super::maintenance::{
    CompatibilityMaintenanceAdmissionWitness, CompatibilityMaintenanceLaneAdmission,
    CompatibilityMaintenanceLaneRejection, CompatibilityMaintenanceLaneRequirement,
    CompatibilityRebuildDebt, DerivedRebuildCompatibilityPlan,
    RetainedAuthorityCompatibilityWitness,
};

use super::declarations::DerivedInvalidationReason;
use super::reuse::DerivedInvalidationPlan;
use super::reuse::{DerivedBasisCompatibilityPlan, DerivedRebuildRequirement};

pub(super) fn plan_invalidation(
    counters: &mut CompatibilityAdmissionCounters,
    artifact: &QuarantinedDecodedArtifact,
    required_window: ArtifactCompatibilityWindow,
    reason_code: DerivedInvalidationReason,
    reason: &'static str,
) -> DerivedBasisCompatibilityPlan {
    counters.record_derived_invalidation();
    counters.record_derived_rebuild_required();
    DerivedBasisCompatibilityPlan::rebuild(DerivedInvalidationPlan::new(
        artifact.family_id().clone(),
        artifact.format_version(),
        artifact.semantic_version(),
        required_window,
        reason_code,
        reason,
    ))
}

pub(crate) fn prove_retained_authority_for_derived_rebuild(
    family_id: ArtifactFamilyId,
) -> RetainedAuthorityCompatibilityWitness {
    RetainedAuthorityCompatibilityWitness::new(family_id)
}

pub(crate) fn prove_maintenance_admission_for_derived_rebuild(
    counters: &mut CompatibilityAdmissionCounters,
    family_id: ArtifactFamilyId,
    maintenance_lane_id: impl Into<String>,
) -> CompatibilityMaintenanceAdmissionWitness {
    counters.record_maintenance_compatibility_rebuild_admission();
    CompatibilityMaintenanceAdmissionWitness::new(family_id, maintenance_lane_id)
}

pub(crate) fn prove_compatibility_maintenance_lane_admission(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &CompatibilityMaintenanceLaneRequirement,
    maintenance_lane_id: impl Into<String>,
) -> CompatibilityMaintenanceLaneAdmission {
    counters.record_maintenance_compatibility_rebuild_admission();
    CompatibilityMaintenanceLaneAdmission::new(CompatibilityMaintenanceAdmissionWitness::for_lane(
        requirement.family_id.clone(),
        requirement.compatibility_lane_id.clone(),
        maintenance_lane_id,
        requirement.maintenance_work_class_label.clone(),
    ))
}

pub(crate) fn require_matching_maintenance_lane(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &CompatibilityMaintenanceLaneRequirement,
    admission: &CompatibilityMaintenanceLaneAdmission,
) -> Result<(), CompatibilityRejection> {
    let witness = admission.witness();
    if witness.family_id() != requirement.family_id()
        || witness.compatibility_lane_id() != requirement.compatibility_lane_id
        || witness.maintenance_work_class_label() != requirement.maintenance_work_class_label
    {
        counters.record_maintenance_lane_mismatch_rejection();
        let _rejection = CompatibilityMaintenanceLaneRejection::new(
            requirement.family_id().clone(),
            "maintenance lane admission does not match compatibility lane requirement",
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MaintenanceLaneMismatch,
            requirement.family_id().clone(),
            "maintenance lane admission does not match compatibility family, lane, or work class",
        ));
    }
    Ok(())
}

pub(crate) fn defer_derived_rebuild(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &DerivedRebuildRequirement,
    debt_record_count: u64,
) -> CompatibilityRebuildDebt {
    counters.record_derived_rebuild_debt(debt_record_count);
    CompatibilityRebuildDebt::new(requirement.family_id().clone(), debt_record_count)
}

pub(crate) fn admit_derived_rebuild_maintenance(
    counters: &mut CompatibilityAdmissionCounters,
    requirement: &DerivedRebuildRequirement,
    retained_authority: Option<&RetainedAuthorityCompatibilityWitness>,
    maintenance_admission: Option<&CompatibilityMaintenanceAdmissionWitness>,
) -> Result<DerivedRebuildCompatibilityPlan, CompatibilityRejection> {
    let Some(retained_authority) = retained_authority else {
        counters.record_derived_stale_version_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedStaleVersion,
            requirement.family_id().clone(),
            "derived rebuild requires retained authoritative basis proof",
        ));
    };
    if retained_authority.family_id() != requirement.family_id() {
        counters.record_derived_rebuild_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedBasisIncompatible,
            requirement.family_id().clone(),
            "retained authoritative basis proof belongs to a different family",
        ));
    }

    let Some(maintenance_admission) = maintenance_admission else {
        counters.record_maintenance_compatibility_rebuild_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedRebuildAdmissionRejected,
            requirement.family_id().clone(),
            "derived rebuild requires Milestone 11 maintenance admission proof",
        ));
    };
    if maintenance_admission.family_id() != requirement.family_id() {
        counters.record_derived_rebuild_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedRebuildAdmissionRejected,
            requirement.family_id().clone(),
            "maintenance admission proof belongs to a different family",
        ));
    }

    Ok(DerivedRebuildCompatibilityPlan::new(
        requirement.family_id().clone(),
        requirement.observed_semantic_version(),
        requirement.target_semantic_version(),
        maintenance_admission.maintenance_lane_id().to_string(),
    ))
}
