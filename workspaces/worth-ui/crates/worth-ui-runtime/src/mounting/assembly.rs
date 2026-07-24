use std::collections::BTreeSet;
use worth_ui_host_contract::{
    UiMountedFrameCanonicalCore, UiMountedFrameIntegrity, UiMountedFrameManifest,
    UiMountedProjectionView, UiMountedSurfaceBindingRequirement, UiSemanticSurfaceIdentity,
};

use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedFrameRequest {
    surfaces: UiMountedSurfaceSelection,
    virtualized_range: Option<crate::runtime::WorthUiVisibleRange>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum UiMountedSurfaceSelection {
    AllBound,
    Exact(Box<[UiSemanticSurfaceIdentity]>),
}

#[derive(Debug, PartialEq)]
pub enum UiMountedFramePreparationDenial {
    DuplicateSurfaceRequirement,
    MissingSurfaceBinding(UiSemanticSurfaceIdentity),
    SurfaceRebindRequired(UiSemanticSurfaceIdentity),
    LaneWorkUnavailable(worth_ui_host_contract::UiMountedLaneParticipation),
    Lane(crate::facade::WorthUiMountedLaneProjectionDenial),
    Projection(super::UiMountedProjectionDenial),
    IncompleteManifest,
    IntegrityMismatch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedSurfaceReceipt {
    requirement: UiMountedSurfaceBindingRequirement,
    projection: UiMountedProjectionView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedFrameReceipt {
    canonical_core: UiMountedFrameCanonicalCore,
    integrity: UiMountedFrameIntegrity,
    surface_count: usize,
}

pub struct UiPreparedMountedFrame {
    candidate: super::UiProjectedMountedFrameCandidate,
    generation: WorthUiPreparedApplicationGenerationIdentity,
    manifest: UiMountedFrameManifest,
    canonical_core: UiMountedFrameCanonicalCore,
    integrity: UiMountedFrameIntegrity,
    surfaces: Box<[UiMountedSurfaceReceipt]>,
}

impl UiMountedFrameRequest {
    pub fn all_bound_surfaces() -> Self {
        Self {
            surfaces: UiMountedSurfaceSelection::AllBound,
            virtualized_range: None,
        }
    }

    pub fn exact_surfaces(surfaces: Vec<UiSemanticSurfaceIdentity>) -> Self {
        Self {
            surfaces: UiMountedSurfaceSelection::Exact(surfaces.into_boxed_slice()),
            virtualized_range: None,
        }
    }

    pub fn with_virtualized_range(mut self, range: crate::runtime::WorthUiVisibleRange) -> Self {
        self.virtualized_range = Some(range);
        self
    }

    pub fn virtualized_range(&self) -> Option<crate::runtime::WorthUiVisibleRange> {
        self.virtualized_range
    }

    pub(crate) fn resolve_requirements(
        &self,
        bindings: &[super::UiSurfaceBindingIdentityView],
    ) -> Result<Vec<super::UiSurfaceBindingIdentityView>, UiMountedFramePreparationDenial> {
        match &self.surfaces {
            UiMountedSurfaceSelection::AllBound => Ok(bindings.to_vec()),
            UiMountedSurfaceSelection::Exact(surfaces) => {
                let mut ordered = surfaces.to_vec();
                ordered.sort();
                if ordered.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(UiMountedFramePreparationDenial::DuplicateSurfaceRequirement);
                }
                ordered
                    .into_iter()
                    .map(|surface| {
                        bindings
                            .iter()
                            .find(|binding| binding.semantic_surface_identity() == surface)
                            .copied()
                            .ok_or(UiMountedFramePreparationDenial::MissingSurfaceBinding(
                                surface,
                            ))
                    })
                    .collect()
            }
        }
    }
}

impl UiMountedSurfaceReceipt {
    pub fn requirement(&self) -> UiMountedSurfaceBindingRequirement {
        self.requirement
    }

    pub fn projection(&self) -> &UiMountedProjectionView {
        &self.projection
    }
}

impl UiMountedFrameReceipt {
    pub fn canonical_core(&self) -> UiMountedFrameCanonicalCore {
        self.canonical_core
    }

    pub fn integrity(&self) -> UiMountedFrameIntegrity {
        self.integrity
    }

    pub fn surface_count(&self) -> usize {
        self.surface_count
    }
}

impl UiPreparedMountedFrame {
    pub(crate) fn admit(
        candidate: super::UiProjectedMountedFrameCandidate,
        generation: WorthUiPreparedApplicationGenerationIdentity,
        manifest: UiMountedFrameManifest,
        graph_world: u64,
        allocation_truth_revision: u64,
    ) -> Result<Self, UiMountedFramePreparationDenial> {
        validate_manifest(&manifest)?;
        let surfaces = manifest
            .surfaces()
            .iter()
            .map(|requirement| {
                let projection = candidate
                    .frame()
                    .view_for(requirement.binding())
                    .expect("validated manifest binding is present in finalized projection");
                Ok(UiMountedSurfaceReceipt {
                    requirement: *requirement,
                    projection,
                })
            })
            .collect::<Result<Vec<_>, UiMountedFramePreparationDenial>>()?;
        let canonical_core = UiMountedFrameCanonicalCore::new(
            candidate.frame().frame_identity(),
            candidate.frame().plan_digest(),
            graph_world,
            allocation_truth_revision,
            table_range_digest(&surfaces),
        );
        let integrity = UiMountedFrameIntegrity::derive(canonical_core, &manifest);
        if !integrity.verifies(canonical_core, &manifest) {
            return Err(UiMountedFramePreparationDenial::IntegrityMismatch);
        }
        Ok(Self {
            candidate,
            generation,
            manifest,
            canonical_core,
            integrity,
            surfaces: surfaces.into_boxed_slice(),
        })
    }

    pub fn generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation
    }

    pub fn manifest(&self) -> &UiMountedFrameManifest {
        &self.manifest
    }

    pub fn canonical_core(&self) -> UiMountedFrameCanonicalCore {
        self.canonical_core
    }

    pub fn integrity(&self) -> UiMountedFrameIntegrity {
        self.integrity
    }

    pub fn surfaces(&self) -> &[UiMountedSurfaceReceipt] {
        &self.surfaces
    }

    pub fn receipt(&self) -> UiMountedFrameReceipt {
        UiMountedFrameReceipt {
            canonical_core: self.canonical_core,
            integrity: self.integrity,
            surface_count: self.surfaces.len(),
        }
    }

    pub fn is_unpublished(&self) -> bool {
        self.candidate.is_unpublished()
    }

    pub(crate) fn presented_receipts(
        &self,
    ) -> impl Iterator<
        Item = (
            worth_ui_host_contract::UiMountedInstanceIdentity,
            worth_ui_host_contract::UiMountedNodeReceiptIdentity,
        ),
    > + '_ {
        self.candidate.presented_receipts()
    }

    pub(crate) fn into_publication_parts(
        self,
    ) -> (
        super::UiProjectedMountedFrameCandidate,
        UiMountedFrameManifest,
        UiMountedFrameCanonicalCore,
    ) {
        (self.candidate, self.manifest, self.canonical_core)
    }
}

