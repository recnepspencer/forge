use crate::support::compile_pass::{CompilePassBundle, CompilePassCase};

pub fn compile_pass_bundle() -> CompilePassBundle {
    CompilePassBundle::new(
        "pleasant_entrypoints_are_additive_guidance",
        vec![
            CompilePassCase::new(
                "prelude-plus-raw-lane",
                "tests/ui/dx/compile_pass/prelude_plus_raw_lane_compiles.rs",
            ),
            CompilePassCase::new(
                "verb-first-progression",
                "tests/ui/dx/compile_pass/verb_first_progression_compiles.rs",
            ),
            CompilePassCase::new(
                "checked-and-boundary-progression",
                "tests/ui/dx/compile_pass/checked_and_boundary_progression_compiles.rs",
            ),
            CompilePassCase::new(
                "composition-and-family-authoring",
                "tests/ui/dx/compile_pass/composition_and_family_authoring_compiles.rs",
            ),
            CompilePassCase::new(
                "grouped-read-inspectors",
                "tests/ui/dx/compile_pass/grouped_read_inspectors_compiles.rs",
            ),
        ],
    )
}
