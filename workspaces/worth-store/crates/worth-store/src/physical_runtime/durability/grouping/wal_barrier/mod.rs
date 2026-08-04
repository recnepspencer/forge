mod declaration;
mod outcome;
mod port;
mod settlement;

pub use declaration::{
    PhysicalWalGroupBarrierDeclaration, PhysicalWalGroupBarrierDeclarationDenial,
};
pub use outcome::{
    IndeterminatePhysicalWalGroupBarrier, PhysicalWalGroupBarrierFailureCause,
    PhysicalWalGroupBarrierOutcome,
};
pub use settlement::PhysicalWalGroupBarrierSettlement;

pub(in crate::physical_runtime) use port::PhysicalWalGroupBarrierPort;
pub(in crate::physical_runtime) use settlement::CompletionBoundPhysicalWalGroupBarrierSettlement;
