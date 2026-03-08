//! Engine contracts layer.
//!
//! DOMAIN: Extension points: Feature trait, FeatureContract trait,
//! FeatureInputs trait, FeatureRegistry trait, and supporting enums.
//!
//! Signal contract note: see `feature_signal_contract.md` in this directory for
//! the kernel-side `forge-signal` embedding rules.

pub mod contract;
pub mod feature_dependency;
pub mod feature_registry;
pub mod feature_signal_policy;
pub mod feature_trait;
