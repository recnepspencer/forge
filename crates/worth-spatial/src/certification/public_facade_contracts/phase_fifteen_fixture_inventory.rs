pub(crate) struct PhaseFifteenSpatialCompileFailFence {
    fixture_path: &'static str,
    fence_class: &'static str,
}

const PHASE_FIFTEEN_SPATIAL_COMPILE_FAIL_FENCES: &[PhaseFifteenSpatialCompileFailFence] = &[
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_family_catalog/struct_literals/family_declaration_not_hand_filled.rs",
        "family-record",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_input_admission/struct_literals/admitted_input_not_hand_filled.rs",
        "admitted-input",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/closeout_not_hand_filled.rs",
        "closeout-product",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_index_product/struct_literals/product_not_hand_filled.rs",
        "compiled-product-identity",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_index_product/struct_literals/rebuild_denial_not_hand_filled.rs",
        "rebuild-denial",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/evidence_lookup_index_product/reuse_decision_not_exported.rs",
        "reuse-decision",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/admitted_input_not_hand_filled.rs",
        "admitted-input",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/declaration_not_hand_filled.rs",
        "family-record",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/selected_family_not_hand_filled.rs",
        "selected-equivalence-family",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/spatial_compiled_product_family/struct_literals/lowered_identity_not_hand_filled.rs",
        "equivalence-policy-identity",
    ),
];

impl PhaseFifteenSpatialCompileFailFence {
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
                .expect("phase 15 spatial fixture must end with .rs")
        )
    }

    pub(crate) const fn fence_class(&self) -> &'static str {
        self.fence_class
    }
}

pub(crate) const fn phase_fifteen_spatial_compile_fail_fences(
) -> &'static [PhaseFifteenSpatialCompileFailFence] {
    PHASE_FIFTEEN_SPATIAL_COMPILE_FAIL_FENCES
}
