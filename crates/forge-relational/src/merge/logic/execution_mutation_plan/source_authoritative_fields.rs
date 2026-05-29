use std::collections::BTreeMap;

use forge_foundational::facade::ContractValidatedAspectValueView;

use crate::capabilities::AspectPlanSource;
use crate::merge::data::{AdoptSourceRecordPlan, MergeExecutionMutationPlanError};
use crate::schema::data::LoweredExecutableAspectBindingKind;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::transactions::data::{AspectFieldPatch, AspectFieldPatchTarget};

pub(super) fn entity_create_fields_from_authoritative_state(
    runtime: &crate::logic::runtime::RelationalRuntime,
    plan: &AdoptSourceRecordPlan,
    entity: &EntityReadRecord,
) -> Result<AspectFieldPatch, MergeExecutionMutationPlanError> {
    let Some(authoritative_state) = entity.authoritative_aspect_state.as_ref() else {
        return Ok(AspectFieldPatch::default());
    };
    let lowered_plan = runtime
        .entity_aspect_plan(entity.kind.kind_id)
        .ok_or_else(
            || MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.source_record.clone(),
                detail:
                    "source entity has authoritative aspect state but no executable aspect plan",
            },
        )?;
    let mut fields = BTreeMap::new();
    for (aspect_key, artifact) in authoritative_state.aspects().entries() {
        let binding = lowered_plan
            .executable_bindings
            .iter()
            .find(|binding| binding.aspect_key == *aspect_key)
            .ok_or_else(|| MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                record: plan.source_record.clone(),
                aspect_key: aspect_key.clone(),
                detail: "source entity authoritative aspect is not declared by the executable aspect plan",
            })?;
        match artifact.view() {
            ContractValidatedAspectValueView::Scalar(value) => {
                let LoweredExecutableAspectBindingKind::EntityFieldScalar { field } =
                    &binding.binding_kind
                else {
                    return Err(MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                        record: plan.source_record.clone(),
                        aspect_key: aspect_key.clone(),
                        detail: "source entity scalar authoritative aspect is not backed by an entity scalar field binding",
                    });
                };
                fields.insert(
                    AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
                    value.clone(),
                );
            }
            ContractValidatedAspectValueView::Struct(struct_value) => {
                if !matches!(
                    &binding.binding_kind,
                    LoweredExecutableAspectBindingKind::EntityFieldStruct { .. }
                ) {
                    return Err(MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                        record: plan.source_record.clone(),
                        aspect_key: aspect_key.clone(),
                        detail: "source entity struct authoritative aspect is not backed by an entity struct field binding",
                    });
                }
                for (field, value) in struct_value.fields() {
                    fields.insert(
                        AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
                        value.clone(),
                    );
                }
            }
        }
    }
    Ok(AspectFieldPatch::from(fields))
}

pub(super) fn relation_create_fields_from_authoritative_state(
    runtime: &crate::logic::runtime::RelationalRuntime,
    plan: &AdoptSourceRecordPlan,
    relation: &RelationReadRecord,
) -> Result<AspectFieldPatch, MergeExecutionMutationPlanError> {
    let Some(authoritative_state) = relation.authoritative_aspect_state.as_ref() else {
        return Ok(AspectFieldPatch::default());
    };
    let lowered_plan = runtime
        .relation_aspect_plan(relation.kind.kind_id)
        .ok_or_else(
            || MergeExecutionMutationPlanError::UnsupportedReconcileRecordKind {
                record: plan.source_record.clone(),
                detail:
                    "source relation has authoritative aspect state but no executable aspect plan",
            },
        )?;
    let mut fields = BTreeMap::new();
    for (aspect_key, artifact) in authoritative_state.aspects().entries() {
        let Some(binding) = lowered_plan
            .executable_bindings
            .iter()
            .find(|binding| binding.aspect_key == *aspect_key)
        else {
            continue;
        };
        match (&binding.binding_kind, artifact.view()) {
            (
                LoweredExecutableAspectBindingKind::RelationFieldScalar { field },
                ContractValidatedAspectValueView::Scalar(value),
            ) => {
                fields.insert(
                    AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
                    value.clone(),
                );
            }
            (
                LoweredExecutableAspectBindingKind::RelationFieldStruct { .. },
                ContractValidatedAspectValueView::Struct(struct_value),
            ) => {
                for (field, value) in struct_value.fields() {
                    fields.insert(
                        AspectFieldPatchTarget::single(aspect_key.clone(), field.clone()),
                        value.clone(),
                    );
                }
            }
            (
                LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity
                | LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity,
                _,
            ) => {}
            _ => {
                return Err(MergeExecutionMutationPlanError::UnsupportedAspectMutationMaterialization {
                    record: plan.source_record.clone(),
                    aspect_key: aspect_key.clone(),
                    detail: "source relation authoritative aspect shape does not match relation field binding",
                });
            }
        }
    }
    Ok(AspectFieldPatch::from(fields))
}
