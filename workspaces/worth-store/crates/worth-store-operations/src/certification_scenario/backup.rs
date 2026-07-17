use std::path::{Path, PathBuf};

use worth_store_authority::{BackupRestoreAdmissionPolicy, StoreCurrentAuthorityWitness};
use worth_store_offline_verifier::{verify_materialized_backup, OfflineInspectionBudget};
use worth_store_physical_isolation::{
    BackupCutCoordinates, BackupCutManifest, BackupReachabilityLeaseRegistry,
};
use worth_store_security::StoreKeyVersionPosture;

use crate::{
    admit_backup_for_production_restore, qualify_backup_custody,
    record_independent_backup_verification, BackupExportCustodyDeclaration,
    BackupExportCustodyMode, BackupExportCustodyReadiness, ConfiguredFailureDomainId,
    OnlineBackupIntent, OperationalControlLocation, OperationalControlStore,
    OperationalControlStorePort, OperationalCounterReceipt, OperationalOperationId,
    ProductionRestoreAdmissibleBackupBundle, ProtectedOperationalMediaLocation,
};

use super::backup_artifacts::{
    canonical_backup_artifacts_at_root_generation, CanonicalBackupArtifacts,
};

pub struct OwnerBackedBackupScenario {
    workspace: tempfile::TempDir,
    source: PathBuf,
    target: PathBuf,
    control: PathBuf,
    artifacts: CanonicalBackupArtifacts,
    leases: BackupReachabilityLeaseRegistry,
    authority: StoreCurrentAuthorityWitness,
}

pub struct OwnerBackedBackupOutcome {
    operation: OperationalOperationId,
    restore_source: ProductionRestoreAdmissibleBackupBundle,
    counters: OperationalCounterReceipt,
}

impl OwnerBackedBackupScenario {
    pub fn materialize(case: &str) -> Self {
        let workspace = tempfile::tempdir().expect("certification scenario workspace");
        let source = workspace.path().join("backup-source");
        let target = workspace.path().join("backup-target");
        let control = workspace.path().join("control/operations.log");
        std::fs::create_dir_all(&source).expect("backup source directory");
        std::fs::create_dir_all(&target).expect("backup target directory");
        Self {
            artifacts: canonical_backup_artifacts_at_root_generation(case, &source, 1),
            leases: BackupReachabilityLeaseRegistry::for_store_runtime(),
            authority: crate::backup::export::current_authority(case),
            workspace,
            source,
            target,
            control,
        }
    }

    pub fn control_store(&self) -> OperationalControlStore {
        OperationalControlStore::open_with_certified_topology(
            OperationalControlLocation::new(&self.control, domain("control")),
            [
                ProtectedOperationalMediaLocation::source(&self.source, domain("source")),
                ProtectedOperationalMediaLocation::backup_target(&self.target, domain("target")),
            ],
        )
        .expect("physically independent certification control store")
    }

    pub const fn authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.authority
    }

    pub fn workspace_root(&self) -> &Path {
        self.workspace.path()
    }

    pub fn execute(
        &self,
        scenario_identity: &str,
        control: &impl OperationalControlStorePort,
    ) -> OwnerBackedBackupOutcome {
        let operation = OperationalOperationId::new(format!("{scenario_identity}/backup"))
            .expect("scenario operation identity");
        let custody = backup_custody(&self.authority);
        let admitted = OnlineBackupIntent::new(
            operation.clone(),
            self.coordinates(),
            BackupCutManifest::canonical(self.artifacts.references.clone())
                .expect("owner-built complete backup cut"),
            custody,
        )
        .admit_cut(&self.authority, control, &self.leases)
        .expect("owner-admitted backup cut");
        let completion = admitted
            .materialize(&self.target, 64 * 1024, control)
            .expect("owner-opened backup materialization")
            .finish()
            .expect("owner-completed backup materialization");
        let counters = OperationalCounterReceipt::from_backup_materialization(&completion)
            .expect("bounded backup counters");
        let (materialized, cut) = completion.into_parts();
        let structural = verify_materialized_backup(
            materialized,
            OfflineInspectionBudget::bounded(64 * 1024, u64::MAX)
                .expect("bounded backup verification"),
        )
        .expect("independent structural backup verification");
        let verified = record_independent_backup_verification(
            &operation,
            structural,
            cut,
            control,
            &self.leases,
        )
        .expect("durable independent verification and lease release");
        let qualified = qualify_backup_custody(verified, &backup_custody(&self.authority))
            .expect("custody-qualified backup");
        let restore_source = admit_backup_for_production_restore(
            qualified,
            &self.authority,
            BackupRestoreAdmissionPolicy::production_default(),
        )
        .expect("production restore admission");
        OwnerBackedBackupOutcome {
            operation,
            restore_source,
            counters,
        }
    }

    fn coordinates(&self) -> BackupCutCoordinates {
        BackupCutCoordinates::new(
            "lineage/s10-certification",
            1,
            1,
            &self.artifacts.checkpoint_identity,
            10,
            10,
            12,
            12,
            "worth-physical-format-v1",
            "posix-file-fsync-dir-sync",
        )
        .expect("coherent certification backup coordinates")
    }
}

impl OwnerBackedBackupOutcome {
    pub const fn operation(&self) -> &OperationalOperationId {
        &self.operation
    }

    pub const fn restore_source(&self) -> &ProductionRestoreAdmissibleBackupBundle {
        &self.restore_source
    }

    pub const fn counters(&self) -> OperationalCounterReceipt {
        self.counters
    }

    pub fn into_restore_source(self) -> ProductionRestoreAdmissibleBackupBundle {
        self.restore_source
    }
}

fn backup_custody(authority: &StoreCurrentAuthorityWitness) -> BackupExportCustodyReadiness {
    let admission = BackupExportCustodyDeclaration::native(
        authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .expect("backup custody declaration")
    .admit_with_current_authority(authority)
    .expect("backup custody admission");
    BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("backup custody readiness")
}

fn domain(label: &str) -> ConfiguredFailureDomainId {
    ConfiguredFailureDomainId::new(label).expect("certification failure domain")
}
