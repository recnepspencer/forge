use worth_query::facade::runtime::{CausalEvidenceFamily, CausalObservationEvidenceIdentity};

fn main() {
    let _ = CausalObservationEvidenceIdentity::new(
        CausalEvidenceFamily::BridgeRoute,
        "raw-bridge-route-digest",
    );
}
