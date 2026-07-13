use super::{DerivedIndexParityView, DerivedIndexRebuildView};
use crate::maintenance::layout_rebuild;
use crate::maintenance::test_support::admitted_materialization;
use crate::strategy::tests_support::{
    admit_btree_page_strategy, admit_persisted_lsm_strategy, admitted_page_key_bytes,
    admitted_wal_key_bytes,
};
use crate::{
    access_shapes, AccessLaneClassification, DerivedIndexCostEnvelopeParity,
    DerivedIndexCounterShapeParity, DerivedIndexParityBasis, DerivedIndexParityRow,
    DerivedIndexRebuildDenied, DerivedIndexRebuildRequest, DerivedIndexRebuildSourceInput,
    DerivedIndexResultIdentity, LayoutCorruptionView,
};
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
    let (rebuild_shape, authority_coverage) = root_rebuild_shape(strategy.lifecycle(), 31);
    let materialization = admitted_materialization(strategy.admitted_family(), authority_coverage);
    let authority_coverage = materialization.coverage().clone();
    let source_witness = root_manifest_source_witness(7, 11);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    assert!(matches!(
        plan.corruption().view(),
        LayoutCorruptionView::RebuildRequired(_)
    ));

    let rebuilt = layout_rebuild()
        .rebuild(
            plan,
            root_rebuilt_parity_basis(authority_coverage.clone(), &source_witness, "rebuilt-page"),
        )
        .into_rebuilt()
        .expect("expected rebuilt outcome");
    assert_eq!(
        rebuilt.plan().result_identity(),
        DerivedIndexResultIdentity::RemainsDerivedProjection
    );

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert!(parity.parity_holds());
    assert_eq!(
        parity.value_identity(),
        crate::maintenance::DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
    assert_eq!(
        parity.counter_shape(),
        DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    );
}

#[test]
fn authoritative_wal_source_quarantines_without_caller_relabeling() {
    let strategy = admit_persisted_lsm_strategy();
    let (rebuild_shape, authority_coverage) = wal_rebuild_shape(strategy.lifecycle(), 37);
    let materialization = admitted_materialization(strategy.admitted_family(), authority_coverage);
    let authority_coverage = materialization.coverage().clone();
    let source_witness = wal_replay_source_witness(43, BlobWalRecordKind::RootCandidate);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    assert!(matches!(
        layout_rebuild().rebuild(
            plan,
            wal_rebuilt_parity_basis(authority_coverage.clone(), &source_witness, "rebuilt-wal")
        ),
        outcome if matches!(outcome.view(), DerivedIndexRebuildView::Quarantined(_))
    ));
}

#[test]
fn derived_data_inputs_are_denied_as_rebuild_sources() {
    let strategy = admit_btree_page_strategy();
    let (rebuild_shape, authority_coverage) = root_rebuild_shape(strategy.lifecycle(), 41);

    for source in [
        DerivedIndexRebuildSourceInput::DerivedProjectionRows,
        DerivedIndexRebuildSourceInput::CertificationRows,
        DerivedIndexRebuildSourceInput::DiagnosticReport,
        DerivedIndexRebuildSourceInput::JsonProjection,
        DerivedIndexRebuildSourceInput::TerminalProjection,
    ] {
        let request = DerivedIndexRebuildRequest::new(
            strategy.admitted_family(),
            strategy.admitted_key_domain(),
            strategy.family(),
            rebuild_shape,
            admitted_materialization(strategy.admitted_family(), authority_coverage.clone()),
            source,
        );

        assert!(matches!(
            layout_rebuild().admit_plan(request),
            Err(DerivedIndexRebuildDenied::SourceInputIsNotAuthority { .. })
        ));
    }
}

