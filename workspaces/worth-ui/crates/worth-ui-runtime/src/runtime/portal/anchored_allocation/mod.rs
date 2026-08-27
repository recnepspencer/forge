mod activation_binding_denial;
mod admitted_contract;
mod anchor_identity;
mod identity_transition;
mod observation;
mod planning_authority;
mod successor;
mod successor_denial;

pub use activation_binding_denial::UiPortalActivationBindingDenial;
pub(crate) use admitted_contract::UiAdmittedPortalAnchorContract;
pub use anchor_identity::UiPortalAnchorIdentity;
pub use identity_transition::UiPortalAnchorIdentityTransition;
pub use observation::UiAdmittedPortalAnchorObservation;
pub(crate) use planning_authority::UiAdmittedPortalPlanningAuthority;
pub use successor::UiPortalAllocationPlanningBasis;
pub use successor_denial::UiPortalAnchorSuccessorDenial;

#[cfg(test)]
mod tests;
