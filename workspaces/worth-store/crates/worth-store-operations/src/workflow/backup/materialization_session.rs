use worth_store_physical_backend::{
    PhysicalBackupMaterializationCancellation, PhysicalBackupMaterializationCounters,
    PhysicalBackupMaterializationDenial, PhysicalBackupMaterializationSession,
    PhysicalBackupPublicationSession,
};
use worth_store_physical_format::{
    BackupBundleFormatAuthority, BackupBundleFormatDenial, BackupBundleManifest,
    MaterializedBackupBundle,
};
use worth_store_physical_isolation::AdmittedBackupCut;

use crate::{
    OperationalControlAppendDenial, OperationalControlRecord, OperationalControlStorePort,
    OperationalOperationId,
};

use super::transition;

pub struct BackupMaterializationSession<'a> {
    pub(super) operation_id: OperationalOperationId,
    pub(super) cut: AdmittedBackupCut,
    pub(super) manifest: BackupBundleManifest,
    pub(super) physical: PhysicalBackupMaterializationSession,
    pub(super) control: &'a dyn OperationalControlStorePort,
    pub(super) format: BackupBundleFormatAuthority,
}

pub struct BackupPublicationSession<'a> {
    pub(super) operation_id: OperationalOperationId,
    pub(super) cut: AdmittedBackupCut,
    pub(super) physical: PhysicalBackupPublicationSession,
    pub(super) control: &'a dyn OperationalControlStorePort,
    pub(super) format: BackupBundleFormatAuthority,
}

#[derive(Debug)]
pub enum BackupMaterializationDenial {
    Plan(crate::BackupMaterializationRecoveryPlanDenial),
    PlanPersistence(OperationalControlAppendDenial),
    Physical(PhysicalBackupMaterializationDenial),
    Format(BackupBundleFormatDenial),
    Control(Box<BackupMaterializationRecordDenial>),
    AdmittedCutInvariant,
    PreparationAllocationFailed,
}

#[derive(Debug)]
pub struct BackupMaterializationCompletion {
    operation_id: OperationalOperationId,
    bundle: MaterializedBackupBundle,
    counters: PhysicalBackupMaterializationCounters,
    cut: AdmittedBackupCut,
}

#[derive(Debug)]
pub struct UnrecordedBackupMaterialization {
    operation_id: OperationalOperationId,
    bundle: MaterializedBackupBundle,
    counters: PhysicalBackupMaterializationCounters,
    cut: AdmittedBackupCut,
}

#[derive(Debug)]
pub struct BackupMaterializationRecordDenial {
    materialization: UnrecordedBackupMaterialization,
    source: OperationalControlAppendDenial,
}

impl BackupMaterializationCompletion {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }
    pub const fn bundle(&self) -> &MaterializedBackupBundle {
        &self.bundle
    }
    pub const fn counters(&self) -> PhysicalBackupMaterializationCounters {
        self.counters
    }
    pub fn into_parts(self) -> (MaterializedBackupBundle, AdmittedBackupCut) {
        (self.bundle, self.cut)
    }
}

impl UnrecordedBackupMaterialization {
    pub fn record(
        self,
        control: &dyn OperationalControlStorePort,
    ) -> Result<BackupMaterializationCompletion, BackupMaterializationRecordDenial> {
        let record = OperationalControlRecord::backup_materialization_recorded(
            self.cut.authority_identity(),
            self.operation_id.clone(),
            transition(&self.operation_id, "materialized"),
            &self.bundle,
        );
        if let Err(source) = control.append(&record) {
            return Err(BackupMaterializationRecordDenial {
                materialization: self,
                source,
            });
        }
        Ok(BackupMaterializationCompletion {
            operation_id: self.operation_id,
            bundle: self.bundle,
            counters: self.counters,
            cut: self.cut,
        })
    }

    pub const fn bundle(&self) -> &MaterializedBackupBundle {
        &self.bundle
    }
}

