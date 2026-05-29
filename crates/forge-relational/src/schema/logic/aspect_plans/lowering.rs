use crate::identity::data::KindId;
use forge_foundational::AspectShape;

use crate::schema::data::{
    AspectBinding, AspectPlanRevision, DeclaredAspect, LoweredAspectBinding,
    LoweredExecutableAspectBindingKind,
};

pub(super) fn lower_binding(
    _kind_id: KindId,
    _plan_revision: AspectPlanRevision,
    aspect: &DeclaredAspect,
) -> LoweredAspectBinding {
    LoweredAspectBinding {
        aspect_key: aspect.aspect_key(),
        contract: aspect.contract.clone(),
        binding_kind: match &aspect.binding {
            AspectBinding::EntityField { field } => match aspect.contract.shape() {
                AspectShape::Scalar(_) => LoweredExecutableAspectBindingKind::EntityFieldScalar {
                    field: field.clone(),
                },
                AspectShape::Struct(_) => LoweredExecutableAspectBindingKind::EntityFieldStruct {
                    field: field.clone(),
                },
                _ => unreachable!("entity field declaration validation rejects non-field shapes"),
            },
            AspectBinding::RelationField { field } => match aspect.contract.shape() {
                AspectShape::Scalar(_) => LoweredExecutableAspectBindingKind::RelationFieldScalar {
                    field: field.clone(),
                },
                AspectShape::Struct(_) => LoweredExecutableAspectBindingKind::RelationFieldStruct {
                    field: field.clone(),
                },
                _ => unreachable!("relation field declaration validation rejects non-field shapes"),
            },
            AspectBinding::RelationSourceEndpoint => {
                LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity
            }
            AspectBinding::RelationTargetEndpoint => {
                LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity
            }
            AspectBinding::LifecycleTransition => {
                LoweredExecutableAspectBindingKind::LifecycleTransitionEquality
            }
        },
    }
}
