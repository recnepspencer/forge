use worth_query::facade::{CausalEvidenceFamily, CausalObservationEvidenceIdentity};

fn main() {
    let _ = CausalObservationEvidenceIdentity::new(
        CausalEvidenceFamily::BridgeRoute,
        "raw-bridge-route-digest",
    );
}
