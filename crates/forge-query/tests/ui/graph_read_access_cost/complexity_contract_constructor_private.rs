use forge_query::facade::runtime::{
    ForgeQueryGraphReadComplexityContract, ForgeQueryGraphReadComplexityContractKind,
};

fn main() {
    let _ = ForgeQueryGraphReadComplexityContract {
        kind: ForgeQueryGraphReadComplexityContractKind::InlineEphemeralCandidate,
    };
}
