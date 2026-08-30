use std::marker::PhantomData;
use std::sync::{Arc, Weak};

use crate::branch::SignalBranchForkOperationDenial;
use crate::data::error::SignalError;
use crate::logic::transaction::BranchState;
use crate::state::SignalBranchHandle;

use super::branch_registry::{
    SignalBranchRegistryDenial, SignalBranchReservation, SignalPreparedBranchInstallation,
};
use super::owner_metadata::SignalOwnerForkLineageReservation;
use super::{
    SignalBranchCellState, SignalBranchExecutionCell, SignalOwner, SignalOwnerCancellationToken,
    SignalOwnerUnavailable,
};

/// Package-private Phase 3 slot for the concrete weak mutation service.
pub struct SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    owner: Weak<SignalOwner<D, I, T>>,
    diagnostic_owner_runtime_instance_id: u64,
    type_contract: PhantomData<fn(E, Ctx)>,
}

/// Owner-issued fork capacity held without retaining the registry lock.
pub(super) struct SignalOwnerForkReservation<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    handle: SignalBranchHandle,
    owner_runtime_instance_id: u64,
    definition_basis: u64,
    reservation: SignalBranchReservation<'a, SignalBranchCellState<D, I, T>>,
    lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
}

impl<'a, D, I, T> SignalOwnerForkReservation<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn new(
        handle: SignalBranchHandle,
        owner_runtime_instance_id: u64,
        definition_basis: u64,
        reservation: SignalBranchReservation<'a, SignalBranchCellState<D, I, T>>,
        lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
    ) -> Self {
        Self {
            handle,
            owner_runtime_instance_id,
            definition_basis,
            reservation,
            lineage,
        }
    }

    pub(super) fn branch(&self) -> &SignalBranchHandle {
        &self.handle
    }

    #[allow(
        dead_code,
        reason = "Phase 4 installs the captured canonical fork state"
    )]
    pub(super) fn install(
        self,
        source_cell: &SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>,
        admission: &super::SignalOwnerOperationAdmission,
        source: &crate::branch::AdmittedSignalBranchBasis,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<
        Arc<SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>>,
        SignalBranchForkOperationDenial,
    > {
        let Some(parent_branch_id) = self.handle.parent_branch_id else {
            return Err(SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
                error: SignalError::internal("fork destination reservation has no source branch"),
            });
        };
        if source_cell.branch_id() != parent_branch_id
            || source.owner_branch_id() != parent_branch_id
        {
            return Err(SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
                error: SignalError::internal(
                    "fork source does not match its owner-issued destination reservation",
                ),
            });
        }
        let builder = SignalOwnerForkCellBuilder {
            handle: self.handle,
            owner_runtime_instance_id: self.owner_runtime_instance_id,
            definition_basis: self.definition_basis,
            reservation: self.reservation,
            lineage: self.lineage,
        };
        let prepared =
            source_cell.capture_fork_source_exact(admission, source, builder, cancellation)?;
        Ok(prepared.install())
    }
}

pub(crate) struct SignalOwnerForkCellBuilder<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    handle: SignalBranchHandle,
    owner_runtime_instance_id: u64,
    definition_basis: u64,
    reservation: SignalBranchReservation<'a, SignalBranchCellState<D, I, T>>,
    lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
}

pub(crate) struct SignalPreparedOwnerFork<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    installation: SignalPreparedBranchInstallation<'a, SignalBranchCellState<D, I, T>>,
    lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
}

impl<'a, D, I, T> SignalOwnerForkCellBuilder<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn destination(&self) -> &SignalBranchHandle {
        &self.handle
    }

    pub(crate) fn prepare(
        self,
        state: BranchState<D, I, T>,
    ) -> Result<SignalPreparedOwnerFork<'a, D, I, T>, SignalBranchForkOperationDenial> {
        if state.branch_id() != self.handle.id {
            return Err(SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
                error: SignalError::internal(
                    "fork destination state does not match its owner-issued reservation",
                ),
            });
        }
        let installation = self
            .reservation
            .prepare_fork_destination(SignalBranchCellState::new(
                self.handle,
                self.owner_runtime_instance_id,
                self.definition_basis,
                state,
                0,
                None,
            ))
            .map_err(map_fork_registry_denial)?;
        Ok(SignalPreparedOwnerFork {
            installation,
            lineage: self.lineage,
        })
    }
}

impl<D, I, T> SignalPreparedOwnerFork<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn install(self) -> Arc<SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>> {
        let cell = self.installation.install();
        self.lineage.commit();
        cell
    }
}

pub(super) fn map_fork_registry_denial(
    denial: SignalBranchRegistryDenial,
) -> SignalBranchForkOperationDenial {
    match denial {
        SignalBranchRegistryDenial::ForeignOwner | SignalBranchRegistryDenial::ExpiredAdmission => {
            SignalBranchForkOperationDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        SignalBranchRegistryDenial::LiveCapacityExhausted {
            maximum_live_branches,
        } => SignalBranchForkOperationDenial::LiveBranchCapacityExhausted {
            maximum_live_branches,
        },
        SignalBranchRegistryDenial::ReservationCapacityExhausted {
            maximum_reservations,
        } => SignalBranchForkOperationDenial::ReservationCapacityExhausted {
            maximum_reservations,
        },
        denial => SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
            error: SignalError::internal(format!(
                "Signal owner fork reservation invariant failed: {denial:?}"
            )),
        },
    }
}

impl<D, I, E, Ctx, T> Clone for SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            diagnostic_owner_runtime_instance_id: self.diagnostic_owner_runtime_instance_id,
            type_contract: PhantomData,
        }
    }
}

impl<D, I, E, Ctx, T> SignalBranchMutationPort<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        owner: Weak<SignalOwner<D, I, T>>,
        diagnostic_owner_runtime_instance_id: u64,
    ) -> Self {
        Self {
            owner,
            diagnostic_owner_runtime_instance_id,
            type_contract: PhantomData,
        }
    }

    pub(crate) fn diagnostic_owner_runtime_instance_id(&self) -> u64 {
        self.diagnostic_owner_runtime_instance_id
    }

    pub(super) fn upgrade_owner(
        &self,
    ) -> Result<Arc<SignalOwner<D, I, T>>, SignalOwnerUnavailable> {
        SignalOwner::upgrade(&self.owner)
    }
}
