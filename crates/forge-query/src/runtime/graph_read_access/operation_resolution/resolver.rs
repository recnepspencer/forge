use super::{
    ForgeQueryGraphReadOperationOutcome, ForgeQueryGraphReadOperationRegistry,
    ForgeQueryGraphReadOperationResolution, ForgeQueryGraphReadResolvedOperation,
};
use crate::runtime::{
    ForgeQueryAdmittedQuerySchemaReferences, ForgeQueryGraphReadBasisBinding,
    ForgeQueryGraphReadPolicyTenantProofBinding, ForgeQueryReadGraph,
};

pub(crate) fn resolve_graph_read_operations_for_read_graph(
    read_graph: &ForgeQueryReadGraph,
    references: ForgeQueryAdmittedQuerySchemaReferences,
    basis_binding: ForgeQueryGraphReadBasisBinding,
    policy_tenant_proof_binding: ForgeQueryGraphReadPolicyTenantProofBinding,
    registry: &ForgeQueryGraphReadOperationRegistry,
) -> ForgeQueryGraphReadOperationOutcome {
    if let Some(declaration) = read_graph.domain_graph_operations().first() {
        let declared_relation_names = declared_operation_relation_names(declaration);
        if let Some(denial) = registry.matching_unsupported_declared_operation(declaration) {
            return ForgeQueryGraphReadOperationOutcome::DeniedUnsupportedShape(
                denial.resolve_for_read_graph(read_graph.digest(), declared_relation_names),
            );
        }
        if let Some(registration) = registry.matching_declared_operation(declaration) {
            let operations = vec![ForgeQueryGraphReadResolvedOperation::domain_registered(
                registration.admitted(),
            )];
            return ForgeQueryGraphReadOperationOutcome::Resolved(
                ForgeQueryGraphReadOperationResolution::new(
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
                super::ForgeQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                    declaration.key().name().as_str(),
                    declaration.key().owner().as_str(),
                    support_family,
                );
            return ForgeQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(
                requirement.resolve_for_read_graph(read_graph.digest(), declared_relation_names),
            );
        }
    }

    let mut operations = read_graph
        .built_in_operators()
        .iter()
        .cloned()
        .map(ForgeQueryGraphReadResolvedOperation::built_in)
        .collect::<Vec<_>>();
    if operations.is_empty() {
        if !references.relations().is_empty() {
            operations.push(ForgeQueryGraphReadResolvedOperation::declaration_traversal());
        }
    }

    ForgeQueryGraphReadOperationOutcome::Resolved(ForgeQueryGraphReadOperationResolution::new(
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
    declaration: &crate::authoring::ForgeQueryGraphReadDomainOperationDeclaration,
) -> Vec<String> {
    let mut relation_names = declaration
        .admitted_references()
        .iter()
        .map(|reference| reference.relation_name().to_string())
        .collect::<Vec<_>>();
    relation_names.sort();
    relation_names.dedup();
    relation_names
}