#[test]
fn visible_parity_does_not_claim_source_value_or_cost_truth_for_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let (rebuild_shape, authority_coverage) = root_rebuild_shape(strategy.lifecycle(), 47);
    let materialization = admitted_materialization(strategy.admitted_family(), authority_coverage);
    let authority_coverage = materialization.coverage().clone();
    let source_witness = root_manifest_source_witness(7, 11);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild()
        .rebuild(
            plan,
            root_rebuilt_parity_basis(
                authority_coverage.clone(),
                &source_witness,
                "rebuilt-page-mismatch",
            ),
        )
        .into_rebuilt()
        .expect("expected rebuilt outcome");

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert_eq!(
        parity.value_identity(),
        crate::maintenance::DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
}

#[test]
fn visible_parity_does_not_claim_source_value_or_cost_truth_for_wal_authority() {
    let strategy = admit_persisted_lsm_strategy();
    let (rebuild_shape, authority_coverage) = wal_rebuild_shape(strategy.lifecycle(), 51);
    let materialization = admitted_materialization(strategy.admitted_family(), authority_coverage);
    let authority_coverage = materialization.coverage().clone();
    let source_witness = wal_replay_source_witness(61, BlobWalRecordKind::GenerationPublication);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild()
        .rebuild(
            plan,
            wal_rebuilt_parity_basis(
                authority_coverage.clone(),
                &source_witness,
                "rebuilt-wal-mismatch",
            ),
        )
        .into_rebuilt()
        .expect("expected rebuilt outcome");

    let parity = layout_rebuild()
        .verify_parity(rebuilt)
        .into_verified()
        .expect("expected parity witness");
    assert_eq!(
        parity.value_identity(),
        crate::maintenance::DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
    );
    assert_eq!(
        parity.cost_envelope(),
        DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
}

#[test]
fn parity_lane_denies_rebuilt_counter_shape_mismatch_against_source_owned_witness() {
    let strategy = admit_btree_page_strategy();
    let (rebuild_shape, authority_coverage) = root_rebuild_shape(strategy.lifecycle(), 53);
    let materialization = admitted_materialization(strategy.admitted_family(), authority_coverage);
    let authority_coverage = materialization.coverage().clone();
    let source_witness = root_manifest_source_witness(7, 11);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source_witness: source_witness.clone(),
        },
    );

    let plan = layout_rebuild().admit_plan(request).unwrap();
    let rebuilt = layout_rebuild()
        .rebuild(
            plan,
            DerivedIndexParityBasis::new(
                vec![DerivedIndexParityRow::new(
                    admitted_page_key_bytes(7, 11),
                    "rebuilt-page",
                )],
                authority_coverage.clone(),
                true,
                vec![999],
            )
            .unwrap(),
        )
        .into_rebuilt()
        .expect("expected rebuilt outcome");

    assert!(matches!(
        layout_rebuild().verify_parity(rebuilt),
        outcome if matches!(
            outcome.view(),
            DerivedIndexParityView::Denied(
                DerivedIndexRebuildDenied::ParityCounterShapeMismatch
            )
        )
    ));
}

fn root_rebuild_shape(
    lifecycle: crate::ArtifactFamilyLifecycleAdmission,
    epoch: u64,
) -> (
    crate::access::shape::AccessShapeContract,
    crate::LayoutCoverageWitness,
) {
    let coverage = crate::facade::access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(epoch).unwrap(),
        )
        .unwrap();
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    (shape, coverage)
}

fn wal_rebuild_shape(
    lifecycle: crate::ArtifactFamilyLifecycleAdmission,
    lsn: u64,
) -> (
    crate::access::shape::AccessShapeContract,
    crate::LayoutCoverageWitness,
) {
    let coverage = crate::facade::access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            LogSequenceNumber::new(lsn),
        )
        .unwrap();
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    (shape, coverage)
}

fn root_rebuilt_parity_basis(
    coverage: crate::LayoutCoverageWitness,
    source_witness: &PhysicalRootManifestRebuildWitness,
    value: &str,
) -> DerivedIndexParityBasis {
    let row = &source_witness.rows()[0];
    DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
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
    coverage: crate::LayoutCoverageWitness,
    source_witness: &BlobWalReplayRebuildWitness,
    value: &str,
) -> DerivedIndexParityBasis {
    DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
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
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
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
        PhysicalManifestUniverseBuilder::for_canonical_physical_format(root)
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
