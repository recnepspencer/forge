use crate::{
    BackupMaterializationCompletion, ExecutedAuthorityAffectingRepair, ExecutedBackupRestore,
    ExecutedPointInTimeRecovery, ExecutedRepair, ExecutedReplicaBootstrap,
    ExecutedReplicaPromotion, ExecutedRollback, OperationalOperationId,
};

use super::{OperationalSessionIdentity, OperationalSessionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalCounterReceipt {
    session: OperationalSessionIdentity,
    kind: OperationalSessionKind,
    source_bytes_read: u64,
    output_bytes_written: u64,
    durable_protocol_transitions: u64,
    external_fence_grants: u64,
    retained_source_leases: u64,
    work_units: u64,
    maximum_resident_bytes: u64,
    authorization_consumptions: u64,
    owner_receipts: u64,
    forbidden_full_materializations: u64,
    foreign_work_units: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalCounterDenial {
    CounterOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalCounterStructureDenial {
    EmptyWork,
    MissingStreamingBreadth,
    MissingResidentBound,
    InvalidAuthorizationCount,
    InvalidFenceCount,
    InvalidLeaseCount,
    InvalidOwnerReceiptCount,
}

impl OperationalCounterReceipt {
    #[cfg(test)]
    pub(super) fn empty_for_test(kind: OperationalSessionKind) -> Self {
        Self {
            session: OperationalSessionIdentity::from_operation(
                &OperationalOperationId::new("counter-test").expect("valid test operation"),
            ),
            kind,
            source_bytes_read: 0,
            output_bytes_written: 0,
            durable_protocol_transitions: 0,
            external_fence_grants: 0,
            retained_source_leases: 0,
            work_units: 0,
            maximum_resident_bytes: 0,
            authorization_consumptions: 0,
            owner_receipts: 0,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    #[cfg(test)]
    pub(super) fn set_test_structure(
        &mut self,
        source_bytes_read: u64,
        work_units: u64,
        maximum_resident_bytes: u64,
        authorization_consumptions: u64,
        external_fence_grants: u64,
        owner_receipts: u64,
    ) {
        self.source_bytes_read = source_bytes_read;
        self.work_units = work_units;
        self.maximum_resident_bytes = maximum_resident_bytes;
        self.authorization_consumptions = authorization_consumptions;
        self.external_fence_grants = external_fence_grants;
        self.owner_receipts = owner_receipts;
    }

    pub fn from_replica_bootstrap(executed: &ExecutedReplicaBootstrap) -> Self {
        let counters = executed.receipt().execution_counters();
        Self {
            session: OperationalSessionIdentity::from_operation(executed.operation_id()),
            kind: OperationalSessionKind::ReplicaBootstrap,
            source_bytes_read: counters.source_bytes_read(),
            output_bytes_written: counters.output_bytes_written(),
            durable_protocol_transitions: 2,
            external_fence_grants: 0,
            retained_source_leases: 1,
            work_units: counters.backend_requests(),
            maximum_resident_bytes: counters.maximum_resident_buffer_bytes(),
            authorization_consumptions: 1,
            owner_receipts: 1,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub fn from_replica_promotion(executed: &ExecutedReplicaPromotion) -> Self {
        Self {
            session: OperationalSessionIdentity::from_operation(executed.operation_id()),
            kind: OperationalSessionKind::ReplicaPromotion,
            source_bytes_read: 0,
            output_bytes_written: 0,
            durable_protocol_transitions: 3,
            external_fence_grants: 1,
            retained_source_leases: 0,
            work_units: 1,
            maximum_resident_bytes: 0,
            authorization_consumptions: 1,
            owner_receipts: 2,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub fn from_forensic_acquisition(
        operation: &OperationalOperationId,
        counters: worth_store_offline_verifier::ForensicAcquisitionCounters,
    ) -> Self {
        Self {
            session: OperationalSessionIdentity::from_operation(operation),
            kind: OperationalSessionKind::ForensicAcquisition,
            source_bytes_read: counters.source_bytes_read(),
            output_bytes_written: counters.output_bytes_written(),
            durable_protocol_transitions: counters.source_files(),
            external_fence_grants: 0,
            retained_source_leases: 0,
            work_units: counters.source_files(),
            maximum_resident_bytes: counters.maximum_resident_buffer_bytes(),
            authorization_consumptions: 0,
            owner_receipts: counters.source_files(),
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub fn from_backup_materialization(
        completed: &BackupMaterializationCompletion,
    ) -> Result<Self, OperationalCounterDenial> {
        let counters = completed.counters();
        Ok(Self {
            session: OperationalSessionIdentity::from_operation(completed.operation_id()),
            kind: OperationalSessionKind::Backup,
            source_bytes_read: counters.source_bytes_read(),
            output_bytes_written: counters
                .total_output_bytes_written()
                .ok_or(OperationalCounterDenial::CounterOverflow)?,
            durable_protocol_transitions: 2,
            external_fence_grants: 0,
            retained_source_leases: 1,
            work_units: 1,
            maximum_resident_bytes: counters.peak_buffer_bytes(),
            authorization_consumptions: 0,
            owner_receipts: 1,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        })
    }

    pub fn from_backup_restore(executed: &ExecutedBackupRestore) -> Self {
        Self::from_staging_receipt(
            executed.operation_id(),
            OperationalSessionKind::Restore,
            executed.receipt().backend(),
            false,
        )
    }

    pub fn from_point_in_time_recovery(executed: &ExecutedPointInTimeRecovery) -> Self {
        Self::from_staging_receipt(
            executed.operation_id(),
            OperationalSessionKind::PointInTimeRecovery,
            executed.receipt().backend(),
            true,
        )
    }

    pub fn from_rollback(executed: &ExecutedRollback) -> Self {
        Self::from_staging_receipt(
            executed.operation_id(),
            OperationalSessionKind::Rollback,
            executed.receipt().backend(),
            true,
        )
    }

    pub fn from_repair(executed: &ExecutedRepair) -> Self {
        let owner_receipts = executed.owner_receipts().receipts().len() as u64;
        Self {
            session: OperationalSessionIdentity::from_operation(executed.operation_id()),
            kind: OperationalSessionKind::Repair,
            source_bytes_read: 0,
            output_bytes_written: 0,
            durable_protocol_transitions: owner_receipts.saturating_mul(2).saturating_add(3),
            external_fence_grants: 0,
            retained_source_leases: 0,
            work_units: owner_receipts,
            maximum_resident_bytes: 0,
            authorization_consumptions: 1,
            owner_receipts,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub fn from_authority_affecting_repair(executed: &ExecutedAuthorityAffectingRepair) -> Self {
        let owner_receipts =
            2 + u64::from(executed.layout().is_some()) + u64::from(executed.blob().is_some());
        Self {
            session: OperationalSessionIdentity::from_operation(executed.operation_id()),
            kind: OperationalSessionKind::Repair,
            source_bytes_read: executed.backend().bytes_copied(),
            output_bytes_written: executed.backend().bytes_copied(),
            durable_protocol_transitions: owner_receipts.saturating_mul(2).saturating_add(3),
            external_fence_grants: 0,
            retained_source_leases: 0,
            work_units: owner_receipts,
            maximum_resident_bytes: executed.backend().maximum_resident_buffer_bytes(),
            authorization_consumptions: 1,
            owner_receipts,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub fn from_offline_verification(
        operation: &OperationalOperationId,
        walked: &worth_store_offline_verifier::StructurallyWalkedMedia,
    ) -> Self {
        let counters = walked.counters();
        Self {
            session: OperationalSessionIdentity::from_operation(operation),
            kind: OperationalSessionKind::OfflineVerification,
            source_bytes_read: counters.bytes_read(),
            output_bytes_written: 0,
            durable_protocol_transitions: 0,
            external_fence_grants: 0,
            retained_source_leases: 0,
            work_units: counters.file_touches(),
            maximum_resident_bytes: counters
                .peak_buffer_bytes()
                .max(counters.peak_owned_allocation_bytes()),
            authorization_consumptions: 0,
            owner_receipts: 0,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    fn from_staging_receipt(
        operation: &OperationalOperationId,
        kind: OperationalSessionKind,
        receipt: &worth_store_physical_backend::NonCurrentStagingExecutionReceipt,
        retains_source_lease: bool,
    ) -> Self {
        Self {
            session: OperationalSessionIdentity::from_operation(operation),
            kind,
            source_bytes_read: receipt.bytes_copied(),
            output_bytes_written: receipt.bytes_copied(),
            durable_protocol_transitions: 4,
            external_fence_grants: 0,
            retained_source_leases: u64::from(retains_source_lease),
            work_units: receipt.artifacts_materialized(),
            maximum_resident_bytes: receipt.maximum_resident_buffer_bytes(),
            authorization_consumptions: 1,
            owner_receipts: 2,
            forbidden_full_materializations: 0,
            foreign_work_units: 0,
        }
    }

    pub const fn session(self) -> OperationalSessionIdentity {
        self.session
    }
    pub const fn kind(self) -> OperationalSessionKind {
        self.kind
    }
    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }
    pub const fn output_bytes_written(self) -> u64 {
        self.output_bytes_written
    }
    pub const fn durable_protocol_transitions(self) -> u64 {
        self.durable_protocol_transitions
    }
    pub const fn external_fence_grants(self) -> u64 {
        self.external_fence_grants
    }
    pub const fn retained_source_leases(self) -> u64 {
        self.retained_source_leases
    }
    pub const fn work_units(self) -> u64 {
        self.work_units
    }
    pub const fn maximum_resident_bytes(self) -> u64 {
        self.maximum_resident_bytes
    }
    pub const fn authorization_consumptions(self) -> u64 {
        self.authorization_consumptions
    }
    pub const fn owner_receipts(self) -> u64 {
        self.owner_receipts
    }
    pub const fn forbidden_full_materializations(self) -> u64 {
        self.forbidden_full_materializations
    }
    pub const fn foreign_work_units(self) -> u64 {
        self.foreign_work_units
    }

    pub fn validate_structure(self) -> Result<(), OperationalCounterStructureDenial> {
        if self.work_units == 0 {
            return Err(OperationalCounterStructureDenial::EmptyWork);
        }
        self.require_streaming_breadth()?;
        self.require_authority_counts()?;
        self.require_owner_receipts()?;
        Ok(())
    }

    fn require_streaming_breadth(self) -> Result<(), OperationalCounterStructureDenial> {
        let reconstructive = matches!(
            self.kind,
            OperationalSessionKind::Backup
                | OperationalSessionKind::Restore
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::ReplicaBootstrap
                | OperationalSessionKind::ForensicAcquisition
        );
        let inspecting = self.kind == OperationalSessionKind::OfflineVerification;
        if (reconstructive || inspecting) && self.source_bytes_read == 0 {
            return Err(OperationalCounterStructureDenial::MissingStreamingBreadth);
        }
        if reconstructive && self.output_bytes_written == 0 {
            return Err(OperationalCounterStructureDenial::MissingStreamingBreadth);
        }
        if (reconstructive || inspecting) && self.maximum_resident_bytes == 0 {
            return Err(OperationalCounterStructureDenial::MissingResidentBound);
        }
        Ok(())
    }

    fn require_authority_counts(self) -> Result<(), OperationalCounterStructureDenial> {
        let expected_authorizations = u64::from(matches!(
            self.kind,
            OperationalSessionKind::Restore
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::Repair
                | OperationalSessionKind::ReplicaBootstrap
                | OperationalSessionKind::ReplicaPromotion
        ));
        if self.authorization_consumptions != expected_authorizations {
            return Err(OperationalCounterStructureDenial::InvalidAuthorizationCount);
        }
        let expected_fences = u64::from(self.kind == OperationalSessionKind::ReplicaPromotion);
        if self.external_fence_grants != expected_fences {
            return Err(OperationalCounterStructureDenial::InvalidFenceCount);
        }
        let expected_leases = u64::from(matches!(
            self.kind,
            OperationalSessionKind::Backup
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::ReplicaBootstrap
        ));
        if self.retained_source_leases != expected_leases {
            return Err(OperationalCounterStructureDenial::InvalidLeaseCount);
        }
        Ok(())
    }

    fn require_owner_receipts(self) -> Result<(), OperationalCounterStructureDenial> {
        let valid = match self.kind {
            OperationalSessionKind::OfflineVerification => self.owner_receipts == 0,
            OperationalSessionKind::Restore
            | OperationalSessionKind::PointInTimeRecovery
            | OperationalSessionKind::Rollback
            | OperationalSessionKind::ReplicaPromotion => self.owner_receipts == 2,
            OperationalSessionKind::Backup | OperationalSessionKind::ReplicaBootstrap => {
                self.owner_receipts == 1
            }
            OperationalSessionKind::Repair | OperationalSessionKind::ForensicAcquisition => {
                self.owner_receipts > 0
            }
        };
        if !valid {
            return Err(OperationalCounterStructureDenial::InvalidOwnerReceiptCount);
        }
        Ok(())
    }
}
