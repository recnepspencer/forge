mod auto_resolution;
mod binding_values;

use crate::schema::data::{LoweredAspectBinding, LoweredAspectTarget};

use super::contexts::RuntimeAspectValueBinding;

pub(super) use auto_resolution::{resolve_aspect_value_strategy, AutoResolutionStrategy};

fn runtime_aspect_value_binding(
    binding: Option<&LoweredAspectBinding>,
) -> Option<RuntimeAspectValueBinding> {
    let binding = binding?;
    match (&binding.target, binding.contract.shape()) {
        (LoweredAspectTarget::EntityField { .. }, forge_foundational::AspectShape::Scalar(_)) => {
            Some(RuntimeAspectValueBinding::EntityScalar(
                binding.aspect_key().clone(),
            ))
        }
        (LoweredAspectTarget::EntityField { .. }, forge_foundational::AspectShape::Struct(_)) => {
            Some(RuntimeAspectValueBinding::EntityStruct)
        }
        (LoweredAspectTarget::RelationField { .. }, forge_foundational::AspectShape::Scalar(_)) => {
            Some(RuntimeAspectValueBinding::RelationScalar(
                binding.aspect_key().clone(),
            ))
        }
        (LoweredAspectTarget::RelationField { .. }, forge_foundational::AspectShape::Struct(_)) => {
            Some(RuntimeAspectValueBinding::RelationStruct)
        }
        _ => None,
    }
}
