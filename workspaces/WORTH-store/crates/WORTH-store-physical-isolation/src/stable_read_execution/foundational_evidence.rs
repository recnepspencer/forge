use worth_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticEvidencePosture, FoundationalDiagnosticLocator,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticProvenanceReadyRow,
    FoundationalDiagnosticRow, FoundationalDiagnosticScopeId,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSubject,
};
use worth_proof::TransitionOutcome;

use super::StablePhysicalReadReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StablePhysicalReadFoundationalEvidence {
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    diagnostic_rows: Vec<FoundationalDiagnosticRow>,
}

impl StablePhysicalReadFoundationalEvidence {
    pub(crate) fn lower(receipt: &StablePhysicalReadReceipt) -> Self {
        let locator = receipt_locator(receipt);
        let source_basis = FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator);
        let provenance = boundary_evidence()
            .provenance()
            .replay_derived(source_basis)
            .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay)
            .success_or_panic("S.5 stable read execution provenance");
        let executed_receipt = boundary_evidence()
            .receipt()
            .execution(FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(locator))
            .with_provenance(provenance);
        let diagnostic_rows = vec![FoundationalDiagnosticRow::ProvenanceReady(
            FoundationalDiagnosticProvenanceReadyRow::new(
                code("s5.stable_read.execution"),
                scope("s5.stable_read.execution"),
                FoundationalDiagnosticSeverity::Info,
                FoundationalDiagnosticSubject::BoundaryArtifact {
                    artifact_locator: locator,
                },
                FoundationalDiagnosticLocator::BoundaryArtifact(locator),
                FoundationalDiagnosticOutcomeKind::Accepted,
                FoundationalDiagnosticSemanticLabelSet::new([code("stable_read_execution")]),
                FoundationalDiagnosticLocator::BoundaryArtifact(locator),
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ),
        )];
        Self {
            executed_receipt,
            diagnostic_rows,
        }
    }

    pub const fn executed_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.executed_receipt
    }

    pub fn diagnostic_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.diagnostic_rows
    }
}

fn receipt_locator(receipt: &StablePhysicalReadReceipt) -> BoundaryArtifactLocator {
    let basis = receipt.read_plan_release().footprint_basis();
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

trait SuccessOrPanic<T> {
    fn success_or_panic(self, context: &str) -> T;
}

impl<T, D, De, St, R, F> SuccessOrPanic<T> for TransitionOutcome<T, D, De, St, R, F>
where
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success_or_panic(self, context: &str) -> T {
        match self {
            TransitionOutcome::Success(value) => value,
            _ => panic!("{context}: expected success"),
        }
    }
}
