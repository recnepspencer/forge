use crate::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityFamilyKind, CompatibilityRelation, CompatibilityRollingPublicationRequest,
    ForgeStoreBuilder, Milestone12CertificationLaneKind, Milestone12CertificationLaneStatus,
    Milestone12CertificationRunner, ReaderCapabilitySet, RollingUpgradeWindow, StoreErrorKind,
    WriterCapabilitySet,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn rolling_commit_publication_executes_through_public_store_path() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let outcome = store
        .append_canonical_commit_with_rolling_compatibility(
            rolling_request(
                CompatibilityFamilyKind::CommitEnvelope.family_id(),
                vec![ArtifactSemanticVersion::new(1)],
                vec![ArtifactSemanticVersion::new(2)],
            ),
            envelope.clone(),
        )
        .unwrap();

    assert_eq!(outcome.relation(), CompatibilityRelation::ForwardRead);
    assert_eq!(
        outcome.store_posture().posture(),
        &crate::MixedVersionPostureKind::AdmittedTwoCapabilityWindow
    );
    assert_eq!(
        outcome.replica_posture().posture(),
        &crate::MixedVersionPostureKind::AdmittedTwoCapabilityWindow
    );
    assert_eq!(outcome.persisted_commit().envelope(), &envelope);
    assert_eq!(outcome.admission_report().rolling_window_admission_count, 1);
    assert_eq!(outcome.admission_report().relation_recheck_count, 1);
}

#[test]
fn rolling_commit_publication_rejects_multi_writer_window() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let error = store
        .append_canonical_commit_with_rolling_compatibility(
            CompatibilityRollingPublicationRequest::new(
                rolling_window(CompatibilityFamilyKind::CommitEnvelope.family_id()),
                vec![ReaderCapabilitySet::new(
                    CompatibilityFamilyKind::CommitEnvelope.family_id(),
                    vec![ArtifactSemanticVersion::new(1)],
                )],
                vec![
                    WriterCapabilitySet::new(
                        CompatibilityFamilyKind::CommitEnvelope.family_id(),
                        vec![ArtifactSemanticVersion::new(2)],
                    ),
                    WriterCapabilitySet::new(
                        CompatibilityFamilyKind::CommitEnvelope.family_id(),
                        vec![ArtifactSemanticVersion::new(2)],
                    ),
                ],
            ),
            envelope,
        )
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityRollingUpgradeRejected
    );
}

#[test]
fn rolling_commit_publication_rejects_missing_edge_window() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let error = store
        .append_canonical_commit_with_rolling_compatibility(
            rolling_request(
                CompatibilityFamilyKind::CommitEnvelope.family_id(),
                vec![ArtifactSemanticVersion::new(2)],
                vec![ArtifactSemanticVersion::new(1)],
            ),
            envelope,
        )
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::CompatibilityEdgeMissing);
}

#[test]
fn rolling_commit_publication_matches_certification_lane_evidence() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let outcome = store
        .append_canonical_commit_with_rolling_compatibility(
            rolling_request(
                CompatibilityFamilyKind::CommitEnvelope.family_id(),
                vec![ArtifactSemanticVersion::new(1)],
                vec![ArtifactSemanticVersion::new(2)],
            ),
            envelope,
        )
        .unwrap();
    let certification = Milestone12CertificationRunner::first_ship().run().unwrap();
    let admitted_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
        })
        .unwrap();
    let rejected_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::RollingMissingEdgeRejected
        })
        .unwrap();

    assert_eq!(outcome.relation(), admitted_lane.relation().unwrap());
    assert_eq!(
        admitted_lane.status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert_eq!(
        rejected_lane.status(),
        Milestone12CertificationLaneStatus::Rejected
    );
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"rolling_writer_publication_deferred"));
}

fn rolling_request(
    family_id: ArtifactFamilyId,
    reader_versions: Vec<ArtifactSemanticVersion>,
    writer_versions: Vec<ArtifactSemanticVersion>,
) -> CompatibilityRollingPublicationRequest {
    CompatibilityRollingPublicationRequest::new(
        rolling_window(family_id.clone()),
        vec![ReaderCapabilitySet::new(family_id.clone(), reader_versions)],
        vec![WriterCapabilitySet::new(family_id, writer_versions)],
    )
}

fn rolling_window(family_id: ArtifactFamilyId) -> RollingUpgradeWindow {
    RollingUpgradeWindow::new(
        family_id,
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    )
}
