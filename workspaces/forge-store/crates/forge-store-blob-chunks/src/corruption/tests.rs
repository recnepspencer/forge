use super::classify_generation_posture;
use super::test_support::{assert_denial, assert_downstream_denial, quarantined_read_corruption};
use crate::lifecycle::generation_registry_test_support::current_authority;
use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::test_support::{
    blob_scope, candidate_for_bytes_and_scope, canonical_equivalence, frontier_for,
};
use crate::{
    AuthoritativeBlobCorruptionPosture, BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionDenial,
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionCapsuleReadiness,
    BlobCorruptionDenial, BlobCorruptionDetectionSource, BlobCorruptionExportAdmission,
    BlobCorruptionGuard, BlobCorruptionImportReadmission, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges, BlobCorruptionReferenceSharingScope,
    BlobDamageCase, BlobObjectClassification, BlobQuarantineLifecycleState,
    DerivedBlobRebuildAuthority,
};
use forge_proof::TransitionOutcome;
use forge_store_security::StoreTenantScope;

#[test]
fn damage_case_decision_table_keeps_spec_cases_distinct() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.damage-case", b"aaaabbbbcccc", 4);
    let frontier = frontier_for("phase11.damage-case", b"aaaabbbbcccc", 4);
    let edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        published.staging_identity(),
    )
    .expect("staging identity should bind");

    let checksum = BlobCorruptedChunkLocalization::from_read_corruption(
        visible.clone(),
        frontier.clone(),
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges.clone(),
    )
    .expect("read corruption should localize");
    assert_eq!(checksum.damage_case(), BlobDamageCase::ChecksumMismatch);

    let import = BlobCorruptedChunkLocalization::from_detected_source(
        BlobCorruptionDetectionSource::ImportReadmission,
        visible,
        frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::ImportStaging,
        edges,
    )
    .expect("import corruption should localize");
    assert_eq!(import.damage_case(), BlobDamageCase::CrossScopeImport);
}

#[test]
fn corruption_localizes_each_detection_source_to_generation_placement_and_edges() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.localize", b"aaaabbbbcccc", 4);
    let frontier = frontier_for("phase11.localize", b"aaaabbbbcccc", 4);
    let reference_edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        published.staging_identity(),
    )
    .expect("published reachability staging identity should bind affected edge");

    for (source, placement) in [
        (
            BlobCorruptionDetectionSource::VerifiedRead,
            BlobCorruptionPlacementClass::LocalPhysical,
        ),
        (
            BlobCorruptionDetectionSource::Scrub,
            BlobCorruptionPlacementClass::LocalPhysical,
        ),
        (
            BlobCorruptionDetectionSource::ColdFetch,
            BlobCorruptionPlacementClass::ExternalCold,
        ),
        (
            BlobCorruptionDetectionSource::ImportReadmission,
            BlobCorruptionPlacementClass::ImportStaging,
        ),
        (
            BlobCorruptionDetectionSource::CapsuleMaterialization,
            BlobCorruptionPlacementClass::CapsuleMaterialization,
        ),
    ] {
        let localized = BlobCorruptedChunkLocalization::from_detected_source(
            source,
            visible.clone(),
            frontier.clone(),
            BlobChunkOrdinal::first().next(),
            placement,
            reference_edges.clone(),
        )
        .expect("published frontier ordinal should localize");

        assert_eq!(localized.source(), source);
        assert_eq!(localized.object_id(), visible.object_id());
        assert_eq!(localized.generation(), visible.generation());
        assert_eq!(localized.placement_class(), placement);
        assert_eq!(
            localized.sharing_scope(),
            BlobCorruptionReferenceSharingScope::SingleReference
        );
        assert_eq!(localized.reference_edges().edge_count(), 1);
        assert_eq!(localized.counters().localizations(), 1);
        assert_eq!(localized.counters().affected_reference_edges(), 1);
    }
}

