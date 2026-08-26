use crate::application_capability::{
    ApplicationCapabilityDisclosureRule, ApplicationCapabilityElevationRule,
    ApplicationCapabilityGraphRule, ApplicationCapabilityScopeGuard,
    ErasedApplicationCapabilityContract,
};

use super::super::ApplicationSchemaDeclarationDenial as Denial;

pub(super) fn validate_canonical_structure(
    contract: &ErasedApplicationCapabilityContract,
) -> Result<(), Denial> {
    if contract.operation_type() != contract.operation()
        || !operation_bindings_are_derived(contract)
    {
        return Err(Denial::InvalidApplicationCapability);
    }
    if !canonical_collections(contract) {
        return Err(Denial::InvalidCanonicalOrdering);
    }
    Ok(())
}

fn operation_bindings_are_derived(contract: &ErasedApplicationCapabilityContract) -> bool {
    let delegation = contract.delegation();
    delegation
        .activation()
        .is_none_or(|value| operation_identity_is_derived(value.operation()))
        && delegation
            .revocation()
            .is_none_or(|value| operation_identity_is_derived(value.operation()))
        && match contract.elevation() {
            ApplicationCapabilityElevationRule::NotApplicable => true,
            ApplicationCapabilityElevationRule::Governed(elevation) => elevation
                .lifecycle()
                .transitions()
                .into_iter()
                .all(|transition| {
                    operation_identity_is_derived(transition.operation())
                        && transition
                            .lifecycle_effect()
                            .is_none_or(|effect| effect.effect() == effect.effect_type())
                }),
        }
}

fn operation_identity_is_derived(
    binding: &crate::application_capability::ApplicationCapabilityOperationBinding,
) -> bool {
    binding.operation() == binding.operation_type()
}

fn canonical_collections(contract: &ErasedApplicationCapabilityContract) -> bool {
    contract
        .delegation()
        .activation()
        .is_none_or(|activation| strictly_increasing(activation.context_relations()))
        && canonical_graph_rule(contract.composition().decision().allow().graph())
        && [
            contract.composition().decision().deny().graph(),
            contract.composition().decision().conflict().graph(),
            contract.composition().actors().separation_of_duty().graph(),
            contract.composition().actors().distinct_actor().graph(),
        ]
        .into_iter()
        .flatten()
        .all(canonical_graph_rule)
        && match contract.composition().propagation().disclosure() {
            ApplicationCapabilityDisclosureRule::NotApplicable => true,
            ApplicationCapabilityDisclosureRule::Permit(guards) => {
                strictly_increasing(guards) && guards.iter().all(canonical_guard)
            }
        }
}

fn canonical_graph_rule(rule: &ApplicationCapabilityGraphRule) -> bool {
    strictly_increasing(rule.requirements())
        && rule.requirements().iter().all(|requirement| {
            strictly_increasing(requirement.clauses())
                && requirement.clauses().iter().all(|clause| {
                    canonical_guard(clause.guard()) && strictly_increasing(clause.context_anchors())
                })
        })
}

fn canonical_guard(guard: &ApplicationCapabilityScopeGuard) -> bool {
    strictly_increasing(guard.requirements())
        && guard
            .requirements()
            .iter()
            .all(|requirement| strictly_increasing(requirement.values()))
}

fn strictly_increasing<Value: Ord>(values: &[Value]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}
