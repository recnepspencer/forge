use std::marker::PhantomData;

use super::vocabulary::FoundationalDescriptiveSurface;
use crate::profiles::{
    BoundaryArtifactTarget, FoundationalProfileAttachmentTargetKind,
    FoundationalProfileAttachmentTargetMarker, ProofBearingArtifactTarget, SupportArtifactTarget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileFamily {
    DiagnosticRichness,
    SupportPosture,
    RetentionDelivery,
    CertificationPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileDecisionKind {
    ActiveRichness,
    RetentionAvailability,
    ReplayReconstructability,
    SupportPostureDeferral,
    CertificationPostureRequirement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTargetSurfaceInventory<Target> {
    target_kind: FoundationalProfileAttachmentTargetKind,
    surfaces: &'static [FoundationalDescriptiveSurface],
    marker: PhantomData<Target>,
}

impl<Target> FoundationalTargetSurfaceInventory<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    pub(crate) fn new(surfaces: &'static [FoundationalDescriptiveSurface]) -> Self {
        Self {
            target_kind: Target::kind(),
            surfaces,
            marker: PhantomData,
        }
    }

    pub const fn target_kind(&self) -> FoundationalProfileAttachmentTargetKind {
        self.target_kind
    }

    pub const fn surfaces(&self) -> &'static [FoundationalDescriptiveSurface] {
        self.surfaces
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileApplicability<Target> {
    inventory: FoundationalTargetSurfaceInventory<Target>,
}

impl<Target> FoundationalProfileApplicability<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    fn new(inventory: FoundationalTargetSurfaceInventory<Target>) -> Self {
        Self { inventory }
    }

    pub const fn inventory(&self) -> &FoundationalTargetSurfaceInventory<Target> {
        &self.inventory
    }

    pub fn governing_families(
        &self,
        surface: FoundationalDescriptiveSurface,
    ) -> Option<&'static [FoundationalProfileFamily]> {
        if !self.inventory.surfaces().contains(&surface) {
            return None;
        }

        Some(governing_families(self.inventory.target_kind(), surface))
    }

    pub fn governing_decisions(
        &self,
        surface: FoundationalDescriptiveSurface,
    ) -> Option<&'static [FoundationalProfileDecisionKind]> {
        if !self.inventory.surfaces().contains(&surface) {
            return None;
        }

        Some(governing_decisions(self.inventory.target_kind(), surface))
    }

    pub fn governs(
        &self,
        surface: FoundationalDescriptiveSurface,
        family: FoundationalProfileFamily,
    ) -> bool {
        self.governing_families(surface)
            .is_some_and(|families| families.contains(&family))
    }

    pub fn governs_decision(
        &self,
        surface: FoundationalDescriptiveSurface,
        decision: FoundationalProfileDecisionKind,
    ) -> bool {
        self.governing_decisions(surface)
            .is_some_and(|decisions| decisions.contains(&decision))
    }
}

pub fn boundary_artifact_surface_inventory(
) -> FoundationalTargetSurfaceInventory<BoundaryArtifactTarget> {
    FoundationalTargetSurfaceInventory::new(&[
        FoundationalDescriptiveSurface::History,
        FoundationalDescriptiveSurface::Replay,
        FoundationalDescriptiveSurface::Lineage,
        FoundationalDescriptiveSurface::Provenance,
        FoundationalDescriptiveSurface::ForensicDiagnostics,
    ])
}

pub fn support_artifact_surface_inventory(
) -> FoundationalTargetSurfaceInventory<SupportArtifactTarget> {
    FoundationalTargetSurfaceInventory::new(&[
        FoundationalDescriptiveSurface::History,
        FoundationalDescriptiveSurface::Replay,
        FoundationalDescriptiveSurface::Provenance,
        FoundationalDescriptiveSurface::ForensicDiagnostics,
    ])
}

