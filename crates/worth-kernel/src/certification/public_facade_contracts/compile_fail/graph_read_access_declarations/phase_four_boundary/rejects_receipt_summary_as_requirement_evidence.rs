use forge_query::facade::runtime::ForgeQueryGraphReadAccessReceiptSummary;
use worth_kernel::graph_read_access_declarations::WorthGraphReadQueryRequirementSetEvidence;

fn smuggle(summary: ForgeQueryGraphReadAccessReceiptSummary) {
    let _: WorthGraphReadQueryRequirementSetEvidence = summary.into();
}

fn main() {}
