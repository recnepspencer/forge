use forge_foundational::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact, FoundationalBoundaryEvidenceReceiptKind,
};

fn main() {
    let _ = FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        kind: FoundationalBoundaryEvidenceReceiptKind::Execution,
    };
}
