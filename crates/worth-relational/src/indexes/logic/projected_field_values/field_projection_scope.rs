use worth_foundational::facade::{AspectFieldLocator, AspectKey, AspectValue, FieldKey};

use crate::capabilities::AspectPlanSource;
use crate::identity::data::KindId;
use crate::logic::runtime::EntityProjectionRecord;
use crate::schema::data::{LoweredAspectContractBinding, LoweredAspectContractPlan};
use crate::visibility::materialization::read_records::ProjectionAspectScope;

#[derive(Debug, Clone)]
pub(super) enum EntityIndexFieldProjectionScope {
    ScalarAspect {
        kind_id: KindId,
        aspect_key: AspectKey,
        projection_scope: ProjectionAspectScope,
    },
    StructField {
        kind_id: KindId,
        aspect_key: AspectKey,
        field: FieldKey,
        projection_scope: ProjectionAspectScope,
    },
}

#[derive(Debug, Clone)]
pub(super) enum RelationIndexFieldProjectionScope {
    ScalarAspect {
        kind_id: KindId,
        aspect_key: AspectKey,
        projection_scope: ProjectionAspectScope,
    },
    StructField {
        kind_id: KindId,
        aspect_key: AspectKey,
        field: FieldKey,
        projection_scope: ProjectionAspectScope,
    },
}

impl EntityIndexFieldProjectionScope {
    pub(super) const fn kind_id(&self) -> KindId {
        match self {
            Self::ScalarAspect { kind_id, .. } | Self::StructField { kind_id, .. } => *kind_id,
        }
    }

    pub(super) fn projection_scope(&self) -> ProjectionAspectScope {
        match self {
            Self::ScalarAspect {
                projection_scope, ..
            }
            | Self::StructField {
                projection_scope, ..
            } => projection_scope.clone(),
        }
    }

    pub(super) fn projected_value<'a>(
        &self,
        record: EntityProjectionRecord<'a>,
    ) -> Option<&'a AspectValue> {
        match self {
            Self::ScalarAspect { aspect_key, .. } => record.aspect_value(aspect_key),
            Self::StructField {
                aspect_key, field, ..
            } => record.aspect_field_value(aspect_key, field),
        }
    }
}

impl RelationIndexFieldProjectionScope {
    pub(super) const fn kind_id(&self) -> KindId {
        match self {
            Self::ScalarAspect { kind_id, .. } | Self::StructField { kind_id, .. } => *kind_id,
        }
    }

    pub(super) fn projection_scope(&self) -> ProjectionAspectScope {
        match self {
            Self::ScalarAspect {
                projection_scope, ..
            }
            | Self::StructField {
                projection_scope, ..
            } => projection_scope.clone(),
        }
    }

    pub(super) fn projected_value<'a>(
        &self,
        record: crate::logic::runtime::RelationProjectionRecord<'a>,
    ) -> Option<&'a AspectValue> {
        match self {
            Self::ScalarAspect { aspect_key, .. } => record.aspect_value(aspect_key),
            Self::StructField {
                aspect_key, field, ..
            } => record.aspect_field_value(aspect_key, field),
        }
    }
}

pub(super) fn entity_index_projection_scopes(
    runtime: &crate::logic::runtime::RelationalRuntime,
    field_locator: &AspectFieldLocator,
) -> Vec<EntityIndexFieldProjectionScope> {
    runtime
        .aspect_plan_catalog()
        .entity_plans
        .values()
        .filter_map(|plan| entity_index_projection_scope(plan, field_locator))
        .collect()
}

pub(super) fn entity_index_projection_scope_for_kind(
    runtime: &crate::logic::runtime::RelationalRuntime,
    kind_id: KindId,
    field_locator: &AspectFieldLocator,
) -> Option<EntityIndexFieldProjectionScope> {
    let plan = runtime.entity_aspect_plan(kind_id)?;
    entity_index_projection_scope(plan, field_locator)
}

pub(super) fn relation_index_projection_scopes(
    runtime: &crate::logic::runtime::RelationalRuntime,
    field_locator: &AspectFieldLocator,
) -> Vec<RelationIndexFieldProjectionScope> {
    runtime
        .aspect_plan_catalog()
        .relation_plans
        .values()
        .filter_map(|plan| relation_index_projection_scope(plan, field_locator))
        .collect()
}

fn entity_index_projection_scope(
    plan: &LoweredAspectContractPlan,
    field_locator: &AspectFieldLocator,
) -> Option<EntityIndexFieldProjectionScope> {
    let field = single_field_locator_key(field_locator)?;
    let binding = matching_binding(plan, field_locator)?;
    if binding.targets_entity_scalar_field(field) {
        return Some(EntityIndexFieldProjectionScope::ScalarAspect {
            kind_id: plan.kind_id,
            aspect_key: binding.aspect_key().clone(),
            projection_scope: ProjectionAspectScope::whole_aspects([binding.aspect_key().clone()]),
        });
    }
    if binding.targets_entity_struct_field(field) {
        return Some(EntityIndexFieldProjectionScope::StructField {
            kind_id: plan.kind_id,
            aspect_key: binding.aspect_key().clone(),
            field: field.clone(),
            projection_scope: ProjectionAspectScope::fields(
                binding.aspect_key().clone(),
                [field.clone()],
            ),
        });
    }
    None
}

fn relation_index_projection_scope(
    plan: &LoweredAspectContractPlan,
    field_locator: &AspectFieldLocator,
) -> Option<RelationIndexFieldProjectionScope> {
    let field = single_field_locator_key(field_locator)?;
    let binding = matching_binding(plan, field_locator)?;
    if binding.targets_relation_scalar_field(field) {
        return Some(RelationIndexFieldProjectionScope::ScalarAspect {
            kind_id: plan.kind_id,
            aspect_key: binding.aspect_key().clone(),
            projection_scope: ProjectionAspectScope::whole_aspects([binding.aspect_key().clone()]),
        });
    }
    if binding.targets_relation_struct_field(field) {
        return Some(RelationIndexFieldProjectionScope::StructField {
            kind_id: plan.kind_id,
            aspect_key: binding.aspect_key().clone(),
            field: field.clone(),
            projection_scope: ProjectionAspectScope::fields(
                binding.aspect_key().clone(),
                [field.clone()],
            ),
        });
    }
    None
}

fn matching_binding<'a>(
    plan: &'a LoweredAspectContractPlan,
    field_locator: &AspectFieldLocator,
) -> Option<&'a LoweredAspectContractBinding> {
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
