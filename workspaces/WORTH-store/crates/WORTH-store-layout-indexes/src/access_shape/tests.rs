use crate::facade::{
    access_planning,
    layout_declarations,
};
use crate::{
    access_shapes, S8AccessAuthorityPosture, S8AccessLaneClassification, S8AccessShape,
    S8AccessShapeDetail, S8AccessShapeUnsupportedDenial, S8AccessStaleDisposition,
    S8BatchPointBasis, S8BoundedScanBasis, S8DegradedExactScanBasis, S8DegradedExactScanRequest,
    S8ExpectedCounterClass, S8FullDeclaredScanBasis, S8GroupedPrefixBasis, S8MaintenanceReadBasis,
    S8ManifestGraphWalkBasis, S8MultiRangeBasis, S8MutationAccessBasis, S8PhysicalMutationShape,
    S8StreamingContinuationBasis, S8StreamingReadBasis,
};
use worth_store_physical_format::PhysicalEpoch;

fn exact_coverage() -> crate::S8LayoutCoverageWitness {
    let family = layout_declarations().seed_family();
    access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(family.family()),
            PhysicalEpoch::from_raw(31).expect("epoch fixture should be valid"),
        )
        .expect("exact coverage should admit")
}

#[test]
fn exact_access_shapes_require_materialization_coverage_witnesses() {
    let coverage = exact_coverage();

    let point = access_shapes()
        .point_lookup(coverage)
        .expect("point access should require exact coverage");
    let range = access_shapes()
        .range_lookup(coverage)
        .expect("range access should require exact coverage");
    let prefix = access_shapes()
        .prefix_lookup(coverage)
        .expect("prefix access should require exact coverage");
    let multi_range = access_shapes()
        .multi_range_lookup(coverage, S8MultiRangeBasis::DeclaredDisjointRangeSet)
        .expect("multi-range access should require declared range-set semantics");
    let grouped_prefix = access_shapes()
        .grouped_prefix_lookup(coverage, S8GroupedPrefixBasis::CanonicalGroupedPrefixes)
        .expect("grouped prefix access should require declared grouping semantics");

    assert_eq!(point.shape(), S8AccessShape::PointLookup);
    assert_eq!(point.detail(), S8AccessShapeDetail::PointLookup);
    assert_eq!(point.coverage(), Some(coverage));
    assert_eq!(
        point.authority_posture(),
        S8AccessAuthorityPosture::ExactMaterialized
    );
    assert_eq!(
        point.stale_disposition(),
        S8AccessStaleDisposition::ExactOnly
    );
    assert_eq!(
        point.expected_counters(),
        S8ExpectedCounterClass::PointLookup
    );

    assert_eq!(range.shape(), S8AccessShape::RangeLookup);
    assert_eq!(
        range.detail(),
        S8AccessShapeDetail::RangeLookup(crate::S8RangeBasis::CanonicalRangeBounds)
    );
    assert_eq!(range.coverage(), Some(coverage));
    assert_eq!(
        range.expected_counters(),
        S8ExpectedCounterClass::RangeLookup
    );

    assert_eq!(prefix.shape(), S8AccessShape::PrefixLookup);
    assert_eq!(
        prefix.detail(),
        S8AccessShapeDetail::PrefixLookup(crate::S8PrefixBasis::CanonicalPrefixBounds)
    );
    assert_eq!(prefix.coverage(), Some(coverage));
    assert_eq!(
        prefix.expected_counters(),
        S8ExpectedCounterClass::PrefixLookup
    );

    assert_eq!(
        multi_range.detail(),
        S8AccessShapeDetail::MultiRangeLookup(S8MultiRangeBasis::DeclaredDisjointRangeSet)
    );
    assert_eq!(
        multi_range.expected_counters(),
        S8ExpectedCounterClass::MultiRangeLookup
    );
    assert_eq!(
        grouped_prefix.detail(),
        S8AccessShapeDetail::GroupedPrefixLookup(S8GroupedPrefixBasis::CanonicalGroupedPrefixes)
    );
    assert_eq!(
        grouped_prefix.expected_counters(),
        S8ExpectedCounterClass::GroupedPrefixLookup
    );
}

