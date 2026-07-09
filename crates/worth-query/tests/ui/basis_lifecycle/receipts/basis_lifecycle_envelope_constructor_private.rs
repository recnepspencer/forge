use worth_query::facade::{
    BasisAuthorityPosture, BasisEligibilityCounters, BasisFamily, BasisUseReceipt,
    BasisUseReceiptKind, SelfDescribingBasisEnvelope,
};

fn counters() -> BasisEligibilityCounters {
    unimplemented!()
}

fn receipt() -> BasisUseReceipt {
    unimplemented!()
}

fn main() {
    let _ = SelfDescribingBasisEnvelope {
        receipt: receipt(),
        support_matrix_digest: String::new(),
        structured_warnings: Vec::new(),
        integrity_digest: String::new(),
        envelope_digest: String::new(),
        counters: counters(),
    };

    let _ = (
        BasisUseReceiptKind::Observation,
        BasisFamily::CurrentHead,
        BasisAuthorityPosture::Runtime,
    );
}
