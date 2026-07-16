use super::{
    registry::WorthQueryGraphReadOperationLookup, WorthQueryGraphReadOperationOutcome,
    WorthQueryGraphReadOperationResolution, WorthQueryGraphReadResolvedOperation,
};
use crate::runtime::{
    WorthQueryAdmittedQuerySchemaReferences, WorthQueryGraphReadBasisBinding,
    WorthQueryGraphReadPolicyTenantProofBinding, WorthQueryReadGraph,
};

pub(crate) fn resolve_graph_read_operations_for_read_graph(
    read_graph: &WorthQueryReadGraph,
    references: WorthQueryAdmittedQuerySchemaReferences,
    basis_binding: WorthQueryGraphReadBasisBinding,
    policy_tenant_proof_binding: WorthQueryGraphReadPolicyTenantProofBinding,
    registry: &impl WorthQueryGraphReadOperationLookup,
) -> WorthQueryGraphReadOperationOutcome {
    if let Some(declaration) = read_graph.domain_graph_operations().first() {
        let declared_relation_names = declared_operation_relation_names(declaration);
        if let Some(denial) = registry.matching_unsupported_declared_operation(declaration) {
            return WorthQueryGraphReadOperationOutcome::DeniedUnsupportedShape(
                denial.resolve_for_read_graph(read_graph.digest(), declared_relation_names),
            );
        }
        if let Some(registration) = registry.matching_declared_operation(declaration) {
            let operations = vec![WorthQueryGraphReadResolvedOperation::domain_registered(
                registration.admitted(),
            )];
            return WorthQueryGraphReadOperationOutcome::Resolved(
                WorthQueryGraphReadOperationResolution::new(
                    read_graph.digest(),
                    read_graph.family().clone(),
                    read_graph.scope_class().clone(),
                    references.relations().len()
                        + references.projections().len()
                        + references.predicates().len()
                        + references.orderings().len(),
                    references,
                    basis_binding,
                    policy_tenant_proof_binding,
                    operations,
                ),
            );
        }
        if let Some(support_family) = declaration.support_families().first() {
            let requirement =
                super::WorthQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                    declaration.key().name().as_str(),
                    declaration.key().owner().as_str(),
                    support_family,
                );
            return WorthQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(
                requirement.resolve_for_read_graph(read_graph.digest(), declared_relation_names),
            );
        }
        let requirement =
            super::WorthQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                declaration.key().name().as_str(),
                declaration.key().owner().as_str(),
                "worth.query.installed-domain-operation",
            );
        return WorthQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(
            requirement.resolve_for_read_graph(read_graph.digest(), declared_relation_names),
        );
    }

    let mut operations = read_graph
        .built_in_operators()
        .iter()
        .cloned()
        .map(WorthQueryGraphReadResolvedOperation::built_in)
        .collect::<Vec<_>>();
    if operations.is_empty() {
        if !references.relations().is_empty() {
            operations.push(WorthQueryGraphReadResolvedOperation::declaration_traversal());
        }
    }

    WorthQueryGraphReadOperationOutcome::Resolved(WorthQueryGraphReadOperationResolution::new(
        read_graph.digest(),
        read_graph.family().clone(),
        read_graph.scope_class().clone(),
        references.relations().len()
            + references.projections().len()
            + references.predicates().len()
            + references.orderings().len(),
        references,
        basis_binding,
        policy_tenant_proof_binding,
        operations,
    ))
}

fn declared_operation_relation_names(
    declaration: &crate::authoring::WorthQueryGraphReadDomainOperationDeclaration,
) -> Vec<String> {
    let mut relation_names = declaration
        .admitted_references()
        .iter()
        .map(|reference| {
            reference
                .terminal_relation_projection_for_boundary()
                .to_string()
        })
        .collect::<Vec<_>>();
    relation_names.sort();
    relation_names.dedup();
    relation_names
}
