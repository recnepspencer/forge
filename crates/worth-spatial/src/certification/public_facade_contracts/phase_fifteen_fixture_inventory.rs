pub(crate) struct PhaseFifteenSpatialCompileFailFence {
    fixture_path: &'static str,
    fence_class: &'static str,
}

const PHASE_FIFTEEN_SPATIAL_COMPILE_FAIL_FENCES: &[PhaseFifteenSpatialCompileFailFence] = &[
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/loop_ledger_only_request_entry_not_available.rs",
        "synthetic-readiness",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/request_not_hand_filled_from_copied_fields.rs",
        "copied-overlap-rows",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/raw_arrangement_cells_do_not_admit_classification_input.rs",
        "bypassed-arrangement-or-cell-proof",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/identity_map_does_not_mint_overlap_region_ledger.rs",
        "raw-loop-and-ledger-bypass",
    ),
    PhaseFifteenSpatialCompileFailFence::new(
        "src/certification/public_facade_contracts/compile_fail/planar_boolean_overlap_region_extraction/public_contract_fence_input_not_exported.rs",
        "phase-fifteen-public-contract-string-bypass",
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
