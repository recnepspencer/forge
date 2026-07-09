use worth_query::facade::{WORTHQueryRecoveryAuthoritySurface, WORTHQueryRecoveryStopFamily};

use crate::aspect_authority::HadwigerAspectPosture;
use crate::domain_artifacts::{
    HadwigerArtifactShapeError, HadwigerCanonicalArtifact, HadwigerCheckerPosture,
    HadwigerDeclaredFamilyCheckedExt, HadwigerQueryDeclarationReference,
};
use crate::domain_declarations::{
    HadwigerResearchDeclarationShapeError, PartialAdmissionExplanationDeclaration,
    RejectionExplanationDeclaration,
};
use crate::proof_claims::HadwigerProofClaimBlockerKind;
use crate::query_entry::HadwigerResearchHandle;

use super::artifacts::{
    HadwigerConservativeEscalationExplanation, HadwigerPartialAdmissionExplanation,
    HadwigerQueryRecoveryExplanation, HadwigerRejectionExplanation,
};
use super::evidence::{
    HadwigerExplanationAuthoritySurface, HadwigerExplanationStopFamily, HadwigerRepairObligation,
    HadwigerReusableNegativeEvidence, HadwigerSurvivingEvidenceReport,
};
use super::requests::{
    graph_reference, ExplainPartialAdmissionRequest, ExplainRejectionRequest,
    HadwigerQueryRecoveryExplanationRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerExplanationError {
    Shape(HadwigerArtifactShapeError),
    DeclarationShape(HadwigerResearchDeclarationShapeError),
    QueryDeclarationNotAdmitted { declaration_kind: &'static str },
    CheckerVerificationNotRejected,
}

impl From<HadwigerArtifactShapeError> for HadwigerExplanationError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

impl From<HadwigerResearchDeclarationShapeError> for HadwigerExplanationError {
    fn from(value: HadwigerResearchDeclarationShapeError) -> Self {
        Self::DeclarationShape(value)
    }
}

pub fn explain_rejection(
    handle: &HadwigerResearchHandle,
    request: ExplainRejectionRequest,
) -> Result<HadwigerRejectionExplanation, HadwigerExplanationError> {
    require_rejected_checker_verification(&request)?;
    let query_reference = rejection_query_reference(handle, &request)?;
    let repair_obligations = request.repair_obligations().to_vec();
    let rejected_aspect_token = request
        .rejected_aspect()
        .map(|aspect| aspect.stable_token());
    let reusable_negative_evidence = HadwigerReusableNegativeEvidence::new(
        request.checker_verification().reference(),
        request.explanation_id(),
        graph_reference(request.graph_version()).stable_token(),
        repair_hint(&repair_obligations),
    )?;
    Ok(HadwigerRejectionExplanation::checker_rejection(
        query_reference,
        request.checker_verification(),
        rejected_aspect_token,
        repair_obligations,
        reusable_negative_evidence,
    )?)
}

pub fn explain_partial_admission(
    handle: &HadwigerResearchHandle,
    request: ExplainPartialAdmissionRequest,
) -> Result<HadwigerPartialAdmissionExplanation, HadwigerExplanationError> {
    let query_reference = partial_query_reference(handle, &request)?;
    let repair_obligations = request.repair_obligations().to_vec();
    let surviving_evidence =
        HadwigerSurvivingEvidenceReport::new(request.surviving_artifacts().to_vec());
    let conservative_escalation = conservative_escalation_for_blocked_claim(&request)?;
    Ok(HadwigerPartialAdmissionExplanation::blocked_proof_claim(
        query_reference,
        request.explanation_id(),
        request.blocked_claim().clone(),
        surviving_evidence,
        repair_obligations,
        conservative_escalation,
    )?)
}

pub fn explain_query_recovery_brief(
    handle: &HadwigerResearchHandle,
    request: HadwigerQueryRecoveryExplanationRequest,
) -> Result<HadwigerQueryRecoveryExplanation, HadwigerExplanationError> {
    let declaration =
        RejectionExplanationDeclaration::try_new(request.explanation_id(), "query_recovery")?;
    let query_reference = admitted_query_reference(
        handle
            .declare_checked(declaration)
            .admitted()
            .map(Into::into),
        "query_recovery_explanation",
    )?;
    let stop_family = explanation_stop_family(request.recovery_brief().stop_family());
    let authority_surface =
        explanation_authority_surface(request.recovery_brief().authority_surface());
    Ok(HadwigerQueryRecoveryExplanation::query_recovery(
        query_reference,
        stop_family,
        authority_surface,
        request.recovery_brief().clone(),
    )?)
}

fn require_rejected_checker_verification(
    request: &ExplainRejectionRequest,
) -> Result<(), HadwigerExplanationError> {
    if request.checker_verification().posture() == HadwigerCheckerPosture::Rejected {
        Ok(())
    } else {
        Err(HadwigerExplanationError::CheckerVerificationNotRejected)
    }
}

fn rejection_query_reference(
    handle: &HadwigerResearchHandle,
    request: &ExplainRejectionRequest,
) -> Result<HadwigerQueryDeclarationReference, HadwigerExplanationError> {
    let declaration = RejectionExplanationDeclaration::try_new(
        request.graph_version().version_id(),
        request.explanation_id(),
    )?;
    admitted_query_reference(
        handle
            .declare_checked(declaration)
            .admitted()
            .map(Into::into),
        "rejection_explanation",
    )
}

fn partial_query_reference(
    handle: &HadwigerResearchHandle,
    request: &ExplainPartialAdmissionRequest,
) -> Result<HadwigerQueryDeclarationReference, HadwigerExplanationError> {
    let declaration =
        PartialAdmissionExplanationDeclaration::try_new(request.graph_version().version_id())?;
    admitted_query_reference(
        handle
            .declare_checked(declaration)
            .admitted()
            .map(Into::into),
        "partial_admission_explanation",
    )
}

fn admitted_query_reference(
    reference: Option<HadwigerQueryDeclarationReference>,
    declaration_kind: &'static str,
) -> Result<HadwigerQueryDeclarationReference, HadwigerExplanationError> {
    reference.ok_or(HadwigerExplanationError::QueryDeclarationNotAdmitted { declaration_kind })
}

fn repair_hint(repair_obligations: &[HadwigerRepairObligation]) -> String {
    repair_obligations
        .first()
        .map(|obligation| obligation.detail().to_string())
        .unwrap_or_else(|| {
            "retain failure until new checked evidence changes the basis".to_string()
        })
}

fn conservative_escalation_for_blocked_claim(
    request: &ExplainPartialAdmissionRequest,
) -> Result<Option<HadwigerConservativeEscalationExplanation>, HadwigerArtifactShapeError> {
    let Some((reason, posture)) = request
        .blocked_claim()
        .blockers()
        .iter()
        .find_map(|blocker| {
            conservative_escalation_basis(
                blocker.blocker_kind(),
                blocker.reason(),
                blocker.observed_posture(),
            )
        })
    else {
        return Ok(None);
    };
    HadwigerConservativeEscalationExplanation::new(
        graph_reference(request.graph_version()),
        reason,
        posture,
    )
    .map(Some)
}

fn conservative_escalation_basis(
    blocker_kind: HadwigerProofClaimBlockerKind,
    reason: &str,
    observed_posture: Option<HadwigerAspectPosture>,
) -> Option<(String, Option<HadwigerAspectPosture>)> {
    if blocker_kind == HadwigerProofClaimBlockerKind::MissingEvidence {
        return Some((reason.to_string(), Some(HadwigerAspectPosture::Missing)));
    }
    match observed_posture {
        Some(HadwigerAspectPosture::Stale)
        | Some(HadwigerAspectPosture::Missing)
        | Some(HadwigerAspectPosture::Conflict) => Some((reason.to_string(), observed_posture)),
        _ => None,
    }
}

fn explanation_stop_family(
    stop_family: WORTHQueryRecoveryStopFamily,
) -> HadwigerExplanationStopFamily {
    match stop_family {
        WORTHQueryRecoveryStopFamily::Binding => HadwigerExplanationStopFamily::QueryBinding,
        WORTHQueryRecoveryStopFamily::Continuation => {
            HadwigerExplanationStopFamily::QueryContinuation
        }
        WORTHQueryRecoveryStopFamily::ContributionComposedOrchestration => {
            HadwigerExplanationStopFamily::QueryContributionComposition
        }
        WORTHQueryRecoveryStopFamily::DeclarationEntry => {
            HadwigerExplanationStopFamily::QueryDeclarationEntry
        }
        WORTHQueryRecoveryStopFamily::DeclarationReceipt
        | WORTHQueryRecoveryStopFamily::DeclarationRoutePlan => {
            HadwigerExplanationStopFamily::QueryRouteOrReceipt
        }
        WORTHQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration => {
            HadwigerExplanationStopFamily::QueryGroupedNeighborhood
        }
        WORTHQueryRecoveryStopFamily::SignalCompatibilityOrchestration => {
            HadwigerExplanationStopFamily::QuerySignalCompatibility
        }
    }
}

fn explanation_authority_surface(
    authority_surface: WORTHQueryRecoveryAuthoritySurface,
) -> HadwigerExplanationAuthoritySurface {
    match authority_surface {
        WORTHQueryRecoveryAuthoritySurface::ContributionComposition => {
            HadwigerExplanationAuthoritySurface::QueryContributionComposition
        }
        WORTHQueryRecoveryAuthoritySurface::DeclarationMeaning
        | WORTHQueryRecoveryAuthoritySurface::HandleIdentity
        | WORTHQueryRecoveryAuthoritySurface::InputNarrowing
        | WORTHQueryRecoveryAuthoritySurface::SupportReadiness => {
            HadwigerExplanationAuthoritySurface::QueryDeclarationProgression
        }
        WORTHQueryRecoveryAuthoritySurface::TruthContinuationContext => {
            HadwigerExplanationAuthoritySurface::ProjectionConsumption
        }
        WORTHQueryRecoveryAuthoritySurface::SignalCompatibility => {
            HadwigerExplanationAuthoritySurface::LowerRuntimeCompatibility
        }
        WORTHQueryRecoveryAuthoritySurface::AdmittedOperatingWorld
        | WORTHQueryRecoveryAuthoritySurface::AutomationBoundary
        | WORTHQueryRecoveryAuthoritySurface::AvailabilityDiscovery
        | WORTHQueryRecoveryAuthoritySurface::BoundInputContext
        | WORTHQueryRecoveryAuthoritySurface::FailureEscalation => {
            HadwigerExplanationAuthoritySurface::QueryRecovery
        }
    }
}
