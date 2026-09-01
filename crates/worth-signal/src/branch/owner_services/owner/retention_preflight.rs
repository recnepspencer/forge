use crate::branch::retention::SignalBranchRetentionOwnerRelationship;
use crate::branch::{
    AdmittedSignalBranchBasis, SignalBranchRetentionAcquisitionDenial, SignalOwnerUnavailable,
};
use crate::state::SignalSnapshotId;

use super::super::{SignalBranchRegistryDenial, SignalOwnerOperationAdmission};

impl<D, I, T> super::SignalOwner<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn preflight_exact_external_retention(
        &self,
        admission: &SignalOwnerOperationAdmission<'_>,
        basis: &AdmittedSignalBranchBasis,
    ) -> Result<(), SignalBranchRetentionAcquisitionDenial> {
        admission
            .authorize(self.runtime_instance_id, self.lifecycle_identity())
            .map_err(|_| {
                SignalBranchRetentionAcquisitionDenial::OwnerUnavailable(SignalOwnerUnavailable)
            })?;
        if basis.owner_identity_relationship(&self.retention.binding())
            != SignalBranchRetentionOwnerRelationship::SameOwner
        {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        }
        let Some(target) = basis.observation().target().as_basis() else {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        };
        if target.graph_instance_id() != self.runtime_instance_id.to_string() {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        }
        if target.definition_basis() != self.definition_basis {
            return Err(SignalBranchRetentionAcquisitionDenial::DefinitionMismatch {
                basis_definition_basis: target.definition_basis(),
                runtime_definition_basis: self.definition_basis,
            });
        }

        let branch_id = basis.owner_branch_id();
        let cell = match self.registry.lookup(admission, branch_id) {
            Ok(cell) => cell,
            Err(SignalBranchRegistryDenial::UnknownBranch(_)) => {
                return if self
                    .metadata
                    .retirement_receipt(admission, branch_id)
                    .map_err(map_metadata_preflight_denial)?
                    .is_some()
                {
                    Err(SignalBranchRetentionAcquisitionDenial::RetiredBranch { branch_id })
                } else {
                    Err(SignalBranchRetentionAcquisitionDenial::UnknownBranch { branch_id })
                };
            }
            Err(SignalBranchRegistryDenial::RetirementInProgress(_)) => {
                return Err(SignalBranchRetentionAcquisitionDenial::RetiredBranch { branch_id });
            }
            Err(SignalBranchRegistryDenial::ForeignOwner)
            | Err(SignalBranchRegistryDenial::ExpiredAdmission) => {
                return Err(SignalBranchRetentionAcquisitionDenial::OwnerUnavailable(
                    SignalOwnerUnavailable,
                ));
            }
            Err(SignalBranchRegistryDenial::OwnerReentry)
            | Err(SignalBranchRegistryDenial::OwnerMetadataOrdering) => {
                return Err(SignalBranchRetentionAcquisitionDenial::OwnerReentry);
            }
            Err(SignalBranchRegistryDenial::DuplicateBranch(_))
            | Err(SignalBranchRegistryDenial::LiveCapacityExhausted { .. })
            | Err(SignalBranchRegistryDenial::ReservationCapacityExhausted { .. })
            | Err(SignalBranchRegistryDenial::ExpiredRetirement(_))
            | Err(SignalBranchRegistryDenial::TargetCellDenied(_)) => {
                return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
            }
        };

        let current = cell.observe_exact(admission).map_err(|denial| {
            match map_basis_preflight_denial(denial, branch_id) {
                Some(denial) => denial,
                None => SignalBranchRetentionAcquisitionDenial::ForeignBasis,
            }
        })?;
        if current.branch_id() != basis.observation().branch_id() {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        }
        let Some(snapshot_id) = target.snapshot_id().map(SignalSnapshotId) else {
            return Ok(());
        };
        let current_snapshot_id = current
            .target()
            .as_basis()
            .and_then(|target| target.snapshot_id())
            .map(SignalSnapshotId);
        if current_snapshot_id == Some(snapshot_id)
            || self
                .metadata
                .has_snapshot_state(admission, branch_id, snapshot_id)
                .map_err(map_metadata_preflight_denial)?
        {
            return Ok(());
        }
        Err(SignalBranchRetentionAcquisitionDenial::UnavailableTarget {
            branch_id,
            snapshot_id,
        })
    }
}

fn map_metadata_preflight_denial(
    denial: super::super::owner_metadata::SignalOwnerMetadataAuthorizationDenial,
) -> SignalBranchRetentionAcquisitionDenial {
    match denial {
        super::super::owner_metadata::SignalOwnerMetadataAuthorizationDenial::OwnerUnavailable => {
            SignalBranchRetentionAcquisitionDenial::OwnerUnavailable(SignalOwnerUnavailable)
        }
        super::super::owner_metadata::SignalOwnerMetadataAuthorizationDenial::OwnerCellMisuse
        | super::super::owner_metadata::SignalOwnerMetadataAuthorizationDenial::OwnerReentry => {
            SignalBranchRetentionAcquisitionDenial::OwnerReentry
        }
    }
}

fn map_basis_preflight_denial(
    denial: crate::branch::SignalBranchBasisObservationDenial,
    branch_id: crate::state::SignalBranchId,
) -> Option<SignalBranchRetentionAcquisitionDenial> {
    use crate::branch::SignalBranchBasisObservationDenial;

    match denial {
        SignalBranchBasisObservationDenial::OwnerUnavailable(_) => Some(
            SignalBranchRetentionAcquisitionDenial::OwnerUnavailable(SignalOwnerUnavailable),
        ),
        SignalBranchBasisObservationDenial::OperationCapacityExhausted {
            maximum_in_flight_operations,
        } => Some(
            SignalBranchRetentionAcquisitionDenial::OperationCapacityExhausted {
                maximum_in_flight_operations,
            },
        ),
        SignalBranchBasisObservationDenial::OwnerReentry
        | SignalBranchBasisObservationDenial::OwnerCellMisuse { .. } => {
            Some(SignalBranchRetentionAcquisitionDenial::OwnerReentry)
        }
        SignalBranchBasisObservationDenial::ManagedReferenceDenied { .. }
        | SignalBranchBasisObservationDenial::UnknownBranch { .. }
        | SignalBranchBasisObservationDenial::OwnerInvariantViolation { .. }
        | SignalBranchBasisObservationDenial::RetentionUnavailable { .. } => None,
        SignalBranchBasisObservationDenial::RetirementInProgress { .. }
        | SignalBranchBasisObservationDenial::RetiredBranch { .. } => {
            Some(SignalBranchRetentionAcquisitionDenial::RetiredBranch { branch_id })
        }
        SignalBranchBasisObservationDenial::QuarantinedBranch { .. }
        | SignalBranchBasisObservationDenial::InvalidOwnerObservation { .. } => None,
    }
}
