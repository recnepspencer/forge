use std::collections::BTreeMap;

use worth_query_declaration::facade::{
    application_schema::ApplicationOperationProgramTarget,
    domain_computation::WorthQueryResourceDimension,
};

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationObservedFact,
    WorthQueryCompleteApplicationReadSet, WorthQueryProjectedApplicationMutation,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationEntityIdentity;

/// Exact `Active -> Revoked` effect minted only from Query-owned transition authority.
pub struct WorthQueryCapabilityRevocationProgram<Schema, Operation, Input, Scope> {
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
    pub fn materialize_capability_revocation_program<Entity>(
        self,
        target: &WorthQueryApplicationEntityIdentity<Schema, Entity>,
    ) -> Result<
        WorthQueryCapabilityRevocationProgram<Schema, Operation, Input, Scope>,
        WorthQueryApplicationAttemptDenial,
    > {
        let binding = self
            .admission
            .capability_revocation_binding()
            .ok_or_else(|| transition_required(self.admission.operation()))?;
        validate_installed_program(
            binding.required_program_target(),
            self.admission.allowed_graph_contract().program(),
        )?;
        validate_target(binding, target, &self.facts, &self.admission)?;
        let emission_retained_bytes_ceiling = self
            .admission
            .allowed_graph_contract()
            .execution_strategy()
            .expect("installed application operation has exactly one execution strategy")
            .envelope()
            .resource_ceiling(WorthQueryResourceDimension::RetainedBytes);
        let effect = WorthQueryApplicationRealizedEffect::UpdateEntity {
            entity: binding.target_entity().to_owned(),
            entity_id: target.entity_id(),
            fields: BTreeMap::from([(binding.status().clone(), binding.revoked().clone())]),
        };
        Ok(WorthQueryCapabilityRevocationProgram {
            program: WorthQueryApplicationEffectProgram {
                read_set: self,
                effects: vec![effect],
                emission_retained_bytes: 0,
                emission_retained_bytes_ceiling,
            },
        })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryCapabilityRevocationProgram<Schema, Operation, Input, Scope>
{
    pub(super) fn into_inner(
        self,
    ) -> WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
        self.program
    }
}

fn validate_installed_program(
    required: &ApplicationOperationProgramTarget,
    installed: &[ApplicationOperationProgramTarget],
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    if installed.len() == 1 && installed.first() == Some(required) {
        Ok(())
    } else {
        Err(program_mismatch("capability revocation operation contract"))
    }
}

fn validate_target<Schema, Entity, Operation, Input, Scope>(
    binding: &crate::domain_computation::authorization::WorthQueryCapabilityRevocationBinding,
    target: &WorthQueryApplicationEntityIdentity<Schema, Entity>,
    facts: &[WorthQueryApplicationObservedFact],
    admission: &crate::domain_computation::authorization::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let exact_identity = target.runtime_authority() == admission.runtime_authority()
        && target.binding_identity() == admission.binding_identity()
        && target.entity_kind() == binding.target_kind()
        && target.entity_name() == binding.target_entity()
        && target.identity_locator() == binding.identity()
        && target.identity_value() == binding.identity_value();
    let active = facts.iter().any(|fact| {
        matches!(
            fact,
            WorthQueryApplicationObservedFact::Field {
                entity_id,
                locator,
                value,
                ..
            } if *entity_id == target.entity_id()
                && locator == binding.status()
                && value == binding.active()
        )
    });
    let resource = facts.iter().any(|fact| {
        matches!(
            fact,
            WorthQueryApplicationObservedFact::Relation {
                relation_kind,
                from,
                to,
                matching_relations,
                ..
            } if *relation_kind == binding.resource_relation()
                && *from == target.entity_id()
                && *to == binding.resource()
                && matching_relations.len() == 1
        )
    });
    if exact_identity && active && resource {
        Ok(())
    } else {
        Err(program_mismatch("capability revocation target"))
    }
}

fn transition_required(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    WorthQueryApplicationAttemptDenial::new(
        WorthQueryApplicationAttemptDenialKind::CapabilityRevocationRequired,
        subject,
    )
}

fn program_mismatch(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    WorthQueryApplicationAttemptDenial::new(
        WorthQueryApplicationAttemptDenialKind::CapabilityRevocationProgramMismatch,
        subject,
    )
}