#[test]
fn scan_and_manifest_shapes_preserve_bound_and_lane_truth() {
    let coverage = exact_coverage();

    let bounded = access_shapes()
        .bounded_scan(
            coverage,
            S8AccessLaneClassification::Foreground,
            S8BoundedScanBasis::LocalityBoundedTraversal,
        )
        .expect("bounded scan should admit on the foreground lane");
    let full = access_shapes()
        .full_declared_scan(
            coverage,
            S8AccessLaneClassification::Verifier,
            S8FullDeclaredScanBasis::DeclaredFullTraversal,
        )
        .expect("full declared scan should admit on the verifier lane");
    let manifest = access_shapes()
        .manifest_graph_walk(coverage, S8AccessLaneClassification::Terminal)
        .expect("manifest graph walk should admit on the terminal lane");

    assert_eq!(
        bounded.detail(),
        S8AccessShapeDetail::BoundedScan(S8BoundedScanBasis::LocalityBoundedTraversal)
    );
    assert_eq!(
        bounded.expected_counters(),
        S8ExpectedCounterClass::BoundedScan
    );
    assert_eq!(
        full.detail(),
        S8AccessShapeDetail::FullDeclaredScan(S8FullDeclaredScanBasis::DeclaredFullTraversal)
    );
    assert_eq!(
        full.expected_counters(),
        S8ExpectedCounterClass::FullDeclaredScan
    );
    assert_eq!(
        manifest.detail(),
        S8AccessShapeDetail::ManifestGraphWalk(S8ManifestGraphWalkBasis::ManifestAuthorityGraph)
    );
    assert_eq!(
        manifest.expected_counters(),
        S8ExpectedCounterClass::ManifestGraphWalk
    );

    assert_eq!(
        access_shapes().full_declared_scan(
            coverage,
            S8AccessLaneClassification::Foreground,
            S8FullDeclaredScanBasis::DeclaredFullTraversal,
        ),
        Err(S8AccessShapeUnsupportedDenial::HiddenBroadScan {
            requested_shape: S8AccessShape::FullDeclaredScan,
        })
    );
    assert_eq!(
        access_shapes().manifest_graph_walk(coverage, S8AccessLaneClassification::Verifier),
        Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::ManifestGraphWalk,
            lane: S8AccessLaneClassification::Verifier,
        })
    );
}

#[test]
fn streaming_and_continuation_shapes_stay_explicit() {
    let coverage = exact_coverage();

    let coalesced = access_shapes()
        .coalesced_page_read(coverage)
        .expect("coalesced page reads should admit on the foreground lane");
    let streaming = access_shapes()
        .streaming_read(coverage, S8AccessLaneClassification::Maintenance)
        .expect("streaming reads should admit on the maintenance lane");
    let continuation = access_shapes()
        .streaming_continuation_read(
            coverage,
            S8AccessLaneClassification::Foreground,
            S8StreamingContinuationBasis::ResumeCursorContinuation,
        )
        .expect("streaming continuation should require an explicit continuation basis");

    assert_eq!(
        coalesced.detail(),
        S8AccessShapeDetail::CoalescedPageRead(crate::S8CoalescedPageReadBasis::AdjacentPageWindow)
    );
    assert_eq!(
        coalesced.expected_counters(),
        S8ExpectedCounterClass::CoalescedPageRead
    );
    assert_eq!(
        streaming.detail(),
        S8AccessShapeDetail::StreamingRead(S8StreamingReadBasis::SequentialStreamTraversal)
    );
    assert_eq!(
        streaming.expected_counters(),
        S8ExpectedCounterClass::StreamingRead
    );
    assert_eq!(
        continuation.detail(),
        S8AccessShapeDetail::StreamingContinuationRead(
            S8StreamingContinuationBasis::ResumeCursorContinuation,
        )
    );
    assert_eq!(
        continuation.expected_counters(),
        S8ExpectedCounterClass::StreamingContinuationRead
    );

    assert_eq!(
        access_shapes().chunk_tree_walk(coverage, S8AccessLaneClassification::Verifier),
        Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::ChunkTreeWalk,
            lane: S8AccessLaneClassification::Verifier,
        })
    );
    assert_eq!(
        access_shapes().streaming_continuation_read(
            coverage,
            S8AccessLaneClassification::Terminal,
            S8StreamingContinuationBasis::ResumeCursorContinuation,
        ),
        Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::StreamingContinuationRead,
            lane: S8AccessLaneClassification::Terminal,
        })
    );
}

