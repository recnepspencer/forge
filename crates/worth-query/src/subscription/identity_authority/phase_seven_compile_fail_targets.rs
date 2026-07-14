pub struct WorthQuerySubscriptionPhaseSevenCompileFailTarget {
    path: &'static str,
    forbidden_substitution: &'static str,
}

impl WorthQuerySubscriptionPhaseSevenCompileFailTarget {
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

pub struct WorthQuerySubscriptionPhaseSevenGoldenPath {
    path: &'static str,
}

impl WorthQuerySubscriptionPhaseSevenGoldenPath {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_TARGETS: &[WorthQuerySubscriptionPhaseSevenCompileFailTarget] = &[
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_posture_cannot_author_admission.rs",
        "derived subscription posture cannot substitute for scoped declaration proof",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_source_digest_cannot_feed_authority.rs",
        "source digest string cannot satisfy subscription authority",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_projection_label_cannot_feed_authority.rs",
        "terminal projection label cannot satisfy subscription authority",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_query_digest_getter_removed.rs",
        "query digest getter removed from live admission artifact",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_plan_digest_getter_removed.rs",
        "plan digest getter removed from live admission artifact",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_declaration_for_reporting_removed.rs",
        "declaration_for_reporting removed in favor of declaration_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_runtime_cert_failure_digest_removed.rs",
        "runtime certification failure_digest removed in favor of failure_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_support_profile_source_digest_removed.rs",
        "support profile source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_trace_digest_removed.rs",
        "diagnostic trace_digest removed in favor of trace_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_equivalence_for_reporting_removed.rs",
        "equivalence_for_reporting removed in favor of equivalence_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_semantic_labels_for_reporting_removed.rs",
        "semantic labels_for_reporting removed in favor of labels_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_semantic_labels_digest_removed.rs",
        "semantic labels digest removed in favor of labels_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_assembly_receipt_for_reporting_removed.rs",
        "assembly_receipt_for_reporting removed in favor of assembly_receipt_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_assembly_receipt_digest_removed.rs",
        "assembly receipt digest removed in favor of assembly_receipt_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_diagnostic_bundle_width_for_reporting_removed.rs",
        "bundle_width_for_reporting removed in favor of bundle_width_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_basis_request_digest_removed.rs",
        "basis request digest removed in favor of basis_binding_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_active_counters_digest_removed.rs",
        "active counters digest removed in favor of counter_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_delivery_error_source_digest_removed.rs",
        "delivery error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_patch_group_for_reporting_removed.rs",
        "patch_group_for_reporting removed in favor of patch_group_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_delivery_batch_receipt_digest_removed.rs",
        "delivery batch receipt_digest removed in favor of receipt_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_lifecycle_closeout_source_digest_removed.rs",
        "lifecycle closeout source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_bridge_parity_failure_source_digest_removed.rs",
        "bridge parity failure source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_delivery_window_digest_removed.rs",
        "delivery window digest removed in favor of delivery_window_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_maintenance_delta_digest_removed.rs",
        "maintenance delta digest removed in favor of maintenance_delta_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_maintenance_delta_scope_digest_removed.rs",
        "maintenance delta affected_scope_digest removed in favor of scope_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_work_packet_digest_removed.rs",
        "work packet digest removed in favor of work_packet_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_active_lifecycle_error_source_digest_removed.rs",
        "active lifecycle error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_terminal_label_as_str_escape_removed.rs",
        "terminal projection label as_terminal_label escape removed",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_equivalence_digest_removed.rs",
        "equivalence digest removed in favor of equivalence_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_declaration_digest_removed.rs",
        "declaration digest removed in favor of declaration_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_attachment_error_source_digest_removed.rs",
        "attachment error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_continuation_error_source_digest_removed.rs",
        "continuation error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_preview_isolation_error_source_digest_removed.rs",
        "preview isolation error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_closeout_error_source_digest_removed.rs",
        "closeout error source_digest removed in favor of source_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_live_policy_digest_removed.rs",
        "live admission policy_digest removed in favor of policy_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_lane_digest_getter_removed.rs",
        "active lane digest getter removed in favor of lane_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_runtime_cert_variation_digest_removed.rs",
        "runtime certification variation digest removed in favor of variation_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_live_tenant_digest_removed.rs",
        "live admission tenant_digest removed in favor of tenant_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_live_relationship_proof_digest_removed.rs",
        "live admission relationship_proof_digest removed in favor of relationship_proof_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_live_collection_digest_removed.rs",
        "live admission collection_digest removed in favor of collection_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_attachment_digest_getter_removed.rs",
        "consumer attachment digest getter removed in favor of attachment_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_future_selection_projection_digest_removed.rs",
        "future selection projection_digest removed in favor of future_selection_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_runtime_cert_scope_digest_removed.rs",
        "runtime certification scope_digest removed in favor of scope_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_phase_seven/subscription_support_capability_digest_removed.rs",
        "support matrix capability_digest removed in favor of capability_projection",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_diagnostic_evidence_constructor_private.rs",
        "diagnostic evidence struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_certification_bundle_constructor_private.rs",
        "certification bundle digest struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_support_report_constructor_private.rs",
        "support report struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_lifecycle_closeout_constructor_private.rs",
        "lifecycle closeout struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_lifecycle_certification_bundle_constructor_private.rs",
        "lifecycle certification bundle struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_support_evidence_constructor_private.rs",
        "support evidence struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_runtime_certification_scope_constructor_private.rs",
        "runtime certification scope struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_diagnostic_bundle_missing_hostile_coverage_forbidden.rs",
        "diagnostic denied bundle hostile coverage struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_support_subject_constructor_private.rs",
        "subscription support subject struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_diagnostic_bundle_constructor_private.rs",
        "subscription diagnostic bundle struct literal forbidden",
    ),
    WorthQuerySubscriptionPhaseSevenCompileFailTarget::new(
        "tests/ui/subscription_runtime_certification_bundle_constructor_private.rs",
        "subscription runtime certification bundle struct literal forbidden",
    ),
];

const GOLDEN_PATHS: &[WorthQuerySubscriptionPhaseSevenGoldenPath] = &[
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_diagnostic_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_input_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_support_profile_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_diagnostic_bundle_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_declaration_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_plan_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_delivery_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_bridge_parity_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_equivalence_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_error_source_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_input_context_projection_golden_path_compiles.rs",
    ),
    WorthQuerySubscriptionPhaseSevenGoldenPath::new(
        "tests/ui/subscription_phase_seven/golden/subscription_runtime_cert_scope_projection_golden_path_compiles.rs",
    ),
];

pub const fn worth_query_subscription_phase_seven_compile_fail_targets(
) -> &'static [WorthQuerySubscriptionPhaseSevenCompileFailTarget] {
    COMPILE_FAIL_TARGETS
}

pub const fn worth_query_subscription_phase_seven_golden_paths(
) -> &'static [WorthQuerySubscriptionPhaseSevenGoldenPath] {
    GOLDEN_PATHS
}

pub const WORTH_QUERY_SUBSCRIPTION_PHASE_SEVEN_COMPILE_FAIL_TARGET_COUNT: usize =
    COMPILE_FAIL_TARGETS.len();

pub const WORTH_QUERY_SUBSCRIPTION_PHASE_SEVEN_GOLDEN_PATH_COUNT: usize = GOLDEN_PATHS.len();
