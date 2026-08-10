use super::super::admission::{
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityEdge,
};

use worth_store_contracts::CompatibilityFamilyKind;

use super::super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
};

use super::super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactSemanticVersion,
};

use super::super::restore::{
    execute_restore_publication, plan_disaster_recovery_compatibility, plan_restore_compatibility,
    DisasterRecoveryCompatibilityClass, DisasterRecoveryCompatibilityWindow, RestoreBackupScope,
    RestoreCompatibilityTarget, RestorePublicationConflictKind, RestorePublicationConflictSet,
    RestorePublicationConflictUnit,
};

use super::scenario_inputs::{backup_manifest, lane_input};

pub(super) fn restore_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id.clone(),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::BackwardRead,
    )]);
    vec![
        restore_lane(
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            edge_registry.clone(),
            Some(CompatibilityRelation::BackwardRead),
            None,
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestoreOutOfScopeRejected,
            family_id.clone(),
            CompatibilityFamilyKind::SnapshotRecord.family_id(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            edge_registry.clone(),
            None,
            Some(CompatibilityRejectionKind::RestoreOutOfScopeScanRejected),
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestorePublicationConflictRejected,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(vec![RestorePublicationConflictUnit::new(
                family_id.clone(),
                RestorePublicationConflictKind::BranchHead,
            )]),
            edge_registry,
            None,
            Some(CompatibilityRejectionKind::RestorePublicationConflictRejected),
        ),
        restore_lane(
            Milestone12CertificationLaneKind::RestoreMissingEdgeRejected,
            family_id.clone(),
            family_id.clone(),
            RestoreBackupScope::new(vec![family_id.clone()]),
            RestorePublicationConflictSet::new(Vec::new()),
            CompatibilityEdgeRegistry::new(Vec::new()),
            None,
            Some(CompatibilityRejectionKind::MissingCompatibilityEdge),
        ),
    ]
}

fn restore_lane(
    lane_kind: Milestone12CertificationLaneKind,
    backup_family_id: ArtifactFamilyId,
    target_family_id: ArtifactFamilyId,
    backup_scope: RestoreBackupScope,
    conflicts: RestorePublicationConflictSet,
    edge_registry: CompatibilityEdgeRegistry,
    expected_relation: Option<CompatibilityRelation>,
    expected_rejection: Option<CompatibilityRejectionKind>,
) -> Milestone12CertificationLaneOutcome {
    let backup_manifest = backup_manifest(backup_family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(target_family_id.clone(), ArtifactSemanticVersion::new(2));
    let mut counters = CompatibilityAdmissionCounters::default();
    let input = lane_input(
        target_family_id,
        1,
        2,
        expected_relation,
        expected_rejection,
    );
    match plan_restore_compatibility(
        &mut counters,
        &edge_registry,
        &backup_scope,
        &backup_manifest,
        &target,
        &conflicts,
    ) {
        Ok(plan) => {
            let receipt = execute_restore_publication(plan);
            Milestone12CertificationLaneOutcome::from_restore_receipt(input, &receipt, &counters)
        }
        Err(rejection) => Milestone12CertificationLaneOutcome::from_compatibility_rejection(
            lane_kind, input, &rejection, &counters,
        ),
    }
}

pub(super) fn disaster_recovery_lanes() -> Vec<Milestone12CertificationLaneOutcome> {
    vec![
        disaster_recovery_lane(
            Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow,
            CompatibilityFamilyKind::CommitEnvelope.family_id(),
            DisasterRecoveryCompatibilityClass::AuthoritativeTruth,
        ),
        disaster_recovery_lane(
            Milestone12CertificationLaneKind::DisasterRecoveryDerivedWindow,
            CompatibilityFamilyKind::SnapshotRecord.family_id(),
            DisasterRecoveryCompatibilityClass::DerivedAcceleration,
        ),
    ]
}

fn disaster_recovery_lane(
    lane_kind: Milestone12CertificationLaneKind,
    family_id: ArtifactFamilyId,
    class: DisasterRecoveryCompatibilityClass,
) -> Milestone12CertificationLaneOutcome {
    let mut counters = CompatibilityAdmissionCounters::default();
    let window = DisasterRecoveryCompatibilityWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::native(1),
        class,
    );
    let plan = plan_disaster_recovery_compatibility(&mut counters, &window);
    Milestone12CertificationLaneOutcome::from_disaster_recovery_plan(
        lane_kind,
        lane_input(family_id, 1, 1, None, None),
        &plan,
        &counters,
    )
}
