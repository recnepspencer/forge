pub(crate) mod diagnostics;
pub(crate) mod facade;
pub(crate) mod packets;
pub(crate) mod planning;
pub(crate) mod proofs;
pub(crate) mod reduction;

pub(crate) use facade::{PreparationWorkPlan, PreparedInvariantExecution};
pub(crate) use packets::invariant::InvariantWorkPacket;
pub(crate) use planning::context::PreparationPlanningContext;
pub(crate) use planning::strategy::{
    ParallelLegality, ParallelProfitability, PreparationFallbackReason, PreparationStrategy,
    PreparationStrategySelection,
};
pub(crate) use proofs::kinds::PreparationProofKind;
pub(crate) use reduction::identity::ValidationResultIdentity;
pub(crate) use reduction::keys::ValidationReductionKey;
