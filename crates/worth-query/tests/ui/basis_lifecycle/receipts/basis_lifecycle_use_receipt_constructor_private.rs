use worth_query::facade::foundation::{BasisAuthorityPosture, BasisEligibilityCounters, BasisFamily, BasisLifecyclePosture, BasisUseReceipt, BasisUseReceiptKind};

fn counters() -> BasisEligibilityCounters {
    unimplemented!()
}

fn main() {
    let _ = BasisUseReceipt {
        kind: BasisUseReceiptKind::Observation,
        basis_family: BasisFamily::CurrentHead,
        authority: BasisAuthorityPosture::Runtime,
        lifecycle: BasisLifecyclePosture::Current,
        capability_digest: String::new(),
        scoped_basis_digest: String::new(),
        lower_runtime_basis_digest: String::new(),
        lower_runtime_binding_digest: String::new(),
        lower_runtime_evidence_digest: String::new(),
        readmission_trace_digest: String::new(),
        permitted_next_transitions: Vec::new(),
        receipt_digest: String::new(),
        counters: counters(),
    };
}
