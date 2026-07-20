use super::{
    WorthQueryDomainOperationDefinitionRecord, WorthQueryDomainOperationRequiredDomainRecord,
    WorthQueryDomainPackageValidationDenial, WorthQueryDomainPackageValidationDenialKind,
};

pub(super) fn validate_operation_required_domains(
    operations: &[WorthQueryDomainOperationDefinitionRecord],
    bindings: &[WorthQueryDomainOperationRequiredDomainRecord],
) -> Result<(), WorthQueryDomainPackageValidationDenial> {
    for binding in bindings {
        let operation = operations
            .iter()
            .find(|operation| {
                operation.operation_marker() == binding.operation_marker()
                    && operation.family_marker() == binding.family_marker()
            })
            .ok_or_else(|| denial("required domain names an uninstalled operation"))?;
        let declared = operation
            .definition()
            .semantics()
            .required_domains
            .iter()
            .any(|role| role.as_str() == binding.role());
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
        for role in &operation.definition().semantics().required_domains {
            if !bindings.iter().any(|binding| {
                binding.operation_marker() == operation.operation_marker()
                    && binding.family_marker() == operation.family_marker()
                    && binding.role() == role.as_str()
            }) {
                return Err(denial(role.as_str()));
            }
        }
    }
    Ok(())
}

fn denial(detail: impl Into<String>) -> WorthQueryDomainPackageValidationDenial {
    WorthQueryDomainPackageValidationDenial::new(
        WorthQueryDomainPackageValidationDenialKind::InvalidDomainOperationRequiredDomain,
        detail,
    )
}
