use std::collections::BTreeSet;

use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::application_operation::{
    WorthQueryInstalledAbilityRequirement, WorthQueryInstalledApplicationOperationAuthorization,
};
use crate::domain_computation::WorthQueryExecutionResourceContract;
use crate::domain_operation::{
    WorthQueryInvariantExecutionContract, WorthQueryOperationEffectContract,
    WorthQueryOperationGraphReadContract, WorthQueryOperationInvariantContract,
    WorthQueryOperationTouchContract,
};

use super::{
    WorthQueryGraphObligationInstallationDenial as Denial,
    WorthQueryInstalledGraphCapabilityRequirement, WorthQueryInstalledGraphObligationContract,
    WorthQueryInstalledGraphObligationResourcePosture, WorthQueryInstalledGraphObligationSet,
};

pub(crate) fn bind_operation_obligations(
    binding: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
    source: WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<WorthQueryInstalledGraphObligationSet, Denial> {
    validate_authorization(&source)?;
    validate_mutation_contracts(&source)?;

    let mut obligations = graph_read_obligations(source.graph_reads)?;
    obligations.push(authorization_obligation(&source)?);
    if let WorthQueryOperationTouchContract::Declared { .. } = source.touches {
        obligations.push(WorthQueryInstalledGraphObligationContract::MutationTouch {
            contract: source.touches.clone(),
        });
    }
    if let WorthQueryOperationEffectContract::Declared { effect_families } = source.effects {
        let mut families = effect_families.clone();
        families.sort();
        families.dedup();
        obligations.extend(families.into_iter().map(|family| {
            WorthQueryInstalledGraphObligationContract::EffectApplication { family }
        }));
    }
    obligations.extend(
        source
            .invariant_execution
            .requirements()
            .iter()
            .cloned()
            .map(
                |requirement| WorthQueryInstalledGraphObligationContract::InvariantExecution {
                    requirement,
                },
            ),
    );
    WorthQueryInstalledGraphObligationSet::for_operation(
        binding,
        operation.to_owned(),
        input_type.to_owned(),
        obligations,
        WorthQueryInstalledGraphObligationResourcePosture::ApplicationOperation(
            source.resources.clone(),
        ),
    )
    .map_err(Into::into)
}

fn graph_read_obligations(
    contract: &WorthQueryOperationGraphReadContract,
) -> Result<Vec<WorthQueryInstalledGraphObligationContract>, Denial> {
    let mut roles = contract.roles().to_vec();
    roles.sort_by(|left, right| left.role.cmp(&right.role));
    if roles.is_empty() {
        return Err(Denial::InvalidContract);
    }
    if roles.windows(2).any(|pair| pair[0].role == pair[1].role) {
        return Err(Denial::InvalidContract);
    }
    Ok(roles
        .into_iter()
        .map(|role| WorthQueryInstalledGraphObligationContract::OperationGraphRead { role })
        .collect())
}

fn authorization_obligation(
    source: &WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<WorthQueryInstalledGraphObligationContract, Denial> {
    match source.authorization {
        WorthQueryInstalledApplicationOperationAuthorization::Principal => {
            Ok(WorthQueryInstalledGraphObligationContract::PrincipalAuthorization)
        }
        WorthQueryInstalledApplicationOperationAuthorization::Abilities => Ok(
            WorthQueryInstalledGraphObligationContract::AbilityAuthorization {
                requirements: source.ability_requirements.to_vec(),
            },
        ),
        WorthQueryInstalledApplicationOperationAuthorization::Capability => Ok(
            WorthQueryInstalledGraphObligationContract::CapabilityAuthorization {
                requirements: source.capability_requirements.to_vec(),
            },
        ),
    }
}

fn validate_authorization(
    source: &WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<(), Denial> {
    let abilities = source.ability_requirements.len();
    let capabilities = source.capability_requirements.len();
    let valid = match source.authorization {
        WorthQueryInstalledApplicationOperationAuthorization::Principal => {
            abilities == 0 && capabilities == 0
        }
        WorthQueryInstalledApplicationOperationAuthorization::Abilities => {
            abilities > 0 && capabilities == 0
        }
        WorthQueryInstalledApplicationOperationAuthorization::Capability => {
            abilities == 0 && capabilities > 0
        }
    };
    valid.then_some(()).ok_or(Denial::InvalidContract)
}

fn validate_mutation_contracts(
    source: &WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<(), Denial> {
    let touch = matches!(
        source.touches,
        WorthQueryOperationTouchContract::Declared { .. }
    );
    let effect = matches!(
        source.effects,
        WorthQueryOperationEffectContract::Declared { .. }
    );
    let invariant = matches!(
        source.invariants,
        WorthQueryOperationInvariantContract::Declared { .. }
    );
    let execution = matches!(
        source.invariant_execution,
        WorthQueryInvariantExecutionContract::Declared { .. }
    );
    if !(touch == effect && effect == invariant && invariant == execution) {
        return Err(Denial::InvalidContract);
    }
    if !touch {
        return Ok(());
    }
    validate_touch_roles(source)?;
    validate_invariant_slots(source)
}

fn validate_touch_roles(
    source: &WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<(), Denial> {
    let declared_roles = source
        .graph_reads
        .roles()
        .iter()
        .map(|role| role.role.as_str())
        .collect::<BTreeSet<_>>();
    let WorthQueryOperationTouchContract::Declared {
        graph_roles,
        scopes,
    } = source.touches
    else {
        unreachable!("mutation-family validation established declared touches")
    };
    if graph_roles.is_empty()
        || scopes.is_empty()
        || graph_roles
            .iter()
            .any(|role| !declared_roles.contains(role.as_str()))
    {
        return Err(Denial::InvalidContract);
    }
    Ok(())
}

fn validate_invariant_slots(
    source: &WorthQueryApplicationOperationObligationSource<'_>,
) -> Result<(), Denial> {
    let WorthQueryOperationInvariantContract::Declared { invariant_slots } = source.invariants
    else {
        unreachable!("mutation-family validation established declared invariants")
    };
    let declared = invariant_slots
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let executable = source
        .invariant_execution
        .requirements()
        .iter()
        .map(|requirement| requirement.slot())
        .collect::<BTreeSet<_>>();
    if declared.is_empty() || declared != executable {
        return Err(Denial::InvalidContract);
    }
    Ok(())
}

pub(crate) struct WorthQueryApplicationOperationObligationSource<'a> {
    pub(crate) authorization: WorthQueryInstalledApplicationOperationAuthorization,
    pub(crate) ability_requirements: &'a [WorthQueryInstalledAbilityRequirement],
    pub(crate) capability_requirements: &'a [WorthQueryInstalledGraphCapabilityRequirement],
    pub(crate) graph_reads: &'a WorthQueryOperationGraphReadContract,
    pub(crate) touches: &'a WorthQueryOperationTouchContract,
    pub(crate) effects: &'a WorthQueryOperationEffectContract,
    pub(crate) invariants: &'a WorthQueryOperationInvariantContract,
    pub(crate) invariant_execution: &'a WorthQueryInvariantExecutionContract,
    pub(crate) resources: &'a WorthQueryExecutionResourceContract,
}

pub(crate) fn capability_requirement(
    identity: crate::application_capability::WorthQueryInstalledApplicationCapabilityIdentity,
    contract: worth_query_declaration::facade::application_capability::ErasedApplicationCapabilityContract,
) -> WorthQueryInstalledGraphCapabilityRequirement {
    WorthQueryInstalledGraphCapabilityRequirement::new(identity, contract)
}
