use std::marker::PhantomData;

use super::inventory::surface_inventory;
use super::vocabulary::{
    FoundationalDescriptiveElisionProfile, FoundationalDescriptiveSurface,
    FoundationalSurfaceAbsenceCause, FoundationalSurfaceAvailabilityDecision,
};
use crate::profiles::{
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileAttachmentTargetMarker,
    MaterializedFoundationalProfileSet, RetentionDeliveryProfile, SupportPostureProfile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalMaterializationCost {
    inventory_surface_count: u32,
    requested_surface_count: u32,
    available_surface_count: u32,
    unavailable_surface_count: u32,
}

impl FoundationalMaterializationCost {
    const fn new(
        inventory_surface_count: u32,
        requested_surface_count: u32,
        available_surface_count: u32,
        unavailable_surface_count: u32,
    ) -> Self {
        Self {
            inventory_surface_count,
            requested_surface_count,
            available_surface_count,
            unavailable_surface_count,
        }
    }

    pub const fn inventory_surface_count(&self) -> u32 {
        self.inventory_surface_count
    }

    pub const fn requested_surface_count(&self) -> u32 {
        self.requested_surface_count
    }

    pub const fn available_surface_count(&self) -> u32 {
        self.available_surface_count
    }

    pub const fn unavailable_surface_count(&self) -> u32 {
        self.unavailable_surface_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalProfileMaterializationPlan<Target> {
    target_kind: FoundationalProfileAttachmentTargetKind,
    decisions: Vec<FoundationalSurfaceAvailabilityDecision>,
    cost: FoundationalMaterializationCost,
    marker: PhantomData<Target>,
}

impl<Target> FoundationalProfileMaterializationPlan<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    fn new(decisions: Vec<FoundationalSurfaceAvailabilityDecision>) -> Self {
        let available_surface_count = decisions
            .iter()
            .filter(|entry| entry.is_available())
            .count();
        let unavailable_surface_count = decisions.len() - available_surface_count;

        Self {
            target_kind: Target::kind(),
            cost: FoundationalMaterializationCost::new(
                decisions.len() as u32,
                decisions
                    .iter()
                    .filter(|entry| {
                        entry.absence_cause()
                            != Some(FoundationalSurfaceAbsenceCause::DeniedByBudget)
                    })
                    .count() as u32,
                available_surface_count as u32,
                unavailable_surface_count as u32,
            ),
            decisions,
            marker: PhantomData,
        }
    }

    pub const fn target_kind(&self) -> FoundationalProfileAttachmentTargetKind {
        self.target_kind
    }

    pub fn decisions(&self) -> &[FoundationalSurfaceAvailabilityDecision] {
        &self.decisions
    }

    pub const fn cost(&self) -> FoundationalMaterializationCost {
        self.cost
    }

    pub fn decision_for(
        &self,
        surface: FoundationalDescriptiveSurface,
    ) -> Option<FoundationalSurfaceAvailabilityDecision> {
        self.decisions
            .iter()
            .find(|entry| entry.surface() == surface)
            .copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalMaterializationPlanningDenial {
    EmptySelectedSurfaceSet,
    DuplicateSelectedSurface,
    SurfaceIllegalForTarget,
}

pub fn plan_foundational_profile_materialization<Target>(
    profile: &MaterializedFoundationalProfileSet,
) -> FoundationalProfileMaterializationPlan<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    let inventory = surface_inventory::<Target>();
    build_materialization_plan(
        profile,
        inventory.target_kind(),
        inventory.surfaces(),
        inventory.surfaces(),
    )
    .expect("closed target inventories are always legal and non-empty")
}

pub fn plan_foundational_profile_materialization_with_elision<Target>(
    profile: &MaterializedFoundationalProfileSet,
    elision: FoundationalDescriptiveElisionProfile,
) -> FoundationalProfileMaterializationPlan<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    let inventory = surface_inventory::<Target>();
    build_materialization_plan(
        profile,
        inventory.target_kind(),
        inventory.surfaces(),
        elision_selected_surfaces::<Target>(elision),
    )
    .expect("named elision profiles resolve to legal non-empty target inventories")
}

pub fn plan_selected_foundational_profile_materialization<Target>(
    profile: &MaterializedFoundationalProfileSet,
    selected: &[FoundationalDescriptiveSurface],
) -> Result<FoundationalProfileMaterializationPlan<Target>, FoundationalMaterializationPlanningDenial>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    let inventory = surface_inventory::<Target>();
    build_materialization_plan(
        profile,
        inventory.target_kind(),
        inventory.surfaces(),
        selected,
    )
}

fn elision_selected_surfaces<Target>(
    elision: FoundationalDescriptiveElisionProfile,
) -> &'static [FoundationalDescriptiveSurface]
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    match (Target::kind(), elision) {
        (_, FoundationalDescriptiveElisionProfile::FullFidelity) => {
            surface_inventory::<Target>().surfaces()
        }
        (
            FoundationalProfileAttachmentTargetKind::BoundaryArtifact,
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        ) => &[FoundationalDescriptiveSurface::History],
        (
            FoundationalProfileAttachmentTargetKind::SupportArtifact,
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        ) => &[
            FoundationalDescriptiveSurface::History,
            FoundationalDescriptiveSurface::Provenance,
        ],
        (
            FoundationalProfileAttachmentTargetKind::ProofBearingArtifact,
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        ) => &[FoundationalDescriptiveSurface::Provenance],
    }
}

