use std::collections::BTreeSet;

use worth_query_declaration::facade::{
    application_schema::ApplicationOperationProgramTarget,
    domain_computation::WorthQueryResourceDimension,
};
use worth_relational::facade::{
    identity::PartitionId,
    symbols::ClientKey,
    transactions::{CreatedEntityRef, EntityReference},
};

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::effect_validation::{canonical_key, denial};
use super::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEffectProgram, WorthQueryCompleteApplicationReadSet,
    WorthQueryProjectedApplicationMutation,
};
use crate::domain_computation::authorization::WorthQueryDelegationActivationBinding;

/// Exact child activation produced from Query-owned delegation authority.
pub struct WorthQueryDelegationActivationProgram<Schema, Operation, Input, Scope> {
    program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryCompleteApplicationReadSet<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >
{
    pub fn materialize_capability_delegation_program(
        self,
    ) -> Result<
        WorthQueryDelegationActivationProgram<Schema, Operation, Input, Scope>,
        WorthQueryApplicationAttemptDenial,
    > {
        let binding = self
            .admission
            .delegation_activation_binding()
            .ok_or_else(|| transition_required(self.admission.operation()))?;
        validate_installed_contract(binding, self.admission.allowed_graph_contract().program())?;
        let effects = activation_effects(binding)?;
        let emission_retained_bytes_ceiling = self
            .admission
            .allowed_graph_contract()
            .execution_strategy()
            .expect("installed application operation has exactly one execution strategy")
            .envelope()
            .resource_ceiling(WorthQueryResourceDimension::RetainedBytes);
        let program = WorthQueryApplicationEffectProgram {
            read_set: self,
            effects,
            emission_retained_bytes: 0,
            emission_retained_bytes_ceiling,
        };
        validate_delegation_activation_program(&program)?;
        Ok(WorthQueryDelegationActivationProgram { program })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryDelegationActivationProgram<Schema, Operation, Input, Scope>
{
    pub(super) fn into_inner(
        self,
    ) -> WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
        self.program
    }
}

pub(in crate::domain_computation::primary_graph) fn validate_delegation_activation_program<
    Schema,
    Operation,
    Input,
    Scope,
>(
    program: &WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let binding = program
        .read_set
        .admission
        .delegation_activation_binding()
        .ok_or_else(|| transition_required(program.read_set.admission.operation()))?;
    validate_installed_contract(
        binding,
        program
            .read_set
            .admission
            .allowed_graph_contract()
            .program(),
    )?;
    let expected = activation_effects(binding)?;
    if program.emission_retained_bytes == 0 && effects_are_exact(&program.effects, &expected) {
        Ok(())
    } else {
        Err(program_mismatch(program.read_set.admission.operation()))
    }
}

fn validate_installed_contract(
    binding: &WorthQueryDelegationActivationBinding,
    installed: &[ApplicationOperationProgramTarget],
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let required = binding
        .required_program_targets
        .iter()
        .collect::<BTreeSet<_>>();
    let installed = installed.iter().collect::<BTreeSet<_>>();
    let creates = required
        .iter()
        .filter(|target| matches!(target, ApplicationOperationProgramTarget::Create { .. }))
        .count();
    let writes = required
        .iter()
        .filter(|target| matches!(target, ApplicationOperationProgramTarget::Write { .. }))
        .count();
    let links = required
        .iter()
        .filter(|target| matches!(target, ApplicationOperationProgramTarget::Link { .. }))
        .count();
    let expected_links = 4usize
        .saturating_add(usize::from(binding.related.is_some()))
        .saturating_add(binding.activation_context.len());
    if required.len() == binding.required_program_targets.len()
        && required.is_subset(&installed)
        && creates == 1
        && writes == binding.fields.len()
        && links == expected_links
        && required.len() == creates.saturating_add(writes).saturating_add(links)
    {
        Ok(())
    } else {
        Err(program_mismatch("delegation activation operation contract"))
    }
}

fn activation_effects(
    binding: &WorthQueryDelegationActivationBinding,
) -> Result<Vec<WorthQueryApplicationRealizedEffect>, WorthQueryApplicationAttemptDenial> {
    let key = canonical_key(binding.child_key.clone(), "delegated capability")?;
    let child = created(binding.child_kind, &key);
    let mut effects = Vec::with_capacity(
        5usize
            .saturating_add(usize::from(binding.related.is_some()))
            .saturating_add(binding.activation_context.len()),
    );
    effects.push(WorthQueryApplicationRealizedEffect::CreateEntity {
        kind: binding.child_kind,
        key: key.clone(),
        fields: binding.fields.clone(),
    });
    effects.push(relation(
        binding.parent_relation,
        &key,
        child.clone(),
        EntityReference::Existing(binding.parent),
    ));
    effects.push(relation(
        binding.grantor_relation,
        &key,
        EntityReference::Existing(binding.grantor),
        child.clone(),
    ));
    effects.push(relation(
        binding.grantee_relation,
        &key,
        EntityReference::Existing(binding.grantee),
        child.clone(),
    ));
    effects.push(relation(
        binding.resource_relation,
        &key,
        child.clone(),
        EntityReference::Existing(binding.resource),
    ));
    match (binding.related_relation, binding.related) {
        (Some(kind), Some(related)) => effects.push(relation(
            kind,
            &key,
            child.clone(),
            EntityReference::Existing(related),
        )),
        (None, None) => {}
        _ => return Err(program_mismatch("delegated capability related entity")),
    }
    effects.extend(binding.activation_context.iter().map(|(kind, entity)| {
        relation(
            *kind,
            &key,
            child.clone(),
            EntityReference::Existing(*entity),
        )
    }));
    Ok(effects)
}

fn relation(
    kind: worth_relational::facade::identity::KindId,
    key: &str,
    from: EntityReference,
    to: EntityReference,
) -> WorthQueryApplicationRealizedEffect {
    WorthQueryApplicationRealizedEffect::CreateRelation {
        kind,
        key: key.to_owned(),
        from,
        to,
    }
}

fn created(kind: worth_relational::facade::identity::KindId, key: &str) -> EntityReference {
    EntityReference::Created(CreatedEntityRef {
        partition_id: PartitionId::main(),
        kind_id: kind,
        client_key: ClientKey::raw(key.to_owned()),
    })
}

fn effects_are_exact(
    actual: &[WorthQueryApplicationRealizedEffect],
    expected: &[WorthQueryApplicationRealizedEffect],
) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| effect_is_exact(actual, expected))
}

fn effect_is_exact(
    actual: &WorthQueryApplicationRealizedEffect,
    expected: &WorthQueryApplicationRealizedEffect,
) -> bool {
    match (actual, expected) {
        (
            WorthQueryApplicationRealizedEffect::CreateEntity { kind, key, fields },
            WorthQueryApplicationRealizedEffect::CreateEntity {
                kind: expected_kind,
                key: expected_key,
                fields: expected_fields,
            },
        ) => kind == expected_kind && key == expected_key && fields == expected_fields,
        (
            WorthQueryApplicationRealizedEffect::CreateRelation {
                kind,
                key,
                from,
                to,
            },
            WorthQueryApplicationRealizedEffect::CreateRelation {
                kind: expected_kind,
                key: expected_key,
                from: expected_from,
                to: expected_to,
            },
        ) => {
            kind == expected_kind
                && key == expected_key
                && from == expected_from
                && to == expected_to
        }
        _ => false,
    }
}

fn transition_required(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::DelegationActivationRequired,
        subject,
    )
}

fn program_mismatch(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::DelegationActivationProgramMismatch,
        subject,
    )
}
