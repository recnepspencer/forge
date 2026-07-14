use worth_query::facade::runtime::{WorthQueryExistingTruthProbeDenial, WorthQueryExistingTruthProbeDenialKind, WorthQueryExistingTruthTargetBinding};

fn main() {}

fn denied_from_terminal_path(binding: &WorthQueryExistingTruthTargetBinding) {
    let _ = WorthQueryExistingTruthProbeDenial::new(
        binding,
        WorthQueryExistingTruthProbeDenialKind::MissingProbedAspect,
        Some("status.value".to_string()),
        "terminal probe denial paths are not proof",
    );
}
