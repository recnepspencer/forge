use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationEnvelope, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind, LowerRuntimeBasisEvidence,
    ScopedInspectionBasis,
};
use worth_spatial::facade::bindings::{
    rebind_curve_on_edge, rebind_geometry_on_vertex, rebind_pcurve_on_coedge,
    rebind_surface_on_face, AdmittedRebindingDecision, SpatialRebindingAuthorityError,
};

use crate::binding::rebinding::{
    canonical_entries::canonical_query_entries_for_intent,
    primitive_rebinding_branch_local_inspection, primitive_rebinding_historical_inspection,
    primitive_rebinding_replay_parity, primitive_rebinding_workflow_transport,
    AuthorPrimitiveRebindingIntent, PrimitiveRebindingBranchLocalInspection,
    PrimitiveRebindingBranchLocalInspectionError, PrimitiveRebindingHistoricalInspection,
    PrimitiveRebindingHistoricalInspectionError, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError,
    PrimitiveRebindingReplaySource,
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
        match primitive_rebinding_workflow_transport(self, handle) {
            Ok(transport) => transport.into_ordinary_outcome(),
            Err(_) => handle.orchestrate_declaration_entry_outcome(self.clone()),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn historical_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        subject: ForgeQueryDeclarationEntryInspectionInput<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        primitive_rebinding_historical_inspection(self, handle, subject)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn branch_local_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        scoped_basis: &ScopedInspectionBasis,
        branch_basis_evidence: LowerRuntimeBasisEvidence,
        subject: ForgeQueryDeclarationEntryInspectionInput<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        primitive_rebinding_branch_local_inspection(
            self,
            handle,
            scoped_basis,
            branch_basis_evidence,
            subject,
        )
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn replay_parity_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        replay_source: PrimitiveRebindingReplaySource,
        other: &Self,
        other_source: PrimitiveRebindingReplaySource,
    ) -> Result<PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        primitive_rebinding_replay_parity(self, replay_source, other, other_source, handle)
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
            AuthorPrimitiveRebindingIntent::ReplaceGeometryBinding {
                prior_binding,
                neighborhood,
            } => rebind_geometry_on_vertex(prior_binding, neighborhood)
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

pub(super) fn rebinding_posture(
    reason: &'static str,
    kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
) -> ForgeQueryOrdinaryPosture {
    ForgeQueryOrdinaryPosture::new(
        reason,
        kind,
        next_step,
        ForgeQueryOrdinaryCheckedTopology::orchestration(
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            None,
            None,
        ),
    )
}