pub fn proof_bearing_artifact_surface_inventory(
) -> FoundationalTargetSurfaceInventory<ProofBearingArtifactTarget> {
    FoundationalTargetSurfaceInventory::new(&[
        FoundationalDescriptiveSurface::Provenance,
        FoundationalDescriptiveSurface::ForensicDiagnostics,
    ])
}

pub fn foundational_profile_applicability<Target>() -> FoundationalProfileApplicability<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    FoundationalProfileApplicability::new(surface_inventory::<Target>())
}

pub(crate) fn surface_inventory<Target>() -> FoundationalTargetSurfaceInventory<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    match Target::kind() {
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact => {
            FoundationalTargetSurfaceInventory::new(
                boundary_artifact_surface_inventory().surfaces(),
            )
        }
        FoundationalProfileAttachmentTargetKind::SupportArtifact => {
            FoundationalTargetSurfaceInventory::new(support_artifact_surface_inventory().surfaces())
        }
        FoundationalProfileAttachmentTargetKind::ProofBearingArtifact => {
            FoundationalTargetSurfaceInventory::new(
                proof_bearing_artifact_surface_inventory().surfaces(),
            )
        }
    }
}

fn governing_families(
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> &'static [FoundationalProfileFamily] {
    match (target_kind, surface) {
        (_, FoundationalDescriptiveSurface::History)
        | (_, FoundationalDescriptiveSurface::Replay)
        | (_, FoundationalDescriptiveSurface::Lineage) => &[
            FoundationalProfileFamily::DiagnosticRichness,
            FoundationalProfileFamily::RetentionDelivery,
        ],
        (
            FoundationalProfileAttachmentTargetKind::SupportArtifact,
            FoundationalDescriptiveSurface::Provenance,
        ) => &[
            FoundationalProfileFamily::SupportPosture,
            FoundationalProfileFamily::RetentionDelivery,
        ],
        (_, FoundationalDescriptiveSurface::Provenance) => {
            &[FoundationalProfileFamily::RetentionDelivery]
        }
        (
            FoundationalProfileAttachmentTargetKind::SupportArtifact,
            FoundationalDescriptiveSurface::ForensicDiagnostics,
        ) => &[
            FoundationalProfileFamily::DiagnosticRichness,
            FoundationalProfileFamily::CertificationPosture,
        ],
        (_, FoundationalDescriptiveSurface::ForensicDiagnostics) => {
            &[FoundationalProfileFamily::DiagnosticRichness]
        }
    }
}

fn governing_decisions(
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> &'static [FoundationalProfileDecisionKind] {
    match (target_kind, surface) {
        (_, FoundationalDescriptiveSurface::History)
        | (_, FoundationalDescriptiveSurface::Lineage) => &[
            FoundationalProfileDecisionKind::ActiveRichness,
            FoundationalProfileDecisionKind::RetentionAvailability,
        ],
        (_, FoundationalDescriptiveSurface::Replay) => &[
            FoundationalProfileDecisionKind::ActiveRichness,
            FoundationalProfileDecisionKind::RetentionAvailability,
            FoundationalProfileDecisionKind::ReplayReconstructability,
        ],
        (
            FoundationalProfileAttachmentTargetKind::SupportArtifact,
            FoundationalDescriptiveSurface::Provenance,
        ) => &[
            FoundationalProfileDecisionKind::SupportPostureDeferral,
            FoundationalProfileDecisionKind::RetentionAvailability,
        ],
        (_, FoundationalDescriptiveSurface::Provenance) => {
            &[FoundationalProfileDecisionKind::RetentionAvailability]
        }
        (
            FoundationalProfileAttachmentTargetKind::SupportArtifact,
            FoundationalDescriptiveSurface::ForensicDiagnostics,
        ) => &[
            FoundationalProfileDecisionKind::ActiveRichness,
            FoundationalProfileDecisionKind::CertificationPostureRequirement,
        ],
        (_, FoundationalDescriptiveSurface::ForensicDiagnostics) => {
            &[FoundationalProfileDecisionKind::ActiveRichness]
        }
    }
}
