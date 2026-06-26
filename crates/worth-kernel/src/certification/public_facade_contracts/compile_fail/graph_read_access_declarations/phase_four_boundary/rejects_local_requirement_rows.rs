use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementKind;
use worth_kernel::graph_read_access_declarations::WorthGraphReadQueryRequirementSetEvidence;

fn main() {
    let local_rows = vec![ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency];
    let _: WorthGraphReadQueryRequirementSetEvidence = local_rows.into();
}
