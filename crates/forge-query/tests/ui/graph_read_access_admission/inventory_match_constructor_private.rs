use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessInventoryMatch,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadRequiredCapabilityOwner,
};

fn main() {
    let _ = ForgeQueryGraphReadAccessInventoryMatch {
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        required_capability_owner: ForgeQueryGraphReadRequiredCapabilityOwner::QueryRuntime,
        resolved_posture: ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
    };
}
