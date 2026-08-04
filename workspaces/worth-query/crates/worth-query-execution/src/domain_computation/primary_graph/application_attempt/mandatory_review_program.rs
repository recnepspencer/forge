use std::collections::{BTreeMap, BTreeSet};

use worth_query_declaration::facade::domain_computation::WorthQueryResourceDimension;
use worth_relational::facade::transactions::EntityReference;

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::effect_validation::{canonical_key, denial};
use super::elevation_lifecycle_effects::lifecycle_effects_are_exact;
use super::elevation_lifecycle_facts::{
    lifecycle_facts_are_exact, WorthQueryElevationLifecycleFactExpectation,
    WorthQueryExpectedLifecycleRelation,
};
use super::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEffectProgram, WorthQueryCompleteApplicationReadSet,
    WorthQueryProjectedApplicationMutation,
};
use crate::domain_computation::authorization::WorthQueryMandatoryReviewBinding;

pub struct WorthQueryMandatoryReviewProgram<Schema, Operation, Input, Scope> {
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
    pub fn materialize_mandatory_review_program(
        self,
    ) -> Result<
        WorthQueryMandatoryReviewProgram<Schema, Operation, Input, Scope>,
        WorthQueryApplicationAttemptDenial,
    > {
        let binding = self
            .admission
            .mandatory_review_binding()
            .ok_or_else(|| transition_required(self.admission.operation()))?;
        validate_installed_contract(binding, &self)?;
        validate_lifecycle_facts(binding, &self.facts)?;
        let effects = review_effects(binding)?;
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
        validate_mandatory_review_program(&program)?;
        Ok(WorthQueryMandatoryReviewProgram { program })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryMandatoryReviewProgram<Schema, Operation, Input, Scope>
{
    pub(super) fn into_inner(
        self,
    ) -> WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
        self.program
    }
}

pub(in crate::domain_computation::primary_graph) fn validate_mandatory_review_program<
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
        .mandatory_review_binding()
        .ok_or_else(|| transition_required(program.read_set.admission.operation()))?;
    validate_installed_contract(binding, &program.read_set)?;
    validate_lifecycle_facts(binding, &program.read_set.facts)?;
    let expected = review_effects(binding)?;
    if lifecycle_effects_are_exact(&program.effects, &expected)
        && program.emission_retained_bytes == 0
        && expected.len() == 2
    {
        Ok(())
    } else {
        Err(program_mismatch(program.read_set.admission.operation()))
    }
}

fn validate_installed_contract<Schema, Operation, Input, Scope>(
    binding: &WorthQueryMandatoryReviewBinding,
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
        Err(program_mismatch("mandatory review operation contract"))
    }
}

fn validate_lifecycle_facts(
    binding: &WorthQueryMandatoryReviewBinding,
    facts: &[super::WorthQueryApplicationObservedFact],
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let approved = binding.mandatory.approved();
    let requested = approved.request_binding();
    if lifecycle_facts_are_exact(
        facts,
        WorthQueryElevationLifecycleFactExpectation {
            elevation: binding.draft.elevation,
            review: binding.draft.review,
            requester: requested.requester(),
            approver: WorthQueryExpectedLifecycleRelation::Present(approved.approver()),
            grant: requested.grant(),
            reviewer: WorthQueryExpectedLifecycleRelation::Absent,
            elevation_identity: (
                &requested.elevation_identity_field,
                &requested.elevation_identity,
            ),
            reason: (&requested.reason_field, &requested.reason),
            status: (&requested.status_field, &binding.draft.terminal_status),
            not_before: (&requested.not_before_field, &requested.issued_at),
            not_after: (&requested.not_after_field, &requested.expires_at),
            review_identity: (&requested.review_identity_field, &requested.review_identity),
            review_status: (
                &requested.review_status_field,
                &requested.review_required_status,
            ),
            requester_relation: requested.requester_relation,
            approver_relation: binding.draft.approver_relation,
            grant_relation: requested.grant_relation,
            review_relation: requested.review_relation,
            reviewer_relation: binding.draft.reviewer_relation,
        },
    ) {
        Ok(())
    } else {
        Err(program_mismatch("closed elevation review facts"))
    }
}

fn review_effects(
    binding: &WorthQueryMandatoryReviewBinding,
) -> Result<Vec<WorthQueryApplicationRealizedEffect>, WorthQueryApplicationAttemptDenial> {
    let key = canonical_key(
        binding
            .mandatory
            .approved()
            .request_binding()
            .review_key
            .clone(),
        "mandatory review",
    )?;
    Ok(vec![
        WorthQueryApplicationRealizedEffect::UpdateEntity {
            entity: binding.draft.review_entity.clone(),
            entity_id: binding.draft.review,
            fields: BTreeMap::from([(
                binding.draft.review_status_field.clone(),
                binding.draft.completed_status.clone(),
            )]),
        },
        WorthQueryApplicationRealizedEffect::CreateRelation {
            kind: binding.draft.reviewer_relation,
            key,
            from: EntityReference::Existing(binding.draft.reviewer),
            to: EntityReference::Existing(binding.draft.review),
        },
    ])
}

fn transition_required(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ElevationTransitionRequired,
        subject,
    )
}

fn program_mismatch(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::MandatoryReviewProgramMismatch,
        subject,
    )
}
