use worth_query::facade::runtime::{
    WorthQueryGraphReadComplexityContract, WorthQueryGraphReadComplexityContractKind,
};

fn main() {
    let _ = WorthQueryGraphReadComplexityContract {
        kind: WorthQueryGraphReadComplexityContractKind::InlineEphemeralCandidate,
    };
}
