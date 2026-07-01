pub(crate) struct PhaseFifteenTopologyCompileFailFence {
    fixture_path: &'static str,
    fence_class: &'static str,
}

const PHASE_FIFTEEN_TOPOLOGY_COMPILE_FAIL_FENCES: &[PhaseFifteenTopologyCompileFailFence] = &[
    PhaseFifteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_family_declaration_constructor_not_exported.rs",
        "family-record",
    ),
    PhaseFifteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_family_proof_products_not_deserializable.rs",
        "compiled-product-proof-products",
    ),
    PhaseFifteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_admission_not_exported.rs",
        "admitted-input",
    ),
    PhaseFifteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_reuse_decision_not_exported.rs",
        "reuse-decision",
    ),
    PhaseFifteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_rebuild_denial_not_exported.rs",
        "rebuild-denial",
    ),
];

impl PhaseFifteenTopologyCompileFailFence {
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
                .expect("phase 15 topology fixture must end with .rs")
        )
    }

    pub(crate) const fn fence_class(&self) -> &'static str {
        self.fence_class
    }
}

pub(crate) const fn phase_fifteen_topology_compile_fail_fences(
) -> &'static [PhaseFifteenTopologyCompileFailFence] {
    PHASE_FIFTEEN_TOPOLOGY_COMPILE_FAIL_FENCES
}