impl BackupMaterializationRecordDenial {
    pub fn into_retry(
        self,
    ) -> (
        UnrecordedBackupMaterialization,
        OperationalControlAppendDenial,
    ) {
        (self.materialization, self.source)
    }
}

impl<'a> BackupMaterializationSession<'a> {
    pub(super) fn new(
        operation_id: OperationalOperationId,
        cut: AdmittedBackupCut,
        manifest: BackupBundleManifest,
        physical: PhysicalBackupMaterializationSession,
        control: &'a impl OperationalControlStorePort,
        format: BackupBundleFormatAuthority,
    ) -> Self {
        Self {
            operation_id,
            cut,
            manifest,
            physical,
            control,
            format,
        }
    }
    pub fn advance(&mut self) -> Result<bool, PhysicalBackupMaterializationDenial> {
        self.physical.advance()
    }
    pub fn advance_with_cancellation(
        &mut self,
        cancellation: &PhysicalBackupMaterializationCancellation,
    ) -> Result<bool, PhysicalBackupMaterializationDenial> {
        self.physical.advance_with_cancellation(cancellation)
    }

    pub fn advance_boundary(
        &mut self,
    ) -> Result<
        Option<worth_store_physical_backend::PhysicalBackupMaterializationProgress>,
        PhysicalBackupMaterializationDenial,
    > {
        self.physical.advance_boundary()
    }

    pub fn advance_boundary_with_cancellation(
        &mut self,
        cancellation: &PhysicalBackupMaterializationCancellation,
    ) -> Result<
        Option<worth_store_physical_backend::PhysicalBackupMaterializationProgress>,
        PhysicalBackupMaterializationDenial,
    > {
        self.physical
            .advance_boundary_with_cancellation(cancellation)
    }

    pub fn begin_publication(
        self,
    ) -> Result<BackupPublicationSession<'a>, BackupMaterializationDenial> {
        let manifest_bytes = self
            .format
            .encode_manifest(&self.manifest)
            .map_err(BackupMaterializationDenial::Format)?;
        let physical = self
            .physical
            .begin_publication(manifest_bytes)
            .map_err(BackupMaterializationDenial::Physical)?;
        Ok(BackupPublicationSession {
            operation_id: self.operation_id,
            cut: self.cut,
            physical,
            control: self.control,
            format: self.format,
        })
    }
    pub fn finish(
        mut self,
    ) -> Result<BackupMaterializationCompletion, BackupMaterializationDenial> {
        while self
            .physical
            .advance()
            .map_err(BackupMaterializationDenial::Physical)?
        {}
        self.begin_publication()?.finish()
    }
}

impl<'a> BackupPublicationSession<'a> {
    pub fn advance(
        &mut self,
    ) -> Result<
        Option<worth_store_physical_backend::PhysicalBackupPublicationProgress>,
        PhysicalBackupMaterializationDenial,
    > {
        self.physical.advance()
    }

    pub fn advance_with_cancellation(
        &mut self,
        cancellation: &PhysicalBackupMaterializationCancellation,
    ) -> Result<
        Option<worth_store_physical_backend::PhysicalBackupPublicationProgress>,
        PhysicalBackupMaterializationDenial,
    > {
        self.physical.advance_with_cancellation(cancellation)
    }

    pub fn finish(self) -> Result<BackupMaterializationCompletion, BackupMaterializationDenial> {
        let physical_bundle = self
            .physical
            .finish()
            .map_err(BackupMaterializationDenial::Physical)?;
        let counters = physical_bundle.counters();
        let bundle = self
            .format
            .admit_materialized(physical_bundle.root())
            .map_err(BackupMaterializationDenial::Format)?;
        UnrecordedBackupMaterialization {
            operation_id: self.operation_id,
            bundle,
            counters,
            cut: self.cut,
        }
        .record(self.control)
        .map_err(|denial| BackupMaterializationDenial::Control(Box::new(denial)))
    }
}
