use forge_query::facade::runtime::{
    ForgeQueryGraphReadOperationCapabilityRequirement,
    ForgeQueryGraphReadOperationCapabilityRequirementKind,
};

fn main() {
    let _ = ForgeQueryGraphReadOperationCapabilityRequirement {
        kind: ForgeQueryGraphReadOperationCapabilityRequirementKind::RequiresAccessCapabilityRegistration,
        operation_name: String::new(),
        domain_owner: String::new(),
        support_family: String::new(),
        read_graph_digest: String::new(),
        matched_relations: Vec::new(),
    };
}

