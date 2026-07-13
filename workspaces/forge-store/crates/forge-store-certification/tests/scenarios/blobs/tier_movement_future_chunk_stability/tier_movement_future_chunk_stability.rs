use forge_store_test_support::harness::physical_isolation::epoch_scope as support;

use forge_foundational::{
    FoundationalBoundaryArtifactRole, FoundationalDiagnosticArtifactKind,
    FoundationalDiagnosticBreachClass,
};
use forge_store_physical_format::{
    PhysicalExtentId, PhysicalFutureChunkId, PhysicalFutureChunkReference, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_physical_isolation::{
    admit_seed_stable_read_plan, ChunkMigrationReadInterlockPlan, FutureBlobMigrationNonClaim,
    FutureChunkStabilityBasis, GenerationCountedPhysicalReference, MovablePhysicalRef,
    PhysicalChunkStabilityPlaceholder, PhysicalReadPlanReleaseSemantics,
    ProtectedPhysicalReferenceSet, PublishedReaderHazard, ReadPlanAdmissionScratchArena,
    StablePhysicalReadPlan, TierMovementAdmissionLabel, TierMovementReadInterlockPlan,
    TierMovementStabilityDenial, TierMovementStabilityVerdict, TraversalAdmissionGuard,
    UnprotectedReadIntent, UnsupportedTierMovementClaim, UnsupportedTierMovementRequest,
};
use support::{
    current_generation_extent_reference, current_generation_page_reference,
    current_generation_segment_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

#[test]
fn tier_and_future_chunk_plans_preserve_stability_only_facts() {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let plan = ordinary_plan();
    let barrier = plan.reachability_barrier();
    let extent = current_generation_extent_reference(17);
    let extent_epoch = root.admit_extent_publication_epoch(extent).unwrap().epoch();
    let movable_extent = MovablePhysicalRef::extent(extent, extent_epoch, barrier).unwrap();

    let first = TierMovementReadInterlockPlan::admit(movable_extent).unwrap();
    let second = TierMovementReadInterlockPlan::admit(movable_extent).unwrap();

    assert_eq!(first.reference(), second.reference());
    assert_eq!(first.verdict(), TierMovementStabilityVerdict::StabilityOnly);
    assert_eq!(
        first.require_cold_tier_qos(),
        Err(TierMovementStabilityDenial::ColdTierQosRemainsS6Scope)
    );
    assert_eq!(
        first.non_claims().blob_lifecycle(),
        FutureBlobMigrationNonClaim::S7OwnsBlobLifecycle
    );

    let placeholder = future_chunk_placeholder(barrier);
    let equivalent_placeholder =
        future_chunk_placeholder_with_reference(placeholder.reference(), barrier);
    let chunk_plan = ChunkMigrationReadInterlockPlan::admit(placeholder).unwrap();
    let equivalent_chunk_plan =
        ChunkMigrationReadInterlockPlan::admit(equivalent_placeholder).unwrap();

    assert_eq!(
        chunk_plan.non_claim().blob_retention(),
        FutureBlobMigrationNonClaim::S7OwnsBlobRetention
    );
    assert_equivalent_future_chunk_stability(chunk_plan, placeholder);
    assert_equivalent_future_chunk_stability(equivalent_chunk_plan, equivalent_placeholder);
    assert_eq!(
        chunk_plan.placeholder(),
        equivalent_chunk_plan.placeholder()
    );
}

#[test]
fn invalid_migration_inputs_deny_before_stability_admission() {
    let barrier = ordinary_plan().reachability_barrier();
    let chunk = future_chunk_reference(7, 3);
    let missing_epoch_basis =
        FutureChunkStabilityBasis::from_stability_receipt(chunk, future_chunk_epoch(), barrier);
    let missing_epoch = PhysicalChunkStabilityPlaceholder::admit(chunk, None, missing_epoch_basis);
    assert_eq!(
        missing_epoch.unwrap_err(),
        TierMovementStabilityDenial::MissingChunkEpoch
    );
    let copied_basis = FutureChunkStabilityBasis::from_stability_receipt(
        future_chunk_reference(8, 3),
        future_chunk_epoch(),
        barrier,
    );
    let mismatched_basis = PhysicalChunkStabilityPlaceholder::admit_with_epoch(
        chunk,
        future_chunk_epoch(),
        copied_basis,
    );
    assert_eq!(
        mismatched_basis.unwrap_err(),
        TierMovementStabilityDenial::PlaceholderBasisMismatch
    );

    let stale_extent = MovablePhysicalRef::extent_from_generation_counted(
        generation_counted_extent_reference(11),
        PhysicalGeneration::from_raw(12).unwrap(),
        current_root_from_authority(&physical_authority_from_complete_closeout())
            .admit_extent_publication_epoch(current_generation_extent_reference(11))
            .unwrap()
            .epoch(),
        barrier,
    );
    assert!(matches!(
        stale_extent,
        Err(TierMovementStabilityDenial::StaleGeneration(_))
    ));

    let extent = movable_extent(barrier, 17);
    let chunk_ref = MovablePhysicalRef::future_chunk_from_placeholder(
        future_chunk_placeholder_with_reference(chunk, barrier),
    );
    let copied = TierMovementReadInterlockPlan::admit_with_label(
        chunk_ref,
        TierMovementAdmissionLabel::copied_from(extent),
    );
    assert_eq!(
        copied.unwrap_err(),
        TierMovementStabilityDenial::CopiedMigrationLabel
    );
    for reference_kind in [
        forge_store_physical_isolation::MovablePhysicalRefKind::Extent,
        forge_store_physical_isolation::MovablePhysicalRefKind::FutureChunk,
    ] {
        for claim in [
            UnsupportedTierMovementClaim::ColdTierQos,
            UnsupportedTierMovementClaim::HardwareMediaPlacement,
            UnsupportedTierMovementClaim::BlobLifecycleMigration,
        ] {
            let unsupported = UnsupportedTierMovementRequest::new(reference_kind, claim);
            assert_eq!(unsupported.reference_kind(), reference_kind);
            assert_eq!(unsupported.claim(), claim);
            assert_eq!(
                TierMovementReadInterlockPlan::reject_unsupported_tier_movement(unsupported)
                    .unwrap_err(),
                TierMovementStabilityDenial::UnsupportedTierMovement
            );
        }
    }
}

#[test]
fn blob_lifecycle_and_io_qos_authority_requests_remain_explicit_non_claims() {
    let chunk_plan = ChunkMigrationReadInterlockPlan::admit(future_chunk_placeholder(
        ordinary_plan().reachability_barrier(),
    ))
    .unwrap();
    let evidence = chunk_plan.foundational_non_claim_evidence();

    assert_eq!(
        evidence.support().role(),
        FoundationalBoundaryArtifactRole::SupportOnly
    );
    assert_eq!(
        evidence.planned().role(),
        FoundationalBoundaryArtifactRole::PlannedWork
    );
    assert_eq!(
        evidence.artifact_kind(),
        FoundationalDiagnosticArtifactKind::SupportReport
    );
    assert_eq!(
        evidence.breach_class(),
        FoundationalDiagnosticBreachClass::CoverageOmission
    );
    assert_eq!(
        evidence.deny_blob_authority_promotion(),
        Err(TierMovementStabilityDenial::FoundationalSurfaceCannotPromoteToBlobAuthority)
    );
    assert_eq!(
        evidence.deny_cold_tier_qos_promotion(),
        Err(TierMovementStabilityDenial::ColdTierQosRemainsS6Scope)
    );
    assert_eq!(
        chunk_plan.deny_proof_assumption_blob_authority_promotion(),
        Err(TierMovementStabilityDenial::ProofAssumptionCannotPromoteToBlobAuthority)
    );
    assert_eq!(
        chunk_plan.deny_proof_assumption_cold_tier_qos_promotion(),
        Err(TierMovementStabilityDenial::ProofAssumptionCannotPromoteToColdTierQos)
    );
    assert_non_claim_report(chunk_plan.non_claim());
    assert_eq!(
        chunk_plan.require_blob_lifecycle_authority(),
        Err(TierMovementStabilityDenial::BlobLifecycleRemainsS7Scope)
    );
    assert_eq!(
        chunk_plan.require_blob_retention_authority(),
        Err(TierMovementStabilityDenial::BlobRetentionRemainsS7Scope)
    );
    assert_eq!(
        chunk_plan.require_blob_dedupe_authority(),
        Err(TierMovementStabilityDenial::BlobDedupeRemainsS7Scope)
    );
    assert_eq!(
        chunk_plan.require_resumable_write_authority(),
        Err(TierMovementStabilityDenial::ResumableWritesRemainS7Scope)
    );
    assert_eq!(
        chunk_plan.require_cold_tier_qos(),
        Err(TierMovementStabilityDenial::ColdTierQosRemainsS6Scope)
    );
}

#[test]
fn ordinary_read_plans_do_not_carry_future_chunk_fields() {
    let plan = ordinary_plan();

    assert!(plan.epoch_vector().chunk_epoch().is_none());
    assert_eq!(plan.footprint().protected().references().len(), 3);
    let placeholder = future_chunk_placeholder_with_reference(
        future_chunk_reference(9, 4),
        plan.reachability_barrier(),
    );
    assert!(matches!(
        MovablePhysicalRef::future_chunk_from_placeholder(placeholder).kind(),
        forge_store_physical_isolation::MovablePhysicalRefKind::FutureChunk
    ));
}

fn ordinary_plan() -> StablePhysicalReadPlan {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let references = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        [
            current_generation_segment_reference(13),
            current_generation_extent_reference(17),
            current_generation_page_reference(19),
        ],
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(4),
    )
    .unwrap();
    let observed = references.clone();
    let intent = UnprotectedReadIntent::for_known_footprint(root, references, 8192)
        .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
    let hazard = PublishedReaderHazard::publish(&authority, intent).unwrap();
    let observation =
        forge_store_physical_isolation::PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
            &authority, &hazard, root, observed,
        )
        .unwrap();
    let validated = hazard
        .observe_authority_after_publication(&authority, observation)
        .unwrap()
        .validate()
        .unwrap();
    let receipt = TraversalAdmissionGuard::from_validated_root(validated)
        .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(4))
        .unwrap();
    admit_seed_stable_read_plan(receipt.into_cursor().finish()).unwrap()
}

