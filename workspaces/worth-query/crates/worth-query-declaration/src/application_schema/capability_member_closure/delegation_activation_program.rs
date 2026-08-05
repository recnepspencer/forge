//! Application-program cutoff for delegation activation operations.

use std::collections::BTreeSet;

use crate::application_capability::ErasedApplicationCapabilityContract;
use crate::application_schema::ApplicationSchemaMember;

pub(super) fn activation_programs_are_framework_owned(
    members: &[ApplicationSchemaMember],
    contracts: &[&ErasedApplicationCapabilityContract],
) -> bool {
    let activation_operations = contracts
        .iter()
        .filter_map(|contract| {
            contract
                .delegation()
                .activation()
                .map(|activation| activation.operation().operation())
        })
        .collect::<BTreeSet<_>>();
    members.iter().all(|member| {
        !matches!(
            member,
            ApplicationSchemaMember::OperationProgram { operation, .. }
                if activation_operations.contains(operation.as_str())
        )
    })
}
