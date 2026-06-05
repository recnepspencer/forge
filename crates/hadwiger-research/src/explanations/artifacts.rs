use forge_query::facade::ForgeQueryRecoveryBrief;

use crate::aspect_authority::HadwigerAspectPosture;
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{
    HadwigerCanonicalArtifact, HadwigerQueryDeclarationReference, UnitDistanceVerification,
};
use crate::proof_claims::HadwigerBlockedProofClaim;

use super::evidence::{
    HadwigerExplanationAuthoritySurface, HadwigerExplanationStopFamily, HadwigerRepairObligation,
    HadwigerReusableNegativeEvidence, HadwigerSurvivingEvidenceReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerRejectionExplanation {
    core: HadwigerArtifactCore,
    stop_family: HadwigerExplanationStopFamily,
    authority_surface: HadwigerExplanationAuthoritySurface,
    checker_artifact_reference: HadwigerArtifactReference,
    rejected_aspect_token: Option<String>,
    repair_obligations: Vec<HadwigerRepairObligation>,
    reusable_negative_evidence: Option<HadwigerReusableNegativeEvidence>,
}

impl HadwigerRejectionExplanation {
    pub(crate) fn checker_rejection(
        query_reference: HadwigerQueryDeclarationReference,
        checker_verification: &UnitDistanceVerification,
        rejected_aspect_token: Option<String>,
        repair_obligations: Vec<HadwigerRepairObligation>,
        reusable_negative_evidence: HadwigerReusableNegativeEvidence,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let checker_artifact_reference = checker_verification.reference();
        let mut parents = vec![
            checker_artifact_reference.clone(),
            reusable_negative_evidence.reference(),
        ];
        parents.extend(
            repair_obligations
                .iter()
                .map(HadwigerRepairObligation::reference),
        );
        let core = explanation_core(
            HadwigerArtifactKind::RejectionExplanation,
            query_reference,
            parents,
            HadwigerExplanationStopFamily::CheckerRejection,
            HadwigerExplanationAuthoritySurface::CheckerArtifact,
            rejected_aspect_token.clone(),
        )?;
        Ok(Self {
            core,
            stop_family: HadwigerExplanationStopFamily::CheckerRejection,
            authority_surface: HadwigerExplanationAuthoritySurface::CheckerArtifact,
            checker_artifact_reference,
            rejected_aspect_token,
            repair_obligations,
            reusable_negative_evidence: Some(reusable_negative_evidence),
        })
    }

    pub fn stop_family(&self) -> HadwigerExplanationStopFamily {
        self.stop_family
    }

    pub fn authority_surface(&self) -> HadwigerExplanationAuthoritySurface {
        self.authority_surface
    }

    pub fn checker_artifact_reference(&self) -> &HadwigerArtifactReference {
        &self.checker_artifact_reference
    }

    pub fn rejected_aspect_token(&self) -> Option<&str> {
        self.rejected_aspect_token.as_deref()
    }

    pub fn repair_obligations(&self) -> &[HadwigerRepairObligation] {
        &self.repair_obligations
    }

    pub fn reusable_negative_evidence(&self) -> Option<&HadwigerReusableNegativeEvidence> {
        self.reusable_negative_evidence.as_ref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerRejectionExplanation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerPartialAdmissionExplanation {
    core: HadwigerArtifactCore,
    stop_family: HadwigerExplanationStopFamily,
    blocked_claim: HadwigerBlockedProofClaim,
    surviving_evidence: HadwigerSurvivingEvidenceReport,
    repair_obligations: Vec<HadwigerRepairObligation>,
    conservative_escalation: Option<HadwigerConservativeEscalationExplanation>,
}

impl HadwigerPartialAdmissionExplanation {
    pub(crate) fn blocked_proof_claim(
        query_reference: HadwigerQueryDeclarationReference,
        explanation_id: impl Into<String>,
        blocked_claim: HadwigerBlockedProofClaim,
        surviving_evidence: HadwigerSurvivingEvidenceReport,
        repair_obligations: Vec<HadwigerRepairObligation>,
        conservative_escalation: Option<HadwigerConservativeEscalationExplanation>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let explanation_id = require_non_empty(explanation_id, "explanation_id")?;
        let mut parents = vec![blocked_claim.proof_claim().reference()];
        parents.extend(surviving_evidence.surviving_artifacts().iter().cloned());
        parents.extend(
            repair_obligations
                .iter()
                .map(HadwigerRepairObligation::reference),
        );
        if let Some(escalation) = conservative_escalation.as_ref() {
            parents.push(escalation.reference());
        }
        let core = explanation_core(
            HadwigerArtifactKind::PartialAdmissionExplanation,
            query_reference,
            parents,
            HadwigerExplanationStopFamily::ProofClaimBlocked,
            HadwigerExplanationAuthoritySurface::HadwigerProofAuthority,
            Some(format!(
                "{}:{}",
                explanation_id,
                blocked_claim.authority_chain().chain_digest()
            )),
        )?;
        Ok(Self {
            core,
            stop_family: HadwigerExplanationStopFamily::ProofClaimBlocked,
            blocked_claim,
            surviving_evidence,
            repair_obligations,
            conservative_escalation,
        })
    }

    pub fn stop_family(&self) -> HadwigerExplanationStopFamily {
        self.stop_family
    }

    pub fn blocked_claim(&self) -> &HadwigerBlockedProofClaim {
        &self.blocked_claim
    }

    pub fn surviving_evidence(&self) -> &[HadwigerArtifactReference] {
        self.surviving_evidence.surviving_artifacts()
    }

    pub fn repair_obligations(&self) -> &[HadwigerRepairObligation] {
        &self.repair_obligations
    }

    pub fn conservative_escalation(&self) -> Option<&HadwigerConservativeEscalationExplanation> {
        self.conservative_escalation.as_ref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerPartialAdmissionExplanation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerQueryRecoveryExplanation {
    core: HadwigerArtifactCore,
    stop_family: HadwigerExplanationStopFamily,
    authority_surface: HadwigerExplanationAuthoritySurface,
    recovery_brief: ForgeQueryRecoveryBrief,
}

impl HadwigerQueryRecoveryExplanation {
    pub(crate) fn query_recovery(
        query_reference: HadwigerQueryDeclarationReference,
        stop_family: HadwigerExplanationStopFamily,
        authority_surface: HadwigerExplanationAuthoritySurface,
        recovery_brief: ForgeQueryRecoveryBrief,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = explanation_core(
            HadwigerArtifactKind::QueryRecoveryExplanation,
            query_reference,
            Vec::new(),
            stop_family,
            authority_surface,
            Some(recovery_brief_token(&recovery_brief)),
        )?;
        Ok(Self {
            core,
            stop_family,
            authority_surface,
            recovery_brief,
        })
    }

    pub fn is_query_owned(&self) -> bool {
        true
    }

    pub fn query_recovery_brief(&self) -> Option<&ForgeQueryRecoveryBrief> {
        Some(&self.recovery_brief)
    }

    pub fn stop_family(&self) -> HadwigerExplanationStopFamily {
        self.stop_family
    }

    pub fn authority_surface(&self) -> HadwigerExplanationAuthoritySurface {
        self.authority_surface
    }

    pub fn has_retained_grouped_member_context(&self) -> bool {
        self.recovery_brief
            .explanation()
            .has_retained_grouped_member_aspect_context()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerQueryRecoveryExplanation, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerConservativeEscalationExplanation {
    core: HadwigerArtifactCore,
    reason: String,
    observed_posture: Option<HadwigerAspectPosture>,
}

impl HadwigerConservativeEscalationExplanation {
    pub fn new(
        affected_artifact: HadwigerArtifactReference,
        reason: impl Into<String>,
        observed_posture: Option<HadwigerAspectPosture>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let reason = require_non_empty(reason, "conservative_escalation_reason")?;
        let core = artifact_core(
            HadwigerArtifactKind::ConservativeEscalationExplanation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hadwiger_conservative_escalation".to_string(),
            },
            vec![affected_artifact],
            vec![
                HadwigerArtifactPayloadEntry::text("reason", reason.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "observed_posture",
                    observed_posture
                        .map(|posture| posture.as_str())
                        .unwrap_or("unknown"),
                ),
            ],
        )?;
        Ok(Self {
            core,
            reason,
            observed_posture,
        })
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn observed_posture(&self) -> Option<HadwigerAspectPosture> {
        self.observed_posture
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerConservativeEscalationExplanation, core);

fn explanation_core(
    artifact_kind: HadwigerArtifactKind,
    query_reference: HadwigerQueryDeclarationReference,
    parents: Vec<HadwigerArtifactReference>,
    stop_family: HadwigerExplanationStopFamily,
    authority_surface: HadwigerExplanationAuthoritySurface,
    source_token: Option<String>,
) -> Result<HadwigerArtifactCore, HadwigerArtifactShapeError> {
    let mut entries = vec![
        HadwigerArtifactPayloadEntry::text("stop_family", stop_family.as_str()),
        HadwigerArtifactPayloadEntry::text("authority_surface", authority_surface.as_str()),
        HadwigerArtifactPayloadEntry::text("query_reference", query_reference.stable_token()),
    ];
    if let Some(source_token) = source_token {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "source_token",
            source_token,
        ));
    }
    artifact_core(
        artifact_kind,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::QueryDeclaration(query_reference),
        parents,
        entries,
    )
}

pub(crate) fn recovery_brief_token(brief: &ForgeQueryRecoveryBrief) -> String {
    format!(
        "{:?}:{:?}:{:?}:{:?}:{}",
        brief.stop_family(),
        brief.stop_kind(),
        brief.authority_surface(),
        brief.recommended_action(),
        brief.reason()
    )
}