fn movable_extent(
    barrier: forge_store_physical_isolation::PhysicalReadReachabilityBarrier,
    generation: u64,
) -> MovablePhysicalRef {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let extent = current_generation_extent_reference(generation);
    let epoch = root.admit_extent_publication_epoch(extent).unwrap().epoch();
    MovablePhysicalRef::extent(extent, epoch, barrier).unwrap()
}

fn future_chunk_placeholder(
    barrier: forge_store_physical_isolation::PhysicalReadReachabilityBarrier,
) -> PhysicalChunkStabilityPlaceholder {
    let reference = future_chunk_reference(7, 3);
    future_chunk_placeholder_with_reference(reference, barrier)
}

fn future_chunk_placeholder_with_reference(
    reference: PhysicalFutureChunkReference,
    barrier: forge_store_physical_isolation::PhysicalReadReachabilityBarrier,
) -> PhysicalChunkStabilityPlaceholder {
    let epoch = future_chunk_epoch();
    let basis = FutureChunkStabilityBasis::from_stability_receipt(reference, epoch, barrier);
    PhysicalChunkStabilityPlaceholder::admit_with_epoch(reference, epoch, basis).unwrap()
}

fn future_chunk_reference(id: u64, generation: u64) -> PhysicalFutureChunkReference {
    PhysicalFutureChunkReference::stability_placeholder(
        PhysicalFutureChunkId::from_raw(id).unwrap(),
        PhysicalGeneration::from_raw(generation).unwrap(),
    )
}

