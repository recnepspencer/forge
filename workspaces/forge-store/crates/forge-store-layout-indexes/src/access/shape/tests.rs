use crate::observation::AccessShape;
use crate::{
    access_shapes, AccessAuthorityPosture, AccessLaneClassification, AccessShapeDetail,
    AccessShapeUnsupportedDenial, AccessStaleDisposition, BoundedScanBasis, DegradedExactScanBasis,
    DegradedExactScanRequest, ExpectedCounterClass, FullDeclaredScanBasis, GroupedPrefixBasis,
    MaintenanceReadBasis, ManifestGraphWalkBasis, MultiRangeBasis, MutationAccessBasis,
    PhysicalMutationShape, StreamingContinuationBasis, StreamingReadBasis,
};
#[test]
fn exact_access_shapes_declare_operation_semantics_without_materialization() {
    let point = access_shapes().point_lookup_declaration();
    let range = access_shapes().range_lookup_declaration();
    let prefix = access_shapes().prefix_lookup_declaration();
    let multi_range =
        access_shapes().multi_range_lookup_declaration(MultiRangeBasis::DeclaredDisjointRangeSet);
    let grouped_prefix = access_shapes()
        .grouped_prefix_lookup_declaration(GroupedPrefixBasis::CanonicalGroupedPrefixes);

    assert_eq!(point.shape(), AccessShape::PointLookup);
    assert_eq!(point.detail(), AccessShapeDetail::PointLookup);
    assert_eq!(
        point.authority_posture(),
        AccessAuthorityPosture::ExactMaterialized
    );
    assert_eq!(point.stale_disposition(), AccessStaleDisposition::ExactOnly);
    assert_eq!(point.expected_counters(), ExpectedCounterClass::PointLookup);

    assert_eq!(range.shape(), AccessShape::RangeLookup);
    assert_eq!(
        range.detail(),
        AccessShapeDetail::RangeLookup(crate::access::shape::RangeBasis::CanonicalRangeBounds)
    );
    assert_eq!(range.expected_counters(), ExpectedCounterClass::RangeLookup);

    assert_eq!(prefix.shape(), AccessShape::PrefixLookup);
    assert_eq!(
        prefix.detail(),
        AccessShapeDetail::PrefixLookup(crate::access::shape::PrefixBasis::CanonicalPrefixBounds)
    );
    assert_eq!(
        prefix.expected_counters(),
        ExpectedCounterClass::PrefixLookup
    );

    assert_eq!(
        multi_range.detail(),
        AccessShapeDetail::MultiRangeLookup(MultiRangeBasis::DeclaredDisjointRangeSet)
    );
    assert_eq!(
        multi_range.expected_counters(),
        ExpectedCounterClass::MultiRangeLookup
    );
    assert_eq!(
        grouped_prefix.detail(),
        AccessShapeDetail::GroupedPrefixLookup(GroupedPrefixBasis::CanonicalGroupedPrefixes)
    );
    assert_eq!(
        grouped_prefix.expected_counters(),
        ExpectedCounterClass::GroupedPrefixLookup
    );
}

