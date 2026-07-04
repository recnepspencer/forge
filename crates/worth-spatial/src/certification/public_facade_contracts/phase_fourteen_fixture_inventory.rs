pub(crate) struct PhaseFourteenSpatialCompileFailFence {
    fixture_path: &'static str,
    fence_class: &'static str,
}

const PHASE_FOURTEEN_SPATIAL_COMPILE_FAIL_FENCES: &[PhaseFourteenSpatialCompileFailFence] = &[
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/planner_route_assembly_input_not_exported.rs",
        "local-routing-helper-import",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_loop_reconstruction/loop_reconstruction_request_not_forgeable.rs",
        "overlap-reconstruction",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_index_product/reuse_decision_not_exported.rs",
        "reuse-basis-fabrication",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/selected_family_not_hand_filled.rs",
        "reuse-basis-fabrication",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/lowered_identity_not_hand_filled.rs",
        "reuse-basis-fabrication",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/assembly_input_function_not_exported.rs",
        "closeout-helper-import",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/selected_route_support_not_exported.rs",
        "closeout-helper-import",
    ),
    PhaseFourteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/fixture_crates/touched_graph_parity_closeout_readiness_input_constructors_not_exported/src/main.rs",
        "readiness-constructor",
    ),
];

impl PhaseFourteenSpatialCompileFailFence {
    const fn new(fixture_path: &'static str, fence_class: &'static str) -> Self {
        Self {
            fixture_path,
            fence_class,
        }
    }

    pub(crate) const fn fixture_path(&self) -> &'static str {
        self.fixture_path
    }

    pub(crate) fn stderr_path(&self) -> String {
        format!(
            "{}.stderr",
            self.fixture_path
                .strip_suffix(".rs")
                .expect("phase 14 spatial fixture must end with .rs")
        )
    }

    pub(crate) const fn fence_class(&self) -> &'static str {
        self.fence_class
    }
}

pub(crate) const fn phase_fourteen_spatial_compile_fail_fences(
) -> &'static [PhaseFourteenSpatialCompileFailFence] {
    PHASE_FOURTEEN_SPATIAL_COMPILE_FAIL_FENCES
}
