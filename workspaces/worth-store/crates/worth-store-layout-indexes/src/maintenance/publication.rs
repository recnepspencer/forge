use crate::{
    AdmittedLayoutMaterialization, LayoutCoverageWitness, LayoutMaterializationSourceKind,
    PhysicalArtifactFamily,
};
use worth_store_physical_isolation::PhysicalPublicationCounterSnapshot;

use super::CopyOnWriteLayoutMutationReceipt;

#[derive(Debug, Clone, Copy)]
pub struct ExactBTreePublicationRequest<'a> {
    execution: &'a CopyOnWriteLayoutMutationReceipt,
    materialization: &'a AdmittedLayoutMaterialization,
}

impl<'a> ExactBTreePublicationRequest<'a> {
    pub const fn new(
        execution: &'a CopyOnWriteLayoutMutationReceipt,
        materialization: &'a AdmittedLayoutMaterialization,
    ) -> Self {
        Self {
            execution,
            materialization,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactBTreePublicationDenied {
    MaintenanceModeIsNotSynchronousExact,
    PublicationFamilyMismatch,
    PublicationDoesNotOwnCoverage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactBTreePublicationEvidence {
    family: crate::AdmittedPhysicalArtifactFamily,
    maintenance_mode: super::IndexMaintenanceMode,
    coverage: LayoutCoverageWitness,
    counters: PhysicalPublicationCounterSnapshot,
}

impl ExactBTreePublicationEvidence {
    fn issue(
        request: ExactBTreePublicationRequest<'_>,
    ) -> Result<Self, ExactBTreePublicationDenied> {
        let coverage = request.materialization.coverage();
        if request.execution.maintenance_mode() != super::IndexMaintenanceMode::SynchronousExact {
            return Err(ExactBTreePublicationDenied::MaintenanceModeIsNotSynchronousExact);
        }
        if request.execution.admitted_family() != request.materialization.family() {
            return Err(ExactBTreePublicationDenied::PublicationFamilyMismatch);
        }
        let expected = request
            .execution
            .publication()
            .new_root_validation()
            .reference();
        if coverage.source().kind() != LayoutMaterializationSourceKind::BTreeRoot(expected) {
            return Err(ExactBTreePublicationDenied::PublicationDoesNotOwnCoverage);
        }
        Ok(Self {
            family: request.execution.admitted_family(),
            maintenance_mode: request.execution.maintenance_mode(),
            coverage: coverage.clone(),
            counters: request.execution.publication().counters(),
        })
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family.declaration().family()
    }

    pub const fn admitted_family(&self) -> crate::AdmittedPhysicalArtifactFamily {
        self.family
    }

    pub const fn security_identity(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.family.security_identity()
    }

    pub const fn maintenance_mode(&self) -> super::IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn coverage(&self) -> &LayoutCoverageWitness {
        &self.coverage
    }

    pub const fn counters(&self) -> PhysicalPublicationCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExactBTreePublicationCase {
    Published(Box<ExactBTreePublicationEvidence>),
    Denied(ExactBTreePublicationDenied),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactBTreePublicationOutcome {
    case: ExactBTreePublicationCase,
}

#[derive(Debug, Clone, Copy)]
pub enum ExactBTreePublicationView<'a> {
    Published(&'a ExactBTreePublicationEvidence),
    Denied(&'a ExactBTreePublicationDenied),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactBTreePublicationCaseId(&'static str);

impl ExactBTreePublicationCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn exact_btree_publication_cases() -> impl Iterator<Item = ExactBTreePublicationCaseId> {
    [
        ExactBTreePublicationCaseId("layout.publication.btree.exact"),
        ExactBTreePublicationCaseId("layout.publication.btree.denied.maintenance_mode"),
        ExactBTreePublicationCaseId("layout.publication.btree.denied.family"),
        ExactBTreePublicationCaseId("layout.publication.btree.denied.coverage_binding"),
    ]
    .into_iter()
}

impl ExactBTreePublicationOutcome {
    pub(super) fn issue(request: ExactBTreePublicationRequest<'_>) -> Self {
        let case = match ExactBTreePublicationEvidence::issue(request) {
            Ok(value) => ExactBTreePublicationCase::Published(Box::new(value)),
            Err(value) => ExactBTreePublicationCase::Denied(value),
        };
        Self { case }
    }

    pub const fn view(&self) -> ExactBTreePublicationView<'_> {
        match &self.case {
            ExactBTreePublicationCase::Published(value) => {
                ExactBTreePublicationView::Published(value)
            }
            ExactBTreePublicationCase::Denied(value) => ExactBTreePublicationView::Denied(value),
        }
    }

    pub const fn case_id(&self) -> ExactBTreePublicationCaseId {
        match &self.case {
            ExactBTreePublicationCase::Published(_) => {
                ExactBTreePublicationCaseId("layout.publication.btree.exact")
            }
            ExactBTreePublicationCase::Denied(
                ExactBTreePublicationDenied::MaintenanceModeIsNotSynchronousExact,
            ) => ExactBTreePublicationCaseId("layout.publication.btree.denied.maintenance_mode"),
            ExactBTreePublicationCase::Denied(
                ExactBTreePublicationDenied::PublicationFamilyMismatch,
            ) => ExactBTreePublicationCaseId("layout.publication.btree.denied.family"),
            ExactBTreePublicationCase::Denied(
                ExactBTreePublicationDenied::PublicationDoesNotOwnCoverage,
            ) => ExactBTreePublicationCaseId("layout.publication.btree.denied.coverage_binding"),
        }
    }

    pub fn into_published(
        self,
    ) -> Result<ExactBTreePublicationEvidence, ExactBTreePublicationDenied> {
        match self.case {
            ExactBTreePublicationCase::Published(value) => Ok(*value),
            ExactBTreePublicationCase::Denied(value) => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutExactPublication;

pub const fn layout_exact_publication() -> LayoutExactPublication {
    LayoutExactPublication
}

impl LayoutExactPublication {
    pub fn observe_btree(
        self,
        request: ExactBTreePublicationRequest<'_>,
    ) -> ExactBTreePublicationOutcome {
        ExactBTreePublicationOutcome::issue(request)
    }
}
