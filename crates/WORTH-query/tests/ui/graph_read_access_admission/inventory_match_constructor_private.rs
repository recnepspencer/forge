use worth_query::facade::runtime::{
    WorthQueryGraphReadAccessAdmissionPosture, WorthQueryGraphReadAccessInventoryMatch,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadRequiredCapabilityOwner,
};

fn main() {
    let _ = WorthQueryGraphReadAccessInventoryMatch {
        requirement_kind: WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        required_capability_owner: WorthQueryGraphReadRequiredCapabilityOwner::QueryRuntime,
        resolved_posture: WorthQueryGraphReadAccessAdmissionPosture::InlineIndexed,
    };
}