fn assert_equivalent_future_chunk_stability(
    chunk_plan: ChunkMigrationReadInterlockPlan,
    placeholder: PhysicalChunkStabilityPlaceholder,
) {
    let recipe = chunk_plan.resolved_stability_recipe();
    let basis = recipe.strong_basis().value();
    assert_eq!(
        recipe.payload().placeholder().reference(),
        placeholder.reference()
    );
    assert_eq!(
        recipe.payload().placeholder().reference().generation(),
        placeholder.reference().generation()
    );
    assert_eq!(basis.reference(), placeholder.reference());
    assert_eq!(
        basis.reference().generation(),
        placeholder.reference().generation()
    );
    assert_eq!(basis.epoch(), placeholder.epoch());
    assert_eq!(basis.reachability(), placeholder.reachability());
}

fn assert_non_claim_report(
    report: forge_store_physical_isolation::FutureBlobMigrationNonClaimReport,
) {
    assert_eq!(
        report.blob_lifecycle(),
        FutureBlobMigrationNonClaim::S7OwnsBlobLifecycle
    );
    assert_eq!(
        report.blob_retention(),
        FutureBlobMigrationNonClaim::S7OwnsBlobRetention
    );
    assert_eq!(
        report.blob_dedupe(),
        FutureBlobMigrationNonClaim::S7OwnsBlobDedupe
    );
    assert_eq!(
        report.resumable_writes(),
        FutureBlobMigrationNonClaim::S7OwnsResumableWrites
    );
    assert_eq!(
        report.cold_tier_qos(),
        FutureBlobMigrationNonClaim::S6OwnsColdTierQos
    );
}

fn future_chunk_epoch() -> forge_store_physical_isolation::ChunkEpoch {
    let authority = physical_authority_from_complete_closeout();
    current_root_from_authority(&authority)
        .future_chunk_publication_epoch_placeholder()
        .epoch()
}

fn generation_counted_extent_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(29).unwrap();
    let extent = PhysicalExtentId::from_raw(31).unwrap();
    let cell = generations
        .extent_cell(segment, extent)
        .with_extent_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_extent(cell))
}
