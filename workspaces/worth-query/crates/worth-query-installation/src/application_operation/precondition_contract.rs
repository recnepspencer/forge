use worth_query_declaration::facade::application_schema::{
    ApplicationMutationPreconditionTarget, ApplicationOperationDecisionReadTarget,
};

use super::WorthQueryInstalledAbilityRequirement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledMutationPrecondition {
    target: ApplicationMutationPreconditionTarget,
}

impl WorthQueryInstalledMutationPrecondition {
    pub(crate) const fn new(target: ApplicationMutationPreconditionTarget) -> Self {
        Self { target }
    }

    pub const fn target(&self) -> &ApplicationMutationPreconditionTarget {
        &self.target
    }
}

pub(crate) fn compile_precondition_contract(
    mut targets: Vec<ApplicationMutationPreconditionTarget>,
    decision_reads: &[ApplicationOperationDecisionReadTarget],
    abilities: &[WorthQueryInstalledAbilityRequirement],
) -> Result<Vec<WorthQueryInstalledMutationPrecondition>, ()> {
    targets.sort();
    if targets.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(());
    }
    targets
        .into_iter()
        .map(|target| {
            if !is_installed_decision_read(&target, decision_reads)
                || abilities
                    .iter()
                    .any(|ability| ability.scope_entity() != target.entity())
            {
                return Err(());
            }
            Ok(WorthQueryInstalledMutationPrecondition::new(target))
        })
        .collect()
}

fn is_installed_decision_read(
    target: &ApplicationMutationPreconditionTarget,
    decision_reads: &[ApplicationOperationDecisionReadTarget],
) -> bool {
    decision_reads.iter().any(|read| {
        matches!(
            read,
            ApplicationOperationDecisionReadTarget::Field {
                entity,
                aspect,
                field,
            } if entity == target.entity()
                && aspect == target.aspect()
                && field == target.field_name()
        )
    })
}
