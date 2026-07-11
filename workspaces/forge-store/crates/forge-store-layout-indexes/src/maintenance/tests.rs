use crate::strategy::tests_support::{
    admit_btree_page_strategy, admit_lsm_wal_strategy, admitted_page_key_bytes,
    admitted_wal_key_bytes,
};
use crate::maintenance::layout_rebuild;
use crate::{
    access_shapes, S8AccessLaneClassification, S8DerivedIndexCostEnvelopeParity,
    S8DerivedIndexCounterShapeParity, S8DerivedIndexParityBasis, S8DerivedIndexParityOutcome,
    S8DerivedIndexParityRow, S8DerivedIndexRebuildDenied, S8DerivedIndexRebuildOutcome,
    S8DerivedIndexRebuildRequest, S8DerivedIndexRebuildSourceInput, S8DerivedIndexResultIdentity,
    S8LayoutCorruptionView,
};
use super::{S8DerivedIndexParityView, S8DerivedIndexRebuildView};
use forge_store_physical_format::{
    PhysicalEpoch, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalManifestUniverseBuilder, PhysicalPageId, PhysicalRecordSlot,
    PhysicalRootManifestRebuildWitness, PhysicalRootReference, PhysicalSegmentId,
};
use forge_store_recovery_physics::LogSequenceNumber;
use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, BlobWalReplayRebuildWitness,
    DurablePublicationDeclaration, WalFrameDurablePublicationScope,
};

#[test]
fn derived_projection_rebuilds_to_visible_parity_from_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let rebuild_shape = root_rebuild_shape(strategy.lifecycle(), 31);
    let authority_coverage = rebuild_shape.coverage().expect("rebuild coverage");
    let source_witness = root_manifest_source_witness(7, 11);
    let request = S8DerivedIndexRebuildRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        rebuild_shape,
        S8DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    assert!(matches!(
        plan.corruption().view(),
        S8LayoutCorruptionView::RebuildRequired(_)
    ));

    let rebuilt = layout_rebuild().rebuild(
        plan,
        root_rebuilt_parity_basis(authority_coverage, &source_witness, "rebuilt-page"),
    ).into_rebuilt().expect("expected rebuilt outcome");
    assert_eq!(
        rebuilt.plan().result_identity(),
        S8DerivedIndexResultIdentity::RemainsDerivedProjection
    );

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert!(parity.parity_holds());
    assert_eq!(
        parity.value_identity(),
        crate::S8DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        S8DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
    assert_eq!(
        parity.counter_shape(),
        S8DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    );
}

#[test]
fn authoritative_wal_source_quarantines_without_caller_relabeling() {
    let strategy = admit_lsm_wal_strategy();
    let rebuild_shape = wal_rebuild_shape(strategy.lifecycle(), 37);
    let authority_coverage = rebuild_shape.coverage().expect("rebuild coverage");
    let source_witness = wal_replay_source_witness(43, BlobWalRecordKind::RootCandidate);
    let request = S8DerivedIndexRebuildRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        rebuild_shape,
        S8DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    assert!(matches!(
        layout_rebuild().rebuild(
            plan,
            wal_rebuilt_parity_basis(authority_coverage, &source_witness, "rebuilt-wal")
        ),
        outcome if matches!(outcome.view(), S8DerivedIndexRebuildView::Quarantined(_))
    ));
}

#[test]
fn derived_data_inputs_are_denied_as_rebuild_sources() {
    let strategy = admit_btree_page_strategy();
    let rebuild_shape = root_rebuild_shape(strategy.lifecycle(), 41);

    for source in [
        S8DerivedIndexRebuildSourceInput::DerivedProjectionRows,
        S8DerivedIndexRebuildSourceInput::CertificationRows,
        S8DerivedIndexRebuildSourceInput::DiagnosticReport,
        S8DerivedIndexRebuildSourceInput::JsonProjection,
        S8DerivedIndexRebuildSourceInput::TerminalProjection,
    ] {
        let request = S8DerivedIndexRebuildRequest::new(
            strategy.lifecycle(),
            strategy.key_domain(),
            strategy.family(),
            rebuild_shape,
            source,
        );

        assert!(matches!(
            layout_rebuild().admit_plan(request),
            Err(S8DerivedIndexRebuildDenied::SourceInputIsNotAuthority { .. })
        ));
    }
}

#[test]
fn visible_parity_does_not_claim_source_value_or_cost_truth_for_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let rebuild_shape = root_rebuild_shape(strategy.lifecycle(), 47);
    let authority_coverage = rebuild_shape.coverage().expect("rebuild coverage");
    let source_witness = root_manifest_source_witness(7, 11);
    let request = S8DerivedIndexRebuildRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        rebuild_shape,
        S8DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild().rebuild(
        plan,
        root_rebuilt_parity_basis(authority_coverage, &source_witness, "rebuilt-page-mismatch"),
    ).into_rebuilt().expect("expected rebuilt outcome");

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert_eq!(
        parity.value_identity(),
        crate::S8DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        S8DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
}

