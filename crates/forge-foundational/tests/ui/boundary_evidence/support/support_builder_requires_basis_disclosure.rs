use forge_foundational::boundary_evidence;

fn main() {
    let _ = boundary_evidence().support().published_evidence().attested_by(
        boundary_evidence()
            .receipt()
            .support_publication(panic_boundary())
            .with_provenance(panic_provenance()),
    );
}

fn panic_boundary() -> forge_foundational::FoundationalBoundaryEvidenceReceiptBoundary {
    panic!("not executed")
}

fn panic_provenance() -> forge_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
    panic!("not executed")
}
