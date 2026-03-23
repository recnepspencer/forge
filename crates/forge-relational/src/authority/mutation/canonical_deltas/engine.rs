use smallvec::SmallVec;

use crate::identity::data::EntityId;
use crate::payloads::data::RecordPayload;
use crate::publication::patch::data::{AspectKey, CanonicalAspectSet, RecordStructuralChange};
use crate::schema::data::{
    AspectPrecision, LoweredAspectPlan, LoweredExecutableAspectBindingKind,
};
use crate::transactions::data::RecordRef;

use super::data::{
    BindingEvaluationContext, BindingEvidence, CanonicalDeltaError, CanonicalRecordAspectDelta,
    EvaluatedAspectBinding,
};
use super::evidence::{
    lifecycle_transition, payload_diagnostic_digest, raw_field_name, serialize_json_value,
};
use crate::authority::mutation::outcomes::RecordMutation;
use crate::authority::mutation::MutationWorkspace;

#[derive(Debug, Clone, Copy)]
struct EntityState<'a> {
    payload: Option<&'a RecordPayload>,
}

#[derive(Debug, Clone, Copy)]
struct RelationState<'a> {
    source: Option<EntityId>,
    target: Option<EntityId>,
    payload: Option<&'a RecordPayload>,
}

pub(crate) fn canonical_delta_for_mutation(
    mutation: &RecordMutation,
    workspace: &MutationWorkspace<'_>,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    match mutation {
        RecordMutation::EntityCreated {
            entity_id,
            kind_id,
            payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            EntityState { payload: None },
            EntityState {
                payload: Some(payload),
            },
            RecordStructuralChange::Created,
        ),
        RecordMutation::EntityUpdated {
            entity_id,
            kind_id,
            old_payload,
            new_payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            EntityState {
                payload: Some(old_payload),
            },
            EntityState {
                payload: Some(new_payload),
            },
            RecordStructuralChange::Updated,
        ),
        RecordMutation::EntityDeleted {
            entity_id,
            kind_id,
            payload,
        } => evaluate_entity_delta(
            workspace,
            *entity_id,
            *kind_id,
            EntityState {
                payload: Some(payload),
            },
            EntityState { payload: None },
            RecordStructuralChange::Deleted,
        ),
        RecordMutation::RelationCreated {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: None,
                target: None,
                payload: None,
            },
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RecordStructuralChange::Created,
        ),
        RecordMutation::RelationDeleted {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RelationState {
                source: None,
                target: None,
                payload: None,
            },
            RecordStructuralChange::Deleted,
        ),
        RecordMutation::RelationRetainedForAudit {
            relation_id,
            kind_id,
            source,
            target,
            payload,
        } => evaluate_relation_delta(
            workspace,
            *relation_id,
            *kind_id,
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RelationState {
                source: Some(*source),
                target: Some(*target),
                payload: payload.as_ref(),
            },
            RecordStructuralChange::RetainedForAudit,
        ),
    }
}

