use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_parameter_space_direction_to_coedge,
    attach_parameter_space_direction_to_edge, attach_parameter_space_direction_to_face,
    attach_parameter_space_point_to_coedge, attach_parameter_space_point_to_edge,
    attach_parameter_space_point_to_face, attach_pcurve_to_coedge, attach_surface_to_face,
    attach_vertex_geometry, SpatialAdmittedPrimitiveBinding, SpatialAnchorAuthorityError,
    SpatialBindingAuthorityError, SpatialBindingKind,
};

use crate::binding::authoring::{
    canonical_entries::canonical_query_entries_for_intent,
    query_domain::PrimitiveBindingQueryDomain, AuthorPrimitiveBindingIntent,
};
use crate::binding::workflow_boundary::{
    canonical_query_workflow_artifacts, KernelCanonicalQueryWorkflowArtifactSet,
    KernelWorkflowBoundaryError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveBindingDeclarationEntry {
    intent: AuthorPrimitiveBindingIntent,
}

impl PrimitiveBindingDeclarationEntry {
    pub(crate) fn new(intent: AuthorPrimitiveBindingIntent) -> Self {
        Self { intent }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        match &self.intent {
            AuthorPrimitiveBindingIntent::AttachSurfaceToFace(_) => SpatialBindingKind::FaceSurface,
            AuthorPrimitiveBindingIntent::AttachCurveToEdge(_) => SpatialBindingKind::EdgeCurve,
            AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(_) => {
                SpatialBindingKind::CoedgePCurve
            }
            AuthorPrimitiveBindingIntent::AttachVertexGeometry(_) => {
                SpatialBindingKind::VertexGeometry
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToFace(_, _) => {
                SpatialBindingKind::FaceSurface
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToEdge(_, _) => {
                SpatialBindingKind::EdgeCurve
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToCoedge(_, _) => {
                SpatialBindingKind::CoedgePCurve
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToFace(_, _) => {
                SpatialBindingKind::FaceSurface
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToEdge(_, _) => {
                SpatialBindingKind::EdgeCurve
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToCoedge(_, _) => {
                SpatialBindingKind::CoedgePCurve
            }
        }
    }

    pub fn progress_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<PrimitiveBindingQueryDomain, Self>,
        ForgeQueryDeclarationEntryProgressionError<PrimitiveBindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
    {
        handle.declare_review_and_progress(self.clone())
    }

    pub fn ordinary_outcome_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<PrimitiveBindingQueryDomain, Self>>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
    {
        handle.orchestrate_declaration_entry_outcome(self.clone())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn canonical_workflow_artifacts_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
    ) -> Result<
        KernelCanonicalQueryWorkflowArtifactSet<PrimitiveBindingQueryDomain, Self>,
        KernelWorkflowBoundaryError<PrimitiveBindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
    {
        canonical_query_workflow_artifacts(handle, self.clone())
    }

    pub(crate) fn canonical_query_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }

    pub fn admit(self) -> Result<SpatialAdmittedPrimitiveBinding, PrimitiveBindingAuthoringError> {
        match self.intent {
            AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => attach_surface_to_face(spec)
                .map(SpatialAdmittedPrimitiveBinding::FaceSurface)
                .map_err(PrimitiveBindingAuthoringError::Spatial),
            AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => attach_curve_to_edge(spec)
                .map(SpatialAdmittedPrimitiveBinding::EdgeCurve)
                .map_err(PrimitiveBindingAuthoringError::Spatial),
            AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => {
                attach_pcurve_to_coedge(spec)
                    .map(SpatialAdmittedPrimitiveBinding::CoedgePCurve)
                    .map_err(PrimitiveBindingAuthoringError::Spatial)
            }
            AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => {
                attach_vertex_geometry(spec)
                    .map(SpatialAdmittedPrimitiveBinding::VertexGeometry)
                    .map_err(PrimitiveBindingAuthoringError::Spatial)
            }
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToFace(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_face(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToEdge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_edge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
            AuthorPrimitiveBindingIntent::AttachParameterSpacePointToCoedge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_coedge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToFace(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_face(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToEdge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_edge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
            AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToCoedge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_coedge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor)
                .map_err(PrimitiveBindingAuthoringError::Anchor),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveBindingAuthoringError {
    Spatial(SpatialBindingAuthorityError),
    Anchor(SpatialAnchorAuthorityError),
}

pub fn author_primitive_binding_declaration(
    intent: AuthorPrimitiveBindingIntent,
) -> PrimitiveBindingDeclarationEntry {
    PrimitiveBindingDeclarationEntry::new(intent)
}
