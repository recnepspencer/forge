use crate::aspect_authority::HadwigerAspectPosture;
use crate::domain_artifacts::{
    admitted_proof_claim, ColorabilityVerificationPosture, HadwigerCanonicalArtifact,
    HadwigerDeclaredFamilyCheckedExt, HadwigerProofClaimKind, HadwigerQueryDeclarationReference,
    LowerBoundWitnessArtifact, RetainedBackgroundTheorem,
};
use crate::domain_declarations::{
    BackgroundTheoremDeclaration, PlaneExactValueClaimDeclaration, PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::admission_results::{
    HadwigerProofClaimAdmissionChecked, HadwigerProofClaimAdmissionError,
    HadwigerProofClaimBlocker, HadwigerProofClaimBlockerKind,
};
use super::operation_support::{
    blocked_from_parts, exact_blocked, exact_chain, lower_blocked, lower_chain, missing,
    upper_blocked, upper_chain,
};
use super::requests::{
    PlaneExactValueClaimRequest, PlaneLowerBoundClaimRequest, PlaneUpperBoundClaimRequest,
};

pub fn admit_plane_lower_bound_claim_checked(
    handle: &HadwigerResearchHandle,
    request: PlaneLowerBoundClaimRequest,
) -> Result<HadwigerProofClaimAdmissionChecked, HadwigerProofClaimAdmissionError> {
    let color_verification = request.not_k_colorable_verification().ok_or_else(|| {
        HadwigerProofClaimAdmissionError::Blocked(
            lower_blocked(&request, missing("not-k-colorable verification")).unwrap(),
        )
    })?;
    let forbidden_color_count = color_verification.color_count();
    let declaration = PlaneLowerBoundClaimDeclaration::new(
        request.claim_id(),
        request.graph_version().version_id(),
        forbidden_color_count,
    )?;
    let query_reference: HadwigerQueryDeclarationReference = handle
        .declare_checked(declaration)
        .admitted()
        .map(Into::into)
        .ok_or(
            HadwigerProofClaimAdmissionError::QueryDeclarationNotAdmitted {
                declaration_kind: "plane_lower_bound_claim",
            },
        )?;

    let mut blockers = Vec::new();
    let unit_verification = match request.unit_distance_verification() {
        Some(value) if value.is_admitted() => value,
        Some(_) => {
            blockers.push(HadwigerProofClaimBlocker::local(
                HadwigerProofClaimBlockerKind::CheckerArtifactNotAdmitted,
                "unit-distance verification is not admitted",
                Some(HadwigerAspectPosture::Rejected),
            ));
            request.unit_distance_verification().unwrap()
        }
        None => {
            return Err(HadwigerProofClaimAdmissionError::Blocked(lower_blocked(
                &request,
                missing("unit-distance verification"),
            )?));
        }
    };
    let unit_aspect = match request.unit_distance_aspect() {
        Some(value) if value.satisfies_mathematical_dependency() => value,
        Some(value) => {
            blockers.push(HadwigerProofClaimBlocker::local(
                HadwigerProofClaimBlockerKind::AspectNotAdmitted,
                "unit-distance aspect is not admitted",
                Some(value.aspect_posture()),
            ));
            value
        }
        None => {
            return Err(HadwigerProofClaimAdmissionError::Blocked(lower_blocked(
                &request,
                missing("unit-distance aspect"),
            )?));
        }
    };
    let color_aspect = match request.not_k_colorable_aspect() {
        Some(value) if value.satisfies_mathematical_dependency() => value,
        Some(value) => {
            blockers.push(HadwigerProofClaimBlocker::local(
                HadwigerProofClaimBlockerKind::AspectNotAdmitted,
                "not-k-colorable aspect is not admitted",
                Some(value.aspect_posture()),
            ));
            value
        }
        None => {
            return Err(HadwigerProofClaimAdmissionError::Blocked(lower_blocked(
                &request,
                missing("not-k-colorable aspect"),
            )?));
        }
    };
    if color_verification.posture() != ColorabilityVerificationPosture::UnsatVerified {
        blockers.push(HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::CheckerArtifactNotAdmitted,
            "colorability verification is not independently checked UNSAT",
            Some(color_aspect.aspect_posture()),
        ));
    }
    let graph_reference = request.graph_version().reference();
    if unit_aspect.artifact_reference() != &graph_reference
        || color_aspect.artifact_reference() != &graph_reference
    {
        blockers.push(HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::ArtifactMismatch,
            "aspect records do not target the requested graph version",
            None,
        ));
    }
    if !unit_verification
        .parent_artifacts()
        .iter()
        .any(|reference| reference == &graph_reference)
    {
        blockers.push(HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::ArtifactMismatch,
            "unit-distance verification does not target the requested graph version",
            None,
        ));
    }
    if color_verification.graph_version_reference() != &graph_reference
        || color_aspect.color_count() != forbidden_color_count
    {
        blockers.push(HadwigerProofClaimBlocker::local(
            HadwigerProofClaimBlockerKind::ArtifactMismatch,
            "colorability verification and aspect do not match the requested graph and color count",
            None,
        ));
    }
    if !blockers.is_empty() {
        return Err(HadwigerProofClaimAdmissionError::Blocked(
            blocked_from_parts(
                request.claim_id(),
                HadwigerProofClaimKind::PlaneLowerBound {
                    color_count: forbidden_color_count + 1,
                },
                graph_reference,
                blockers,
            )?,
        ));
    }

    let witness = LowerBoundWitnessArtifact::admitted(
        request.claim_id(),
        graph_reference,
        unit_verification.reference(),
        color_verification.reference(),
        forbidden_color_count,
        query_reference.clone(),
    )?;
    let proof_claim = admitted_proof_claim(
        witness.reference(),
        request.claim_id(),
        HadwigerProofClaimKind::PlaneLowerBound {
            color_count: forbidden_color_count + 1,
        },
        query_reference.clone(),
    )?;
    let chain = lower_chain(
        &proof_claim,
        query_reference,
        unit_verification.reference(),
        color_verification.reference(),
        unit_aspect.stable_token(),
        color_aspect.stable_token(),
    )?;
    Ok(HadwigerProofClaimAdmissionChecked::new(proof_claim, chain))
}

