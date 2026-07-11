//! Store-owned transition facts emitted by ordinary layout operations.

mod compaction_projection;
mod declaration;
mod fact;
mod issued_outcome;
#[cfg(test)]
mod observation;
mod owner_case;
mod owner_family;

pub(crate) use fact::owner_transition;
pub use fact::{
    S8LayoutMachineEdge, S8LayoutMachineState, S8LayoutMachineTransition,
    S8LayoutProductionOperation, S8LayoutProductionTransition, S8LayoutStateMachine,
};
pub use owner_case::S8OwnerOutcomeCase;
pub use owner_family::{S8LayoutMachineContract, S8OwnerOutcomeFamilyContract};

pub(crate) use declaration::define_owner_outcome;
pub(crate) use issued_outcome::{S8OwnerIssuedCase, S8OwnerIssuedResult};
#[cfg(test)]
pub(crate) use observation::capture_issued_transitions;
pub(crate) use owner_family::S8OwnerTransitionContract;
