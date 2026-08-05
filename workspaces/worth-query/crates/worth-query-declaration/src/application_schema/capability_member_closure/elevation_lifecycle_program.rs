use crate::application_capability::{
    ApplicationCapabilityElevationRule, ErasedApplicationCapabilityContract,
};

use super::super::ApplicationSchemaMember;

pub(super) fn lifecycle_program_targets_are_framework_owned(
    members: &[ApplicationSchemaMember],
    contracts: &[&ErasedApplicationCapabilityContract],
) -> bool {
    contracts.iter().all(|contract| {
        let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
            return true;
        };
        elevation.lifecycle().transitions().into_iter().all(|transition| {
            let operation = transition.operation().operation();
            let explicit_framework_target = members.iter().any(|member| {
                matches!(
                    member,
                    ApplicationSchemaMember::OperationProgram {
                        operation: installed,
                        target: crate::application_schema::ApplicationOperationProgramTarget::Emit { .. },
                    } if installed == operation
                )
            }) || transition == elevation.lifecycle().request()
                && elevation.resource_relation().is_some_and(|resource| {
                    members.iter().any(|member| {
                        matches!(
                            member,
                            ApplicationSchemaMember::OperationProgram {
                                operation: installed,
                                target: crate::application_schema::ApplicationOperationProgramTarget::Link {
                                    relation,
                                    from,
                                    to,
                                },
                            } if installed == operation
                                && relation == resource.relation()
                                && from == resource.from()
                                && to == resource.to()
                        )
                    })
                });
            !explicit_framework_target && transition_effect_is_declared(members, transition)
        })
    })
}

fn transition_effect_is_declared(
    members: &[ApplicationSchemaMember],
    transition: &crate::application_capability::ApplicationCapabilityTransitionBinding,
) -> bool {
    let Some(binding) = transition.lifecycle_effect() else {
        return true;
    };
    members.iter().any(|member| {
        matches!(
            member,
            ApplicationSchemaMember::Effect { effect, payload_type }
                if effect == binding.effect() && payload_type == binding.payload_type()
        )
    })
}
