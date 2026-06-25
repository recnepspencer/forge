use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementKind;
use worth_kernel::graph_read_access_declarations::WorthGraphReadAccessDeclarationPhaseSevenSeed;

fn main() {
    let local_requirement_rows = vec![ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency];
    let _: WorthGraphReadAccessDeclarationPhaseSevenSeed = local_requirement_rows.into();
}