fn build_materialization_plan<Target>(
    profile: &MaterializedFoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    inventory: &[FoundationalDescriptiveSurface],
    selected: &[FoundationalDescriptiveSurface],
) -> Result<FoundationalProfileMaterializationPlan<Target>, FoundationalMaterializationPlanningDenial>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    validate_selected_surfaces(inventory, selected)?;

    Ok(FoundationalProfileMaterializationPlan::new(
        inventory
            .iter()
            .copied()
            .map(|surface| availability_decision(profile, target_kind, surface, selected))
            .collect(),
    ))
}

fn validate_selected_surfaces(
    inventory: &[FoundationalDescriptiveSurface],
    selected: &[FoundationalDescriptiveSurface],
) -> Result<(), FoundationalMaterializationPlanningDenial> {
    if selected.is_empty() {
        return Err(FoundationalMaterializationPlanningDenial::EmptySelectedSurfaceSet);
    }

    let mut seen = Vec::with_capacity(selected.len());
    for surface in selected {
        if !inventory.contains(surface) {
            return Err(FoundationalMaterializationPlanningDenial::SurfaceIllegalForTarget);
        }
        if seen.contains(surface) {
            return Err(FoundationalMaterializationPlanningDenial::DuplicateSelectedSurface);
        }
        seen.push(*surface);
    }

    Ok(())
}

fn availability_decision(
    profile: &MaterializedFoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
    selected: &[FoundationalDescriptiveSurface],
) -> FoundationalSurfaceAvailabilityDecision {
    if !selected.contains(&surface) {
        return FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::DeniedByBudget,
        );
    }

    let materialized = profile.materialized();
    match surface {
        FoundationalDescriptiveSurface::History => history_decision(materialized, surface),
        FoundationalDescriptiveSurface::Replay => replay_decision(materialized, surface),
        FoundationalDescriptiveSurface::Lineage => lineage_decision(materialized, surface),
        FoundationalDescriptiveSurface::Provenance => {
            provenance_decision(materialized, target_kind, surface)
        }
        FoundationalDescriptiveSurface::ForensicDiagnostics => {
            forensic_decision(materialized, target_kind, surface)
        }
    }
}

fn history_decision(
    materialized: &crate::profiles::FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::NotRetained,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn replay_decision(
    materialized: &crate::profiles::FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::NotRetained,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Retained
        || materialized.compatibility_posture() == CompatibilityPostureProfile::NativeOnly
    {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::NotReconstructable,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn lineage_decision(
    materialized: &crate::profiles::FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::NotRetained,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn provenance_decision(
    materialized: &crate::profiles::FoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if target_kind == FoundationalProfileAttachmentTargetKind::SupportArtifact
        && materialized.support_posture() != SupportPostureProfile::CertificationReady
    {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::DeferredBySupportPosture,
        )
    } else if target_kind != FoundationalProfileAttachmentTargetKind::ProofBearingArtifact
        && materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral
    {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::NotRetained,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn forensic_decision(
    materialized: &crate::profiles::FoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() != DiagnosticRichnessProfile::Forensic {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if target_kind == FoundationalProfileAttachmentTargetKind::SupportArtifact
        && materialized.certification_posture()
            != crate::profiles::CertificationPostureProfile::ProductionCertified
    {
        FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::UncertifiedForRequestedPosture,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}
