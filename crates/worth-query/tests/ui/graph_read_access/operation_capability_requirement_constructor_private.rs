use worth_query::facade::runtime::{
    WorthQueryGraphReadOperationCapabilityRequirement,
    WorthQueryGraphReadOperationCapabilityRequirementKind,
};

fn main() {
    let _ = WorthQueryGraphReadOperationCapabilityRequirement {
        kind: WorthQueryGraphReadOperationCapabilityRequirementKind::RequiresAccessCapabilityRegistration,
        operation_name: String::new(),
        domain_owner: String::new(),
        support_family: String::new(),
        read_graph_digest: String::new(),
        matched_relations: Vec::new(),
    };
}

