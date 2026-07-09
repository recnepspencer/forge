use worth_query::facade::runtime::WorthQueryGraphReadOperationRegistration;

fn main() {
    let _ = WorthQueryGraphReadOperationRegistration {
        operation_name: String::new(),
        operation_version: 1,
        domain_owner: String::new(),
        accepted_relations: Vec::new(),
        traversal_operator: worth_query::facade::runtime::WorthQueryGraphReadTraversalOperator::DeclarationTraversal,
        capability_requirements: Vec::new(),
    };
}

