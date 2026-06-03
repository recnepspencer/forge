use worth_kernel::facade::{
    preview_primitive_intent_continuity_with_capabilities_and_profile,
    preview_primitive_intent_continuity_with_profile,
    preview_primitive_intent_with_capabilities_and_profile, preview_primitive_intent_with_profile,
};
use worth_kernel::facade::diagnostics::preview::PrimitiveIntentPreview;

fn main() {
    let _ = preview_primitive_intent_with_profile;
    let _ = preview_primitive_intent_with_capabilities_and_profile;
    let _ = preview_primitive_intent_continuity_with_profile;
    let _ = preview_primitive_intent_continuity_with_capabilities_and_profile;
    let _ = PrimitiveIntentPreview::analyze;
}