pub fn admit_plane_upper_bound_claim_checked(
    handle: &HadwigerResearchHandle,
    request: PlaneUpperBoundClaimRequest,
) -> Result<HadwigerProofClaimAdmissionChecked, HadwigerProofClaimAdmissionError> {
    let verification = request.checked_upper_bound();
    let color_count = verification.verified_color_count();
    let query_reference: HadwigerQueryDeclarationReference = handle
        .declare_checked(PlaneUpperBoundClaimDeclaration::new(
            request.claim_id(),
            color_count,
            verification.reference().stable_token(),
        )?)
        .admitted()
        .map(Into::into)
        .ok_or(
            HadwigerProofClaimAdmissionError::QueryDeclarationNotAdmitted {
                declaration_kind: "plane_upper_bound_claim",
            },
        )?;
    if !verification.admits_upper_bound_evidence() {
        return Err(HadwigerProofClaimAdmissionError::Blocked(upper_blocked(
            request.claim_id(),
            verification,
            "checked upper-bound verification is not admitted",
        )?));
    }
    let proof_claim = admitted_proof_claim(
        verification.reference(),
        request.claim_id(),
        HadwigerProofClaimKind::PlaneUpperBound { color_count },
        query_reference.clone(),
    )?;
    let chain = upper_chain(&proof_claim, query_reference, verification)?;
    Ok(HadwigerProofClaimAdmissionChecked::new(proof_claim, chain))
}

pub fn retain_background_plane_seven_upper_bound_checked(
    handle: &HadwigerResearchHandle,
    theorem_id: impl Into<String>,
    source: impl Into<String>,
    provenance_digest: impl Into<String>,
) -> Result<RetainedBackgroundTheorem, HadwigerProofClaimAdmissionError> {
    let theorem_id = theorem_id.into();
    let source = source.into();
    let provenance_digest = provenance_digest.into();
    let declaration = BackgroundTheoremDeclaration::plane_seven_upper_bound(
        &theorem_id,
        &source,
        &provenance_digest,
    )?;
    let query_reference: HadwigerQueryDeclarationReference = handle
        .declare_checked(declaration)
        .admitted()
        .map(Into::into)
        .ok_or(
            HadwigerProofClaimAdmissionError::QueryDeclarationNotAdmitted {
                declaration_kind: "background_theorem",
            },
        )?;
    Ok(RetainedBackgroundTheorem::admitted_plane_seven_upper_bound(
        theorem_id,
        source,
        provenance_digest,
        "sealed classical hexagonal tiling upper-bound theorem retention",
        query_reference,
    )?)
}

pub fn admit_plane_exact_value_claim_checked(
    handle: &HadwigerResearchHandle,
    request: PlaneExactValueClaimRequest,
) -> Result<HadwigerProofClaimAdmissionChecked, HadwigerProofClaimAdmissionError> {
    if !request.lower_bound_claim().admits_theorem_authority() {
        return Err(HadwigerProofClaimAdmissionError::Blocked(exact_blocked(
            &request,
            "lower-bound proof claim is not admitted theorem authority",
        )?));
    }
    let color_count = request.lower_bound_claim().color_count();
    let upper_source = request.upper_bound_source_token();
    let query_reference: HadwigerQueryDeclarationReference = handle
        .declare_checked(PlaneExactValueClaimDeclaration::new(
            request.claim_id(),
            color_count,
            request.lower_bound_claim().artifact_digest().stable_token(),
            upper_source.clone(),
        )?)
        .admitted()
        .map(Into::into)
        .ok_or(
            HadwigerProofClaimAdmissionError::QueryDeclarationNotAdmitted {
                declaration_kind: "plane_exact_value_claim",
            },
        )?;

    let parent = if let Some(checked) = request.checked_upper_bound() {
        if !checked.admits_upper_bound_evidence() || checked.verified_color_count() != color_count {
            return Err(HadwigerProofClaimAdmissionError::Blocked(exact_blocked(
                &request,
                "checked upper-bound evidence does not match the lower-bound color count",
            )?));
        }
        checked.reference()
    } else if let Some(background) = request.background_upper_bound() {
        if background.theorem_statement() != "chi(plane) <= 7" || color_count != 7 {
            return Err(HadwigerProofClaimAdmissionError::Blocked(exact_blocked(
                &request,
                "background theorem is not the sealed plane seven upper bound",
            )?));
        }
        background.reference()
    } else {
        return Err(HadwigerProofClaimAdmissionError::Blocked(exact_blocked(
            &request,
            "upper-bound evidence is missing",
        )?));
    };
    let proof_claim = admitted_proof_claim(
        parent,
        request.claim_id(),
        HadwigerProofClaimKind::PlaneExactValue { color_count },
        query_reference.clone(),
    )?;
    let chain = exact_chain(&proof_claim, query_reference, request)?;
    Ok(HadwigerProofClaimAdmissionChecked::new(proof_claim, chain))
}
