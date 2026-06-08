use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
};
use worth_spatial::facade::bindings::{
    attach_parameter_space_direction_to_coedge, attach_parameter_space_direction_to_edge,
    attach_parameter_space_direction_to_face, attach_parameter_space_point_to_coedge,
    attach_parameter_space_point_to_edge, attach_parameter_space_point_to_face,
    SpatialAdmittedPrimitiveBinding, SpatialAnchorAuthorityError, SpatialBindingKind,
};

use crate::binding::anchoring::{
    canonical_entries::canonical_query_entries_for_intent, AuthorPrimitiveAnchorBindingIntent,
    PrimitiveAnchorBindingQueryDomain,
};
use crate::binding::workflow_boundary::{
    canonical_query_workflow_artifacts, KernelCanonicalQueryWorkflowArtifactSet,
    KernelWorkflowBoundaryError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveAnchorBindingDeclarationEntry {
    intent: AuthorPrimitiveAnchorBindingIntent,
}

impl PrimitiveAnchorBindingDeclarationEntry {
    pub(crate) fn new(intent: AuthorPrimitiveAnchorBindingIntent) -> Self {
        Self { intent }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        match &self.intent {
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(_, _) => {
                SpatialBindingKind::FaceSurface
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(_, _) => {
                SpatialBindingKind::EdgeCurve
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(_, _) => {
                SpatialBindingKind::CoedgePCurve
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(_, _) => {
                SpatialBindingKind::FaceSurface
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(_, _) => {
                SpatialBindingKind::EdgeCurve
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(_, _) => {
                SpatialBindingKind::CoedgePCurve
            }
        }
    }

    pub fn progress_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<PrimitiveAnchorBindingQueryDomain, Self>,
        ForgeQueryDeclarationEntryProgressionError<PrimitiveAnchorBindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
    {
        handle.declare_review_and_progress(self.clone())
    }

    pub fn ordinary_outcome_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
    ) -> ForgeQueryOrdinaryOutcome<
        ForgeQueryDeclarationEnvelope<PrimitiveAnchorBindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
    {
        handle.orchestrate_declaration_entry_outcome(self.clone())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn canonical_workflow_artifacts_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
    ) -> Result<
        KernelCanonicalQueryWorkflowArtifactSet<PrimitiveAnchorBindingQueryDomain, Self>,
        KernelWorkflowBoundaryError<PrimitiveAnchorBindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
    {
        canonical_query_workflow_artifacts(handle, self.clone())
    }

    pub(crate) fn canonical_query_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }

    pub fn admit(
        self,
    ) -> Result<SpatialAdmittedPrimitiveBinding, PrimitiveAnchorBindingAuthoringError> {
        match self.intent {
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_face(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_edge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_point_to_coedge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_face(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_edge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(
                binding_spec,
                anchor_spec,
            ) => attach_parameter_space_direction_to_coedge(binding_spec, anchor_spec)
                .map(SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor)
                .map_err(PrimitiveAnchorBindingAuthoringError::Anchor),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveAnchorBindingAuthoringError {
    Anchor(SpatialAnchorAuthorityError),
}

pub fn author_primitive_anchor_binding_declaration(
    intent: AuthorPrimitiveAnchorBindingIntent,
) -> PrimitiveAnchorBindingDeclarationEntry {
    PrimitiveAnchorBindingDeclarationEntry::new(intent)
}
