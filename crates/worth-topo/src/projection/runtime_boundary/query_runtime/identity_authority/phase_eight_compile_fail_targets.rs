pub struct TopologyQueryRuntimePhaseEightCompileFailTarget {
    path: &'static str,
    forbidden_substitution: &'static str,
}

impl TopologyQueryRuntimePhaseEightCompileFailTarget {
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

pub struct TopologyQueryRuntimePhaseEightGoldenPath {
    path: &'static str,
}

impl TopologyQueryRuntimePhaseEightGoldenPath {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_TARGETS: &[TopologyQueryRuntimePhaseEightCompileFailTarget] = &[
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/source_adapter_snapshot_token_removed.rs",
        "runtime source adapter snapshot_token removed in favor of snapshot_identity adapter",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/commit_external_authority_label_removed.rs",
        "derived surface commit cannot mint from external authority label",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/continuity_rebind_format_strings_removed.rs",
        "continuity rebind cannot use formatted relation debug labels",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/mutation_authority_raw_string_removed.rs",
        "mutation authority cannot mint from raw existing-truth string",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/bridge_identity_evidence_as_str_removed.rs",
        "bridge admission evidence as_str is not a reporting authority edge",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/truth_commit_identity_new_format_removed.rs",
        "truth commit identity cannot mint from formatted commit label",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/existing_relation_target_string_authority_removed.rs",
        "existing relation target cannot bind with raw string authority",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/entity_identity_display_authority_removed.rs",
        "entity debug display cannot satisfy mutation authority binding",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/truth_snapshot_identity_new_format_removed.rs",
        "truth snapshot identity cannot mint from formatted snapshot label",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/truth_branch_identity_new_format_removed.rs",
        "truth branch identity cannot mint from formatted branch label",
    ),
    TopologyQueryRuntimePhaseEightCompileFailTarget::new(
        "tests/ui/query_runtime_phase_eight/snapshot_external_authority_label_removed.rs",
        "query snapshot identity cannot mint from external authority label",
    ),
];

const GOLDEN_PATHS: &[TopologyQueryRuntimePhaseEightGoldenPath] = &[
    TopologyQueryRuntimePhaseEightGoldenPath::new(
        "tests/ui/query_runtime_phase_eight/golden/derived_surface_commit_identity_golden_path.rs",
    ),
    TopologyQueryRuntimePhaseEightGoldenPath::new(
        "tests/ui/query_runtime_phase_eight/golden/snapshot_identity_adapter_golden_path.rs",
    ),
    TopologyQueryRuntimePhaseEightGoldenPath::new(
        "tests/ui/query_runtime_phase_eight/golden/existing_truth_authority_golden_path.rs",
    ),
];

pub const fn topology_query_runtime_phase_eight_compile_fail_targets(
) -> &'static [TopologyQueryRuntimePhaseEightCompileFailTarget] {
    COMPILE_FAIL_TARGETS
}

pub const fn topology_query_runtime_phase_eight_golden_paths(
) -> &'static [TopologyQueryRuntimePhaseEightGoldenPath] {
    GOLDEN_PATHS
}

pub const TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_COMPILE_FAIL_TARGET_COUNT: usize =
    COMPILE_FAIL_TARGETS.len();

pub const TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_GOLDEN_PATH_COUNT: usize = GOLDEN_PATHS.len();
