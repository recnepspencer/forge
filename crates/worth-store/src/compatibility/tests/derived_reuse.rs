use super::super::{
    admission, admit_derived_rebuild_maintenance, check_artifact_with_read_receipt, derived,
    plan_read_compatibility, prove_retained_authority_for_derived_rebuild,
    ArtifactCompatibilityWindow, ArtifactSemanticVersion, CompatibilityAdmissionBatch,
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityFamilyKind,
    CompatibilityManifestIndex, CompatibilityReadIntent, CompatibilityRegistry,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityEdge,
    DerivedBasisCompatibilityPosture, DerivedInvalidationReason, DerivedRebuildRequirement,
    DerivedReusePosture, ReaderCapabilitySet,
};
use super::{
    derived_family_declaration, derived_rebuild_plan_for_test, native_edge,
    quarantined_artifact_for_family, quarantined_artifact_for_versions, synthetic_read_receipt,
};

#[test]
fn compatibility_native_read_receipt_admits_exact_derived_reuse() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(family_id.clone())]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("native derived read should admit");
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let plan =
        derived::plan_exact_derived_reuse(batch.counters_mut(), &declaration, &artifact, &receipt)
            .expect("native receipt should admit exact derived reuse");
    assert_eq!(plan.posture(), DerivedReusePosture::ReuseAdmitted);
    assert!(plan.reuse_receipt().is_some());

    let checked = admission::check_artifact_with_read_receipt(artifact, &receipt)
        .expect("native receipt should check artifact");
    let witness = derived::admit_checked_derived_reuse(checked, &plan)
        .expect("checked native artifact should produce reuse witness");
    assert_eq!(witness.family_id().as_str(), "snapshot_record");
}

#[test]
fn compatibility_non_native_read_receipt_requires_derived_rebuild() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
    let mut batch = CompatibilityAdmissionBatch::new();
    let edge = DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::BackwardRead,
    );
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![edge]),
        &ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]),
        &CompatibilityReadIntent::new(family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("declared non-native read should admit read");
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let plan =
        derived::plan_exact_derived_reuse(batch.counters_mut(), &declaration, &artifact, &receipt)
            .expect("non-native read should become rebuild plan");
    assert_eq!(plan.posture(), DerivedReusePosture::RebuildRequired);
    assert!(plan.reuse_receipt().is_none());
    assert_eq!(batch.counters().derived_rebuild_required_count(), 1);

    let requirement =
        DerivedRebuildRequirement::from_reuse_plan(&plan, ArtifactCompatibilityWindow::native(1))
            .expect("rebuild plan should produce a rebuild requirement");
    assert_eq!(requirement.family_id().as_str(), "snapshot_record");
}

#[test]
fn compatibility_mismatched_receipt_rejects_derived_reuse() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let receipt_family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact_family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let receipt_artifact =
        quarantined_artifact_for_family(receipt_family_id.clone(), 1, "authoritative");
    let derived_artifact =
        quarantined_artifact_for_family(artifact_family_id.clone(), 1, "derived");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(receipt_family_id.clone())]),
        &ReaderCapabilitySet::new(
            receipt_family_id.clone(),
            vec![ArtifactSemanticVersion::new(1)],
        ),
        &CompatibilityReadIntent::new(receipt_family_id, ArtifactSemanticVersion::new(1)),
        &receipt_artifact,
    )
    .expect("receipt source should admit");
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let rejection = derived::plan_exact_derived_reuse(
        batch.counters_mut(),
        &declaration,
        &derived_artifact,
        &receipt,
    )
    .expect_err("mismatched receipt must reject derived reuse");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DerivedReuseIncompatible
    );
    assert_eq!(
        rejection.store_error_kind(),
        crate::StoreErrorKind::CompatibilityDerivedReuseIncompatible
    );
    assert_eq!(batch.counters().derived_reuse_incompatibility_count(), 1);
}

#[test]
fn compatibility_authoritative_receipt_cannot_admit_derived_reuse() {
    let snapshot = CompatibilityRegistry::first_ship();
    let index = CompatibilityManifestIndex::rebuild_from_registry(&snapshot);
    let authoritative_family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let artifact =
        quarantined_artifact_for_family(authoritative_family_id.clone(), 1, "authoritative");
    let mut batch = CompatibilityAdmissionBatch::new();
    let receipt = admission::plan_read_compatibility(
        &mut batch,
        &index,
        &CompatibilityEdgeRegistry::new(vec![native_edge(authoritative_family_id.clone())]),
        &ReaderCapabilitySet::new(
            authoritative_family_id.clone(),
            vec![ArtifactSemanticVersion::new(1)],
        ),
        &CompatibilityReadIntent::new(authoritative_family_id, ArtifactSemanticVersion::new(1)),
        &artifact,
    )
    .expect("authoritative read should admit");
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let rejection =
        derived::plan_exact_derived_reuse(batch.counters_mut(), &declaration, &artifact, &receipt)
            .expect_err("authoritative artifact cannot satisfy a derived family declaration");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DerivedReuseIncompatible
    );
}

