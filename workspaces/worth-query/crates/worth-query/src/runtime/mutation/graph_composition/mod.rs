mod builder;
mod capability_families;
mod denial;
mod domain_invariant_denial;
mod existing_lifecycle;
mod relation_builder;
mod symbols;
mod touch_descriptor;

pub use builder::WorthQueryGraphCompositionBuilder;
pub(crate) use capability_families::{
    GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES, GRAPH_COMPOSITION_LIFECYCLE_FAMILIES,
    GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};
pub(crate) use denial::graph_composition_error;
pub use denial::{WorthQueryGraphCompositionDenial, WorthQueryGraphCompositionDenialKind};
pub use domain_invariant_denial::WorthQueryGraphCompositionDomainInvariantDenial;
pub use relation_builder::WorthQueryGraphRelationMutationBuilder;
pub use symbols::{WorthQueryGraphEntitySymbol, WorthQueryGraphRelationSymbol};
pub use touch_descriptor::{
    WorthQueryGraphReadTouchShape, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchDescriptorDenial, WorthQueryGraphTouchDescriptorDenialKind,
    WorthQueryGraphTouchDescriptorKind, WorthQueryGraphTouchDescriptorRow,
    WorthQueryGraphTouchLifecycleFamily, WorthQueryGraphTouchReadVerb,
};
