use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::branch::{
    AdmittedSignalBranchBasis, SignalBranchForkOperationDenial, SignalBranchObservation,
    ValidatedSignalBranchName,
};
use crate::data::error::SignalError;
use crate::logic::transaction::BranchState;
use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId};

use super::super::branch_execution_cell::SignalBranchForkSourceCustody;
use super::super::branch_registry::{
    SignalBranchRegistryDenial, SignalBranchReservation, SignalInstalledBranchCell,
    SignalPreparedBranchCell, SignalPreparedBranchInstallation,
};
use super::super::operation_control::SignalOwnerOperationBoundary;
use super::super::owner_metadata::SignalOwnerForkLineageReservation;
use super::super::{
    SignalBranchCellState, SignalBranchExecutionCell, SignalOwnerCancellationToken,
    SignalOwnerOperationAdmission, SignalOwnerUnavailable,
};

impl<D, I, T> super::SignalOwner<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::branch::owner_services) fn reserve_fork_destination<'a>(
        &'a self,
        admission: &'a SignalOwnerOperationAdmission<'_>,
        source: &AdmittedSignalBranchBasis,
        requested_identity: ValidatedSignalBranchName,
    ) -> Result<SignalOwnerForkReservation<'a, D, I, T>, SignalBranchForkOperationDenial> {
        admission
            .authorize(self.runtime_instance_id, self.lifecycle_identity())
            .map_err(|_| {
                SignalBranchForkOperationDenial::OwnerUnavailable(SignalOwnerUnavailable)
            })?;
        admission.owner_lock_acquisition().map_err(|denial| match denial {
            crate::branch::owner_services::lifecycle_state::SignalOwnerMetadataHoldDenial::BranchCellOrMetadataAlreadyHeld => {
                SignalBranchForkOperationDenial::OwnerCellMisuse {
                    branch_id: source.owner_branch_id(),
                }
            }
            crate::branch::owner_services::lifecycle_state::SignalOwnerMetadataHoldDenial::ExecutingThreadReentry => {
                SignalBranchForkOperationDenial::OwnerReentry
            }
        })?;
        let branch_id = self
            .next_branch_id
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                current.checked_add(1)
            })
            .map(SignalBranchId)
            .map_err(|_| SignalBranchForkOperationDenial::BranchIdentityExhausted)?;
        let parent_head_snapshot_id = source
            .observation()
            .target()
            .as_basis()
            .and_then(|target| target.snapshot_id())
            .map(SignalSnapshotId);
        let handle = SignalBranchHandle {
            id: branch_id,
            name: requested_identity.into_inner(),
            parent_branch_id: Some(source.owner_branch_id()),
            head_snapshot_id: parent_head_snapshot_id,
        };
        let reservation = self
            .registry
            .reserve(admission, branch_id)
            .map_err(|denial| map_fork_registry_denial(denial, source.owner_branch_id()))?;
        let lineage =
            self.metadata
                .reserve_fork_child(admission, source.owner_branch_id(), branch_id)?;
        Ok(SignalOwnerForkReservation::new(
            handle,
            self.runtime_instance_id,
            self.definition_basis,
            reservation,
            lineage,
        ))
    }
}

pub(in crate::branch::owner_services) struct SignalOwnerForkReservation<'a, D, I, T>
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

    pub(in crate::branch::owner_services) fn branch(&self) -> &SignalBranchHandle {
        &self.handle
    }

    pub(in crate::branch::owner_services) fn install(
        self,
        source_custody: &SignalBranchForkSourceCustody<'_, '_, SignalBranchCellState<D, I, T>>,
        source: &AdmittedSignalBranchBasis,
        cancellation: &SignalOwnerCancellationToken,
    ) -> Result<SignalInstalledOwnerFork<'a, D, I, T>, SignalBranchForkOperationDenial> {
        let Some(parent_branch_id) = self.handle.parent_branch_id else {
            return Err(owner_fork_invariant(
                "fork destination reservation has no source branch",
            ));
        };
        if source_custody.cell().branch_id() != parent_branch_id
            || source.owner_branch_id() != parent_branch_id
        {
            return Err(owner_fork_invariant(
                "fork source does not match its owner-issued destination reservation",
            ));
        }
        let builder = SignalOwnerForkCellBuilder {
            handle: self.handle,
            owner_runtime_instance_id: self.owner_runtime_instance_id,
            definition_basis: self.definition_basis,
            reservation: self.reservation,
            lineage: self.lineage,
            destination_observation: None,
        };
        source_custody
            .cell()
            .capture_fork_source_exact(source_custody, source, builder, cancellation)?
            .install()
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
    destination_observation: Option<SignalBranchObservation>,
}

pub(crate) struct SignalPreparedOwnerFork<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    handle: SignalBranchHandle,
    admission: &'a SignalOwnerOperationAdmission<'a>,
    installation: SignalPreparedBranchInstallation<'a, SignalBranchCellState<D, I, T>>,
    lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
    destination_observation: SignalBranchObservation,
}

