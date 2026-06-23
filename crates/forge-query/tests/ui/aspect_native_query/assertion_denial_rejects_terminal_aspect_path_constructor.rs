use forge_query::facade::{
    ForgeQueryExistingTruthAssertionDenial, ForgeQueryExistingTruthAssertionDenialKind,
    ForgeQueryExistingTruthTargetBinding,
};

fn main() {
}

fn denied_from_terminal_path(binding: &ForgeQueryExistingTruthTargetBinding) {
    let _ = ForgeQueryExistingTruthAssertionDenial::new(
        binding,
        ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
        Some("status.value".to_string()),
        None,
        None,
        "terminal assertion denial paths are not proof",
    );
}
