mod builder;
mod capability_families;
mod denial;
mod domain_invariant_denial;
mod existing_lifecycle;
mod hooks;
mod relation_builder;
mod symbols;

pub use builder::ForgeQueryGraphCompositionBuilder;
pub(crate) use capability_families::{
    GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES, GRAPH_COMPOSITION_LIFECYCLE_FAMILIES,
    GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};
pub(crate) use denial::graph_composition_error;
pub use denial::{ForgeQueryGraphCompositionDenial, ForgeQueryGraphCompositionDenialKind};
pub use domain_invariant_denial::ForgeQueryGraphCompositionDomainInvariantDenial;
pub use hooks::{
    ForgeQueryGraphCompositionInvariantPackContext,
    ForgeQueryGraphCompositionInvariantPackViolation,
};
pub use relation_builder::ForgeQueryGraphRelationMutationBuilder;
pub use symbols::{ForgeQueryGraphEntitySymbol, ForgeQueryGraphRelationSymbol};