#[test]
fn scan_and_manifest_shapes_preserve_bound_and_lane_truth() {
    let bounded = access_shapes()
        .bounded_scan(
            AccessLaneClassification::Foreground,
            BoundedScanBasis::LocalityBoundedTraversal,
        )
        .expect("bounded scan should admit on the foreground lane");
    let full = access_shapes()
        .full_declared_scan(
            AccessLaneClassification::Verifier,
            FullDeclaredScanBasis::DeclaredFullTraversal,
        )
        .expect("full declared scan should admit on the verifier lane");
    let manifest = access_shapes()
        .manifest_graph_walk(AccessLaneClassification::Terminal)
        .expect("manifest graph walk should admit on the terminal lane");

    assert_eq!(
        bounded.detail(),
        AccessShapeDetail::BoundedScan(BoundedScanBasis::LocalityBoundedTraversal)
    );
    assert_eq!(
        bounded.expected_counters(),
        ExpectedCounterClass::BoundedScan
    );
    assert_eq!(
        full.detail(),
        AccessShapeDetail::FullDeclaredScan(FullDeclaredScanBasis::DeclaredFullTraversal)
    );
    assert_eq!(
        full.expected_counters(),
        ExpectedCounterClass::FullDeclaredScan
    );
    assert_eq!(
        manifest.detail(),
        AccessShapeDetail::ManifestGraphWalk(ManifestGraphWalkBasis::ManifestAuthorityGraph)
    );
    assert_eq!(
        manifest.expected_counters(),
        ExpectedCounterClass::ManifestGraphWalk
    );

    assert_eq!(
        access_shapes().full_declared_scan(
            AccessLaneClassification::Foreground,
            FullDeclaredScanBasis::DeclaredFullTraversal,
        ),
        Err(AccessShapeUnsupportedDenial::HiddenBroadScan {
            requested_shape: AccessShape::FullDeclaredScan,
        })
    );
    assert_eq!(
        access_shapes().manifest_graph_walk(AccessLaneClassification::Verifier),
        Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::ManifestGraphWalk,
            lane: AccessLaneClassification::Verifier,
        })
    );
}

#[test]
fn streaming_and_continuation_shapes_stay_explicit() {
    let coalesced = access_shapes()
        .coalesced_page_read()
        .expect("coalesced page reads should admit on the foreground lane");
    let streaming = access_shapes()
        .streaming_read(AccessLaneClassification::Maintenance)
        .expect("streaming reads should admit on the maintenance lane");
    let continuation = access_shapes()
        .streaming_continuation_read(
            AccessLaneClassification::Foreground,
            StreamingContinuationBasis::ResumeCursorContinuation,
        )
        .expect("streaming continuation should require an explicit continuation basis");

    assert_eq!(
        coalesced.detail(),
        AccessShapeDetail::CoalescedPageRead(
            crate::access::shape::CoalescedPageReadBasis::AdjacentPageWindow
        )
    );
    assert_eq!(
        coalesced.expected_counters(),
        ExpectedCounterClass::CoalescedPageRead
    );
    assert_eq!(
        streaming.detail(),
        AccessShapeDetail::StreamingRead(StreamingReadBasis::SequentialStreamTraversal)
    );
    assert_eq!(
        streaming.expected_counters(),
        ExpectedCounterClass::StreamingRead
    );
    assert_eq!(
        continuation.detail(),
        AccessShapeDetail::StreamingContinuationRead(
            StreamingContinuationBasis::ResumeCursorContinuation,
        )
    );
    assert_eq!(
        continuation.expected_counters(),
        ExpectedCounterClass::StreamingContinuationRead
    );

    assert_eq!(
        access_shapes().chunk_tree_walk(AccessLaneClassification::Verifier),
        Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::ChunkTreeWalk,
            lane: AccessLaneClassification::Verifier,
        })
    );
    assert_eq!(
        access_shapes().streaming_continuation_read(
            AccessLaneClassification::Terminal,
            StreamingContinuationBasis::ResumeCursorContinuation,
        ),
        Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::StreamingContinuationRead,
            lane: AccessLaneClassification::Terminal,
        })
    );
}

