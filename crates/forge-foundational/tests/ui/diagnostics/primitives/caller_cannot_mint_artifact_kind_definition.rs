use forge_foundational::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticArtifactKindDefinition,
};

fn main() {
    let _definition = FoundationalDiagnosticArtifactKindDefinition::new(
        FoundationalDiagnosticArtifactKind::Summary,
        "summary",
        "forged meaning",
        "nothing",
    );
}
