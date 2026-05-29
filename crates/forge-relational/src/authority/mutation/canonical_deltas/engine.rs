use smallvec::SmallVec;

use crate::identity::data::EntityId;
use crate::publication::patch::data::{CanonicalAspectSet, RecordStructuralChange};
use crate::schema::data::LoweredAspectPlan;
use crate::transactions::data::RecordRef;

use super::data::{
    BindingEvaluationContext, CanonicalDeltaError, CanonicalRecordAspectDelta,
    EvaluatedAspectBinding,
};
use super::materialized_state::evaluate_materialized_binding;
use super::patch_authority::{
    authoritative_patch_binding_evidence, evaluate_authoritative_patch_delta,
};
use crate::authority::mutation::outcomes::RecordMutation;
use crate::authority::mutation::MutationWorkspace;

#[derive(Debug, Clone, Copy)]
struct RelationState<'a> {
    source: Option<EntityId>,
    target: Option<EntityId>,
    authoritative_state: Option<&'a forge_foundational::facade::AuthoritativeRecordAspectState>,
}

#[derive(Debug, Clone, Copy)]
struct EntityAuthoritativeState<'a> {
    authoritative_state: Option<&'a forge_foundational::facade::AuthoritativeRecordAspectState>,
}

pub(crate) fn canonical_delta_for_mutation(
    mutation: &RecordMutation,
    workspace: &MutationWorkspace<'_>,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    match mutation {
        RecordMutation::EntityCreated {
            entity_id,
            kind_id,
            authoritative_patch,
            ..
        } => evaluate_entity_lifecycle_delta(
            workspace,
            *entity_id,
            *kind_id,
            authoritative_patch.as_ref(),
            RecordStructuralChange::Created,
        ),
        RecordMutation::EntityUpdated {
            entity_id,
            kind_id,
            old_authoritative_aspect_state,
            new_authoritative_aspect_state,
            authoritative_patch,
            ..
        } => evaluate_entity_update_delta(
            workspace,
            *entity_id,
            *kind_id,
            EntityAuthoritativeState {
                authoritative_state: old_authoritative_aspect_state.as_ref(),
            },
            EntityAuthoritativeState {
                authoritative_state: new_authoritative_aspect_state.as_ref(),
            },
            authoritative_patch.as_ref(),
        ),
        RecordMutation::EntityDeleted {
            entity_id,
            kind_id,
            authoritative_patch,
            ..
        } => evaluate_entity_lifecycle_delta(
            workspace,
            *entity_id,
            *kind_id,
            authoritative_patch.as_ref(),
            RecordStructuralChange::Deleted,
        ),
        RecordMutation::RelationCreated {
            relation_id,
            kind_id,
            source,
            target,
            authoritative_patch,
            ..
        } => evaluate_relation_lifecycle_delta(
            workspace,
            *relation_id,
            *kind_id,
            *source,
            *target,
            authoritative_patch.as_ref(),
        ),
        RecordMutation::RelationUpdated {
            relation_id,
            kind_id,
            old_source,
            old_target,
            new_source,
            new_target,
            old_authoritative_aspect_state,
            new_authoritative_aspect_state,
            ..
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: Some(*old_source),
                target: Some(*old_target),
                authoritative_state: old_authoritative_aspect_state.as_ref(),
            },
            RelationState {
                source: Some(*new_source),
                target: Some(*new_target),
                authoritative_state: new_authoritative_aspect_state.as_ref(),
            },
            RecordStructuralChange::Updated,
        ),
        RecordMutation::RelationDeleted {
            relation_id,
            kind_id,
            source,
            target,
            authoritative_aspect_state,
            ..
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                authoritative_state: authoritative_aspect_state.as_ref(),
            },
            RelationState {
                source: None,
                target: None,
                authoritative_state: None,
            },
            RecordStructuralChange::Deleted,
        ),
        RecordMutation::RelationRetainedForAudit {
            relation_id,
            kind_id,
            source,
            target,
            authoritative_aspect_state,
            ..
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                authoritative_state: authoritative_aspect_state.as_ref(),
            },
            RelationState {
                source: Some(*source),
                target: Some(*target),
                authoritative_state: authoritative_aspect_state.as_ref(),
            },
            RecordStructuralChange::RetainedForAudit,
        ),
    }
}

fn evaluate_relation_lifecycle_delta(
    workspace: &MutationWorkspace<'_>,
    relation_id: crate::identity::data::RelationId,
    kind_id: crate::identity::data::KindId,
    source: EntityId,
    target: EntityId,
    authoritative_patch: Option<&forge_foundational::facade::AuthoritativeRecordAspectPatch>,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    match authoritative_patch {
        Some(authoritative_patch) => {
            let plan = workspace
                .relation_aspect_plan(kind_id)
                .ok_or(CanonicalDeltaError::MissingRelationAspectPlan { kind_id })?;
            Ok(evaluate_authoritative_patch_delta(
                RecordRef::Relation(relation_id),
                kind_id,
                plan,
                RecordStructuralChange::Created,
                authoritative_patch,
            ))
        }
        None => evaluate_relation_delta(
            workspace,
            relation_id,
            kind_id,
            RelationState {
                source: None,
                target: None,
                authoritative_state: None,
            },
            RelationState {
                source: Some(source),
                target: Some(target),
                authoritative_state: None,
            },
            RecordStructuralChange::Created,
        ),
    }
}

