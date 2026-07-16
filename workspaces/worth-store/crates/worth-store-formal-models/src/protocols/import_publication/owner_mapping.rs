use worth_store_operations::{
    ImportPublicationDenial, ImportPublicationReadiness, PublishedImportedLayout,
};
use worth_store_physical_backend::{
    ProductionStorageBoundarySeam, StorageBoundaryFault, StorageBoundaryTrace,
};
use worth_store_physical_isolation::{
    CopyOnWritePublicationBinding, PhysicalPublicationDenial, PhysicalRootPublicationAttempt,
};

use super::ImportPublicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportPublicationReadinessObservation {
    physical_binding: CopyOnWritePublicationBinding,
    actions: [ImportPublicationAction; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublishedImportObservation {
    physical_binding: CopyOnWritePublicationBinding,
    actions: [ImportPublicationAction; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPublicationCrashMappingDenial {
    ExecutionIdentityMismatch,
    AbortFaultWasNotInjected,
    PublicationDidNotAbort,
}

pub fn map_import_publication_readiness(
    readiness: &ImportPublicationReadiness,
) -> ImportPublicationReadinessObservation {
    ImportPublicationReadinessObservation {
        physical_binding: readiness.physical_binding(),
        actions: [
            ImportPublicationAction::RawDeclarationObserved,
            ImportPublicationAction::CurrentScopeReadmitted,
            ImportPublicationAction::RecoveredArtifactAdmitted,
            ImportPublicationAction::LayoutMaterializationAdmitted,
            ImportPublicationAction::PublicationPending,
        ],
    }
}

pub fn map_published_import(published: &PublishedImportedLayout) -> PublishedImportObservation {
    PublishedImportObservation {
        physical_binding: published.physical_binding(),
        actions: [
            ImportPublicationAction::RawDeclarationObserved,
            ImportPublicationAction::CurrentScopeReadmitted,
            ImportPublicationAction::RecoveredArtifactAdmitted,
            ImportPublicationAction::LayoutMaterializationAdmitted,
            ImportPublicationAction::PublicationPending,
            ImportPublicationAction::PublicationDurable,
        ],
    }
}

pub const fn map_import_publication_denial(
    _denial: &ImportPublicationDenial,
) -> ImportPublicationAction {
    ImportPublicationAction::PublicationDenied
}

pub fn map_import_publication_crash_attempt(
    attempt: &PhysicalRootPublicationAttempt,
    trace: &StorageBoundaryTrace,
) -> Result<ImportPublicationAction, ImportPublicationCrashMappingDenial> {
    if attempt.storage_boundary_execution_identity() != trace.execution_identity() {
        return Err(ImportPublicationCrashMappingDenial::ExecutionIdentityMismatch);
    }
    let aborted = trace.injected().contains(&(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        StorageBoundaryFault::AbortBeforeDurabilityBarrier,
    ));
    if !aborted {
        return Err(ImportPublicationCrashMappingDenial::AbortFaultWasNotInjected);
    }
    if attempt.denial() != Some(PhysicalPublicationDenial::PublicationStoreIo) {
        return Err(ImportPublicationCrashMappingDenial::PublicationDidNotAbort);
    }
    Ok(ImportPublicationAction::CrashBeforePublication)
}

impl ImportPublicationReadinessObservation {
    pub const fn physical_binding(&self) -> CopyOnWritePublicationBinding {
        self.physical_binding
    }

    pub fn actions(&self) -> impl Iterator<Item = ImportPublicationAction> + '_ {
        self.actions.iter().copied()
    }
}

impl PublishedImportObservation {
    pub const fn physical_binding(&self) -> CopyOnWritePublicationBinding {
        self.physical_binding
    }

    pub fn actions(&self) -> impl Iterator<Item = ImportPublicationAction> + '_ {
        self.actions.iter().copied()
    }
}
