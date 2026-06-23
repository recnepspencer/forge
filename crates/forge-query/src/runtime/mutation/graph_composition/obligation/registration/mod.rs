mod graph_scoped_custom_invariant_registration;
mod operating_world_selector;
mod registration;
mod registration_catalog;
mod registration_denial;
mod relational_schema_contracts;
mod selector_class;
mod selector_helpers;
mod support_posture;
mod touch_selector;

pub use graph_scoped_custom_invariant_registration::ForgeQueryGraphScopedCustomInvariantRegistration;
pub use operating_world_selector::ForgeQueryGraphObligationOperatingWorldSelector;
pub use registration::ForgeQueryGraphObligationRegistration;
pub use registration_catalog::ForgeQueryGraphObligationRegistrationCatalog;
pub use registration_denial::{
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationRegistrationDenialKind,
};
pub(crate) use relational_schema_contracts::registrations_from_relational_invariant_catalog;
pub(in crate::runtime::mutation::graph_composition::obligation) use selector_class::ForgeQueryGraphTouchSelectorClass;
pub use support_posture::{
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphObligationSupportStatus,
};
pub use touch_selector::ForgeQueryGraphTouchSelector;

#[cfg(test)]
mod tests;
