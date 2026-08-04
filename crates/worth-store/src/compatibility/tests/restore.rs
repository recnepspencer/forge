use super::super::{
    plan_restore_compatibility, restore, ArtifactCompatibilityWindow, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityFamilyKind,
    CompatibilityRejectionKind, CompatibilityRelation, DeclaredCompatibilityEdge,
    DisasterRecoveryCompatibilityClass, DisasterRecoveryCompatibilityWindow, RestoreBackupScope,
    RestoreCompatibilityTarget, RestorePublicationConflictKind, RestorePublicationConflictSet,
    RestorePublicationConflictUnit,
};
use super::{backup_manifest_for_family, native_edge};

#[test]
fn compatibility_restore_admits_scoped_backup_with_declared_edge() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(1));
    let scope = RestoreBackupScope::new(vec![family_id.clone()]);
    let conflicts = RestorePublicationConflictSet::new(Vec::new());
    let edges = CompatibilityEdgeRegistry::new(vec![native_edge(family_id)]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = restore::plan_restore_compatibility(
        &mut counters,
        &edges,
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect("scoped restore with declared native edge should admit");
    assert_eq!(plan.relation(), CompatibilityRelation::Native);
    assert_eq!(plan.publication_conflict_count(), 0);
    assert_eq!(counters.restore_accept_count(), 1);
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.artifact_row_scan_count(), 0);
}

#[test]
fn compatibility_restore_rejects_out_of_scope_target_before_edge_scan() {
    let backup_family = CompatibilityFamilyKind::SnapshotRecord.family_id();
    let target_family = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(backup_family.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(target_family.clone(), ArtifactSemanticVersion::new(1));
    let scope = RestoreBackupScope::new(vec![backup_family]);
    let conflicts = RestorePublicationConflictSet::new(Vec::new());
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = restore::plan_restore_compatibility(
        &mut counters,
        &CompatibilityEdgeRegistry::default(),
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect_err("restore must not scan target families outside backup scope");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RestoreOutOfScopeScanRejected
    );
    assert_eq!(counters.restore_out_of_scope_scan_count(), 1);
    assert_eq!(counters.restore_rejection_count(), 1);
    assert_eq!(counters.relation_recheck_count(), 0);
}

#[test]
fn compatibility_restore_rejects_publication_conflicts_before_witness() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(1));
    let scope = RestoreBackupScope::new(vec![family_id.clone()]);
    let conflicts = RestorePublicationConflictSet::new(vec![RestorePublicationConflictUnit::new(
        family_id.clone(),
        RestorePublicationConflictKind::BranchHead,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = restore::plan_restore_compatibility(
        &mut counters,
        &CompatibilityEdgeRegistry::default(),
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect_err("publication conflicts must reject before restore witness construction");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RestorePublicationConflictRejected
    );
    assert_eq!(counters.restore_publication_conflict_rejection_count(), 1);
    assert_eq!(counters.restore_rejection_count(), 1);
    assert_eq!(counters.relation_recheck_count(), 0);
}

#[test]
fn compatibility_restore_missing_edge_rejects_numeric_proximity() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
    let scope = RestoreBackupScope::new(vec![family_id]);
    let conflicts = RestorePublicationConflictSet::new(Vec::new());
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = restore::plan_restore_compatibility(
        &mut counters,
        &CompatibilityEdgeRegistry::default(),
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect_err("restore must not infer compatibility from adjacent semantic versions");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MissingCompatibilityEdge
    );
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.edge_missing_rejection_count(), 1);
    assert_eq!(counters.restore_rejection_count(), 1);
}

#[test]
fn compatibility_restore_incompatible_edge_rejects_before_publication_witness() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
    let scope = RestoreBackupScope::new(vec![family_id.clone()]);
    let conflicts = RestorePublicationConflictSet::new(Vec::new());
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::Incompatible,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = restore::plan_restore_compatibility(
        &mut counters,
        &edge_registry,
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect_err("incompatible restore edge must reject before witness construction");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RestoreCompatibilityRejected
    );
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.restore_rejection_count(), 1);
    assert_eq!(counters.restore_accept_count(), 0);
}

#[test]
fn compatibility_disaster_recovery_windows_distinguish_truth_from_derived_acceleration() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let truth_window = DisasterRecoveryCompatibilityWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::native(1),
        DisasterRecoveryCompatibilityClass::AuthoritativeTruth,
    );
    let derived_window = DisasterRecoveryCompatibilityWindow::new(
        CompatibilityFamilyKind::SnapshotRecord.family_id(),
        ArtifactCompatibilityWindow::native(1),
        DisasterRecoveryCompatibilityClass::DerivedAcceleration,
    );
    let mut counters = CompatibilityAdmissionCounters::default();
    let truth_plan = restore::plan_disaster_recovery_compatibility(&mut counters, &truth_window);
    let derived_plan =
        restore::plan_disaster_recovery_compatibility(&mut counters, &derived_window);
    assert_eq!(
        truth_plan.class(),
        DisasterRecoveryCompatibilityClass::AuthoritativeTruth
    );
    assert_eq!(
        derived_plan.class(),
        DisasterRecoveryCompatibilityClass::DerivedAcceleration
    );
    assert_eq!(counters.disaster_recovery_truth_window_count(), 1);
    assert_eq!(counters.disaster_recovery_derived_window_count(), 1);
}
