use crate::domain_installation::{
    foundational_boundary_artifact_id, foundational_boundary_handle, WorthQueryAftermathKind,
};
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use worth_foundational::facade::{
    admit_current_basis_boundary_evidence_attachment_bundle, boundary_evidence,
    foundational_boundary_evidence_attachment_readmission_authority, BoundaryArtifactField,
    BoundaryArtifactLocator, CurrentBasisBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLineageSubject,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthQueryFoundationalAftermathAttachment {
    original_trace_evidence_identity: WorthQueryEvidenceIdentity,
    aftermath_trace_evidence_identity: WorthQueryEvidenceIdentity,
    materialized: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
}

impl WorthQueryFoundationalAftermathAttachment {
    pub fn original_trace_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.original_trace_evidence_identity
    }

    pub fn aftermath_trace_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.aftermath_trace_evidence_identity
    }

    pub fn materialized(&self) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        &self.materialized
    }

    pub fn admit_current_basis(&self) -> CurrentBasisBoundaryEvidenceAttachmentBundle {
        admit_current_basis_boundary_evidence_attachment_bundle(
            self.materialized.clone(),
            foundational_boundary_evidence_attachment_readmission_authority(),
        )
    }
}

pub(crate) fn materialize_aftermath_attachment(
    original_trace_identity: &str,
    aftermath_trace_identity: &str,
    kind: WorthQueryAftermathKind,
) -> WorthQueryFoundationalAftermathAttachment {
    let original = trace_evidence_identity("aftermath-original-trace-v1", original_trace_identity);
    let aftermath =
        trace_evidence_identity("aftermath-candidate-trace-v1", aftermath_trace_identity);
    let source = BoundaryArtifactLocator::new(
        foundational_boundary_artifact_id(&original),
        BoundaryArtifactField::Basis,
    );
    let target = BoundaryArtifactLocator::new(
        foundational_boundary_artifact_id(&aftermath),
        BoundaryArtifactField::Proofs,
    );
    let provenance = boundary_evidence()
        .provenance()
        .current(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
            source,
        ))
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
        .into_result()
        .expect("fixed current Foundational aftermath provenance is legal");
    let receipt = match kind {
        WorthQueryAftermathKind::ExactInverse => boundary_evidence()
            .receipt()
            .restoration(FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(target)),
        WorthQueryAftermathKind::Compensation => boundary_evidence()
            .receipt()
            .execution(FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(target)),
    }
    .with_provenance(provenance.clone());
    let mut attachment = boundary_evidence()
        .attachment()
        .for_boundary_artifact(target)
        .with_provenance_attachment(provenance)
        .with_receipt_attachment(receipt.completed_receipt().clone());
    if kind == WorthQueryAftermathKind::ExactInverse {
        let restored = boundary_evidence()
            .lineage()
            .restored_continuity(FoundationalBoundaryEvidenceLineageSubject::new(
                foundational_boundary_handle(&original),
            ))
            .attested_by(receipt)
            .into_result()
            .expect("restoration receipt admits restored Foundational continuity");
        attachment = attachment.with_restored_continuity(restored);
    }
    WorthQueryFoundationalAftermathAttachment {
        original_trace_evidence_identity: original,
        aftermath_trace_evidence_identity: aftermath,
        materialized: attachment.materialize_under(
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        ),
    }
}

fn trace_evidence_identity(role: &'static str, trace_identity: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::InstalledDomainExecution)
        .field_shape(WorthQueryEvidenceTag::new("identity_family"), role)
        .field_value(
            WorthQueryEvidenceTag::new("query_trace_identity"),
            trace_identity,
        )
        .seal()
}
