use super::compile_fail::{CompileFailBundle, CompileFailCase};

pub fn compile_fail_bundle() -> CompileFailBundle {
    CompileFailBundle::new(
        "sealed_minting_and_witness_authority",
        vec![
            CompileFailCase::new(
                "witness_minting",
                "tests/ui/witnesses_are_not_publicly_mintable.rs",
            ),
            CompileFailCase::new(
                "witness_boundaries",
                "tests/ui/witness_required_apis_reject_callers_without_witness.rs",
            ),
            CompileFailCase::new(
                "recipe_boundaries",
                "tests/ui/recipe_stages_are_not_publicly_skippable.rs",
            ),
        ],
    )
}