fn evaluate_entity_lifecycle_delta(
    workspace: &MutationWorkspace<'_>,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    authoritative_patch: Option<&forge_foundational::facade::AuthoritativeRecordAspectPatch>,
    structural_change: RecordStructuralChange,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    match authoritative_patch {
        Some(authoritative_patch) => {
            let plan = workspace
                .entity_aspect_plan(kind_id)
                .ok_or(CanonicalDeltaError::MissingEntityAspectPlan { kind_id })?;
            Ok(evaluate_authoritative_patch_delta(
                RecordRef::Entity(entity_id),
                kind_id,
                plan,
                structural_change,
                authoritative_patch,
            ))
        }
        None => evaluate_entity_delta(
            workspace,
            entity_id,
            kind_id,
            EntityAuthoritativeState {
                authoritative_state: None,
            },
            EntityAuthoritativeState {
                authoritative_state: None,
            },
            structural_change,
        ),
    }
}

fn evaluate_entity_update_delta(
    workspace: &MutationWorkspace<'_>,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    old_state: EntityAuthoritativeState<'_>,
    new_state: EntityAuthoritativeState<'_>,
    authoritative_patch: Option<&forge_foundational::facade::AuthoritativeRecordAspectPatch>,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    match authoritative_patch {
        Some(authoritative_patch) => {
            let plan = workspace
                .entity_aspect_plan(kind_id)
                .ok_or(CanonicalDeltaError::MissingEntityAspectPlan { kind_id })?;
            let mut delta = evaluate_entity_delta(
                workspace,
                entity_id,
                kind_id,
                old_state,
                new_state,
                RecordStructuralChange::Updated,
            )?;
            for binding in &mut delta.evaluated_bindings {
                let Some(lowered_binding) = plan
                    .executable_bindings
                    .iter()
                    .find(|candidate| candidate.aspect_key == binding.aspect_key)
                else {
                    continue;
                };
                if let Some(evidence) = authoritative_patch_binding_evidence(
                    lowered_binding,
                    RecordStructuralChange::Updated,
                    authoritative_patch,
                ) {
                    binding.evidence = evidence;
                    binding.changed = true;
                }
            }
            delta.changed_aspects = CanonicalAspectSet::new(
                delta
                    .evaluated_bindings
                    .iter()
                    .filter(|binding| binding.changed)
                    .map(|binding| binding.aspect_key.clone()),
            );
            Ok(delta)
        }
        None => evaluate_entity_delta(
            workspace,
            entity_id,
            kind_id,
            old_state,
            new_state,
            RecordStructuralChange::Updated,
        ),
    }
}

fn evaluate_entity_delta(
    workspace: &MutationWorkspace<'_>,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    old_state: EntityAuthoritativeState<'_>,
    new_state: EntityAuthoritativeState<'_>,
    structural_change: RecordStructuralChange,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    let plan = workspace
        .entity_aspect_plan(kind_id)
        .ok_or(CanonicalDeltaError::MissingEntityAspectPlan { kind_id })?;
    let evaluated_bindings = evaluate_bindings(
        plan,
        BindingEvaluationContext::Entity {
            structural_change,
            old_authoritative_state: old_state.authoritative_state,
            new_authoritative_state: new_state.authoritative_state,
        },
    )?;
    Ok(assemble_delta(
        RecordRef::Entity(entity_id),
        kind_id,
        plan,
        structural_change,
        evaluated_bindings,
    ))
}

fn evaluate_relation_delta(
    workspace: &MutationWorkspace<'_>,
    relation_id: crate::identity::data::RelationId,
    kind_id: crate::identity::data::KindId,
    old_state: RelationState<'_>,
    new_state: RelationState<'_>,
    structural_change: RecordStructuralChange,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    let plan = workspace
        .relation_aspect_plan(kind_id)
        .ok_or(CanonicalDeltaError::MissingRelationAspectPlan { kind_id })?;
    let evaluated_bindings = evaluate_bindings(
        plan,
        BindingEvaluationContext::Relation {
            structural_change,
            old_authoritative_state: old_state.authoritative_state,
            new_authoritative_state: new_state.authoritative_state,
            old_source: old_state.source,
            new_source: new_state.source,
            old_target: old_state.target,
            new_target: new_state.target,
        },
    )?;
    Ok(assemble_delta(
        RecordRef::Relation(relation_id),
        kind_id,
        plan,
        structural_change,
        evaluated_bindings,
    ))
}

fn assemble_delta(
    target: RecordRef,
    kind_id: crate::identity::data::KindId,
    plan: &LoweredAspectPlan,
    structural_change: RecordStructuralChange,
    evaluated_bindings: SmallVec<[EvaluatedAspectBinding; 4]>,
) -> CanonicalRecordAspectDelta {
    let changed_aspects = CanonicalAspectSet::new(
        evaluated_bindings
            .iter()
            .filter(|binding| binding.changed)
            .map(|binding| binding.aspect_key.clone()),
    );
    let contains_opaque_aspect = evaluated_bindings.iter().any(|binding| {
        matches!(
            binding.aspect_shape,
            forge_foundational::AspectShape::Opaque(_)
        )
    });
    CanonicalRecordAspectDelta {
        target,
        kind_id,
        plan_revision: plan.plan_revision,
        structural_change,
        changed_aspects,
        evaluated_bindings,
        contains_opaque_aspect,
    }
}

fn evaluate_bindings(
    plan: &LoweredAspectPlan,
    context: BindingEvaluationContext<'_>,
) -> Result<SmallVec<[EvaluatedAspectBinding; 4]>, CanonicalDeltaError> {
    let mut evaluated = SmallVec::new();
    for binding in &plan.executable_bindings {
        let (evidence, changed) = evaluate_materialized_binding(binding, context)?;
        evaluated.push(EvaluatedAspectBinding {
            aspect_key: binding.aspect_key.clone(),
            contract: binding.contract.clone(),
            changed,
            aspect_shape: binding.aspect_shape(),
            evidence,
        });
    }
    Ok(evaluated)
}
