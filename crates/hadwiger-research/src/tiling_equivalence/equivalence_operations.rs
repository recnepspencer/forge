use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, TilingEquivalenceClassificationDeclaration,
    TilingReactivationDeclaration, TilingSuppressionDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::equivalence_classification_requests::TilingCandidateEquivalenceRequest;
use super::equivalence_errors::TilingEquivalenceError;
use super::equivalence_proofs::TilingCandidateEquivalenceProof;
use super::reactivation_requests::TilingReactivationRequest;
use super::reactivation_results::TilingReactivationChecked;
use super::suppression_proofs::TilingCandidateSuppressionProof;
use super::suppression_requests::TilingCandidateSuppressionRequest;

pub fn classify_tiling_candidate_equivalence_checked(
    handle: &HadwigerResearchHandle,
    request: TilingCandidateEquivalenceRequest,
) -> Result<TilingCandidateEquivalenceProof, TilingEquivalenceError> {
    let query_reference = declare_equivalence_classification(handle, &request)?;
    Ok(TilingCandidateEquivalenceProof::checked(
        request,
        query_reference,
    )?)
}

pub fn suppress_equivalent_tiling_candidate_checked(
    handle: &HadwigerResearchHandle,
    request: TilingCandidateSuppressionRequest,
) -> Result<TilingCandidateSuppressionProof, TilingEquivalenceError> {
    require_retained_suppression_evidence(&request)?;
    let query_reference = declare_tiling_suppression(handle, &request)?;
    Ok(TilingCandidateSuppressionProof::checked(
        request,
        query_reference,
    )?)
}

pub fn reactivate_tiling_candidate_checked(
    handle: &HadwigerResearchHandle,
    request: TilingReactivationRequest,
) -> Result<TilingReactivationChecked, TilingEquivalenceError> {
    reject_self_reactivation(&request)?;
    let query_reference = declare_tiling_reactivation(handle, &request)?;
    Ok(TilingReactivationChecked::checked(
        request,
        query_reference,
    )?)
}

fn declare_equivalence_classification(
    handle: &HadwigerResearchHandle,
    request: &TilingCandidateEquivalenceRequest,
) -> Result<crate::domain_artifacts::HadwigerQueryDeclarationReference, TilingEquivalenceError> {
    let checked = declare_research_request_checked(
        handle,
        TilingEquivalenceClassificationDeclaration::new(
            request.equivalence_id(),
            request.scope().as_str(),
            request.left_reference().stable_token(),
            request.right_reference().stable_token(),
        ),
    );
    checked
        .admitted()
        .map(Into::into)
        .ok_or(TilingEquivalenceError::QueryDeclarationNotAdmitted {
            declaration: "tiling_equivalence_classification",
        })
}

fn declare_tiling_suppression(
    handle: &HadwigerResearchHandle,
    request: &TilingCandidateSuppressionRequest,
) -> Result<crate::domain_artifacts::HadwigerQueryDeclarationReference, TilingEquivalenceError> {
    let checked = declare_research_request_checked(
        handle,
        TilingSuppressionDeclaration::new(
            request.suppression_id(),
            request.equivalence().reference().stable_token(),
            request.suppression_proof().reference().stable_token(),
        ),
    );
    checked
        .admitted()
        .map(Into::into)
        .ok_or(TilingEquivalenceError::QueryDeclarationNotAdmitted {
            declaration: "tiling_suppression",
        })
}

fn declare_tiling_reactivation(
    handle: &HadwigerResearchHandle,
    request: &TilingReactivationRequest,
) -> Result<crate::domain_artifacts::HadwigerQueryDeclarationReference, TilingEquivalenceError> {
    let checked = declare_research_request_checked(
        handle,
        TilingReactivationDeclaration::new(
            request.reactivation_id(),
            request.suppression().reference().stable_token(),
            request
                .reactivation_condition()
                .qualifying_evidence()
                .stable_token(),
        ),
    );
    checked
        .admitted()
        .map(Into::into)
        .ok_or(TilingEquivalenceError::QueryDeclarationNotAdmitted {
            declaration: "tiling_reactivation",
        })
}

fn require_retained_suppression_evidence(
    request: &TilingCandidateSuppressionRequest,
) -> Result<(), TilingEquivalenceError> {
    if request
        .corpus()
        .has_reference(&request.suppression_proof().reference())
    {
        Ok(())
    } else {
        Err(TilingEquivalenceError::MissingDeadEndEvidence)
    }
}

fn reject_self_reactivation(
    request: &TilingReactivationRequest,
) -> Result<(), TilingEquivalenceError> {
    let qualifying = request.reactivation_condition().qualifying_evidence();
    if request
        .suppression()
        .parent_artifacts()
        .iter()
        .any(|parent| parent == qualifying)
    {
        Err(TilingEquivalenceError::ReactivationEvidenceNotNew)
    } else {
        Ok(())
    }
}
