use worth_query::facade::runtime::{WorthQueryEphemeralGraphIndexAllocationRow, WorthQueryGraphReadAccessRequirementKind};

fn main() {
    let _ = WorthQueryEphemeralGraphIndexAllocationRow {
        requirement_kind: WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        requirement_row_digest: "requirement".to_string(),
        estimated_bytes: 1,
    };
}
