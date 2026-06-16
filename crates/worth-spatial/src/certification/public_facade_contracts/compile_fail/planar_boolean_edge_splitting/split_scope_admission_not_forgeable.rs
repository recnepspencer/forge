use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitDegeneracyPolicy, PlanarBooleanEdgeSplitDeterminismPolicy,
    PlanarBooleanEdgeSplitOverlapPolicy, PlanarBooleanEdgeSplitPolicyOutcome,
    PlanarBooleanEdgeSplitScopeAdmission, PlanarBooleanEdgeSplitScopeAdmissionCounters,
    PlanarBooleanEdgeSplitScopeClass,
};

fn main() {
    let _ = PlanarBooleanEdgeSplitScopeAdmission {
        scope_admission_identity: String::new(),
        split_request_identity: String::new(),
        event_ledger_identity: String::new(),
        downstream_consumption_identity: String::new(),
        reduced_pair_identity: String::new(),
        segment_carrier_set_identity: String::new(),
        candidate_index_product_identity: String::new(),
        query_index_plan_digest: String::new(),
        scope_class: PlanarBooleanEdgeSplitScopeClass::PlanarBRepLineSegmentEdgeSurgery,
        degeneracy_policy: PlanarBooleanEdgeSplitDegeneracyPolicy::fail_closed(),
        determinism_policy: PlanarBooleanEdgeSplitDeterminismPolicy::canonical_replay(),
        overlap_policy: PlanarBooleanEdgeSplitOverlapPolicy::preserve_interval_posture(),
        policy_outcome: policy_outcome(),
        counters: PlanarBooleanEdgeSplitScopeAdmissionCounters::default(),
    };
}

fn policy_outcome() -> PlanarBooleanEdgeSplitPolicyOutcome {
    unimplemented!("external callers cannot construct policy outcomes")
}