fn evaluate_entity_delta(
    workspace: &MutationWorkspace<'_>,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    old_state: EntityState<'_>,
    new_state: EntityState<'_>,
    structural_change: RecordStructuralChange,
) -> Result<CanonicalRecordAspectDelta, CanonicalDeltaError> {
    let plan = workspace
        .entity_aspect_plan(kind_id)
        .ok_or(CanonicalDeltaError::MissingEntityAspectPlan { kind_id })?;
    let evaluated_bindings = evaluate_bindings(
        plan,
        BindingEvaluationContext::Entity {
            structural_change,
            old_payload: old_state.payload,
            new_payload: new_state.payload,
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
            old_payload: old_state.payload,
            new_payload: new_state.payload,
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
    let contains_degraded_precision = evaluated_bindings
        .iter()
        .any(|binding| binding.precision == AspectPrecision::Opaque);
    CanonicalRecordAspectDelta {
        target,
        kind_id,
        plan_revision: plan.plan_revision,
        structural_change,
        changed_aspects,
        evaluated_bindings,
        contains_degraded_precision,
    }
}

fn evaluate_bindings(
    plan: &LoweredAspectPlan,
    context: BindingEvaluationContext<'_>,
) -> Result<SmallVec<[EvaluatedAspectBinding; 4]>, CanonicalDeltaError> {
    let mut evaluated = SmallVec::new();
    let lifecycle_transition = lifecycle_transition(context.structural_change());
    for binding in &plan.executable_bindings {
        let (evidence, changed) = match &binding.binding_kind {
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { field } => {
                if !matches!(context, BindingEvaluationContext::Entity { .. }) {
                    return Err(CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
                        aspect_key: binding.aspect_key.clone(),
                        detail: format!(
                            "entity JSON aspect {:?} cannot be evaluated against relation context",
                            binding.aspect_key
                        ),
                    });
                }
                evaluate_json_field(
                    &binding.aspect_key,
                    context.old_payload(),
                    context.new_payload(),
                    raw_field_name(&binding.aspect_key, binding, field)?,
                )?
            }
            LoweredExecutableAspectBindingKind::RelationJsonScalarField { field } => {
                if !matches!(context, BindingEvaluationContext::Relation { .. }) {
                    return Err(CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
                        aspect_key: binding.aspect_key.clone(),
                        detail: format!(
                            "relation JSON aspect {:?} cannot be evaluated against entity context",
                            binding.aspect_key
                        ),
                    });
                }
                evaluate_json_field(
                    &binding.aspect_key,
                    context.old_payload(),
                    context.new_payload(),
                    raw_field_name(&binding.aspect_key, binding, field)?,
                )?
            }
            LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity => {
                let Some((old_source, new_source, _, _)) = context.relation_endpoints() else {
                    return Err(CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
                        aspect_key: binding.aspect_key.clone(),
                        detail: format!(
                            "relation source endpoint aspect {:?} cannot be evaluated against entity context",
                            binding.aspect_key
                        ),
                    });
                };
                let evidence = BindingEvidence::EndpointIdentity {
                    old: old_source,
                    new: new_source,
                };
                (evidence.clone(), endpoint_evidence_changed(&evidence))
            }
            LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity => {
                let Some((_, _, old_target, new_target)) = context.relation_endpoints() else {
                    return Err(CanonicalDeltaError::InvalidLoweredBindingForRecordClass {
                        aspect_key: binding.aspect_key.clone(),
                        detail: format!(
                            "relation target endpoint aspect {:?} cannot be evaluated against entity context",
                            binding.aspect_key
                        ),
                    });
                };
                let evidence = BindingEvidence::EndpointIdentity {
                    old: old_target,
                    new: new_target,
                };
                (evidence.clone(), endpoint_evidence_changed(&evidence))
            }
            LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => {
                let evidence = BindingEvidence::Lifecycle {
                    transition: lifecycle_transition,
                };
                (evidence.clone(), lifecycle_transition != super::data::LifecycleTransitionClass::NoTransition)
            }
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes => {
                evaluate_opaque_payload(context.old_payload(), context.new_payload())?
            }
        };
        evaluated.push(EvaluatedAspectBinding {
            aspect_key: binding.aspect_key.clone(),
            changed,
            precision: binding.precision,
            evidence,
        });
    }
    Ok(evaluated)
}

fn evaluate_json_field(
    aspect_key: &AspectKey,
    old_payload: Option<&RecordPayload>,
    new_payload: Option<&RecordPayload>,
    field_name: &str,
) -> Result<(BindingEvidence, bool), CanonicalDeltaError> {
    let old_value = extract_json_field(old_payload, field_name);
    let new_value = extract_json_field(new_payload, field_name);
    let changed = (old_value.is_some() || new_value.is_some()) && old_value != new_value;
    let evidence = BindingEvidence::JsonFieldPresenceOrValue {
        old_present: old_value.is_some(),
        new_present: new_value.is_some(),
        old_canonical_json: if changed {
            old_value
                .map(|value| serialize_json_value(aspect_key, value))
                .transpose()?
        } else {
            None
        },
        new_canonical_json: if changed {
            new_value
                .map(|value| serialize_json_value(aspect_key, value))
                .transpose()?
        } else {
            None
        },
    };
    Ok((evidence, changed))
}

fn extract_json_field<'a>(
    payload: Option<&'a RecordPayload>,
    field_name: &str,
) -> Option<&'a serde_json::Value> {
    payload
        .and_then(RecordPayload::as_json)
        .and_then(serde_json::Value::as_object)
        .and_then(|object| object.get(field_name))
}

fn endpoint_evidence_changed(evidence: &BindingEvidence) -> bool {
    match evidence {
        BindingEvidence::EndpointIdentity { old, new } => old != new,
        _ => false,
    }
}

