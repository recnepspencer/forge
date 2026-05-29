use crate::identity::data::KindId;

use crate::schema::data::{
    AspectBinding, AspectPlanRevision, DeclaredAspect, LoweredAspectBinding, LoweredAspectTarget,
};

pub(super) fn lower_binding(
    _kind_id: KindId,
    _plan_revision: AspectPlanRevision,
    aspect: &DeclaredAspect,
) -> LoweredAspectBinding {
    LoweredAspectBinding {
        contract: aspect.contract.clone(),
        target: match &aspect.binding {
            AspectBinding::EntityField { field } => LoweredAspectTarget::EntityField {
                field: field.clone(),
            },
            AspectBinding::RelationField { field } => LoweredAspectTarget::RelationField {
                field: field.clone(),
            },
            AspectBinding::RelationSourceEndpoint => LoweredAspectTarget::RelationSourceEndpoint,
            AspectBinding::RelationTargetEndpoint => LoweredAspectTarget::RelationTargetEndpoint,
            AspectBinding::LifecycleTransition => LoweredAspectTarget::LifecycleTransition,
        },
    }
}