pub(in crate::branch::owner_services) struct SignalInstalledOwnerFork<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    handle: SignalBranchHandle,
    observation: SignalBranchObservation,
    installation: SignalInstalledBranchCell<'a, SignalBranchCellState<D, I, T>>,
    lineage: SignalOwnerForkLineageReservation<'a, D, I, T>,
}

impl<'a, D, I, T> SignalInstalledOwnerFork<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(in crate::branch::owner_services) fn cell(
        &self,
    ) -> &Arc<SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>> {
        self.installation.cell()
    }

    pub(super) fn into_handoff_parts(
        self,
    ) -> (
        SignalBranchHandle,
        SignalBranchObservation,
        SignalInstalledBranchCell<'a, SignalBranchCellState<D, I, T>>,
        SignalOwnerForkLineageReservation<'a, D, I, T>,
    ) {
        (
            self.handle,
            self.observation,
            self.installation,
            self.lineage,
        )
    }
}

impl<D, I, T> std::ops::Deref for SignalInstalledOwnerFork<'_, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    type Target = SignalBranchExecutionCell<SignalBranchCellState<D, I, T>>;

    fn deref(&self) -> &Self::Target {
        self.installation.cell()
    }
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

    pub(crate) fn prepare_cell(
        &mut self,
        state: BranchState<D, I, T>,
    ) -> Result<
        SignalPreparedBranchCell<SignalBranchCellState<D, I, T>>,
        SignalBranchForkOperationDenial,
    > {
        if state.branch_id() != self.handle.id {
            return Err(owner_fork_invariant(
                "fork destination state does not match its owner-issued reservation",
            ));
        }
        let destination_branch_id = self.handle.id;
        let destination_state = SignalBranchCellState::new(
            self.handle.clone(),
            self.owner_runtime_instance_id,
            self.definition_basis,
            state,
            0,
            None,
        );
        let destination_observation = destination_state
            .observation()
            .map_err(|error| SignalBranchForkOperationDenial::OwnerDeniedNoMovement { error })?;
        let prepared = self
            .reservation
            .prepare_fork_destination_cell(destination_state)
            .map_err(|denial| map_fork_registry_denial(denial, destination_branch_id))?;
        self.destination_observation = Some(destination_observation);
        Ok(prepared)
    }

    pub(crate) fn validate_prepared_cell(
        &self,
        prepared: &SignalPreparedBranchCell<SignalBranchCellState<D, I, T>>,
    ) -> Result<(), SignalBranchForkOperationDenial> {
        self.reservation
            .matches_prepared_fork_destination(prepared)
            .then_some(())
            .ok_or_else(|| {
                owner_fork_invariant(
                    "prepared fork destination does not match its exact owner reservation",
                )
            })
    }

    pub(crate) fn bind_prepared_cell(
        self,
        prepared: SignalPreparedBranchCell<SignalBranchCellState<D, I, T>>,
    ) -> SignalPreparedOwnerFork<'a, D, I, T> {
        let admission = self.reservation.admission();
        SignalPreparedOwnerFork {
            handle: self.handle,
            admission,
            installation: self.reservation.bind_prepared_fork_destination(prepared),
            lineage: self.lineage,
            destination_observation: self
                .destination_observation
                .expect("prepared fork destination must carry its sealed observation"),
        }
    }
}

impl<'a, D, I, T> SignalPreparedOwnerFork<'a, D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn install(
        self,
    ) -> Result<SignalInstalledOwnerFork<'a, D, I, T>, SignalBranchForkOperationDenial> {
        let SignalPreparedOwnerFork {
            handle,
            admission,
            installation,
            lineage,
            destination_observation,
        } = self;
        let branch_id = handle.id;
        let installation = installation
            .install_recoverable()
            .map_err(|denial| map_fork_registry_denial(denial, branch_id))?;
        admission
            .reach_operation_boundary(SignalOwnerOperationBoundary::ForkDestinationInstallation);
        Ok(SignalInstalledOwnerFork {
            handle,
            observation: destination_observation,
            installation,
            lineage,
        })
    }
}

pub(super) fn map_fork_registry_denial(
    denial: SignalBranchRegistryDenial,
    branch_id: crate::state::SignalBranchId,
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
        SignalBranchRegistryDenial::OwnerReentry => SignalBranchForkOperationDenial::OwnerReentry,
        SignalBranchRegistryDenial::OwnerMetadataOrdering => {
            SignalBranchForkOperationDenial::OwnerCellMisuse { branch_id }
        }
        denial => SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
            error: SignalError::internal(format!(
                "Signal owner fork reservation invariant failed: {denial:?}"
            )),
        },
    }
}

fn owner_fork_invariant(message: &str) -> SignalBranchForkOperationDenial {
    SignalBranchForkOperationDenial::OwnerDeniedNoMovement {
        error: SignalError::internal(message),
    }
}
