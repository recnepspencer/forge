use worth_query::facade::{
    CausalEvidenceFamily, CausalEvidenceOwner, CausalEvidenceReferenceIndexRecord,
};

fn main() {
    let _ = CausalEvidenceReferenceIndexRecord {
        owner: CausalEvidenceOwner::RuntimeBridge,
        family: CausalEvidenceFamily::BridgeRoute,
        reference_digest: "bridge-route-reference".to_string(),
        record_digest: "record-digest".to_string(),
    };
}
