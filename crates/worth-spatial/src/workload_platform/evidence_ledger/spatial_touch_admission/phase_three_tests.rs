use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceReceiptSealed, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger,
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationCounters;

#[test]
fn spatial_touch_digest_is_stable_across_equivalent_ledger_row_order() {
    let receipt = FakeSplitReceipt::admitted("split-stable-digest");
    let ordinary = complete_ledger_with_receipt(&receipt);
    let reordered = complete_ledger_with_reordered_receipt(&receipt);

    let ordinary_authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&ordinary)
        .admit()
        .expect("ordinary row order should admit");
    let reordered_authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&reordered)
        .admit()
        .expect("reordered equivalent row order should admit");

    assert_eq!(ordinary_authority.digest(), reordered_authority.digest());
    assert_eq!(
        ordinary_authority.stage_index_identity(),
        reordered_authority.stage_index_identity()
    );
}

#[test]
fn boundary_denial_rejects_non_authority_vocabulary_before_construction() {
    struct RawSpatialId(&'static str);
    struct QueryDescriptor;
    struct TopologyTouchedBasisProof;
    struct SchemaVocabularyRow;
    struct CopiedReceiptFields {
        stage: BooleanEvidenceStageKind,
        identity: &'static str,
    }

    let raw_id = RawSpatialId("split-id");
    let raw_string = "split-id";
    let copied_fields = CopiedReceiptFields {
        stage: BooleanEvidenceStageKind::Split,
        identity: "split-id",
    };
    let denied = [
        SpatialGeometryEvidenceTouchRejectedInput::raw_id(&raw_id).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::raw_string(raw_string).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::copied_receipt_fields(&copied_fields).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::schema_vocabulary(&SchemaVocabularyRow).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::topology_proof(&TopologyTouchedBasisProof)
            .deny(),
        SpatialGeometryEvidenceTouchRejectedInput::query_descriptor(&QueryDescriptor).deny(),
    ];

    for denial in denied {
        assert_eq!(
            denial.kind(),
            SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
        );
    }
    assert_eq!(raw_id.0, "split-id");
    assert_eq!(copied_fields.stage, BooleanEvidenceStageKind::Split);
    assert_eq!(copied_fields.identity, "split-id");
}

#[test]
fn counter_honesty_exposes_honest_rows_and_denies_zeroed_products() {
    let receipt = FakeSplitReceipt::admitted("split-counter-honesty");
    let complete = complete_ledger_with_receipt(&receipt);
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&complete)
        .admit()
        .expect("receipt-backed counters should admit");

    assert!(authority.counter_honesty().is_honest());
    assert_eq!(authority.operating_world().posture(), "current-head");

    let violation = super::counter_honesty::spatial_touch_counter_honesty(
        WorkloadEvidenceStage::BooleanSplit,
        WorkloadEvidenceStageCounters::default(),
    )
    .violation()
    .expect("zero counters should expose a violation row");
    assert_eq!(violation.stage(), WorkloadEvidenceStage::BooleanSplit);
    assert_eq!(violation.observed_receipt_backed_counter_total(), 0);

    let counterless_complete = complete_ledger_with_counterless_receipt(&receipt);
    let denial = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&counterless_complete)
        .admit()
        .expect_err("zeroed receipt-backed counters must deny before authority");
    assert_eq!(
        denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::CounterHonesty
    );

    let segment_pair_receipt =
        FakeSegmentPairReceipt::admitted("segment-pair-counter-honesty", 2, 3, 6, 0);
    let mismatched_nonzero_complete =
        complete_ledger_with_mismatched_segment_pair_counters(&segment_pair_receipt);
    let denial = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&segment_pair_receipt)
        .with_complete_ledger(&mismatched_nonzero_complete)
        .admit()
        .expect_err("nonzero but mismatched counters must deny before authority");
    assert_eq!(
        denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::CounterHonesty
    );
}

#[test]
fn stage_vocabulary_maps_every_boolean_stage_to_workload_evidence_stage() {
    assert_eq!(
        SPATIAL_TOUCH_BOOLEAN_EVIDENCE_STAGE_KINDS.len(),
        WorkloadEvidenceStage::BOOLEAN_STAGES.len()
    );

    for (boolean_stage, expected_stage) in SPATIAL_TOUCH_BOOLEAN_EVIDENCE_STAGE_KINDS
        .iter()
        .zip(WorkloadEvidenceStage::BOOLEAN_STAGES.iter())
    {
        assert_eq!(
            spatial_touch_workload_evidence_stage(*boolean_stage),
            *expected_stage
        );
        assert!(expected_stage.is_boolean_stage());
    }
}

