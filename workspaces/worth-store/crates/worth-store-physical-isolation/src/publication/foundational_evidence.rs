use super::PhysicalPublicationReceipt;
use worth_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    BoundaryHandle, FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLineageSubject,
    FoundationalBoundaryEvidenceLineageSubjectSet,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalDiagnosticCodeId, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticLocator, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticProvenanceReadyRow, FoundationalDiagnosticRow,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSemanticLabelSet,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticSubject,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPublicationFoundationalEvidence {
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    lineage: FoundationalBoundaryEvidenceAttestedLineageArtifact,
    continuity_scope: FoundationalBoundaryEvidenceContinuityAttachmentScope,
    diagnostic_rows: Vec<FoundationalDiagnosticRow>,
}

impl PhysicalPublicationFoundationalEvidence {
    pub(crate) fn lower(
        receipt: &PhysicalPublicationReceipt,
    ) -> Result<Self, FoundationalBoundaryEvidenceProvenanceConstructionDenial> {
        let locator = receipt_locator(receipt);
        let source_basis = FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator);
        let provenance = boundary_evidence()
            .provenance()
            .replay_derived(source_basis)
            .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay)
            .into_result()?;
        let executed_receipt = boundary_evidence()
            .receipt()
            .execution(FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(locator))
            .with_provenance(provenance);
        let lineage = publication_lineage(receipt, executed_receipt.clone());
        let diagnostic_rows = vec![FoundationalDiagnosticRow::ProvenanceReady(
            FoundationalDiagnosticProvenanceReadyRow::new(
                code("s5.copy_on_write.publication"),
                scope("s5.copy_on_write.publication"),
                FoundationalDiagnosticSeverity::Info,
                FoundationalDiagnosticSubject::BoundaryArtifact {
                    artifact_locator: locator,
                },
                FoundationalDiagnosticLocator::BoundaryArtifact(locator),
                FoundationalDiagnosticOutcomeKind::Accepted,
                FoundationalDiagnosticSemanticLabelSet::new([code("copy_on_write_publication")]),
                FoundationalDiagnosticLocator::BoundaryArtifact(locator),
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
        )];
        Ok(Self {
            executed_receipt,
            lineage,
            continuity_scope: FoundationalBoundaryEvidenceContinuityAttachmentScope::ObjectLevel,
            diagnostic_rows,
        })
    }

    pub const fn executed_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.executed_receipt
    }

    pub fn diagnostic_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.diagnostic_rows
    }

    pub const fn lineage(&self) -> &FoundationalBoundaryEvidenceAttestedLineageArtifact {
        &self.lineage
    }

    pub const fn continuity_scope(&self) -> FoundationalBoundaryEvidenceContinuityAttachmentScope {
        self.continuity_scope
    }
}

fn publication_lineage(
    receipt: &PhysicalPublicationReceipt,
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
) -> FoundationalBoundaryEvidenceAttestedLineageArtifact {
    let old_subject = lineage_subject(receipt.old_root().epoch().get());
    let new_subject = lineage_subject(receipt.new_root().epoch().get());
    let related = FoundationalBoundaryEvidenceLineageSubjectSet::new(vec![old_subject])
        .expect("publication lineage has old root subject");
    boundary_evidence()
        .lineage()
        .continuity(new_subject)
        .related_subjects(related)
        .attested_by(executed_receipt)
}

fn lineage_subject(root_epoch: u64) -> FoundationalBoundaryEvidenceLineageSubject {
    FoundationalBoundaryEvidenceLineageSubject::new(BoundaryHandle::new(root_epoch))
}

fn receipt_locator(receipt: &PhysicalPublicationReceipt) -> BoundaryArtifactLocator {
    let basis = receipt.old_reachability().footprint_basis();
    BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(basis.canonical_digest()),
        BoundaryArtifactField::Payload,
    )
}

fn code(value: &str) -> FoundationalDiagnosticCodeId {
    FoundationalDiagnosticCodeId::new(value).unwrap()
}

fn scope(value: &str) -> FoundationalDiagnosticScopeId {
    FoundationalDiagnosticScopeId::new(value).unwrap()
}
