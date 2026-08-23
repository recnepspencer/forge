use std::marker::PhantomData;

use super::decisions::availability_decision;
use super::inventory::surface_inventory;
use super::observation::FoundationalObservationDisposition;
use super::vocabulary::{
    FoundationalDescriptiveElisionProfile, FoundationalDescriptiveSurface,
    FoundationalSurfaceAbsenceCause, FoundationalSurfaceAvailabilityDecision,
};
use crate::profiles::{
    FoundationalProfileAttachmentTargetKind, FoundationalProfileAttachmentTargetMarker,
    MaterializedFoundationalProfileSet, ObservationActivationProfile,
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
    observation_disposition: FoundationalObservationDisposition,
    decisions: Vec<FoundationalSurfaceAvailabilityDecision>,
    cost: FoundationalMaterializationCost,
    marker: PhantomData<Target>,
}

impl<Target> FoundationalProfileMaterializationPlan<Target>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    fn new(
        observation_disposition: FoundationalObservationDisposition,
        decisions: Vec<FoundationalSurfaceAvailabilityDecision>,
    ) -> Self {
        let available_surface_count = decisions
            .iter()
            .filter(|entry| entry.is_available())
            .count();
        let unavailable_surface_count = decisions.len() - available_surface_count;

        Self {
            target_kind: Target::kind(),
            observation_disposition,
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

    pub const fn observation_disposition(&self) -> FoundationalObservationDisposition {
        self.observation_disposition
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
    ObservationDispositionRequired,
}

pub fn plan_foundational_profile_materialization<Target>(
    profile: &MaterializedFoundationalProfileSet,
) -> Result<FoundationalProfileMaterializationPlan<Target>, FoundationalMaterializationPlanningDenial>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    let inventory = surface_inventory::<Target>();
    build_materialization_plan(
        profile,
        inventory.target_kind(),
        inventory.surfaces(),
        inventory.surfaces(),
        profile_default_observation_disposition(profile)?,
    )
}

pub fn plan_foundational_profile_materialization_with_elision<Target>(
    profile: &MaterializedFoundationalProfileSet,
    elision: FoundationalDescriptiveElisionProfile,
) -> Result<FoundationalProfileMaterializationPlan<Target>, FoundationalMaterializationPlanningDenial>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    let inventory = surface_inventory::<Target>();
    build_materialization_plan(
        profile,
        inventory.target_kind(),
        inventory.surfaces(),
        elision_selected_surfaces::<Target>(elision),
        profile_default_observation_disposition(profile)?,
    )
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
        profile_default_observation_disposition(profile)?,
    )
}

pub fn plan_selected_foundational_profile_materialization_with_disposition<Target>(
    profile: &MaterializedFoundationalProfileSet,
    selected: &[FoundationalDescriptiveSurface],
    disposition: FoundationalObservationDisposition,
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
        disposition,
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

fn profile_default_observation_disposition(
    profile: &MaterializedFoundationalProfileSet,
) -> Result<FoundationalObservationDisposition, FoundationalMaterializationPlanningDenial> {
    match profile.materialized().observation_activation() {
        ObservationActivationProfile::Continuous => {
            Ok(FoundationalObservationDisposition::Continuous)
        }
        ObservationActivationProfile::OnDemand => {
            Err(FoundationalMaterializationPlanningDenial::ObservationDispositionRequired)
        }
    }
}

fn build_materialization_plan<Target>(
    profile: &MaterializedFoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    inventory: &[FoundationalDescriptiveSurface],
    selected: &[FoundationalDescriptiveSurface],
    disposition: FoundationalObservationDisposition,
) -> Result<FoundationalProfileMaterializationPlan<Target>, FoundationalMaterializationPlanningDenial>
where
    Target: FoundationalProfileAttachmentTargetMarker,
{
    validate_selected_surfaces(inventory, selected)?;

    Ok(FoundationalProfileMaterializationPlan::new(
        disposition,
        inventory
            .iter()
            .copied()
            .map(|surface| {
                availability_decision(profile, target_kind, surface, selected, disposition)
            })
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
