use forge_store::{
    ArtifactFamilyId, ArtifactSemanticVersion, CompatibilityAdmissionCounters,
    CompatibilityRelation, Milestone12CertificationLaneInput, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome,
};

fn main() {
    let input = Milestone12CertificationLaneInput::new(
        ArtifactFamilyId::new("commit_envelope"),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(1),
        Some(CompatibilityRelation::Native),
        None,
    );
    let counters = CompatibilityAdmissionCounters::default();
    let _ = Milestone12CertificationLaneOutcome::accepted(
        Milestone12CertificationLaneKind::CatalogCompleteness,
        input,
        CompatibilityRelation::Native,
        &counters,
    );
}
