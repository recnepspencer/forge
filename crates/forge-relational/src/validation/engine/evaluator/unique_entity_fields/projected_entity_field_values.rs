use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, AuthoritativeRecordAspectState,
    ContractValidatedAspectValueView, FieldKey,
};

use crate::identity::data::{EntityId, KindId};
use crate::schema::data::{LoweredAspectBinding, LoweredAspectPlan};
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
};
use crate::validation::engine::state_view::VisibleEntityMetadata;
use crate::visibility::materialization::read_records::ProjectionAspectScope;

use super::super::super::context::InvariantExecutionContext;

#[derive(Debug, Clone)]
pub(super) struct ProjectedEntityAspectFieldValue {
    pub(super) entity_id: EntityId,
    pub(super) value: AspectValue,
}

#[derive(Debug, Clone)]
enum AdmittedEntityAspectFieldProjection {
    ScalarAspect {
        aspect_key: AspectKey,
        projection_scope: ProjectionAspectScope,
    },
    StructField {
        aspect_key: AspectKey,
        field: FieldKey,
        projection_scope: ProjectionAspectScope,
    },
}

pub(super) fn projected_entity_aspect_field_value(
    context: &InvariantExecutionContext<'_>,
    entity_id: EntityId,
    field_locator: &AspectFieldLocator,
) -> Option<ProjectedEntityAspectFieldValue> {
    let state_view = context.state_view();
    let metadata = state_view.entity_metadata(entity_id)?;
    projected_entity_aspect_field_value_for_metadata(context, &metadata, field_locator)
}

pub(super) fn projected_entity_aspect_field_value_for_metadata(
    context: &InvariantExecutionContext<'_>,
    metadata: &VisibleEntityMetadata,
    field_locator: &AspectFieldLocator,
) -> Option<ProjectedEntityAspectFieldValue> {
    let projection = AdmittedEntityAspectFieldProjection::for_entity_kind(
        context,
        metadata.kind_id,
        field_locator,
    )?;
    let state = context
        .state_view()
        .entity_aspect_state(metadata.entity_id)?;
    projection
        .value_from_state(state)
        .cloned()
        .map(|value| ProjectedEntityAspectFieldValue {
            entity_id: metadata.entity_id,
            value,
        })
}

pub(super) fn visible_entity_field_value_conflict(
    context: &InvariantExecutionContext<'_>,
    field_locator: &AspectFieldLocator,
    comparison_key: &AuthoritativeFieldComparisonKey,
    include_entity: impl Fn(EntityId) -> bool,
) -> bool {
    let state_view = context.state_view();
    for partition_id in state_view.state().partition_ids() {
        if state_view.state().get_partition(partition_id).is_none() {
            continue;
        }
        let Some(slot_count) = state_view.entity_slot_scan_count(partition_id) else {
            continue;
        };
        for slot in 0..slot_count {
            let Some(metadata) = state_view.entity_metadata_for_slot(partition_id, slot) else {
                continue;
            };
            if !include_entity(metadata.entity_id) {
                continue;
            }
            let Some(projected) =
                projected_entity_aspect_field_value_for_metadata(context, &metadata, field_locator)
            else {
                continue;
            };
            if &authoritative_aspect_value_field_comparison_key(&projected.value) == comparison_key
            {
                return true;
            }
        }
    }
    false
}

impl AdmittedEntityAspectFieldProjection {
    fn for_entity_kind(
        context: &InvariantExecutionContext<'_>,
        kind_id: KindId,
        field_locator: &AspectFieldLocator,
    ) -> Option<Self> {
        let field = single_field_locator_key(field_locator)?;
        let plan = context.entity_aspect_plan(kind_id)?;
        let binding = matching_binding(plan, field_locator)?;
        let projection = if binding.targets_entity_scalar_field(field) {
            Self::ScalarAspect {
                aspect_key: binding.aspect_key().clone(),
                projection_scope: ProjectionAspectScope::whole_aspects([binding
                    .aspect_key()
                    .clone()]),
            }
        } else if binding.targets_entity_struct_field(field) {
            Self::StructField {
                aspect_key: binding.aspect_key().clone(),
                field: field.clone(),
                projection_scope: ProjectionAspectScope::fields(
                    binding.aspect_key().clone(),
                    [field.clone()],
                ),
            }
        } else {
            return None;
        };
        projection.admitted_by(binding).then_some(projection)
    }

    fn value_from_state<'a>(
        &self,
        state: &'a AuthoritativeRecordAspectState,
    ) -> Option<&'a AspectValue> {
        match self {
            Self::ScalarAspect { aspect_key, .. } => match state.get(aspect_key)?.view() {
                ContractValidatedAspectValueView::Scalar(value) => Some(value),
                ContractValidatedAspectValueView::Struct(_) => None,
            },
            Self::StructField {
                aspect_key, field, ..
            } => match state.get(aspect_key)?.view() {
                ContractValidatedAspectValueView::Scalar(_) => None,
                ContractValidatedAspectValueView::Struct(value) => value.get(field),
            },
        }
    }

    fn admitted_by(&self, binding: &LoweredAspectBinding) -> bool {
        self.projection_scope()
            .requirements()
            .iter()
            .all(|requirement| {
                binding
                    .contract
                    .admits_projection_mask(requirement.mask())
                    .is_ok()
                    && requirement.mask_basis().is_some()
            })
    }

    fn projection_scope(&self) -> &ProjectionAspectScope {
        match self {
            Self::ScalarAspect {
                projection_scope, ..
            }
            | Self::StructField {
                projection_scope, ..
            } => projection_scope,
        }
    }
}

fn matching_binding<'a>(
    plan: &'a LoweredAspectPlan,
    field_locator: &AspectFieldLocator,
) -> Option<&'a LoweredAspectBinding> {
    plan.executable_bindings
        .iter()
        .find(|binding| binding.aspect_key() == field_locator.aspect().aspect_key())
}

fn single_field_locator_key(field_locator: &AspectFieldLocator) -> Option<&FieldKey> {
    match field_locator.field_path().fields() {
        [field] => Some(field),
        _ => None,
    }
}
