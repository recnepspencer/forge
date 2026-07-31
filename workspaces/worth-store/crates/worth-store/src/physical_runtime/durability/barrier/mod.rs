mod declaration;
mod outcome;
mod port;
mod settlement;

pub use declaration::PhysicalWalBarrierDeclaration;
pub use outcome::{
    PhysicalWalBarrierFailureCause, PhysicalWalBarrierOutcome,
    WalBarrierIndeterminatePhysicalMutation,
};
pub use settlement::PhysicalWalBarrierSettlement;

pub(in crate::physical_runtime) use port::PhysicalWalBarrierPort;
pub(in crate::physical_runtime) use settlement::CompletionBoundPhysicalWalBarrierSettlement;
