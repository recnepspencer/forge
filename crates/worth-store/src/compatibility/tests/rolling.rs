use super::super::{
    plan_first_ship_rolling_upgrade, rolling, ArtifactCompatibilityWindow, ArtifactFormatVersion,
    ArtifactSemanticVersion, CompatibilityAdapterCostClass, CompatibilityAdmissionCounters,
    CompatibilityEdgeRegistry, CompatibilityFamilyKind, CompatibilityRejectionKind,
    CompatibilityRelation, DeclaredCompatibilityEdge, MixedVersionPostureKind, ReaderCapabilitySet,
    RollingUpgradePolicy, RollingUpgradeWindow, WriterCapabilitySet,
};
use super::Milestone12AdmissionReport;
use super::{adapter, native_edge};

#[test]
fn compatibility_rolling_two_capability_window_admits() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = RollingUpgradeWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    );
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::ForwardRead,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect("one reader plus one writer inside declared window should admit");
    assert_eq!(plan.policy(), RollingUpgradePolicy::FirstShipTwoCapability);
    assert_eq!(plan.relation(), CompatibilityRelation::ForwardRead);
    assert_eq!(
        plan.store_posture().posture(),
        &MixedVersionPostureKind::AdmittedTwoCapabilityWindow
    );
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.rolling_window_admission_count(), 1);
}

#[test]
fn compatibility_rolling_multi_writer_rejects() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window =
        RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let first_writer =
        WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let second_writer = WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(1)]);
    let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[first_writer, second_writer],
    )
    .expect_err("multi-writer first-ship rolling window should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RollingMultiWriterRejected
    );
    assert_eq!(counters.rolling_multi_writer_rejection_count(), 1);
}

#[test]
fn compatibility_rolling_skew_outside_window_rejects() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window =
        RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::ForwardRead,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect_err("writer outside rolling semantic window should reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MixedVersionSkewRejected
    );
    assert_eq!(counters.mixed_version_skew_count(), 1);
    assert_eq!(counters.rolling_window_rejection_count(), 1);
}

#[test]
fn compatibility_rolling_counters_project_to_milestone_12_report() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window =
        RollingUpgradeWindow::new(family_id.clone(), ArtifactCompatibilityWindow::native(1));
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![native_edge(family_id)]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect("rolling window should admit");
    assert_eq!(plan.relation(), CompatibilityRelation::Native);
    let report = crate::Milestone12AdmissionReport::from_admission_counters(&counters);
    assert_eq!(report.rolling_window_admission_count, 1);
    assert_eq!(report.rolling_window_rejection_count, 0);
}

#[test]
fn compatibility_rolling_missing_edge_rejects_numeric_proximity() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = RollingUpgradeWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    );
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(2)]);
    let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect_err("numeric proximity without a declared edge must reject");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::MissingCompatibilityEdge
    );
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.edge_missing_rejection_count(), 1);
    assert_eq!(counters.rolling_window_rejection_count(), 1);
}

#[test]
fn compatibility_rolling_single_set_multi_version_rejects() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = RollingUpgradeWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    );
    let reader = ReaderCapabilitySet::new(
        family_id.clone(),
        vec![
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ],
    );
    let writer = WriterCapabilitySet::new(family_id, vec![ArtifactSemanticVersion::new(2)]);
    let edge_registry = CompatibilityEdgeRegistry::new(Vec::new());
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect_err("a single capability set cannot hide multiple semantic versions");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RollingWindowRejected
    );
    assert_eq!(counters.relation_recheck_count(), 0);
    assert_eq!(counters.rolling_window_rejection_count(), 1);
}

#[test]
fn compatibility_rolling_adapter_edge_rejects_without_execution() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = RollingUpgradeWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    );
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    let edge = DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::AdapterRequired,
    )
    .with_adapter(adapter(CompatibilityAdapterCostClass::BoundedRecordLocal));
    let edge_registry = CompatibilityEdgeRegistry::new(vec![edge]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let rejection = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect_err("first-ship rolling policy must not execute adapter edges");
    assert_eq!(
        rejection.kind(),
        CompatibilityRejectionKind::RollingWindowRejected
    );
    assert_eq!(counters.relation_recheck_count(), 1);
    assert_eq!(counters.rolling_window_rejection_count(), 1);
    assert_eq!(counters.adapter_hot_path_rejection_count(), 0);
}