#[test]
fn localization_denies_ordinals_outside_the_published_frontier() {
    let (_published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.outside-frontier", b"aaaabbbb", 4);
    let frontier = frontier_for("phase11.outside-frontier", b"aaaabbbb", 4);
    let edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        _published.staging_identity(),
    )
    .expect("published reachability staging identity should bind");

    let denied = BlobCorruptedChunkLocalization::from_read_corruption(
        visible,
        frontier,
        BlobChunkOrdinal::first().next().next().next(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect_err("corrupt ordinal outside frontier must deny");

    match denied {
        BlobCorruptionDenial::CorruptOrdinalNotInPublishedFrontier { counters, .. } => {
            assert_eq!(counters.denials(), 1);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn localization_denies_mismatched_generation_and_frontier() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.frontier.visible", b"aaaabbbb", 4);
    let mismatched_frontier = frontier_for("phase11.frontier.other", b"ccccdddd", 4);
    let edges = BlobCorruptionReferenceEdges::from_reachability_staging_identity(
        published.staging_identity(),
    )
    .expect("published reachability staging identity should bind");

    let denied = BlobCorruptedChunkLocalization::from_read_corruption(
        visible,
        mismatched_frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect_err("mismatched frontier must not localize corruption");

    assert!(matches!(
        denied,
        BlobCorruptionDenial::GenerationFrontierMismatch { .. }
    ));
}

#[test]
fn unrelated_publication_edges_cannot_inflate_shared_corruption_impact() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.edge.visible", b"aaaabbbb", 4);
    let (unrelated_published, _unrelated_visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.edge.unrelated", b"aaaabbbb", 4);
    let frontier = frontier_for("phase11.edge.visible", b"aaaabbbb", 4);
    let edges = BlobCorruptionReferenceEdges::from_admitted_edges(&[
        BlobCorruptionReferenceEdge::from_reachability_staging_identity(
            published.staging_identity(),
        ),
        BlobCorruptionReferenceEdge::from_reachability_staging_identity(
            unrelated_published.staging_identity(),
        ),
    ])
    .expect("distinct edge witnesses should construct before localization binding");

    let denied = BlobCorruptedChunkLocalization::from_read_corruption(
        visible,
        frontier,
        BlobChunkOrdinal::first(),
        BlobCorruptionPlacementClass::LocalPhysical,
        edges,
    )
    .expect_err("unrelated publication edge must not localize as affected");

    assert!(matches!(
        denied,
        BlobCorruptionDenial::AffectedReferenceEdgeMismatch { .. }
    ));
}

#[test]
fn duplicate_reference_edges_cannot_prove_shared_corruption() {
    let (published, _visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.duplicate-edge", b"aaaabbbb", 4);
    let edge = BlobCorruptionReferenceEdge::from_reachability_staging_identity(
        published.staging_identity(),
    );

    let denied = BlobCorruptionReferenceEdges::from_admitted_edges(&[edge.clone(), edge])
        .expect_err("duplicate edge cannot prove a shared affected set");

    assert!(matches!(
        denied,
        BlobCorruptionDenial::DuplicateAffectedReferenceEdge { .. }
    ));
}

#[test]
fn quarantined_corruption_denies_all_downstream_publishers_with_exact_counters() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.downstream-denial", b"aaaabbbb", 4);
    let quarantine = quarantined_read_corruption("phase11.downstream-denial", &published, visible);
    let guard = BlobCorruptionGuard::from_quarantine(quarantine);

    assert_denial(guard.deny_dedupe(), 1, 0, 0, 0, 0);
    assert_downstream_denial(
        BlobCorruptionExportAdmission::deny_for_quarantine(&guard),
        0,
        1,
        0,
        0,
        0,
    );
    assert_downstream_denial(
        BlobCorruptionImportReadmission::deny_for_quarantine(&guard),
        0,
        0,
        1,
        0,
        0,
    );
    assert_downstream_denial(
        BlobCorruptionCapsuleReadiness::deny_for_quarantine(&guard),
        0,
        0,
        0,
        1,
        0,
    );
    assert_denial(guard.deny_verified_read_publication(), 0, 0, 0, 0, 1);
}

#[test]
fn ordinary_dedupe_admission_denies_quarantined_chunks() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.dedupe-denial", b"aaaabbbb", 4);
    let quarantine = quarantined_read_corruption("phase11.dedupe-denial", &published, visible);
    let guard = BlobCorruptionGuard::from_quarantine(quarantine);

    match BlobChunkDedupeAdmission::deny_for_quarantine(&guard) {
        TransitionOutcome::Denied(BlobChunkDedupeAdmissionDenial::QuarantinedChunkDenied {
            quarantine,
            posture,
            counters,
        }) => {
            assert_eq!(
                posture,
                crate::BlobChunkDedupeCollisionPosture::DigestAlgorithmQuarantined
            );
            assert_eq!(quarantine.counters().quarantine_holds(), 1);
            assert_eq!(quarantine.ordinal(), BlobChunkOrdinal::first());
            assert_eq!(counters.quarantine_denials(), 1);
        }
        other => panic!("unexpected dedupe outcome: {other:?}"),
    }
}

#[test]
fn derived_and_authoritative_corruption_have_separate_repair_postures() {
    let (published, visible) =
        publish_generation_with_bytes_and_chunk_size("phase11.classify", b"aaaabbbb", 4);
    let quarantine = quarantined_read_corruption("phase11.classify", &published, visible);
    let derived = classify_generation_posture(&quarantine, BlobObjectClassification::derived());
    let rebuild = derived
        .admit_derived_rebuild(DerivedBlobRebuildAuthority::from_current_store_authority(
            current_authority("phase11.classify.derived", "derived-rebuild"),
        ))
        .expect("derived corruption should rebuild only with authority");
    assert_eq!(rebuild.counters().derived_rebuild_admissions(), 1);
    assert!(matches!(
        derived.authoritative_posture(),
        Err(BlobCorruptionDenial::AuthoritativeRepairRequiresAuthoritativeBlob { .. })
    ));

    let authoritative =
        classify_generation_posture(&quarantine, BlobObjectClassification::authoritative());
    assert!(matches!(
        authoritative.admit_derived_rebuild(
            DerivedBlobRebuildAuthority::from_current_store_authority(current_authority(
                "phase11.classify.authoritative",
                "derived-rebuild"
            ),)
        ),
        Err(BlobCorruptionDenial::DerivedRebuildRequiresDerivedBlob { .. })
    ));
    let posture = authoritative
        .authoritative_posture()
        .expect("authoritative corruption should enter authoritative posture");
    assert_eq!(
        posture.state(),
        BlobQuarantineLifecycleState::RepairRequiredAuthoritative
    );
    assert_eq!(posture.counters().authoritative_repair_postures(), 1);
    let restore = authoritative
        .authoritative_restore_posture()
        .expect("authoritative corruption should enter restore posture");
    assert_eq!(
        restore.state(),
        BlobQuarantineLifecycleState::RestoreRequiredAuthoritative
    );
    assert_eq!(restore.counters().authoritative_restore_postures(), 1);
    let degraded = authoritative
        .authoritative_degraded_truth_posture()
        .expect("authoritative corruption should enter degraded-truth posture");
    assert_eq!(
        degraded.state(),
        BlobQuarantineLifecycleState::DegradedTruthAuthoritative
    );
    assert_eq!(
        degraded.counters().authoritative_degraded_truth_postures(),
        1
    );
}
