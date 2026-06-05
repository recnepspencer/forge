use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
};
use worth_spatial::facade::bindings::{
    rebind_curve_on_edge, rebind_pcurve_on_coedge, rebind_surface_on_face,
    AdmittedRebindingDecision, SpatialRebindingAuthorityError,
};

use crate::binding::rebinding::{
    canonical_entries::canonical_query_entries_for_intent, AuthorPrimitiveRebindingIntent,
    PrimitiveRebindingQueryDomain,
};
use crate::binding::workflow_boundary::{
    canonical_query_workflow_artifacts, KernelCanonicalQueryWorkflowArtifactSet,
    KernelWorkflowBoundaryError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveRebindingDeclarationEntry {
    intent: AuthorPrimitiveRebindingIntent,
}

impl PrimitiveRebindingDeclarationEntry {
    pub(crate) fn new(intent: AuthorPrimitiveRebindingIntent) -> Self {
        Self { intent }
    }

    pub fn binding_kind(&self) -> worth_spatial::facade::bindings::SpatialBindingKind {
        self.intent.binding_kind()
    }

    pub fn progress_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<PrimitiveRebindingQueryDomain, Self>,
        ForgeQueryDeclarationEntryProgressionError<PrimitiveRebindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        handle.declare_review_and_progress(self.clone())
    }

    pub fn ordinary_outcome_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<PrimitiveRebindingQueryDomain, Self>>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        handle.orchestrate_declaration_entry_outcome(self.clone())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn canonical_workflow_artifacts_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    ) -> Result<
        KernelCanonicalQueryWorkflowArtifactSet<PrimitiveRebindingQueryDomain, Self>,
        KernelWorkflowBoundaryError<PrimitiveRebindingQueryDomain, Self>,
    >
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        canonical_query_workflow_artifacts(handle, self.clone())
    }

    pub(crate) fn canonical_query_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }

    pub fn admit(self) -> Result<AdmittedRebindingDecision, PrimitiveRebindingAuthoringError> {
        match self.intent {
            AuthorPrimitiveRebindingIntent::ReplaceSurfaceBinding {
                prior_binding,
                neighborhood,
            } => rebind_surface_on_face(prior_binding, neighborhood)
                .map_err(PrimitiveRebindingAuthoringError::Spatial),
            AuthorPrimitiveRebindingIntent::ReplaceCurveBinding {
                prior_binding,
                neighborhood,
            } => rebind_curve_on_edge(prior_binding, neighborhood)
                .map_err(PrimitiveRebindingAuthoringError::Spatial),
            AuthorPrimitiveRebindingIntent::ReplacePCurveBinding {
                prior_binding,
                neighborhood,
            } => rebind_pcurve_on_coedge(prior_binding, neighborhood)
                .map_err(PrimitiveRebindingAuthoringError::Spatial),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveRebindingAuthoringError {
    Spatial(SpatialRebindingAuthorityError),
}

pub fn author_primitive_rebinding_declaration(
    intent: AuthorPrimitiveRebindingIntent,
) -> PrimitiveRebindingDeclarationEntry {
    PrimitiveRebindingDeclarationEntry::new(intent)
}
