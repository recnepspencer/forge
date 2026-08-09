use worth_ui_host_contract::{
    UiMountedFrameCanonicalCore, UiMountedFrameIntegrity, UiMountedFrameManifest,
    UiMountedProjectionView, UiMountedSurfaceBindingRequirement, UiSemanticSurfaceIdentity,
};

use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

#[derive(Clone, Debug)]
pub struct UiMountedFrameRequest {
    surfaces: UiMountedSurfaceSelection,
    virtualized_range: Option<crate::runtime::WorthUiVisibleRange>,
    visual_overlay_revision: u64,
    visual_overlay: Option<super::UiMountedVisualOverlayProjectionInput>,
    reuse_identity: UiMountedFrameRequestIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum UiMountedSurfaceSelection {
    AllBound,
    Exact(std::rc::Rc<[UiSemanticSurfaceIdentity]>),
}

#[derive(Clone)]
pub(crate) struct UiMountedFrameRequestIdentity(std::rc::Rc<()>);

#[derive(Debug, PartialEq)]
pub enum UiMountedFramePreparationDenial {
    DuplicateSurfaceRequirement,
    MissingSurfaceBinding(UiSemanticSurfaceIdentity),
    SurfaceRebindRequired(UiSemanticSurfaceIdentity),
    LaneWorkUnavailable(worth_ui_host_contract::UiMountedLaneParticipation),
    Lane(crate::facade::WorthUiMountedLaneProjectionDenial),
    Projection(super::UiMountedProjectionDenial),
    TraceSourceGenerationMismatch,
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
    cost: super::UiMountCostReport,
}

pub struct UiPreparedMountedFrame {
    candidate: super::UiProjectedMountedFrameCandidate,
    generation: WorthUiPreparedApplicationGenerationIdentity,
    manifest: UiMountedFrameManifest,
    canonical_core: UiMountedFrameCanonicalCore,
    integrity: UiMountedFrameIntegrity,
    surfaces: Box<[UiMountedSurfaceReceipt]>,
    identity_trace_basis: super::UiMountedIdentityTraceBasis,
    cost: super::UiMountCostReport,
    reuse_contract: super::UiMountedFrameReuseContract,
}

pub(crate) struct UiPreparedMountedFrameAdmission {
    pub candidate: super::UiProjectedMountedFrameCandidate,
    pub generation: WorthUiPreparedApplicationGenerationIdentity,
    pub manifest: UiMountedFrameManifest,
    pub graph_world: u64,
    pub allocation_truth_revision: u64,
    pub trace_source:
        crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    pub reuse_contract: super::UiMountedFrameReuseContract,
}

impl UiMountedFrameRequest {
    pub fn all_bound_surfaces() -> Self {
        Self {
            surfaces: UiMountedSurfaceSelection::AllBound,
            virtualized_range: None,
            visual_overlay_revision: 0,
            visual_overlay: None,
            reuse_identity: UiMountedFrameRequestIdentity(std::rc::Rc::new(())),
        }
    }

    pub fn exact_surfaces(surfaces: Vec<UiSemanticSurfaceIdentity>) -> Self {
        Self {
            surfaces: UiMountedSurfaceSelection::Exact(surfaces.into()),
            virtualized_range: None,
            visual_overlay_revision: 0,
            visual_overlay: None,
            reuse_identity: UiMountedFrameRequestIdentity(std::rc::Rc::new(())),
        }
    }

    pub fn with_virtualized_range(mut self, range: crate::runtime::WorthUiVisibleRange) -> Self {
        self.virtualized_range = Some(range);
        self.reuse_identity = UiMountedFrameRequestIdentity(std::rc::Rc::new(()));
        self
    }

    pub fn virtualized_range(&self) -> Option<crate::runtime::WorthUiVisibleRange> {
        self.virtualized_range
    }

    pub(crate) fn with_visual_overlay(
        mut self,
        revision: u64,
        visual_overlay: Option<super::UiMountedVisualOverlayProjectionInput>,
    ) -> Self {
        self.visual_overlay_revision = revision;
        self.visual_overlay = visual_overlay;
        self.reuse_identity = UiMountedFrameRequestIdentity(std::rc::Rc::new(()));
        self
    }

    pub(crate) const fn visual_overlay_revision(&self) -> u64 {
        self.visual_overlay_revision
    }

    pub(crate) const fn visual_overlay(
        &self,
    ) -> Option<super::UiMountedVisualOverlayProjectionInput> {
        self.visual_overlay
    }

