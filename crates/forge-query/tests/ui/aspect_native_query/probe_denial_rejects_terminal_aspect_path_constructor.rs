use forge_query::facade::{
    ForgeQueryExistingTruthProbeDenial, ForgeQueryExistingTruthProbeDenialKind,
    ForgeQueryExistingTruthTargetBinding,
};

fn main() {}

fn denied_from_terminal_path(binding: &ForgeQueryExistingTruthTargetBinding) {
    let _ = ForgeQueryExistingTruthProbeDenial::new(
        binding,
        ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
        Some("status.value".to_string()),
        "terminal probe denial paths are not proof",
    );
}
