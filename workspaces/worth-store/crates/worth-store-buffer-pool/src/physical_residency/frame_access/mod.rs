mod bounded_fault_owner;
mod bounded_fault_waiter;
mod fault_owner;
mod fault_waiter;
mod loading_identity;
mod terminal;

pub use fault_owner::{PhysicalFrameFaultError, PhysicalFrameFaultOwner};
pub use fault_waiter::PhysicalFrameFaultWaiter;
pub use loading_identity::PhysicalFrameLoadingIdentity;
pub use terminal::{PhysicalFrameLoadTerminal, PhysicalFrameLoadTerminalKind};

use super::PhysicalFrameLease;

/// The exhaustive result of consulting the exact live pool identity.
///
/// Only `Fault` carries authority to allocate and execute a source load.
#[derive(Debug)]
pub enum PhysicalFrameAccess {
    Hit(PhysicalFrameLease),
    Fault(PhysicalFrameFaultOwner),
    Coalesced(PhysicalFrameFaultWaiter),
}

/// The exhaustive result of consulting one bounded artifact identity.
#[derive(Debug)]
pub enum PhysicalBoundedFrameAccess {
    Hit(PhysicalFrameLease),
    Fault(PhysicalBoundedFrameFaultOwner),
    Coalesced(PhysicalBoundedFrameFaultWaiter),
}
pub use bounded_fault_owner::PhysicalBoundedFrameFaultOwner;
pub use bounded_fault_waiter::PhysicalBoundedFrameFaultWaiter;