    pub(crate) fn reuse_identity(&self) -> UiMountedFrameRequestIdentity {
        self.reuse_identity.clone()
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

impl PartialEq for UiMountedFrameRequest {
    fn eq(&self, other: &Self) -> bool {
        self.surfaces == other.surfaces
            && self.virtualized_range == other.virtualized_range
            && self.visual_overlay_revision == other.visual_overlay_revision
            && self.visual_overlay == other.visual_overlay
    }
}

impl Eq for UiMountedFrameRequest {}

impl PartialEq for UiMountedFrameRequestIdentity {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for UiMountedFrameRequestIdentity {}

impl std::fmt::Debug for UiMountedFrameRequestIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("UiMountedFrameRequestIdentity")
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

    pub fn cost_report(&self) -> super::UiMountCostReport {
        self.cost
    }

    pub fn delta(&self) -> super::UiMountedFrameDelta {
        super::UiMountedFrameDelta::from_cost(self.cost)
    }
}

impl UiPreparedMountedFrame {
    pub(crate) fn admit(
        admission: UiPreparedMountedFrameAdmission,
    ) -> Result<Self, UiMountedFramePreparationDenial> {
        let UiPreparedMountedFrameAdmission {
            candidate,
            generation,
            manifest,
            graph_world,
            allocation_truth_revision,
            trace_source,
            reuse_contract,
        } = admission;
        if trace_source.generation() != &generation {
            return Err(UiMountedFramePreparationDenial::TraceSourceGenerationMismatch);
        }
        super::validate_manifest(&manifest)?;
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
        let cost = candidate.frame().cost_report();
        let identity_trace_basis = candidate.frame().identity_trace_basis(trace_source);
        Ok(Self {
            candidate,
            generation,
            manifest,
            canonical_core,
            integrity,
            surfaces: surfaces.into_boxed_slice(),
            identity_trace_basis,
            cost,
            reuse_contract,
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
            cost: self.cost,
        }
    }

    pub fn cost_report(&self) -> super::UiMountCostReport {
        self.cost
    }

    pub fn reuse_contract(&self) -> &super::UiMountedFrameReuseContract {
        &self.reuse_contract
    }

    pub(crate) fn visual_region_basis(&self) -> super::UiMountedVisualRegionBasis {
        super::UiMountedVisualRegionBasis::new(
            self.candidate.frame().static_paint_rows(),
            self.candidate.frame().hit_test_rows(),
        )
    }

    pub(crate) fn identity_trace_basis(&self) -> &super::UiMountedIdentityTraceBasis {
        &self.identity_trace_basis
    }

    pub fn is_unpublished(&self) -> bool {
        self.candidate.is_unpublished()
    }

    pub(crate) fn presented_receipt_basis(&self) -> &super::UiMountedNodeReceiptBasis {
        self.candidate.presented_receipt_basis()
    }

    pub(crate) fn into_publication_parts(
        self,
    ) -> (
        super::UiProjectedMountedFrameCandidate,
        UiMountedFrameManifest,
        UiMountedFrameCanonicalCore,
        super::UiMountedFrameReuseContract,
    ) {
        (
            self.candidate,
            self.manifest,
            self.canonical_core,
            self.reuse_contract,
        )
    }
}

fn table_range_digest(surfaces: &[UiMountedSurfaceReceipt]) -> u64 {
    surfaces.iter().fold(0_u64, |digest, receipt| {
        let view = receipt.projection();
        [
            view.nodes().len(),
            view.clips().rows().len(),
            view.layers().rows().len(),
            view.hit_tests().rows().len(),
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
        .rotate_left(11)
            ^ view
                .filled_rects()
                .rows()
                .iter()
                .fold(0_u64, |paint_digest, row| {
                    paint_digest.rotate_left(9) ^ row.semantic_digest()
                })
            ^ view
                .hit_tests()
                .rows()
                .iter()
                .fold(0_u64, |hit_digest, row| {
                    hit_digest.rotate_left(9) ^ row.semantic_digest()
                })
    })
}

pub(crate) fn binding_requirement(
    binding: super::UiSurfaceBindingIdentityView,
) -> UiMountedSurfaceBindingRequirement {
    UiMountedSurfaceBindingRequirement::with_baseline(
        binding.semantic_surface_identity(),
        binding.host_surface_identity(),
        binding.binding_generation(),
        binding.capability_observation_generation(),
        binding.capability_profile_digest(),
        binding.presentation_mode(),
        binding.baseline(),
    )
}

#[cfg(test)]
#[path = "assembly_visual_overlay_tests.rs"]
mod visual_overlay_tests;
