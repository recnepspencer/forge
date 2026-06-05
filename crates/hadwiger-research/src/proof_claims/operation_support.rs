use crate::aspect_authority::{HadwigerAspectKind, HadwigerAspectPosture};
use crate::domain_artifacts::{
    blocked_proof_claim, HadwigerArtifactReference, HadwigerCanonicalArtifact,
    HadwigerProofClaimKind, HadwigerQueryDeclarationReference, ProofClaim,
};
use crate::mathematical_verification::WholePlaneColoringVerification;

use super::admission_results::{
    HadwigerBlockedProofClaim, HadwigerProofClaimAdmissionError, HadwigerProofClaimBlocker,
    HadwigerProofClaimBlockerKind,
};
use super::authority_chain::{
    HadwigerProofAuthorityChain, HadwigerProofAuthorityStep, HadwigerProofAuthorityStepKind,
};
use super::requests::{PlaneExactValueClaimRequest, PlaneLowerBoundClaimRequest};

pub(super) fn missing(reason: &'static str) -> HadwigerProofClaimBlocker {
    HadwigerProofClaimBlocker::local(
        HadwigerProofClaimBlockerKind::MissingEvidence,
        format!("missing {reason}"),
        Some(HadwigerAspectPosture::Missing),
    )
}

pub(super) fn lower_blocked(
    request: &PlaneLowerBoundClaimRequest,
    blocker: HadwigerProofClaimBlocker,
) -> Result<HadwigerBlockedProofClaim, HadwigerProofClaimAdmissionError> {
    blocked_from_parts(
        request.claim_id(),
        HadwigerProofClaimKind::PlaneLowerBound {
            color_count: request
                .not_k_colorable_verification()
                .map(|verification| verification.color_count() + 1)
                .unwrap_or(1),
        },
        request.graph_version().reference(),
        vec![blocker],
    )
}

pub(super) fn blocked_from_parts(
    claim_id: &str,
    claim_kind: HadwigerProofClaimKind,
    parent: HadwigerArtifactReference,
    blockers: Vec<HadwigerProofClaimBlocker>,
) -> Result<HadwigerBlockedProofClaim, HadwigerProofClaimAdmissionError> {
    let proof_claim = blocked_proof_claim(parent, claim_id, claim_kind)?;
    let chain = HadwigerProofAuthorityChain::new(
        &proof_claim,
        blockers
            .iter()
            .filter_map(HadwigerProofClaimBlocker::observed_posture)
            .next()
            .unwrap_or(HadwigerAspectPosture::Missing),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
        false,
        false,
    )?;
    Ok(HadwigerBlockedProofClaim::new(proof_claim, chain, blockers))
}

pub(super) fn lower_chain(
    proof_claim: &ProofClaim,
    query_reference: HadwigerQueryDeclarationReference,
    unit_verification_reference: HadwigerArtifactReference,
    color_verification_reference: HadwigerArtifactReference,
    unit_aspect_token: String,
    color_aspect_token: String,
) -> Result<HadwigerProofAuthorityChain, HadwigerProofClaimAdmissionError> {
    HadwigerProofAuthorityChain::new(
        proof_claim,
        HadwigerAspectPosture::Admitted,
        vec![
            HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::QueryDeclaration,
                "plane lower-bound claim declaration",
                query_reference.stable_token(),
            ),
            HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::CheckerArtifact,
                "unit-distance verification",
                unit_verification_reference.stable_token(),
            ),
            HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::CheckerArtifact,
                "not-k-colorable verification",
                color_verification_reference.stable_token(),
            ),
            HadwigerProofAuthorityStep::aspect(
                "unit-distance aspect",
                unit_aspect_token.clone(),
                HadwigerAspectKind::UnitDistanceEmbedding,
                HadwigerAspectPosture::Admitted,
            ),
            HadwigerProofAuthorityStep::aspect(
                "not-k-colorable aspect",
                color_aspect_token.clone(),
                HadwigerAspectKind::NotKColorable,
                HadwigerAspectPosture::Admitted,
            ),
        ],
        vec![query_reference],
        vec![unit_verification_reference, color_verification_reference],
        vec![unit_aspect_token, color_aspect_token],
        None,
        false,
        false,
    )
    .map_err(Into::into)
}

