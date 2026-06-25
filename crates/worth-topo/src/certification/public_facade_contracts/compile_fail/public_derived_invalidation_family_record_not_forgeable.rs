use topology::derived_invalidation_family_catalog::{
    DerivedTopologyConsumedGraphFacts, DerivedTopologyDiagnosticPosture,
    DerivedTopologyInvalidationPredicate, DerivedTopologyLegalityReceiptPosture,
    DerivedTopologyProductFamilyIdentity, DerivedTopologyProductFamilyRecord,
    DerivedTopologyQueryReceiptPosture, DerivedTopologySpatialEvidencePosture,
    DerivedTopologySupportPosture, DerivedTopologyUpdatePosture,
};

fn main() {
    let _ = DerivedTopologyProductFamilyRecord {
        identity: DerivedTopologyProductFamilyIdentity::LoopCycles,
        consumed_graph_facts: consumed_graph_facts(),
        invalidation_predicate:
            DerivedTopologyInvalidationPredicate::ConsumedGraphFactsIntersectTouchedClosure,
        update_posture: DerivedTopologyUpdatePosture::IncrementalEligible,
        spatial_evidence_posture: DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed,
        query_receipt_posture: DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        legality_receipt_posture:
            DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
        diagnostic_posture: DerivedTopologyDiagnosticPosture::ProductFamilyWitnessRequired,
        support_posture: DerivedTopologySupportPosture::QuerySupportRequired,
        family_digest: String::new(),
    };
}

fn consumed_graph_facts() -> DerivedTopologyConsumedGraphFacts {
    panic!("compile-fail fixture never executes")
}
