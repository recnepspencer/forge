use worth_foundational::facade::{AspectFieldLocator, AspectValue};

use crate::capabilities::AspectPlanSource;
use crate::identity::data::{EntityId, KindId};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::visibility::materialization::read_records::{
    ProjectionAspectRequirement, ProjectionAspectScope, VisibilityProjectionView,
};

use super::RelationalAuthorizationObservationCounters;

pub(super) fn observed_field(
    runtime: &RelationalRuntime,
    view: &VisibilityProjectionView<'_>,
    entity: EntityId,
    kind: KindId,
    locator: &AspectFieldLocator,
) -> Option<AspectValue> {
    let field = locator.field_path().fields().first()?.clone();
    let plan = runtime.entity_aspect_plan(kind)?;
    let binding = plan
        .executable_bindings
        .iter()
        .find(|binding| binding.aspect_key() == locator.aspect().aspect_key())?;
    let scalar_aspect = binding.targets_entity_scalar_field(&field);
    let requirement = if scalar_aspect {
        ProjectionAspectRequirement::whole_aspect(locator.aspect().aspect_key().clone())
    } else if binding.targets_entity_struct_field(&field) {
        ProjectionAspectRequirement::fields(locator.aspect().aspect_key().clone(), [field.clone()])
    } else {
        return None;
    };
    let scope = ProjectionAspectScope::from_requirements([requirement]);
    view.entity_record_with_projection_scope(entity, scope, |record| {
        (record.kind_id() == kind && record.lifecycle() == RecordLifecycleState::Live)
            .then(|| {
                if scalar_aspect {
                    record.aspect_value(locator.aspect().aspect_key()).cloned()
                } else {
                    record
                        .aspect_field_value(locator.aspect().aspect_key(), &field)
                        .cloned()
                }
            })
            .flatten()
    })
}

pub(super) fn entity_is_live_kind(
    view: &VisibilityProjectionView<'_>,
    entity: EntityId,
    kind: KindId,
    counters: &mut RelationalAuthorizationObservationCounters,
) -> bool {
    counters.entity_records_inspected += 1;
    view.entity_record_with_projection_scope(entity, ProjectionAspectScope::empty(), |record| {
        Some(record.kind_id() == kind && record.lifecycle() == RecordLifecycleState::Live)
    })
    .unwrap_or(false)
}
