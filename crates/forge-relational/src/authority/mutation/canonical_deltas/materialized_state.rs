use crate::identity::data::EntityId;
use crate::publication::patch::data::AspectKey;
use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};
use forge_foundational::facade::{
    AuthoritativeRecordAspectState, ContractValidatedAspectValueView, StructAspectValue,
};

use super::data::{
    BindingEvaluationContext, CanonicalAspectDeltaEvidence, CanonicalDeltaError,
    LifecycleTransitionClass,
};
use super::lifecycle_transition_evidence::lifecycle_transition;
use crate::transactions::data::AspectFieldPatchTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MaterializedAspectState {
    ScalarValue(Option<forge_foundational::facade::AspectValue>),
    StructValue(Option<StructAspectValue>),
    EndpointIdentity(Option<EntityId>),
    LifecycleTransition(LifecycleTransitionClass),
}

pub(super) fn evaluate_authoritative_binding_delta(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
) -> Result<(CanonicalAspectDeltaEvidence, bool), CanonicalDeltaError> {
    let old_state = materialize_old_binding_state(binding, context)?;
    let new_state = materialize_new_binding_state(binding, context)?;
    binding_evidence_from_states(&binding.aspect_key, old_state, new_state)
}

fn materialize_old_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    materialize_binding_state(binding, context, MaterializationSide::Old)
}

fn materialize_new_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    materialize_binding_state(binding, context, MaterializationSide::New)
}

#[derive(Debug, Clone, Copy)]
enum MaterializationSide {
    Old,
    New,
}

fn materialize_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { field }
        | LoweredExecutableAspectBindingKind::EntityFieldStruct { field } => {
            materialize_entity_field_binding_state(binding, context, side, field)
        }
        LoweredExecutableAspectBindingKind::RelationFieldScalar { .. } => {
            materialize_relation_scalar_binding_state(binding, context, side)
        }
        LoweredExecutableAspectBindingKind::RelationFieldStruct { .. } => {
            materialize_relation_struct_binding_state(binding, context, side)
        }
        LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity => {
            materialize_relation_source_endpoint_binding_state(binding, context, side)
        }
        LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity => {
            materialize_relation_target_endpoint_binding_state(binding, context, side)
        }
        LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => {
            Ok(MaterializedAspectState::LifecycleTransition(
                lifecycle_transition(context.structural_change()),
            ))
        }
    }
}

fn materialize_entity_field_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
    field: &forge_foundational::facade::FieldKey,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    if !matches!(context, BindingEvaluationContext::Entity { .. }) {
        return Err(invalid_binding_context(
            &binding.aspect_key,
            "entity field aspect cannot be evaluated against relation context",
        ));
    }
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { .. } => Ok(
            MaterializedAspectState::ScalarValue(materialize_authoritative_scalar_state(
                &binding.aspect_key,
                authoritative_state_for_side(context, side),
            )?),
        ),
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. } => Ok(
            MaterializedAspectState::StructValue(materialize_authoritative_struct_state(
                &binding.aspect_key,
                authoritative_state_for_side(context, side),
            )?),
        ),
        _ => Err(entity_field_binding_requires_authoritative_patch(
            &binding.aspect_key,
            field,
        )),
    }
}

fn materialize_relation_scalar_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    if !matches!(context, BindingEvaluationContext::Relation { .. }) {
        return Err(invalid_binding_context(
            &binding.aspect_key,
            "relation field aspect cannot be evaluated against entity context",
        ));
    }
    Ok(MaterializedAspectState::ScalarValue(
        materialize_authoritative_scalar_state(
            &binding.aspect_key,
            authoritative_state_for_side(context, side),
        )?,
    ))
}

fn materialize_relation_struct_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    if !matches!(context, BindingEvaluationContext::Relation { .. }) {
        return Err(invalid_binding_context(
            &binding.aspect_key,
            "relation struct field aspect cannot be evaluated against entity context",
        ));
    }
    Ok(MaterializedAspectState::StructValue(
        materialize_authoritative_struct_state(
            &binding.aspect_key,
            authoritative_state_for_side(context, side),
        )?,
    ))
}

fn materialize_relation_source_endpoint_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    let Some((old_source, new_source, _, _)) = context.relation_endpoints() else {
        return Err(invalid_binding_context(
            &binding.aspect_key,
            "relation source endpoint aspect cannot be evaluated against entity context",
        ));
    };
    Ok(MaterializedAspectState::EndpointIdentity(match side {
        MaterializationSide::Old => old_source,
        MaterializationSide::New => new_source,
    }))
}

fn materialize_relation_target_endpoint_binding_state(
    binding: &LoweredAspectBinding,
    context: BindingEvaluationContext<'_>,
    side: MaterializationSide,
) -> Result<MaterializedAspectState, CanonicalDeltaError> {
    let Some((_, _, old_target, new_target)) = context.relation_endpoints() else {
        return Err(invalid_binding_context(
            &binding.aspect_key,
            "relation target endpoint aspect cannot be evaluated against entity context",
        ));
    };
    Ok(MaterializedAspectState::EndpointIdentity(match side {
        MaterializationSide::Old => old_target,
        MaterializationSide::New => new_target,
    }))
}