#[test]
fn mutation_and_maintenance_shapes_keep_denial_boundaries() {
    let append = access_shapes()
        .append(PhysicalMutationShape::LogStructuredAppend)
        .expect("append should admit only for log-structured mutation shapes");
    let compaction = access_shapes()
        .compaction_read(PhysicalMutationShape::CompactionRewrite)
        .expect("compaction should admit only for compaction rewrite shapes");
    let rebuild = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .expect("rebuild should admit on the maintenance lane");
    let verifier = access_shapes()
        .verifier_read(AccessLaneClassification::Verifier)
        .expect("verifier should admit on the verifier lane");
    let repair = access_shapes()
        .repair_read(AccessLaneClassification::Maintenance)
        .expect("repair should admit on the maintenance lane");
    let quarantine = access_shapes()
        .quarantine_read(AccessLaneClassification::Verifier)
        .expect("quarantine should admit on the verifier lane");

    assert_eq!(
        append.detail(),
        AccessShapeDetail::Append(MutationAccessBasis::WalBeforeDataAppend)
    );
    assert_eq!(
        append.expected_counters(),
        ExpectedCounterClass::AppendTraversal
    );
    assert_eq!(
        compaction.detail(),
        AccessShapeDetail::CompactionRead(MutationAccessBasis::CompactionRewriteTraversal)
    );
    assert_eq!(
        compaction.expected_counters(),
        ExpectedCounterClass::CompactionTraversal
    );
    assert_eq!(
        rebuild.detail(),
        AccessShapeDetail::RebuildRead(MaintenanceReadBasis::RebuildTraversal)
    );
    assert_eq!(
        rebuild.expected_counters(),
        ExpectedCounterClass::RebuildTraversal
    );
    assert_eq!(
        verifier.detail(),
        AccessShapeDetail::VerifierRead(MaintenanceReadBasis::VerifierTraversal)
    );
    assert_eq!(
        verifier.expected_counters(),
        ExpectedCounterClass::VerifierTraversal
    );
    assert_eq!(
        repair.detail(),
        AccessShapeDetail::RepairRead(MaintenanceReadBasis::RepairTraversal)
    );
    assert_eq!(
        repair.expected_counters(),
        ExpectedCounterClass::RepairTraversal
    );
    assert_eq!(
        quarantine.detail(),
        AccessShapeDetail::QuarantineRead(MaintenanceReadBasis::QuarantineTraversal)
    );
    assert_eq!(
        quarantine.expected_counters(),
        ExpectedCounterClass::QuarantineTraversal
    );

    assert_eq!(
        access_shapes().append(PhysicalMutationShape::ObservationOnly),
        Err(
            AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: AccessShape::Append,
                mutation_shape: PhysicalMutationShape::ObservationOnly,
            },
        )
    );
    assert_eq!(
        access_shapes().compaction_read(PhysicalMutationShape::LogStructuredAppend),
        Err(
            AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: AccessShape::CompactionRead,
                mutation_shape: PhysicalMutationShape::LogStructuredAppend,
            },
        )
    );
    assert_eq!(
        access_shapes().rebuild_read(AccessLaneClassification::Verifier),
        Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::RebuildRead,
            lane: AccessLaneClassification::Verifier,
        })
    );
    assert_eq!(
        access_shapes().verifier_read(AccessLaneClassification::Maintenance),
        Err(AccessShapeUnsupportedDenial::LaneDoesNotSupportShape {
            shape: AccessShape::VerifierRead,
            lane: AccessLaneClassification::Maintenance,
        })
    );
}

#[test]
fn degraded_exact_scan_stays_budgeted_and_counter_distinct() {
    assert_eq!(
        access_shapes().explicit_degraded_exact_scan(DegradedExactScanRequest::new()),
        Err(AccessShapeUnsupportedDenial::DegradedExactScanBudgetRequired)
    );

    let degraded = access_shapes()
        .explicit_degraded_exact_scan(DegradedExactScanRequest::new().with_budget_rows(64))
        .expect("degraded exact scans should require an explicit budget");

    assert_eq!(
        degraded.detail(),
        AccessShapeDetail::DegradedExactScan(
            DegradedExactScanBasis::BudgetedCounterBoundedTraversal,
        )
    );
    assert_eq!(
        degraded.expected_counters(),
        ExpectedCounterClass::DegradedExactScan
    );
    assert_eq!(degraded.budget_rows(), Some(64));
}
