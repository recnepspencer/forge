use std::collections::{BTreeMap, BTreeSet};

use worth_query_declaration::facade::domain_computation::WorthQueryResourceDimension;
use worth_relational::facade::transactions::EntityReference;

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::effect_validation::{canonical_key, denial};
use super::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationObservedFact,
    WorthQueryCompleteApplicationReadSet, WorthQueryProjectedApplicationMutation,
};
use crate::domain_computation::authorization::WorthQueryElevationApprovalBinding;

/// Exact approval mutation derived from Query-owned lifecycle authority.
///
/// Callers cannot construct this wrapper or choose its status, approver, or
/// relation effects. Query derives them after re-observing the requested state.
pub struct WorthQueryElevationApprovalProgram<Schema, Operation, Input, Scope> {
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
    pub fn materialize_elevation_approval_program(
        self,
    ) -> Result<
        WorthQueryElevationApprovalProgram<Schema, Operation, Input, Scope>,
        WorthQueryApplicationAttemptDenial,
    > {
        let binding = self
            .admission
            .elevation_approval_binding()
            .ok_or_else(|| transition_required(self.admission.operation()))?;
        validate_installed_contract(binding, &self)?;
        validate_lifecycle_facts(binding, &self.facts)?;
        let effects = approval_effects(binding)?;
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
        validate_elevation_approval_program(&program)?;
        Ok(WorthQueryElevationApprovalProgram { program })
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryElevationApprovalProgram<Schema, Operation, Input, Scope>
{
    pub(super) fn into_inner(
        self,
    ) -> WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
        self.program
    }
}

pub(in crate::domain_computation::primary_graph) fn validate_elevation_approval_program<
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
        .elevation_approval_binding()
        .ok_or_else(|| transition_required(program.read_set.admission.operation()))?;
    validate_installed_contract(binding, &program.read_set)?;
    validate_lifecycle_facts(binding, &program.read_set.facts)?;
    let expected = approval_effects(binding)?;
    if same_effects(&program.effects, &expected)
        && program.emission_retained_bytes == 0
        && expected.len() == 2
    {
        Ok(())
    } else {
        Err(program_mismatch(program.read_set.admission.operation()))
    }
}

fn validate_installed_contract<Schema, Operation, Input, Scope>(
    binding: &WorthQueryElevationApprovalBinding,
    read_set: &WorthQueryCompleteApplicationReadSet<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >,
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let expected_reads = binding
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
        .required_program_targets
        .iter()
        .collect::<BTreeSet<_>>();
    let installed_targets = read_set
        .admission
        .allowed_graph_contract()
        .program()
        .iter()
        .collect::<BTreeSet<_>>();
    if expected_reads.len() == binding.required_decision_reads.len()
        && expected_reads == installed_reads
        && expected_targets.len() == binding.required_program_targets.len()
        && expected_targets == installed_targets
    {
        Ok(())
    } else {
        Err(program_mismatch("elevation approval operation contract"))
    }
}

fn validate_lifecycle_facts(
    binding: &WorthQueryElevationApprovalBinding,
    facts: &[WorthQueryApplicationObservedFact],
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    let requested = &binding.requested;
    let fields = [
        (
            binding.elevation,
            &requested.elevation_identity_field,
            &requested.elevation_identity,
        ),
        (
            binding.elevation,
            &requested.reason_field,
            &requested.reason,
        ),
        (
            binding.elevation,
            &requested.status_field,
            &requested.requested_status,
        ),
        (
            binding.elevation,
            &requested.not_before_field,
            &requested.issued_at,
        ),
        (
            binding.elevation,
            &requested.not_after_field,
            &requested.expires_at,
        ),
        (
            binding.review,
            &requested.review_identity_field,
            &requested.review_identity,
        ),
        (
            binding.review,
            &requested.review_status_field,
            &requested.review_required_status,
        ),
    ];
    let fields_match = fields
        .into_iter()
        .all(|(entity, locator, value)| exact_field(facts, entity, locator, value));
    let relations_match = exact_relation(
        facts,
        requested.requester_relation,
        requested.requester(),
        binding.elevation,
        1,
    ) && exact_relation(
        facts,
        binding.approver_relation,
        binding.approver,
        binding.elevation,
        0,
    ) && exact_relation(
        facts,
        requested.grant_relation,
        binding.elevation,
        requested.grant(),
        1,
    ) && exact_relation(
        facts,
        requested.review_relation,
        binding.elevation,
        binding.review,
        1,
    );
    if facts.len() == 11 && fields_match && relations_match {
        Ok(())
    } else {
        Err(program_mismatch("requested elevation lifecycle facts"))
    }
}

fn exact_field(
    facts: &[WorthQueryApplicationObservedFact],
    entity: worth_relational::facade::identity::EntityId,
    locator: &worth_foundational::facade::AspectFieldLocator,
    value: &worth_foundational::facade::AspectValue,
) -> bool {
    facts
        .iter()
        .filter(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Field {
                    entity_id,
                    locator: observed_locator,
                    value: observed_value,
                    ..
                } if *entity_id == entity && observed_locator == locator && observed_value == value
            )
        })
        .count()
        == 1
}

fn exact_relation(
    facts: &[WorthQueryApplicationObservedFact],
    kind: worth_relational::facade::identity::KindId,
    from: worth_relational::facade::identity::EntityId,
    to: worth_relational::facade::identity::EntityId,
    count: usize,
) -> bool {
    facts
        .iter()
        .filter(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Relation {
                    relation_kind,
                    from: observed_from,
                    to: observed_to,
                    matching_relations,
                    ..
                } if *relation_kind == kind
                    && *observed_from == from
                    && *observed_to == to
                    && matching_relations.len() == count
            )
        })
        .count()
        == 1
}

fn approval_effects(
    binding: &WorthQueryElevationApprovalBinding,
) -> Result<Vec<WorthQueryApplicationRealizedEffect>, WorthQueryApplicationAttemptDenial> {
    let fields = BTreeMap::from([(
        binding.status_field.clone(),
        binding.approved_status.clone(),
    )]);
    let key = canonical_key(
        binding.requested.elevation_key.clone(),
        "elevation approval",
    )?;
    Ok(vec![
        WorthQueryApplicationRealizedEffect::UpdateEntity {
            entity: binding.elevation_entity.clone(),
            entity_id: binding.elevation,
            fields,
        },
        WorthQueryApplicationRealizedEffect::CreateRelation {
            kind: binding.approver_relation,
            key,
            from: EntityReference::Existing(binding.approver),
            to: EntityReference::Existing(binding.elevation),
        },
    ])
}

fn same_effects(
    actual: &[WorthQueryApplicationRealizedEffect],
    expected: &[WorthQueryApplicationRealizedEffect],
) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| match (actual, expected) {
                (
                    WorthQueryApplicationRealizedEffect::UpdateEntity {
                        entity,
                        entity_id,
                        fields,
                    },
                    WorthQueryApplicationRealizedEffect::UpdateEntity {
                        entity: expected_entity,
                        entity_id: expected_id,
                        fields: expected_fields,
                    },
                ) => {
                    entity == expected_entity
                        && entity_id == expected_id
                        && fields == expected_fields
                }
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
            })
}

fn transition_required(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ElevationTransitionRequired,
        subject,
    )
}

fn program_mismatch(subject: impl Into<String>) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ElevationApprovalProgramMismatch,
        subject,
    )
}
