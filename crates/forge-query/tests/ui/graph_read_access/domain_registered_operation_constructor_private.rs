use forge_query::facade::runtime::ForgeQueryDomainRegisteredGraphReadOperation;

fn main() {
    let _ = ForgeQueryDomainRegisteredGraphReadOperation {
        operation_name: String::new(),
        operation_version: 1,
        domain_owner: String::new(),
        accepted_relations: Vec::new(),
        traversal_operator: forge_query::facade::runtime::ForgeQueryGraphReadTraversalOperator::DeclarationTraversal,
        capability_requirements: Vec::new(),
    };
}

