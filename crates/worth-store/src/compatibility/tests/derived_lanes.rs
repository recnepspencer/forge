use super::super::{
    admission, derived, plan_derived_lane_compatibility,
    prove_compatibility_maintenance_lane_admission, require_matching_maintenance_lane,
    CompatibilityAdmissionCounters, CompatibilityAuthorityClassification, CompatibilityFamilyKind,
    CompatibilityMaintenanceLaneRequirement, CompatibilityRegistry, CompatibilityRejectionKind,
    CompatibilityRelation, DerivedCompatibilityLaneKind, DerivedCompatibilityLaneRegistry,
    DerivedLaneCompatibilityPosture, TierCompatibilityNonAuthorityPosture,
};
use super::derived_lane_fixture;
use super::Milestone12AdmissionReport;

#[test]
fn compatibility_derived_lane_registry_covers_every_derived_family_once() {
    let snapshot = CompatibilityRegistry::first_ship();
    let lanes = DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
    let derived_family_count = snapshot
        .declarations()
        .iter()
        .filter(|declaration| {
            declaration.authority_classification() == CompatibilityAuthorityClassification::Derived
        })
        .count();
    assert_eq!(lanes.declarations().len(), derived_family_count);
    for declaration in snapshot.declarations().iter().filter(|declaration| {
        declaration.authority_classification() == CompatibilityAuthorityClassification::Derived
    }) {
        assert!(
            lanes.get_by_family_kind(declaration.kind()).is_some(),
            "missing derived lane for {:?}",
            declaration.kind()
        );
    }
}

#[test]
fn compatibility_derived_lane_snapshot_is_deterministic() {
    let snapshot = CompatibilityRegistry::first_ship();
    let first = DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
    let second =
        DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
    let first_lanes: Vec<_> = first
        .declarations()
        .iter()
        .map(|declaration| declaration.lane_kind())
        .collect();
    let second_lanes: Vec<_> = second
        .declarations()
        .iter()
        .map(|declaration| declaration.lane_kind())
        .collect();
    assert_eq!(first_lanes, second_lanes);
}

#[test]
fn compatibility_snapshot_lane_admits_exact_native_reuse() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::SnapshotRecord,
        CompatibilityRelation::Native,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
        .expect("native snapshot lane should admit exact reuse");
    assert_eq!(
        plan.lane_kind(),
        DerivedCompatibilityLaneKind::SnapshotReuse
    );
    assert_eq!(
        plan.posture(),
        DerivedLaneCompatibilityPosture::ReuseAdmitted
    );
    assert_eq!(counters.derived_lane_reuse_count(), 1);
    assert_eq!(counters.derived_snapshot_reuse_count(), 1);
}

#[test]
fn compatibility_delta_lane_admits_exact_native_reuse() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::DeltaRecord,
        CompatibilityRelation::Native,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
        .expect("native delta lane should admit exact reuse");
    assert_eq!(
        plan.lane_kind(),
        DerivedCompatibilityLaneKind::BranchDeltaReuse
    );
    assert_eq!(counters.derived_delta_reuse_count(), 1);
}

#[test]
fn compatibility_layout_lane_rejects_basis_drift() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
        CompatibilityRelation::Native,
        2,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection =
        derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
            .expect_err("layout lane must reject basis drift");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DerivedBasisIncompatible
    );
    assert_eq!(counters.derived_layout_basis_rejection_count(), 1);
}

#[test]
fn compatibility_bulk_resume_lane_rejects_changed_interpretation() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::Milestone9BulkRecord,
        CompatibilityRelation::ForwardRead,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection =
        derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
            .expect_err("bulk resume must reject changed interpretation");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::BulkResumeCompatibilityRejected
    );
    assert_eq!(counters.derived_bulk_resume_rejection_count(), 1);
}

#[test]
fn compatibility_tier_manifest_preserves_non_authority() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::Milestone13TieringRecord,
        CompatibilityRelation::Native,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
        .expect("native tier manifest should admit only placement support");
    assert_eq!(
        plan.tier_manifest().expect("tier plan").posture(),
        TierCompatibilityNonAuthorityPosture::PlacementSupportOnly
    );
    assert_eq!(counters.tier_non_authority_preserved_count(), 1);
}

#[test]
fn compatibility_tier_manifest_skew_rejects_without_authority() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::Milestone13TieringRecord,
        CompatibilityRelation::ForwardRead,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection =
        derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
            .expect_err("tier manifest drift should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::TierManifestCompatibilityRejected
    );
    assert_eq!(counters.tier_manifest_rejection_count(), 1);
}

#[test]
fn compatibility_maintenance_lane_requires_matching_work_class() {
    let family_id = CompatibilityFamilyKind::Milestone11MaintenanceRecord.family_id();
    let requirement = CompatibilityMaintenanceLaneRequirement::new(
        family_id.clone(),
        "certification.derived.lane.maintenance_summary_support",
        "DerivedFamilyRebuild",
    );
    let wrong_requirement = CompatibilityMaintenanceLaneRequirement::new(
        family_id,
        "certification.derived.lane.maintenance_summary_support",
        "MaintenanceAudit",
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let admission = derived::prove_compatibility_maintenance_lane_admission(
        &mut counters,
        &requirement,
        "m11-derived-rebuild-lane",
    );
    let rejection =
        derived::require_matching_maintenance_lane(&mut counters, &wrong_requirement, &admission)
            .expect_err("maintenance work class mismatch should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MaintenanceLaneMismatch
    );
    assert_eq!(counters.maintenance_lane_mismatch_rejection_count(), 1);
}

#[test]
fn compatibility_derived_lane_counters_project_to_milestone_12_report() {
    let (input, artifact, receipt) = derived_lane_fixture(
        CompatibilityFamilyKind::SnapshotRecord,
        CompatibilityRelation::Native,
        1,
        1,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let _ = derived::plan_derived_lane_compatibility(&mut counters, &input, &artifact, &receipt)
        .expect("snapshot lane should admit");
    let report = crate::Milestone12AdmissionReport::from_admission_counters(&counters);
    assert_eq!(report.derived_lane_plan_count, 1);
    assert_eq!(report.derived_lane_reuse_count, 1);
    assert_eq!(report.derived_snapshot_reuse_count, 1);
}
