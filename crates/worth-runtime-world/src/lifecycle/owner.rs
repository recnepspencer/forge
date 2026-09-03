use std::sync::{Arc, Mutex};

use worth_relational::facade::branch::RelationalOwnerServicePorts;
use worth_runtime_bridge::facade::RuntimeWorldCorrespondencePort;
use worth_signal::facade::branch::SignalOwnerServicePorts;

use crate::branch::registry::ProductBranchRegistry;
use crate::budget::RuntimeWorldBudgets;
use crate::history::CompositeHistoryCatalog;
use crate::identity::{
    RuntimeWorldIdentityExhaustion, RuntimeWorldIdentityIssuer, RuntimeWorldOwnerIdentity,
};
use crate::recovery::RecoveryCatalog;
use crate::retention::RuntimeWorldRetentionOwner;

use super::clock::RuntimeWorldClock;
use super::close::RuntimeWorldCloseContract;
use super::owner_inputs::RuntimeWorldOwnerInputs;

mod bootstrap;
mod construction;
mod operation;

pub(crate) use construction::{
    RuntimeWorldOwnerConstructionCapability, RuntimeWorldOwnerConstructionContract,
};
pub(crate) use operation::{
    ReservedPublicationAttemptCapacity, RuntimeWorldOperationLedger,
    RuntimeWorldOperationReservation, RuntimeWorldOperationState,
    RuntimeWorldPublicationCapacityLedger,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeWorldBootstrapState {
    Unperformed,
    InProgress,
    Performed,
}

/// The sole managed Runtime World owner. All identity, budget, component,
/// history, retention, branch, recovery, and lifecycle state is rooted here.
pub struct RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(super) state: Arc<RuntimeWorldOwnerState<D, I, E, Ctx, T>>,
}

impl<D, I, E, Ctx, T> std::fmt::Debug for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeWorldOwnerRoot")
            .field("owner_identity", &self.owner_identity())
            .finish_non_exhaustive()
    }
}

pub(super) struct RuntimeWorldOwnerState<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(super) owner_identity: RuntimeWorldOwnerIdentity,
    pub(super) identities: Mutex<RuntimeWorldIdentityIssuer>,
    pub(super) relational: RelationalOwnerServicePorts,
    pub(super) signal: SignalOwnerServicePorts<D, I, E, Ctx, T>,
    pub(super) bridge: RuntimeWorldCorrespondencePort,
    pub(super) budgets: RuntimeWorldBudgets,
    pub(super) clock: RuntimeWorldClock,
    pub(super) history: CompositeHistoryCatalog,
    pub(super) retention: RuntimeWorldRetentionOwner<D, I, T>,
    pub(super) branches: ProductBranchRegistry,
    pub(super) recovery: RecoveryCatalog,
    pub(super) bootstrap: Mutex<RuntimeWorldBootstrapState>,
    pub(super) close: Mutex<RuntimeWorldCloseContract>,
    pub(super) operation: RuntimeWorldOperationLedger,
    pub(super) publication_capacity: RuntimeWorldPublicationCapacityLedger,
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub fn new(
        inputs: RuntimeWorldOwnerInputs<D, I, E, Ctx, T>,
    ) -> Result<Self, RuntimeWorldIdentityExhaustion> {
        let (relational, signal, bridge, budgets, clock) = inputs.into_parts();
        let construction = RuntimeWorldOwnerConstructionContract::new()?;
        let owner_identity = construction.owner_identity();
        let identities = construction.into_issuer();
        let history = CompositeHistoryCatalog::new(
            owner_identity,
            crate::history::RuntimeWorldHistoryCatalogContract::installed(
                budgets.retained_composite_commits(),
                budgets.history_metadata_bytes(),
            ),
        );
        let retention = RuntimeWorldRetentionOwner::from_component_services(
            owner_identity,
            &relational,
            &signal,
            budgets.unique_exact_component_pins(),
            budgets.in_flight_pin_acquisition_reservations(),
        );
        let branches = ProductBranchRegistry::new(owner_identity, budgets.live_product_branches());
        let recovery = RecoveryCatalog::new(
            owner_identity,
            budgets.retained_product_unpublished_records(),
        );
        let publication_capacity =
            RuntimeWorldPublicationCapacityLedger::new(budgets.active_publication_attempts());
        Ok(Self {
            state: Arc::new(RuntimeWorldOwnerState {
                owner_identity,
                identities: Mutex::new(identities),
                relational,
                signal,
                bridge,
                budgets,
                clock,
                history,
                retention,
                branches,
                recovery,
                bootstrap: Mutex::new(RuntimeWorldBootstrapState::Unperformed),
                close: Mutex::new(RuntimeWorldCloseContract::open()),
                operation: RuntimeWorldOperationLedger::new(),
                publication_capacity,
            }),
        })
    }

    pub fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.state.owner_identity
    }
}

#[cfg(test)]
#[path = "owner_tests.rs"]
mod tests;
