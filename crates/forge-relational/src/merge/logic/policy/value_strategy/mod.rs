mod auto_resolution;
mod binding_values;

use crate::schema::data::{LoweredAspectBinding, LoweredExecutableAspectBindingKind};

use super::contexts::RuntimeAspectValueBinding;

pub(super) use auto_resolution::{resolve_aspect_value_strategy, AutoResolutionStrategy};

fn runtime_aspect_value_binding(
    binding: Option<&LoweredAspectBinding>,
) -> Option<RuntimeAspectValueBinding> {
    let binding = binding?;
    match &binding.binding_kind {
        LoweredExecutableAspectBindingKind::EntityFieldScalar { .. } => Some(
            RuntimeAspectValueBinding::EntityScalar(binding.aspect_key.clone()),
        ),
        LoweredExecutableAspectBindingKind::EntityFieldStruct { .. } => {
            Some(RuntimeAspectValueBinding::EntityStruct)
        }
        LoweredExecutableAspectBindingKind::RelationFieldScalar { .. } => Some(
            RuntimeAspectValueBinding::RelationScalar(binding.aspect_key.clone()),
        ),
        LoweredExecutableAspectBindingKind::RelationFieldStruct { .. } => {
            Some(RuntimeAspectValueBinding::RelationStruct)
        }
        _ => None,
    }
}
