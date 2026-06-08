use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPostureKind,
};
use worth_spatial::facade::bindings::{AdmittedRebindingDecision, RebindingOutcomeClass};

use crate::binding::rebinding::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingQueryDomain,
};
use crate::binding::workflow_boundary::{
    canonical_query_workflow_artifacts_with_ordinary_shape, ordinary_outcome_shape,
    KernelCanonicalQueryWorkflowArtifactSet, KernelWorkflowBoundaryError,
};

#[allow(dead_code)]
pub(crate) struct PrimitiveRebindingWorkflowTransport {
    artifacts: KernelCanonicalQueryWorkflowArtifactSet<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    decision: AdmittedRebindingDecision,
    ordinary_outcome: ForgeQueryOrdinaryOutcome<
        ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    >,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RebindingOrdinaryOutcomeShape {
    kind: &'static str,
    posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    next_step: Option<ForgeQueryOrdinaryNextStep>,
}

impl RebindingOrdinaryOutcomeShape {
    pub(crate) fn kind(&self) -> &'static str {
        self.kind
    }

    pub(crate) fn posture_kind(&self) -> Option<ForgeQueryOrdinaryPostureKind> {
        self.posture_kind
    }

    pub(crate) fn next_step(&self) -> Option<ForgeQueryOrdinaryNextStep> {
        self.next_step
    }
}

#[allow(dead_code)]
impl PrimitiveRebindingWorkflowTransport {
    pub(crate) fn artifacts(
        &self,
    ) -> &KernelCanonicalQueryWorkflowArtifactSet<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    > {
        &self.artifacts
    }

    pub(crate) fn decision(&self) -> &AdmittedRebindingDecision {
        &self.decision
    }

    pub(crate) fn ordinary_outcome(
        &self,
    ) -> &ForgeQueryOrdinaryOutcome<
        ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    > {
        &self.ordinary_outcome
    }

    pub(crate) fn into_ordinary_outcome(
        self,
    ) -> ForgeQueryOrdinaryOutcome<
        ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    > {
        self.ordinary_outcome
    }
}

#[allow(dead_code)]
pub(crate) enum PrimitiveRebindingWorkflowTransportError {
    WorkflowBoundary(
        KernelWorkflowBoundaryError<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ),
    Spatial(PrimitiveRebindingAuthoringError),
}

pub(crate) fn primitive_rebinding_workflow_transport<C>(
    entry: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingWorkflowTransport, PrimitiveRebindingWorkflowTransportError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let ordinary_outcome = handle.orchestrate_declaration_entry_outcome(entry.clone());
    let (ordinary_outcome_label, ordinary_posture_kind) = ordinary_outcome_shape(&ordinary_outcome);
    let artifacts = entry.clone();
    let artifacts = canonical_query_workflow_artifacts_with_ordinary_shape(
        handle,
        artifacts,
        ordinary_outcome_label,
        ordinary_posture_kind,
    )
    .map_err(PrimitiveRebindingWorkflowTransportError::WorkflowBoundary)?;
    let decision = entry
        .clone()
        .admit()
        .map_err(PrimitiveRebindingWorkflowTransportError::Spatial)?;
    let ordinary_outcome = match ordinary_outcome {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            ordinary_outcome_from_rebinding_decision(decision.clone(), envelope)
        }
        other => other,
    };
    Ok(PrimitiveRebindingWorkflowTransport {
        artifacts,
        decision,
        ordinary_outcome,
    })
}

fn ordinary_outcome_from_rebinding_decision(
    decision: AdmittedRebindingDecision,
    envelope: ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
) -> ForgeQueryOrdinaryOutcome<
    ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
> {
    match decision.outcome_class() {
        RebindingOutcomeClass::Preserved
        | RebindingOutcomeClass::ExactReattachment
        | RebindingOutcomeClass::ContinuityJustifiedReattachment
        | RebindingOutcomeClass::CorrespondenceOnly => ForgeQueryOrdinaryOutcome::Bound(envelope),
        RebindingOutcomeClass::Ambiguous => {
            ForgeQueryOrdinaryOutcome::Ambiguous(super::workflow::rebinding_posture(
                "rebinding remained ambiguous within the admitted local replacement neighborhood",
                forge_query::facade::ForgeQueryOrdinaryPostureKind::Ambiguous,
                forge_query::facade::ForgeQueryOrdinaryNextStep::NarrowInput,
            ))
        }
        RebindingOutcomeClass::Orphaned => {
            ForgeQueryOrdinaryOutcome::RebindRequired(super::workflow::rebinding_posture(
                "rebinding remained orphaned within the admitted local replacement neighborhood",
                forge_query::facade::ForgeQueryOrdinaryPostureKind::RebindRequired,
                forge_query::facade::ForgeQueryOrdinaryNextStep::RebindContext,
            ))
        }
        RebindingOutcomeClass::Unsupported => {
            ForgeQueryOrdinaryOutcome::Unsupported(super::workflow::rebinding_posture(
                "rebinding family is unsupported for the admitted local replacement neighborhood",
                forge_query::facade::ForgeQueryOrdinaryPostureKind::Unsupported,
                forge_query::facade::ForgeQueryOrdinaryNextStep::CheckSupport,
            ))
        }
    }
}

pub(crate) fn ordinary_shape_from_rebinding_decision(
    decision: &AdmittedRebindingDecision,
) -> RebindingOrdinaryOutcomeShape {
    match decision.outcome_class() {
        RebindingOutcomeClass::Preserved
        | RebindingOutcomeClass::ExactReattachment
        | RebindingOutcomeClass::ContinuityJustifiedReattachment
        | RebindingOutcomeClass::CorrespondenceOnly => RebindingOrdinaryOutcomeShape {
            kind: "bound",
            posture_kind: None,
            next_step: None,
        },
        RebindingOutcomeClass::Ambiguous => RebindingOrdinaryOutcomeShape {
            kind: "ambiguous",
            posture_kind: Some(ForgeQueryOrdinaryPostureKind::Ambiguous),
            next_step: Some(ForgeQueryOrdinaryNextStep::NarrowInput),
        },
        RebindingOutcomeClass::Orphaned => RebindingOrdinaryOutcomeShape {
            kind: "rebind_required",
            posture_kind: Some(ForgeQueryOrdinaryPostureKind::RebindRequired),
            next_step: Some(ForgeQueryOrdinaryNextStep::RebindContext),
        },
        RebindingOutcomeClass::Unsupported => RebindingOrdinaryOutcomeShape {
            kind: "unsupported",
            posture_kind: Some(ForgeQueryOrdinaryPostureKind::Unsupported),
            next_step: Some(ForgeQueryOrdinaryNextStep::CheckSupport),
        },
    }
}