fn complete_ledger_with_receipt(receipt: &FakeSplitReceipt) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.push(WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt));
    complete_ledger_from_rows(rows)
}

fn complete_ledger_with_reordered_receipt(
    receipt: &FakeSplitReceipt,
) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.reverse();
    rows.insert(
        0,
        WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt),
    );
    complete_ledger_from_rows(rows)
}

fn complete_ledger_with_counterless_receipt(
    receipt: &FakeSplitReceipt,
) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.push(WorkloadEvidenceRow::receipt_backed_with_receipt_type::<
        FakeSplitReceipt,
    >(
        receipt.boolean_stage().evidence_stage(),
        receipt.evidence_identity(),
        receipt.evidence_support(),
        WorkloadEvidenceStageCounters::default(),
    ));
    complete_ledger_from_rows(rows)
}

fn complete_ledger_with_mismatched_segment_pair_counters(
    receipt: &FakeSegmentPairReceipt,
) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.push(WorkloadEvidenceRow::receipt_backed_with_receipt_type::<
        FakeSegmentPairReceipt,
    >(
        receipt.boolean_stage().evidence_stage(),
        receipt.evidence_identity(),
        receipt.evidence_support(),
        segment_pair_evidence_counters(1, 1, 1, 0),
    ));
    complete_ledger_from_rows(rows)
}

fn complete_ledger_from_rows(rows: Vec<WorkloadEvidenceRow>) -> CompleteWorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("rows should index")
        .certify_complete()
        .expect("authority rows should complete")
}

fn authority_rows() -> Vec<WorkloadEvidenceRow> {
    vec![
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Topology,
            "topology",
            WorkloadEvidenceStageCounters::topology(1, 1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            "geometry",
            WorkloadEvidenceStageCounters::binding(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            "surface",
            WorkloadEvidenceStageCounters::surface_support(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Projection,
            "projection",
            WorkloadEvidenceStageCounters::projection(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Transform,
            "transform",
            WorkloadEvidenceStageCounters::transform(1, 1, 0),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::RetainedReplay,
            "replay",
            WorkloadEvidenceStageCounters::retained_replay(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Diagnostics,
            "diagnostics",
            WorkloadEvidenceStageCounters::diagnostics(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Response,
            "response",
            WorkloadEvidenceStageCounters::response(1),
        ),
    ]
}

struct FakeSplitReceipt {
    identity: &'static str,
}

impl FakeSplitReceipt {
    fn admitted(identity: &'static str) -> Self {
        Self { identity }
    }
}

impl BooleanEvidenceReceipt for FakeSplitReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}

impl BooleanEvidenceReceiptSealed for FakeSplitReceipt {}

impl BooleanEvidenceRowAuthority for FakeSplitReceipt {}

struct FakeSegmentPairReceipt {
    identity: &'static str,
    counters: WorkloadEvidenceStageCounters,
}

impl FakeSegmentPairReceipt {
    fn admitted(
        identity: &'static str,
        left_segment_count: usize,
        right_segment_count: usize,
        emitted_pair_breadth: usize,
        skipped_pair_count: usize,
    ) -> Self {
        Self {
            identity,
            counters: segment_pair_evidence_counters(
                left_segment_count,
                right_segment_count,
                emitted_pair_breadth,
                skipped_pair_count,
            ),
        }
    }
}

impl BooleanEvidenceReceipt for FakeSegmentPairReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SegmentPairEnumeration
    }

    fn evidence_identity(&self) -> &str {
        self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}

impl BooleanEvidenceReceiptSealed for FakeSegmentPairReceipt {}

impl BooleanEvidenceRowAuthority for FakeSegmentPairReceipt {}

fn segment_pair_evidence_counters(
    left_segment_count: usize,
    right_segment_count: usize,
    emitted_pair_breadth: usize,
    skipped_pair_count: usize,
) -> WorkloadEvidenceStageCounters {
    WorkloadEvidenceStageCounters::boolean_segment_pair_enumeration(
        PlanarBooleanSegmentPairEnumerationCounters::new(
            left_segment_count,
            right_segment_count,
            emitted_pair_breadth,
            skipped_pair_count,
        ),
    )
}
