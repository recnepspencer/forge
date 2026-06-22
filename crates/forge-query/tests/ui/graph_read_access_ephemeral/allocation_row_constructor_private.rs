use forge_query::facade::runtime::{
    ForgeQueryEphemeralGraphIndexAllocationRow, ForgeQueryGraphReadAccessRequirementKind,
};

fn main() {
    let _ = ForgeQueryEphemeralGraphIndexAllocationRow {
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        requirement_row_digest: "requirement".to_string(),
        estimated_bytes: 1,
    };
}
