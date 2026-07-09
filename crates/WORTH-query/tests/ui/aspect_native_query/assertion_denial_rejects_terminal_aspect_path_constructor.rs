use worth_query::facade::{
    WorthQueryExistingTruthAssertionDenial, WorthQueryExistingTruthAssertionDenialKind,
    WorthQueryExistingTruthTargetBinding,
};

fn main() {
}

fn denied_from_terminal_path(binding: &WorthQueryExistingTruthTargetBinding) {
    let _ = WorthQueryExistingTruthAssertionDenial::new(
        binding,
        WorthQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
        Some("status.value".to_string()),
        None,
        None,
        "terminal assertion denial paths are not proof",
    );
}
