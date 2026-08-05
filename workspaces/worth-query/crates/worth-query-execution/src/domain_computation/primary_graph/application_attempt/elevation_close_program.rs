use std::collections::{BTreeMap, BTreeSet};

use worth_query_declaration::facade::domain_computation::WorthQueryResourceDimension;

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::effect_validation::denial;
use super::elevation_lifecycle_effects::lifecycle_effects_are_exact;
use super::elevation_lifecycle_emission::{append_lifecycle_emission, lifecycle_emission_is_exact};
use super::elevation_lifecycle_facts::{
    lifecycle_facts_are_exact, WorthQueryElevationLifecycleFactExpectation,
    WorthQueryExpectedLifecycleRelation,
};
use super::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEffectProgram, WorthQueryCompleteApplicationReadSet,
    WorthQueryProjectedApplicationMutation,
};
use crate::domain_computation::authorization::WorthQueryElevationCloseBinding;

pub struct WorthQueryElevationCloseProgram<Schema, Operation, Input, Scope> {
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
    pub fn materialize_elevation_close_program(
        self,
    ) -> Result<
        WorthQueryElevationCloseProgram<Schema, Operation, Input, Scope>,
        WorthQueryApplicationAttemptDenial,
    > {
        let binding = self
            .admission
            .elevation_close_binding()
            .ok_or_else(|| transition_required(self.admission.operation()))?;
        validate_installed_contract(binding, &self)?;
        validate_lifecycle_facts(binding, &self.facts)?;
        let mut effects = close_effects(binding);
        let emission_retained_bytes_ceiling = self
            .admission
            .allowed_graph_contract()
            .execution_strategy()
            .expect("installed application operation has exactly one execution strategy")
            .envelope()
            .resource_ceiling(WorthQueryResourceDimension::RetainedBytes);
        let emission_retained_bytes = append_lifecycle_emission(
            &mut effects,
            binding.draft.lifecycle_effect.as_ref(),
            emission_retained_bytes_ceiling,
            self.admission.operation(),
        )?;
        let program = WorthQueryApplicationEffectProgram {
            read_set: self,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        };
        validate_elevation_close_program(&program)?;
        Ok(WorthQueryElevationCloseProgram { program })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryElevationCloseProgram<Schema, Operation, Input, Scope>
{
    pub(super) fn into_inner(
        self,
    ) -> WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
        self.program
    }
}

pub(in crate::domain_computation::primary_graph) fn validate_elevation_close_program<
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
        .elevation_close_binding()
        .ok_or_else(|| transition_required(program.read_set.admission.operation()))?;
    validate_installed_contract(binding, &program.read_set)?;
    validate_lifecycle_facts(binding, &program.read_set.facts)?;
    let expected = close_effects(binding);
    if expected.len() == 1
        && program
            .effects
            .get(..1)
            .is_some_and(|actual| lifecycle_effects_are_exact(actual, &expected))
        && lifecycle_emission_is_exact(
            &program.effects,
            1,
            binding.draft.lifecycle_effect.as_ref(),
            program.emission_retained_bytes,
        )
    {
        Ok(())
    } else {
        Err(program_mismatch(program.read_set.admission.operation()))
    }
}

fn validate_installed_contract<Schema, Operation, Input, Scope>(
    binding: &WorthQueryElevationCloseBinding,
    read_set: &WorthQueryCompleteApplicationReadSet<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >,
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let expected_reads = binding
        .draft
        .required_decision_reads
        .iter()
        .collect::<BTreeSet<_>>();
    let installed_reads = read_set
        .admission
        .allowed_graph_contract()
        .decision_reads()
        .iter()
        .collect::<BTreeSet<_>>();
    let expected_targets = binding
        .draft
        .required_program_targets
        .iter()
        .collect::<BTreeSet<_>>();
    let installed_targets = read_set
        .admission
        .allowed_graph_contract()
        .program()
        .iter()
        .collect::<BTreeSet<_>>();
    if expected_reads.len() == binding.draft.required_decision_reads.len()
        && expected_reads == installed_reads
        && expected_targets.len() == binding.draft.required_program_targets.len()
        && expected_targets == installed_targets
    {
        Ok(())
    } else {
        Err(program_mismatch("elevation close operation contract"))
    }
}

fn validate_lifecycle_facts(
    binding: &WorthQueryElevationCloseBinding,
    facts: &[super::WorthQueryApplicationObservedFact],
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let requested = binding.approved.request_binding();
    if lifecycle_facts_are_exact(
        facts,
        WorthQueryElevationLifecycleFactExpectation {
            elevation: binding.draft.elevation,
            review: binding.draft.review,
            resource: requested.resource(),
            requester: requested.requester(),
            approver: WorthQueryExpectedLifecycleRelation::Present(binding.approved.approver()),
            grant: requested.grant(),
            reviewer: WorthQueryExpectedLifecycleRelation::Absent,
            elevation_identity: (
                &requested.elevation_identity_field,
                &requested.elevation_identity,
            ),
            reason: (&requested.reason_field, &requested.reason),
            status: (&requested.status_field, &binding.draft.approved_status),
            not_before: (&requested.not_before_field, &requested.issued_at),
            not_after: (&requested.not_after_field, &requested.expires_at),
            review_identity: (&requested.review_identity_field, &requested.review_identity),
            review_type: (&requested.review_type_field, &requested.review_type),
            review_status: (
                &requested.review_status_field,
                &requested.review_required_status,
            ),
            requester_relation: requested.requester_relation,
            approver_relation: binding.draft.approver_relation,
            grant_relation: requested.grant_relation,
            resource_relation: requested.resource_relation,
            review_relation: requested.review_relation,
            review_scope_relation: requested.review_scope_relation,
            reviewer_relation: binding.draft.reviewer_relation,
        },
    ) {
        Ok(())
    } else {
        Err(program_mismatch("approved elevation lifecycle facts"))
    }
}

fn close_effects(
    binding: &WorthQueryElevationCloseBinding,
) -> Vec<WorthQueryApplicationRealizedEffect> {
    vec![WorthQueryApplicationRealizedEffect::UpdateEntity {
        entity: binding.draft.elevation_entity.clone(),
        entity_id: binding.draft.elevation,
        fields: BTreeMap::from([(
            binding.draft.status_field.clone(),
            binding.draft.closed_status.clone(),
        )]),
    }]
}

fn transition_required(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ElevationTransitionRequired,
        subject,
    )
}

fn program_mismatch(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ElevationCloseProgramMismatch,
        subject,
    )
}
