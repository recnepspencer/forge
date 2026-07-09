use worth_query::facade::{
    CausalEvidenceReference, CausalEvidenceReferenceDigest, CausalEvidenceReferenceReceipt,
    CausalEvidenceReferenceSet, CausalObservationAnchor,
};

fn main() {
    let anchor: CausalObservationAnchor = todo!();
    let references: Vec<CausalEvidenceReference> = Vec::new();
    let reference_set_digest: CausalEvidenceReferenceDigest = todo!();
    let receipt: CausalEvidenceReferenceReceipt = todo!();

    let _ = CausalEvidenceReferenceSet {
        anchor,
        references,
        reference_set_digest,
        receipt,
    };
}
