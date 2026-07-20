use super::{
    WorthQueryDomainOperationDefinitionRecord, WorthQueryDomainOperationGraphParticipationRecord,
    WorthQueryDomainPackageValidationDenial, WorthQueryDomainPackageValidationDenialKind,
};

pub(super) fn validate_operation_graph_participations(
    operations: &[WorthQueryDomainOperationDefinitionRecord],
    bindings: &[WorthQueryDomainOperationGraphParticipationRecord],
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    for binding in bindings {
        let operation = operations.iter().find(|operation| {
            operation.operation_marker() == binding.operation_marker()
                && operation.family_marker() == binding.family_marker()
        });
        let Some(operation) = operation else {
            return Err(denial("graph participation names an uninstalled operation"));
        };
        let declared = operation
            .definition()
            .semantics()
            .graph_reads
            .roles()
            .iter()
            .any(|role| role.role == binding.role());
        let duplicate = bindings
            .iter()
            .filter(|candidate| {
                candidate.operation_marker() == binding.operation_marker()
                    && candidate.family_marker() == binding.family_marker()
                    && candidate.role() == binding.role()
            })
            .count()
            > 1;
        if !declared || duplicate || binding.role().trim().is_empty() {
            return Err(denial(binding.role()));
        }
    }
    for operation in operations {
        for role in operation.definition().semantics().graph_reads.roles() {
            let separate = matches!(
                role.participation,
                worth_query_installation::facade::WorthQueryOperationGraphParticipation::SeparateAuthority { .. }
            );
            let bound = bindings.iter().any(|binding| {
                binding.operation_marker() == operation.operation_marker()
                    && binding.family_marker() == operation.family_marker()
                    && binding.role() == role.role
            });
            if separate && !bound {
                return Err(denial(&role.role));
            }
        }
    }
    Ok(())
}

fn denial(detail: impl Into<String>) -> WorthQueryDomainPackageValidationDenial {
    WorthQueryDomainPackageValidationDenial::new(
        WorthQueryDomainPackageValidationDenialKind::InvalidDomainOperationGraphParticipation,
        detail,
    )
}
