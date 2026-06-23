pub struct TopologyQueryRuntimePhaseNineCompileFailTarget {
    path: &'static str,
    forbidden_substitution: &'static str,
}

impl TopologyQueryRuntimePhaseNineCompileFailTarget {
    pub const fn new(path: &'static str, forbidden_substitution: &'static str) -> Self {
        Self {
            path,
            forbidden_substitution,
        }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn forbidden_substitution(&self) -> &'static str {
        self.forbidden_substitution
    }
}

pub struct TopologyQueryRuntimePhaseNineGoldenPath {
    path: &'static str,
}

impl TopologyQueryRuntimePhaseNineGoldenPath {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_TARGETS: &[TopologyQueryRuntimePhaseNineCompileFailTarget] = &[
    TopologyQueryRuntimePhaseNineCompileFailTarget::new(
        "tests/ui/query_runtime_phase_nine/mutation_receipt_struct_literal_private.rs",
        "harness cannot construct ForgeQueryMutationReceipt via struct literal",
    ),
    TopologyQueryRuntimePhaseNineCompileFailTarget::new(
        "tests/ui/query_runtime_phase_nine/commit_terminal_projection_authority_removed.rs",
        "commit terminal projection cannot satisfy typed commit identity admission",
    ),
    TopologyQueryRuntimePhaseNineCompileFailTarget::new(
        "tests/ui/query_runtime_phase_nine/mutation_delta_entity_string_authority_removed.rs",
        "mutation delta entity authority cannot be a raw string label",
    ),
];

const GOLDEN_PATHS: &[TopologyQueryRuntimePhaseNineGoldenPath] =
    &[TopologyQueryRuntimePhaseNineGoldenPath::new(
        "tests/ui/query_runtime_phase_nine/golden/typed_mutation_receipt_golden_path.rs",
    )];

pub const fn topology_query_runtime_phase_nine_compile_fail_targets(
) -> &'static [TopologyQueryRuntimePhaseNineCompileFailTarget] {
    COMPILE_FAIL_TARGETS
}

pub const fn topology_query_runtime_phase_nine_golden_paths(
) -> &'static [TopologyQueryRuntimePhaseNineGoldenPath] {
    GOLDEN_PATHS
}

pub const TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_COMPILE_FAIL_TARGET_COUNT: usize =
    COMPILE_FAIL_TARGETS.len();

pub const TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_GOLDEN_PATH_COUNT: usize = GOLDEN_PATHS.len();
