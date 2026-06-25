use topology::derived_invalidation_milestone_ten_closeout::{
    DerivedInvalidationMilestoneElevenLookupReadiness,
    DerivedInvalidationMilestoneElevenProductReceiptRef, DerivedInvalidationMilestoneElevenSeed,
};

fn main() {
    let _ = DerivedInvalidationMilestoneElevenSeed {
        milestone_ten_closeout_digest: String::new(),
        selected_plan_digest: String::new(),
        execution_receipt_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        product_summary_digest: String::new(),
        performance_proof_digest: String::new(),
        deletion_audit_digest: String::new(),
        counters_digest: String::new(),
        lookup_readiness:
            DerivedInvalidationMilestoneElevenLookupReadiness::TopologyDerivedReceiptsReadySpatialEvidenceNotBound,
        topology_derived_product_receipts: fake_product_receipt_refs(),
        seed_digest: String::new(),
    };
}

fn fake_product_receipt_refs() -> Vec<DerivedInvalidationMilestoneElevenProductReceiptRef> {
    panic!("compile-fail fixture does not execute")
}
