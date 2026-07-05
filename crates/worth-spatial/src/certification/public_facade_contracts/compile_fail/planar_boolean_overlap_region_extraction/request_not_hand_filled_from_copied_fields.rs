use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapReadinessLoopLedgerBinding, PlanarBooleanOverlapRegionExtractionRequest,
};

fn main() {
    let binding: PlanarBooleanOverlapReadinessLoopLedgerBinding = todo!();
    let _ = PlanarBooleanOverlapRegionExtractionRequest {
        request_identity: String::new(),
        readiness_loop_ledger_binding: binding,
        counters: todo!(),
    };
}