fn materialize_authoritative_scalar_state(
    aspect_key: &AspectKey,
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
) -> Result<Option<forge_foundational::facade::AspectValue>, CanonicalDeltaError> {
    let Some(entry) = authoritative_state.and_then(|state| state.get(aspect_key)) else {
        return Ok(None);
    };
    match entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Ok(Some(value.clone())),
        ContractValidatedAspectValueView::Struct(_) => {
            Err(CanonicalDeltaError::AspectValueMaterialization {
                aspect_key: aspect_key.clone(),
                detail: format!(
                    "authoritative state for relation aspect {:?} is struct-valued but scalar evidence was requested",
                    aspect_key
                ),
            })
        }
    }
}

fn materialize_authoritative_struct_state(
    aspect_key: &AspectKey,
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
) -> Result<Option<StructAspectValue>, CanonicalDeltaError> {
    let Some(entry) = authoritative_state.and_then(|state| state.get(aspect_key)) else {
        return Ok(None);
    };
    match entry.view() {
        ContractValidatedAspectValueView::Struct(value) => Ok(Some(value.clone())),
        ContractValidatedAspectValueView::Scalar(_) => {
            Err(CanonicalDeltaError::AspectValueMaterialization {
                aspect_key: aspect_key.clone(),
                detail: format!(
                    "authoritative state for relation aspect {:?} is scalar-valued but struct evidence was requested",
                    aspect_key
                ),
            })
        }
    }
}

fn binding_evidence_from_states(
    aspect_key: &AspectKey,
    old_state: MaterializedAspectState,
    new_state: MaterializedAspectState,
) -> Result<(CanonicalAspectDeltaEvidence, bool), CanonicalDeltaError> {
    let locator = authoritative_value_locator(aspect_key);
    match (old_state, new_state) {
        (
            MaterializedAspectState::ScalarValue(old_value),
            MaterializedAspectState::ScalarValue(new_value),
        ) => {
            let changed = (old_value.is_some() || new_value.is_some()) && old_value != new_value;
            Ok((
                CanonicalAspectDeltaEvidence::ScalarAspectValueTransition {
                    locator,
                    old_present: old_value.is_some(),
                    new_present: new_value.is_some(),
                    old_value: if changed { old_value } else { None },
                    new_value: if changed { new_value } else { None },
                },
                changed,
            ))
        }
        (
            MaterializedAspectState::StructValue(old_value),
            MaterializedAspectState::StructValue(new_value),
        ) => {
            let changed = (old_value.is_some() || new_value.is_some()) && old_value != new_value;
            Ok((
                CanonicalAspectDeltaEvidence::StructAspectValueTransition {
                    locator,
                    old_present: old_value.is_some(),
                    new_present: new_value.is_some(),
                    old_value: if changed { old_value } else { None },
                    new_value: if changed { new_value } else { None },
                },
                changed,
            ))
        }
        (
            MaterializedAspectState::EndpointIdentity(old_endpoint),
            MaterializedAspectState::EndpointIdentity(new_endpoint),
        ) => {
            let changed = old_endpoint != new_endpoint;
            Ok((
                CanonicalAspectDeltaEvidence::EndpointIdentity {
                    locator,
                    old: old_endpoint,
                    new: new_endpoint,
                },
                changed,
            ))
        }
        (
            MaterializedAspectState::LifecycleTransition(transition),
            MaterializedAspectState::LifecycleTransition(_),
        ) => Ok((
            CanonicalAspectDeltaEvidence::Lifecycle {
                locator,
                transition,
            },
            transition != LifecycleTransitionClass::NoTransition,
        )),
        (old_state, new_state) => Err(CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
            aspect_key: aspect_key.clone(),
            detail: format!(
                "materialized aspect state mismatch during canonical delta evaluation: old={old_state:?}, new={new_state:?}"
            ),
        }),
    }
}

fn authoritative_value_locator(
    aspect_key: &AspectKey,
) -> forge_foundational::facade::AspectValueLocator {
    forge_foundational::facade::AspectValueLocator::whole_aspect(
        forge_foundational::facade::AspectLocator::new(
            forge_foundational::facade::LocatorAuthority::Authoritative,
            aspect_key.clone(),
        ),
    )
}

fn authoritative_state_for_side<'a>(
    context: BindingEvaluationContext<'a>,
    side: MaterializationSide,
) -> Option<&'a AuthoritativeRecordAspectState> {
    match side {
        MaterializationSide::Old => context.old_authoritative_state(),
        MaterializationSide::New => context.new_authoritative_state(),
    }
}

fn invalid_binding_context(aspect_key: &AspectKey, detail: &str) -> CanonicalDeltaError {
    CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
        aspect_key: aspect_key.clone(),
        detail: format!("{detail} during canonical delta evaluation"),
    }
}

fn entity_field_binding_requires_authoritative_patch(
    aspect_key: &AspectKey,
    field: &forge_foundational::facade::FieldKey,
) -> CanonicalDeltaError {
    CanonicalDeltaError::EntityFieldBindingRequiresAuthoritativePatchEvidence {
        target: AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
    }
}