fn evaluate_opaque_payload(
    old_payload: Option<&RecordPayload>,
    new_payload: Option<&RecordPayload>,
) -> Result<(BindingEvidence, bool), CanonicalDeltaError> {
    let changed = old_payload != new_payload;
    let evidence = BindingEvidence::OpaquePayloadDigest {
        old_present: old_payload.is_some(),
        new_present: new_payload.is_some(),
        old_diagnostic_digest: if changed {
            old_payload.map(payload_diagnostic_digest).transpose()?
        } else {
            None
        },
        new_diagnostic_digest: if changed {
            new_payload.map(payload_diagnostic_digest).transpose()?
        } else {
            None
        },
    };
    Ok((evidence, changed))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::authority::mutation::outcomes::RecordMutation;
    use crate::authority::mutation::MutationWorkspace;
    use crate::config::data::{
        AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CrossContextPolicy,
        PatchSurfacePolicy,
    };
    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::payloads::data::RecordPayload;
    use crate::publication::patch::data::AspectKey;
    use crate::schema::data::{
        AspectPlanCatalog, AspectPlanRevision, AspectPrecision, LoweredAspectBinding,
        LoweredAspectPlan, LoweredExecutableAspectBindingKind, RelationalSchemaRegistry,
    };
    use crate::storage::overlay::WorkingState;
    use crate::symbols::data::{InternedString, StringInterner};

    use super::{canonical_delta_for_mutation, CanonicalDeltaError};

    fn mutation_config() -> crate::config::data::MutationConfig {
        crate::config::data::MutationConfig {
            patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            adjacency_policy: AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 4,
            },
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            execution_model: crate::logic::planning::RelationalExecutionModel::SerialAuthority,
        }
    }

    fn empty_workspace<'a>(
        state: &'a mut WorkingState,
        symbols: &'a mut StringInterner,
        aspect_plans: &'a AspectPlanCatalog,
        config: &'a crate::config::data::MutationConfig,
    ) -> MutationWorkspace<'a> {
        MutationWorkspace::new(
            state,
            symbols,
            config,
            &RelationalSchemaRegistry::new(),
            aspect_plans,
            VersionId(1),
        )
    }

    #[test]
    fn missing_entity_aspect_plan_returns_typed_error() {
        let config = mutation_config();
        let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
        let mut symbols = StringInterner::default();
        let catalog = AspectPlanCatalog::empty();
        let mutation = RecordMutation::EntityCreated {
            entity_id: EntityId::new(PartitionId(1), 0, 1),
            kind_id: KindId(999),
            payload: RecordPayload::StructuredJson(serde_json::json!({"name":"missing-plan"})),
        };

        let error = canonical_delta_for_mutation(
            &mutation,
            &empty_workspace(&mut state, &mut symbols, &catalog, &config),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            CanonicalDeltaError::MissingEntityAspectPlan {
                kind_id: KindId(999)
            }
        ));
    }

    #[test]
    fn missing_relation_aspect_plan_returns_typed_error() {
        let config = mutation_config();
        let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
        let mut symbols = StringInterner::default();
        let catalog = AspectPlanCatalog::empty();
        let source = EntityId::new(PartitionId(1), 0, 1);
        let target = EntityId::new(PartitionId(1), 1, 1);
        let mutation = RecordMutation::RelationCreated {
            relation_id: RelationId::new(PartitionId(2), 0, 1),
            kind_id: KindId(777),
            source,
            target,
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"label":"missing-plan"}),
            )),
        };

        let error = canonical_delta_for_mutation(
            &mutation,
            &empty_workspace(&mut state, &mut symbols, &catalog, &config),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            CanonicalDeltaError::MissingRelationAspectPlan {
                kind_id: KindId(777)
            }
        ));
    }

    #[test]
    fn symbolic_lowered_field_name_returns_typed_error() {
        let config = mutation_config();
        let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
        let mut symbols = StringInterner::default();
        let symbolic = InternedString::Symbol(symbols.intern("symbolic-field"));
        let mut catalog = AspectPlanCatalog::empty();
        catalog.entity_plans.insert(
            KindId(1),
            LoweredAspectPlan {
                kind_id: KindId(1),
                plan_revision: AspectPlanRevision(7),
                executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                    aspect_key: AspectKey(InternedString::Raw("name".to_string())),
                    binding_kind: LoweredExecutableAspectBindingKind::EntityJsonScalarField {
                        field: symbolic,
                    },
                    precision: AspectPrecision::Structured,
                }],
            },
        );
        let mutation = RecordMutation::EntityCreated {
            entity_id: EntityId::new(PartitionId(1), 0, 1),
            kind_id: KindId(1),
            payload: RecordPayload::StructuredJson(serde_json::json!({"name":"symbolic"})),
        };

        let error = canonical_delta_for_mutation(
            &mutation,
            &empty_workspace(&mut state, &mut symbols, &catalog, &config),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            CanonicalDeltaError::SymbolicLoweredFieldName { .. }
        ));
    }
}