#[test]
fn visible_parity_does_not_claim_source_value_or_cost_truth_for_wal_authority() {
    let strategy = admit_lsm_wal_strategy();
    let rebuild_shape = wal_rebuild_shape(strategy.lifecycle(), 51);
    let authority_coverage = rebuild_shape.coverage().expect("rebuild coverage");
    let source_witness = wal_replay_source_witness(61, BlobWalRecordKind::GenerationPublication);
    let request = S8DerivedIndexRebuildRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        rebuild_shape,
        S8DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild().rebuild(
        plan,
        wal_rebuilt_parity_basis(authority_coverage, &source_witness, "rebuilt-wal-mismatch"),
    ).into_rebuilt().expect("expected rebuilt outcome");

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert_eq!(
        parity.value_identity(),
        crate::S8DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        S8DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
}

#[test]
fn parity_lane_denies_rebuilt_counter_shape_mismatch_against_source_owned_witness() {
    let strategy = admit_btree_page_strategy();
    let rebuild_shape = root_rebuild_shape(strategy.lifecycle(), 53);
    let authority_coverage = rebuild_shape.coverage().expect("rebuild coverage");
    let source_witness = root_manifest_source_witness(7, 11);
    let request = S8DerivedIndexRebuildRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        rebuild_shape,
        S8DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild().rebuild(
        plan,
        S8DerivedIndexParityBasis::new(
            vec![S8DerivedIndexParityRow::new(
                admitted_page_key_bytes(7, 11),
                "rebuilt-page",
            )],
            authority_coverage,
            true,
            vec![999],
        )
        .unwrap(),
    ).into_rebuilt().expect("expected rebuilt outcome");

    assert!(matches!(
        layout_rebuild().verify_parity(rebuilt),
        outcome if matches!(
            outcome.view(),
            S8DerivedIndexParityView::Denied(
                S8DerivedIndexRebuildDenied::ParityCounterShapeMismatch
            )
        )
    ));
}

fn root_rebuild_shape(
    lifecycle: crate::ArtifactFamilyLifecycleAdmission,
    epoch: u64,
) -> crate::S8AccessShapeContract {
    access_shapes()
        .rebuild_read(
            crate::facade::access_planning()
                .exact_root_epoch_coverage(
                    crate::bootstrap::test_support::bootstrap_exact_materialization(
                        lifecycle.declaration().family(),
                    ),
                    PhysicalEpoch::from_raw(epoch).unwrap(),
                )
                .unwrap(),
            S8AccessLaneClassification::Maintenance,
        )
        .unwrap()
}

fn wal_rebuild_shape(
    lifecycle: crate::ArtifactFamilyLifecycleAdmission,
    lsn: u64,
) -> crate::S8AccessShapeContract {
    access_shapes()
        .rebuild_read(
            crate::facade::access_planning()
                .exact_wal_lsn_coverage(
                    crate::bootstrap::test_support::bootstrap_exact_materialization(
                        lifecycle.declaration().family(),
                    ),
                    LogSequenceNumber::new(lsn),
                )
                .unwrap(),
            S8AccessLaneClassification::Maintenance,
        )
        .unwrap()
}

fn root_rebuilt_parity_basis(
    coverage: crate::S8LayoutCoverageWitness,
    source_witness: &PhysicalRootManifestRebuildWitness,
    value: &str,
) -> S8DerivedIndexParityBasis {
    let row = &source_witness.rows()[0];
    S8DerivedIndexParityBasis::new(
        vec![S8DerivedIndexParityRow::new(
            admitted_page_key_bytes(row.segment_id().get(), row.page_id().get()),
            value,
        )],
        coverage,
        true,
        source_witness.counter_shape().to_vec(),
    )
    .unwrap()
}

fn wal_rebuilt_parity_basis(
    coverage: crate::S8LayoutCoverageWitness,
    source_witness: &BlobWalReplayRebuildWitness,
    value: &str,
) -> S8DerivedIndexParityBasis {
    S8DerivedIndexParityBasis::new(
        vec![S8DerivedIndexParityRow::new(
            admitted_wal_key_bytes(source_witness.record().identity().sequence()),
            value,
        )],
        coverage,
        true,
        source_witness.counter_shape().to_vec(),
    )
    .unwrap()
}

fn root_manifest_source_witness(segment: u64, page: u64) -> PhysicalRootManifestRebuildWitness {
    let generations = PhysicalGenerationAuthority::s1();
    let segment_id = PhysicalSegmentId::from_raw(segment).unwrap();
    let root = generations
        .root_publication_cell(PhysicalRootReference::from_raw(1).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(5).unwrap());
    let segment = generations
        .segment_cell(segment_id)
        .with_segment_generation(PhysicalGeneration::from_raw(6).unwrap());
    let slot = generations
        .slot_cell(
            segment_id,
            PhysicalPageId::from_raw(page).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(7).unwrap());

    PhysicalRootManifestRebuildWitness::admit(
        PhysicalManifestUniverseBuilder::s1(root)
            .segment(segment)
            .ordinary_page(slot)
            .publish(),
    )
}

fn wal_replay_source_witness(
    sequence: u64,
    kind: BlobWalRecordKind,
) -> BlobWalReplayRebuildWitness {
    BlobWalReplayRebuildWitness::admit(
        BlobWalRecordEnvelope::new(
            BlobWalRecordIdentity::new(sequence, kind).unwrap(),
            DurablePublicationDeclaration::wal_frame(
                WalFrameDurablePublicationScope::new(1, 1, 10, 20, "sha256:wal-frame", 4096)
                    .unwrap(),
            ),
            "sha256:payload",
        )
        .unwrap(),
    )
}