#[test]
fn mutation_and_maintenance_shapes_keep_denial_boundaries() {
    let coverage = exact_coverage();

    let append = access_shapes()
        .append(S8PhysicalMutationShape::LogStructuredAppend)
        .expect("append should admit only for log-structured mutation shapes");
    let compaction = access_shapes()
        .compaction_read(S8PhysicalMutationShape::CompactionRewrite)
        .expect("compaction should admit only for compaction rewrite shapes");
    let rebuild = access_shapes()
        .rebuild_read(coverage, S8AccessLaneClassification::Maintenance)
        .expect("rebuild should admit on the maintenance lane");
    let verifier = access_shapes()
        .verifier_read(coverage, S8AccessLaneClassification::Verifier)
        .expect("verifier should admit on the verifier lane");
    let repair = access_shapes()
        .repair_read(coverage, S8AccessLaneClassification::Maintenance)
        .expect("repair should admit on the maintenance lane");
    let quarantine = access_shapes()
        .quarantine_read(coverage, S8AccessLaneClassification::Verifier)
        .expect("quarantine should admit on the verifier lane");

    assert_eq!(
        append.detail(),
        S8AccessShapeDetail::Append(S8MutationAccessBasis::WalBeforeDataAppend)
    );
    assert_eq!(
        append.expected_counters(),
        S8ExpectedCounterClass::AppendTraversal
    );
    assert_eq!(
        compaction.detail(),
        S8AccessShapeDetail::CompactionRead(S8MutationAccessBasis::CompactionRewriteTraversal)
    );
    assert_eq!(
        compaction.expected_counters(),
        S8ExpectedCounterClass::CompactionTraversal
    );
    assert_eq!(
        rebuild.detail(),
        S8AccessShapeDetail::RebuildRead(S8MaintenanceReadBasis::RebuildTraversal)
    );
    assert_eq!(
        rebuild.expected_counters(),
        S8ExpectedCounterClass::RebuildTraversal
    );
    assert_eq!(
        verifier.detail(),
        S8AccessShapeDetail::VerifierRead(S8MaintenanceReadBasis::VerifierTraversal)
    );
    assert_eq!(
        verifier.expected_counters(),
        S8ExpectedCounterClass::VerifierTraversal
    );
    assert_eq!(
        repair.detail(),
        S8AccessShapeDetail::RepairRead(S8MaintenanceReadBasis::RepairTraversal)
    );
    assert_eq!(
        repair.expected_counters(),
        S8ExpectedCounterClass::RepairTraversal
    );
    assert_eq!(
        quarantine.detail(),
        S8AccessShapeDetail::QuarantineRead(S8MaintenanceReadBasis::QuarantineTraversal)
    );
    assert_eq!(
        quarantine.expected_counters(),
        S8ExpectedCounterClass::QuarantineTraversal
    );

    assert_eq!(
        access_shapes().append(S8PhysicalMutationShape::ObservationOnly),
        Err(
            S8AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: S8AccessShape::Append,
                mutation_shape: S8PhysicalMutationShape::ObservationOnly,
            },
        )
    );
    assert_eq!(
        access_shapes().compaction_read(S8PhysicalMutationShape::LogStructuredAppend),
        Err(
            S8AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: S8AccessShape::CompactionRead,
                mutation_shape: S8PhysicalMutationShape::LogStructuredAppend,
            },
        )
    );
    assert_eq!(
        access_shapes().rebuild_read(coverage, S8AccessLaneClassification::Verifier),
        Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::RebuildRead,
            lane: S8AccessLaneClassification::Verifier,
        })
    );
    assert_eq!(
        access_shapes().verifier_read(coverage, S8AccessLaneClassification::Maintenance),
        Err(S8AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: S8AccessShape::VerifierRead,
            lane: S8AccessLaneClassification::Maintenance,
        })
    );
}

#[test]
fn degraded_exact_scan_stays_budgeted_and_counter_distinct() {
    let coverage = exact_coverage();

    assert_eq!(
        access_shapes().explicit_degraded_exact_scan(S8DegradedExactScanRequest::new(coverage)),
        Err(S8AccessShapeUnsupportedDenial::DegradedExactScanBudgetRequired)
    );

    let degraded = access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(coverage).with_budget_rows(64),
        )
        .expect("degraded exact scans should require an explicit budget");

    assert_eq!(
        degraded.detail(),
        S8AccessShapeDetail::DegradedExactScan(
            S8DegradedExactScanBasis::BudgetedCounterBoundedTraversal,
        )
    );
    assert_eq!(
        degraded.expected_counters(),
        S8ExpectedCounterClass::DegradedExactScan
    );
    assert_eq!(degraded.budget_rows(), Some(64));
}
