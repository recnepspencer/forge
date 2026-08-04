use worth_store_authority::SelectedControlStoreGeneration;
use worth_store_physical_backend::{ControlMediaFault, ControlMediaIdentity};
use worth_store_physical_isolation::{
    BackupReachabilityLeaseRegistry, BackupReachabilityLeaseRegistryDenial,
};

use super::selected_control_replay_contract::ReplayedSelectedControlHistory;
use super::{
    IndeterminateRecoveryStagingHandle, IndeterminateRepairRecoveryHandle,
    OperationalControlHistoryViolation, OperationalControlReplayResource, SelectedRecoveryHandles,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveBackupRecoveryHandle {
    operation_id: super::OperationalOperationId,
    recovery: worth_store_physical_isolation::BackupCutRecoveryRecord,
    materialization_plan: Option<super::BackupMaterializationRecoveryPlan>,
}

impl ActiveBackupRecoveryHandle {
    pub(crate) const fn new(
        operation_id: super::OperationalOperationId,
        recovery: worth_store_physical_isolation::BackupCutRecoveryRecord,
        materialization_plan: Option<super::BackupMaterializationRecoveryPlan>,
    ) -> Self {
        Self {
            operation_id,
            recovery,
            materialization_plan,
        }
    }

    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }

    pub const fn recovery(&self) -> &worth_store_physical_isolation::BackupCutRecoveryRecord {
        &self.recovery
    }

    pub const fn materialization_plan(&self) -> Option<&super::BackupMaterializationRecoveryPlan> {
        self.materialization_plan.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::OperationalOperationId,
        worth_store_physical_isolation::BackupCutRecoveryRecord,
        Option<super::BackupMaterializationRecoveryPlan>,
    ) {
        (self.operation_id, self.recovery, self.materialization_plan)
    }
}

#[derive(Debug)]
pub struct SelectedOperationalControlState {
    selected_generation: SelectedControlStoreGeneration,
    media_identity: ControlMediaIdentity,
    history_summary: OperationalControlHistorySummary,
    durable_records: Vec<super::OperationalControlRecord>,
    recovery_handles: Box<SelectedRecoveryHandles>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalControlHistorySummary {
    record_count: u64,
    completed_backups: u64,
    abandoned_backups: u64,
}

#[derive(Debug)]
pub enum ControlStoreAvailabilityDenial {
    Media(ControlMediaFault),
    FencingUnsupported,
    FencingUnavailable,
    ReplayBudgetExceeded {
        resource: OperationalControlReplayResource,
        required: u64,
        limit: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlStoreSelectionIndeterminate {
    SelectedMediaUnavailable {
        media_identity_fingerprint: [u8; 32],
    },
    SelectedGenerationNotReadable {
        selected: worth_store_authority::ControlStoreGeneration,
        observed: Option<worth_store_authority::ControlStoreGeneration>,
    },
    SelectedPrefixDigestMismatch {
        selected: [u8; 32],
        observed: [u8; 32],
    },
    SelectedMediaCopiesDivergent,
    SelectedAuthorityMismatch {
        selected: [u8; 32],
        observed: [u8; 32],
    },
    InvalidHistory(OperationalControlHistoryViolation),
}

impl SelectedOperationalControlState {
    pub(crate) fn new(
        selected_generation: SelectedControlStoreGeneration,
        media_identity: ControlMediaIdentity,
        history_summary: OperationalControlHistorySummary,
        durable_records: Vec<super::OperationalControlRecord>,
        replayed: ReplayedSelectedControlHistory,
    ) -> Self {
        Self {
            selected_generation,
            media_identity,
            history_summary,
            durable_records,
            recovery_handles: Box::new(SelectedRecoveryHandles {
                active_backups: replayed.active_backups,
                indeterminate_repairs: replayed.indeterminate_repairs,
                indeterminate_recovery_staging: replayed.indeterminate_recovery_staging,
                replica_bootstraps: replayed.replica_bootstraps,
                replica_promotions: replayed.replica_promotions,
            }),
        }
    }
    pub const fn selected_generation(&self) -> SelectedControlStoreGeneration {
        self.selected_generation
    }
    pub const fn media_identity(&self) -> ControlMediaIdentity {
        self.media_identity
    }
    pub const fn history_summary(&self) -> OperationalControlHistorySummary {
        self.history_summary
    }

    /// Canonical decoded records from the exact selected durable prefix.
    /// Audit, formal refinement, and certification consume this view rather
    /// than accepting a caller-reconstructed parallel history.
    pub fn durable_records(&self) -> &[super::OperationalControlRecord] {
        &self.durable_records
    }

    pub fn recover_backup_reachability_leases(
        &self,
    ) -> Result<BackupReachabilityLeaseRegistry, BackupReachabilityLeaseRegistryDenial> {
        BackupReachabilityLeaseRegistry::recover_from_persisted_results(
            self.recovery_handles.active_backups.iter().map(|handle| {
                let holder = super::backup_lease_holder_id(handle.operation_id());
                handle
                    .recovery
                    .lease_persistence_record()
                    .map(|lease| (holder, lease))
                    .map_err(|_| BackupReachabilityLeaseRegistryDenial::ConflictingCutIdentity)
            }),
        )
    }

    pub fn active_backup_recovery_handles(&self) -> &[ActiveBackupRecoveryHandle] {
        &self.recovery_handles.active_backups
    }

    pub fn into_active_backup_recovery_handles(self) -> Vec<ActiveBackupRecoveryHandle> {
        self.recovery_handles.active_backups
    }

    pub fn indeterminate_repair_recovery_handles(&self) -> &[IndeterminateRepairRecoveryHandle] {
        &self.recovery_handles.indeterminate_repairs
    }

    pub fn indeterminate_recovery_staging_handles(&self) -> &[IndeterminateRecoveryStagingHandle] {
        &self.recovery_handles.indeterminate_recovery_staging
    }

    pub fn replica_bootstrap_recovery_handles(&self) -> &[super::ReplicaBootstrapRecoveryHandle] {
        &self.recovery_handles.replica_bootstraps
    }

    pub fn replica_promotion_recovery_handles(&self) -> &[super::ReplicaPromotionRecoveryHandle] {
        &self.recovery_handles.replica_promotions
    }
}

impl OperationalControlHistorySummary {
    pub(crate) const fn new(
        record_count: u64,
        completed_backups: u64,
        abandoned_backups: u64,
    ) -> Self {
        Self {
            record_count,
            completed_backups,
            abandoned_backups,
        }
    }

    pub const fn record_count(self) -> u64 {
        self.record_count
    }

    pub const fn completed_backups(self) -> u64 {
        self.completed_backups
    }

    pub const fn abandoned_backups(self) -> u64 {
        self.abandoned_backups
    }
}

#[derive(Debug)]
pub enum ControlStoreTrustPosture {
    Selected(SelectedOperationalControlState),
    Damaged(ControlMediaFault),
    Indeterminate(ControlStoreSelectionIndeterminate),
    Unavailable(ControlStoreAvailabilityDenial),
    Empty,
}