pub(crate) fn validate_manifest(
    manifest: &UiMountedFrameManifest,
) -> Result<(), UiMountedFramePreparationDenial> {
    let mut surfaces = BTreeSet::new();
    let mut bindings = BTreeSet::new();
    for requirement in manifest.surfaces() {
        if !surfaces.insert(requirement.semantic_surface())
            || !bindings.insert(requirement.binding())
        {
            return Err(UiMountedFramePreparationDenial::IncompleteManifest);
        }
    }
    let expected = surfaces
        .iter()
        .flat_map(|surface| {
            [
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Ordinary,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Virtualized,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::CanvasSpatial,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Realtime,
                ),
                (
                    *surface,
                    worth_ui_host_contract::UiMountedLaneParticipation::Preview,
                ),
            ]
        })
        .collect::<BTreeSet<_>>();
    let actual = manifest
        .lane_contributions()
        .iter()
        .map(|cell| (cell.surface(), cell.lane()))
        .collect::<BTreeSet<_>>();
    if actual != expected || actual.len() != manifest.lane_contributions().len() {
        return Err(UiMountedFramePreparationDenial::IncompleteManifest);
    }
    Ok(())
}

fn table_range_digest(surfaces: &[UiMountedSurfaceReceipt]) -> u64 {
    surfaces.iter().fold(0_u64, |digest, receipt| {
        let view = receipt.projection();
        [
            view.nodes().len(),
            view.clips().rows().len(),
            view.layers().rows().len(),
            view.paint_batches().rows().len(),
            view.spatial_batches().rows().len(),
            view.realtime_batches().rows().len(),
            view.resources().entries().len(),
        ]
        .into_iter()
        .fold(
            digest ^ view.binding().diagnostic_value(),
            |value, length| value.rotate_left(7) ^ u64::try_from(length).unwrap_or(u64::MAX),
        )
    })
}

pub(crate) fn binding_requirement(
    binding: super::UiSurfaceBindingIdentityView,
) -> UiMountedSurfaceBindingRequirement {
    UiMountedSurfaceBindingRequirement::new(
        binding.semantic_surface_identity(),
        binding.host_surface_identity(),
        binding.binding_generation(),
        binding.capability_observation_generation(),
        binding.capability_profile_digest(),
        binding.presentation_mode(),
    )
}
