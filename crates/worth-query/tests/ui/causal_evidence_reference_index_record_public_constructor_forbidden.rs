use worth_query::facade::runtime::{CausalEvidenceFamily, CausalEvidenceOwner};
use worth_query::facade::causal_evidence_reference_index_record;

fn main() {
    let _ = causal_evidence_reference_index_record(
        CausalEvidenceOwner::RuntimeBridge,
        CausalEvidenceFamily::BridgeRoute,
        "bridge-route-reference",
    );
}
