use forge_foundational::facade::{
    attachment, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, BoundaryHandle,
    DiagnosticRichnessProfile, EquivalenceBasisId, FoundationalBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidencePublishedSupportArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceReceiptFrontDoor,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    FoundationalBoundaryEvidenceSupportConstructionDenial,
    FoundationalBoundaryEvidenceSupportFrontDoor, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle, FoundationalTransitionLocator,
};
use forge_proof::TransitionOutcome;

use crate::{ForgeServerResponseEnvelope, ForgeServerResponseReceipt};

use super::classification::ForgeServerOperatorEvidenceClass;

pub(crate) fn build_attachment_bundle(
    class: &ForgeServerOperatorEvidenceClass,
    response: &ForgeServerResponseEnvelope,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<
    (
        FoundationalBoundaryEvidenceAttachmentBundle,
        FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    ),
    ForgeServerOperatorEvidenceAttachmentError,
> {
    let locator = BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(boundary_artifact_id(&[
            "forge-server.operator-evidence".to_string(),
            response.canonical_digest().to_string(),
            format!("{class:?}"),
        ])),
        BoundaryArtifactField::Basis,
    );
    let receipt = response_receipt(response);
    let support = build_support_attachment(class, response, receipt)?;
    let bundle = match support {
        BuiltSupportAttachment::Published(artifact) => attachment()
            .for_boundary_artifact(locator)
            .with_provenance_attachment(response.provenance().clone())
            .with_receipt_attachment(response_completed_receipt(receipt).clone())
            .with_published_support(artifact),
        BuiltSupportAttachment::Closeout(artifact) => attachment()
            .for_boundary_artifact(locator)
            .with_provenance_attachment(response.provenance().clone())
            .with_receipt_attachment(response_completed_receipt(receipt).clone())
            .with_support_closeout(artifact),
    };
    let materialized = bundle.materialize_under(materialization_profile(diagnostics_profile));
    Ok((bundle, materialized))
}

fn build_support_attachment(
    class: &ForgeServerOperatorEvidenceClass,
    response: &ForgeServerResponseEnvelope,
    receipt: &ForgeServerResponseReceipt,
) -> Result<BuiltSupportAttachment, ForgeServerOperatorEvidenceAttachmentError> {
    match class {
        ForgeServerOperatorEvidenceClass::QueryReadSucceeded
        | ForgeServerOperatorEvidenceClass::QueryMutationSucceeded
        | ForgeServerOperatorEvidenceClass::DownstreamDeliverySucceeded => {
            let executed_receipt = receipt.executed().ok_or(
                ForgeServerOperatorEvidenceAttachmentError::SuccessRequiresExecutedReceipt,
            )?;
            let support_publication_receipt = FoundationalBoundaryEvidenceReceiptFrontDoor
                .support_publication(support_receipt_boundary(response.canonical_digest()))
                .with_provenance(executed_receipt.provenance().clone());
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .published_evidence()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
                )
                .attested_by(support_publication_receipt)
            {
                TransitionOutcome::Success(artifact) => {
                    Ok(BuiltSupportAttachment::Published(artifact))
                }
                TransitionOutcome::Denied(denial) => {
                    Err(ForgeServerOperatorEvidenceAttachmentError::SupportConstruction(denial))
                }
                outcome => panic!("unexpected support publication outcome: {outcome:?}"),
            }
        }
        ForgeServerOperatorEvidenceClass::RequestContextDenied(_)
        | ForgeServerOperatorEvidenceClass::MiddlewareDenied(_)
        | ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_) => {
            match FoundationalBoundaryEvidenceSupportFrontDoor
                .degraded_recovery_report()
                .with_basis_disclosure(
                    FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis,
                )
                .closed_out_by(response_completed_receipt(receipt).clone())
            {
                TransitionOutcome::Success(artifact) => {
                    Ok(BuiltSupportAttachment::Closeout(artifact))
                }
                TransitionOutcome::Denied(denial) => {
                    Err(ForgeServerOperatorEvidenceAttachmentError::SupportConstruction(denial))
                }
                outcome => panic!("unexpected support closeout outcome: {outcome:?}"),
            }
        }
    }
}

fn response_completed_receipt(
    receipt: &ForgeServerResponseReceipt,
) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
    receipt.completed()
}

fn response_receipt(response: &ForgeServerResponseEnvelope) -> &ForgeServerResponseReceipt {
    if let Some(success) = response.success() {
        success.receipt()
    } else {
        response
            .denial()
            .expect("response envelope must carry success or denial receipt")
            .receipt()
    }
}

fn materialization_profile(
    diagnostics_profile: DiagnosticRichnessProfile,
) -> FoundationalBoundaryEvidenceMaterializationProfile {
    match diagnostics_profile {
        DiagnosticRichnessProfile::OperationalMinimal => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
        }
        DiagnosticRichnessProfile::Standard => {
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
        }
        DiagnosticRichnessProfile::Forensic => {
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
        }
    }
}

fn boundary_artifact_id(parts: &[String]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x1f;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn support_receipt_boundary(canonical_digest: &str) -> FoundationalBoundaryEvidenceReceiptBoundary {
    let commit_id = FoundationalCommitId::new(BoundaryHandle::new(boundary_artifact_id(&[
        "forge-server.operator-evidence.support.commit".to_string(),
        canonical_digest.to_string(),
    ])));
    let parent_basis =
        FoundationalCommitParentBasis::new(EquivalenceBasisId::new(boundary_artifact_id(&[
            "forge-server.operator-evidence.support.parent".to_string(),
            canonical_digest.to_string(),
        ])));
    FoundationalBoundaryEvidenceReceiptBoundary::transition(
        FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
            commit_id,
            parent_basis,
        )),
    )
}

enum BuiltSupportAttachment {
    Published(FoundationalBoundaryEvidencePublishedSupportArtifact),
    Closeout(FoundationalBoundaryEvidenceSupportCloseoutArtifact),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperatorEvidenceAttachmentError {
    SuccessRequiresExecutedReceipt,
    SupportConstruction(FoundationalBoundaryEvidenceSupportConstructionDenial),
}
