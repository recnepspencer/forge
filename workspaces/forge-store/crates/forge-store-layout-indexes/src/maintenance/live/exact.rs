use crate::{
    maintenance::{ExactBTreePublicationEvidence, IndexMaintenanceMode},
    LayoutCoverageWitness, PhysicalArtifactFamily,
};

use super::IndexPublicationProtocol;

#[derive(Debug, Clone, Copy)]
pub struct LiveExactMaintenanceRequest<'a> {
    publication: &'a ExactBTreePublicationEvidence,
}

impl<'a> LiveExactMaintenanceRequest<'a> {
    pub const fn from_btree_publication(publication: &'a ExactBTreePublicationEvidence) -> Self {
        Self { publication }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveExactMaintenanceWitness {
    family: PhysicalArtifactFamily,
    exact_coverage: LayoutCoverageWitness,
    maintenance_mode: IndexMaintenanceMode,
    publication_authority: ExactBTreePublicationEvidence,
}

impl LiveExactMaintenanceWitness {
    fn issue(request: LiveExactMaintenanceRequest<'_>) -> Self {
        Self {
            family: request.publication.family(),
            exact_coverage: request.publication.coverage().clone(),
            maintenance_mode: request.publication.maintenance_mode(),
            publication_authority: request.publication.clone(),
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn exact_coverage(&self) -> &LayoutCoverageWitness {
        &self.exact_coverage
    }

    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        IndexPublicationProtocol::CopyOnWriteRootSwap
    }

    pub const fn publication_authority(&self) -> &ExactBTreePublicationEvidence {
        &self.publication_authority
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveExactMaintenance;

pub const fn live_exact_maintenance() -> LiveExactMaintenance {
    LiveExactMaintenance
}

impl LiveExactMaintenance {
    pub fn admit(self, request: LiveExactMaintenanceRequest<'_>) -> LiveExactMaintenanceOutcome {
        LiveExactMaintenanceOutcome::issue(LiveExactMaintenanceWitness::issue(request))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveExactMaintenanceOutcome {
    witness: Box<LiveExactMaintenanceWitness>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveExactMaintenanceView<'a> {
    Admitted(&'a LiveExactMaintenanceWitness),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiveExactMaintenanceCaseId(&'static str);

impl LiveExactMaintenanceCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn live_exact_maintenance_cases() -> impl Iterator<Item = LiveExactMaintenanceCaseId> {
    [LiveExactMaintenanceCaseId(
        "layout.maintenance.live_exact.admitted",
    )]
    .into_iter()
}

impl LiveExactMaintenanceOutcome {
    fn issue(witness: LiveExactMaintenanceWitness) -> Self {
        Self {
            witness: Box::new(witness),
        }
    }

    pub const fn view(&self) -> LiveExactMaintenanceView<'_> {
        LiveExactMaintenanceView::Admitted(&self.witness)
    }

    pub const fn case_id(&self) -> LiveExactMaintenanceCaseId {
        LiveExactMaintenanceCaseId("layout.maintenance.live_exact.admitted")
    }

    pub fn into_admitted(self) -> LiveExactMaintenanceWitness {
        *self.witness
    }
}
