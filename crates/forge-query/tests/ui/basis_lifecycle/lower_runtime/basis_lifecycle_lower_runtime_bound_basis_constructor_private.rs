use forge_query::facade::{
    BasisEligibilityCounters, BasisEligibilityDecisionTrace, LowerRuntimeBoundBasis,
    LowerRuntimeEvidenceAuthority, ScopedObservationBasis,
};

fn scoped_basis() -> ScopedObservationBasis {
    unimplemented!()
}

fn counters() -> BasisEligibilityCounters {
    unimplemented!()
}

fn readmission_trace() -> BasisEligibilityDecisionTrace {
    unimplemented!()
}

fn main() {
    let _ = LowerRuntimeBoundBasis::<ScopedObservationBasis> {
        scoped_basis: scoped_basis(),
        authority: LowerRuntimeEvidenceAuthority::Runtime,
        basis_digest: String::new(),
        evidence_digest: String::new(),
        lower_runtime_binding_digest: String::new(),
        readmission_trace: readmission_trace(),
        counters: counters(),
    };
}
