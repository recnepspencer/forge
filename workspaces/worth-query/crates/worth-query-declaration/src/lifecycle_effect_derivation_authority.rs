//! Execution-only erased lifecycle-effect derivation authority.

pub use crate::application_capability::lifecycle_effect::DerivedApplicationCapabilityLifecycleEffect;

pub fn derive_application_capability_lifecycle_effect(
    binding: &crate::application_capability::ApplicationCapabilityLifecycleEffectBinding,
    input: &dyn std::any::Any,
) -> Option<DerivedApplicationCapabilityLifecycleEffect> {
    binding.derive_from_retained_input(input)
}