#[test]
fn compatibility_derived_basis_format_drift_invalidates_without_runtime_rebuild() {
    let snapshot = CompatibilityRegistry::first_ship();
    let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let artifact = quarantined_artifact_for_versions(family_id.clone(), 2, 1, 1, "derived");
    let receipt = synthetic_read_receipt(
        &artifact,
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::Native,
    );
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = derived::plan_derived_basis_compatibility(
        &mut counters,
        &declaration,
        &artifact,
        &receipt,
        ArtifactCompatibilityWindow::native(1),
    )
    .expect("format drift should produce an invalidation plan");
    assert_eq!(
        plan.posture(),
        DerivedBasisCompatibilityPosture::InvalidateAndRebuild
    );
    assert_eq!(
        plan.invalidation().expect("invalidation").reason_code(),
        DerivedInvalidationReason::FormatWindowMismatch
    );
    assert!(plan.rebuild_requirement().is_some());
    assert_eq!(counters.derived_invalidation_count(), 1);
    assert_eq!(counters.derived_rebuild_required_count(), 1);
}

#[test]
fn compatibility_derived_basis_non_native_relation_invalidates() {
    let snapshot = CompatibilityRegistry::first_ship();
    let family_id = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let artifact = quarantined_artifact_for_family(family_id.clone(), 1, "derived");
    let receipt = synthetic_read_receipt(
        &artifact,
        ArtifactSemanticVersion::new(1),
        CompatibilityRelation::ForwardRead,
    );
    let declaration =
        derived_family_declaration(&snapshot, CompatibilityFamilyKind::SnapshotRecord);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = derived::plan_derived_basis_compatibility(
        &mut counters,
        &declaration,
        &artifact,
        &receipt,
        ArtifactCompatibilityWindow::native(1),
    )
    .expect("non-native read should force derived rebuild");
    assert_eq!(
        plan.invalidation().expect("invalidation").reason_code(),
        DerivedInvalidationReason::NonNativeReadRelation
    );
}

#[test]
fn compatibility_derived_rebuild_requires_retained_authority() {
    let requirement = DerivedRebuildRequirement::from_reuse_plan(
        &derived_rebuild_plan_for_test(),
        ArtifactCompatibilityWindow::native(1),
    )
    .expect("rebuild requirement");
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection =
        derived::admit_derived_rebuild_maintenance(&mut counters, &requirement, None, None)
            .expect_err("retained authority is required before rebuild admission");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DerivedStaleVersion
    );
    assert_eq!(counters.derived_stale_version_rejection_count(), 1);
}

#[test]
fn compatibility_derived_rebuild_requires_maintenance_admission() {
    let requirement = DerivedRebuildRequirement::from_reuse_plan(
        &derived_rebuild_plan_for_test(),
        ArtifactCompatibilityWindow::native(1),
    )
    .expect("rebuild requirement");
    let mut counters = CompatibilityAdmissionCounters::default();
    let authority =
        derived::prove_retained_authority_for_derived_rebuild(requirement.family_id().clone());
    let rejection = derived::admit_derived_rebuild_maintenance(
        &mut counters,
        &requirement,
        Some(&authority),
        None,
    )
    .expect_err("maintenance admission is required before rebuild planning");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::DerivedRebuildAdmissionRejected
    );
    assert_eq!(
        counters.maintenance_compatibility_rebuild_rejection_count(),
        1
    );
}

#[test]
fn compatibility_derived_rebuild_admission_and_debt_are_counted() {
    let requirement = DerivedRebuildRequirement::from_reuse_plan(
        &derived_rebuild_plan_for_test(),
        ArtifactCompatibilityWindow::native(1),
    )
    .expect("rebuild requirement");
    let mut counters = CompatibilityAdmissionCounters::default();
    let debt = derived::defer_derived_rebuild(&mut counters, &requirement, 3);
    assert_eq!(debt.debt_record_count(), 3);
    assert_eq!(counters.derived_rebuild_debt_count(), 3);

    let authority =
        derived::prove_retained_authority_for_derived_rebuild(requirement.family_id().clone());
    let maintenance = derived::prove_maintenance_admission_for_derived_rebuild(
        &mut counters,
        requirement.family_id().clone(),
        "m11-derived-rebuild-lane",
    );
    let rebuild = derived::admit_derived_rebuild_maintenance(
        &mut counters,
        &requirement,
        Some(&authority),
        Some(&maintenance),
    )
    .expect("matching authority and maintenance proofs should admit rebuild plan");
    assert_eq!(rebuild.family_id(), requirement.family_id());
    assert_eq!(rebuild.maintenance_lane_id(), "m11-derived-rebuild-lane");
    assert_eq!(
        counters.maintenance_compatibility_rebuild_admission_count(),
        1
    );
}
