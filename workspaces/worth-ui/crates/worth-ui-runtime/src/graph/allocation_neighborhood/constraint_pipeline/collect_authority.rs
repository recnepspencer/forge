use crate::evidence::{UiAllocationNeighborhood, UiMeasurementBasis};

use super::types::ConstraintAuthorityContext;

pub(super) fn collect_constraint_authority_context<'a>(
    _measurement_basis: &UiMeasurementBasis,
    neighborhood: &'a UiAllocationNeighborhood,
) -> ConstraintAuthorityContext<'a> {
    let contract = neighborhood.layout_operator_planning_contract();
    let root = neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("allocation neighborhood must preserve a root member");
    ConstraintAuthorityContext {
        contract,
        neighborhood_identity_digest: neighborhood.identity().identity_digest(),
        contract_identity_digest: contract.identity().identity_digest(),
        allowed_families: contract.semantics().allowed_propagation_families(),
        admitted_cycle_families: contract.semantics().admitted_cycle_families(),
        root_identity_digest: root.identity_digest(),
        special_input_requirements: contract.semantics().special_input_requirements(),
    }
}
