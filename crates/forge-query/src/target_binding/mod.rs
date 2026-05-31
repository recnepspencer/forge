mod core;
mod resolve;
mod semantics;
mod targets;

pub use core::{
    ForgeQueryBindingTarget, ForgeQueryBindingTargetKind, ForgeQueryBindingTargetWitness,
};
pub use semantics::ForgeQueryBindingTargetSemantics;
pub use targets::{
    ForgeQueryAdmittedDeclarationProgressionBindingTarget,
    ForgeQueryAdmittedIntentPlanBindingTarget, ForgeQueryDeclarationEnvelopeBindingTarget,
    ForgeQueryDeclarationReceiptBindingTarget, ForgeQueryDeclarationRoutePlanBindingTarget,
    ForgeQueryIntentDeclarationBindingTarget, ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
};

pub(crate) use core::sealed;
pub(crate) use resolve::resolve_admitted_progression_target;
pub(crate) use targets::ForgeQueryAdmittedDeclarationProgressionBindingTargetSource;