pub(super) fn upper_chain(
    proof_claim: &ProofClaim,
    query_reference: HadwigerQueryDeclarationReference,
    verification: &WholePlaneColoringVerification,
) -> Result<HadwigerProofAuthorityChain, HadwigerProofClaimAdmissionError> {
    HadwigerProofAuthorityChain::new(
        proof_claim,
        HadwigerAspectPosture::Admitted,
        vec![
            HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::QueryDeclaration,
                "plane upper-bound claim declaration",
                query_reference.stable_token(),
            ),
            HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::CheckerArtifact,
                "whole-plane coloring verification",
                verification.reference().stable_token(),
            ),
        ],
        vec![query_reference],
        vec![verification.reference()],
        Vec::new(),
        None,
        true,
        false,
    )
    .map_err(Into::into)
}

pub(super) fn exact_chain(
    proof_claim: &ProofClaim,
    query_reference: HadwigerQueryDeclarationReference,
    request: PlaneExactValueClaimRequest,
) -> Result<HadwigerProofAuthorityChain, HadwigerProofClaimAdmissionError> {
    let mut steps = vec![
        HadwigerProofAuthorityStep::new(
            HadwigerProofAuthorityStepKind::QueryDeclaration,
            "plane exact-value claim declaration",
            query_reference.stable_token(),
        ),
        HadwigerProofAuthorityStep::new(
            HadwigerProofAuthorityStepKind::ProofClaim,
            "lower-bound proof claim",
            request.lower_bound_claim().reference().stable_token(),
        ),
    ];
    let (checked_refs, background_ref, uses_checked, uses_background) =
        if let Some(checked) = request.checked_upper_bound() {
            steps.push(HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::CheckerArtifact,
                "checked upper-bound verification",
                checked.reference().stable_token(),
            ));
            (vec![checked.reference()], None, true, false)
        } else {
            let background = request
                .background_upper_bound()
                .expect("validated upper bound");
            steps.push(HadwigerProofAuthorityStep::new(
                HadwigerProofAuthorityStepKind::BackgroundTheorem,
                "retained background upper-bound theorem",
                background.reference().stable_token(),
            ));
            (Vec::new(), Some(background.reference()), false, true)
        };
    HadwigerProofAuthorityChain::new(
        proof_claim,
        HadwigerAspectPosture::Admitted,
        steps,
        vec![query_reference],
        checked_refs,
        Vec::new(),
        background_ref,
        uses_checked,
        uses_background,
    )
    .map_err(Into::into)
}

pub(super) fn upper_blocked(
    claim_id: &str,
    verification: &WholePlaneColoringVerification,
    reason: &str,
) -> Result<HadwigerBlockedProofClaim, HadwigerProofClaimAdmissionError> {
    blocked_from_parts(
        claim_id,
        HadwigerProofClaimKind::PlaneUpperBound {
            color_count: verification.verified_color_count(),
        },
        verification.reference(),
        vec![HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::CheckerArtifactNotAdmitted,
            reason,
            Some(HadwigerAspectPosture::Rejected),
        )],
    )
}

pub(super) fn exact_blocked(
    request: &PlaneExactValueClaimRequest,
    reason: &str,
) -> Result<HadwigerBlockedProofClaim, HadwigerProofClaimAdmissionError> {
    blocked_from_parts(
        request.claim_id(),
        HadwigerProofClaimKind::PlaneExactValue {
            color_count: request.lower_bound_claim().color_count(),
        },
        request.lower_bound_claim().reference(),
        vec![HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::MissingEvidence,
            reason,
            Some(HadwigerAspectPosture::Missing),
        )],
    )
}
