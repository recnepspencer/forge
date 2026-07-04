pub(crate) struct PhaseFourteenTopologyCompileFailFence {
    fixture_path: &'static str,
    fence_class: &'static str,
}

const PHASE_FOURTEEN_TOPOLOGY_COMPILE_FAIL_FENCES: &[PhaseFourteenTopologyCompileFailFence] = &[
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_selected_route_authority_not_exported.rs",
        "local-routing-helper-import",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_selected_route_admission_not_exported.rs",
        "local-routing-helper-import",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_invalidation_route_input_not_mintable_from_milestone_ten_summary_row.rs",
        "route-rediscovery",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_invalidation_route_input_not_mintable_from_projection_read_stage_receipt.rs",
        "route-rediscovery",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_reuse_decision_not_exported.rs",
        "reuse-basis-fabrication",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_topology_compiled_product_rebuild_denial_not_exported.rs",
        "reuse-basis-fabrication",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_derived_read_diagnostic_support_not_exported.rs",
        "closeout-helper-import",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_derived_read_diagnostic_support_wrapper_not_exported.rs",
        "closeout-helper-import",
    ),
    PhaseFourteenTopologyCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/public_touched_graph_parity_internal_readiness_constructors_not_exported.rs",
        "readiness-constructor",
    ),
];

impl PhaseFourteenTopologyCompileFailFence {
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
                .expect("phase 14 topology fixture must end with .rs")
        )
    }

    pub(crate) const fn fence_class(&self) -> &'static str {
        self.fence_class
    }
}

pub(crate) const fn phase_fourteen_topology_compile_fail_fences(
) -> &'static [PhaseFourteenTopologyCompileFailFence] {
    PHASE_FOURTEEN_TOPOLOGY_COMPILE_FAIL_FENCES
}
