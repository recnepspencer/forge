use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionCompileFailTarget {
    path: &'static str,
}

impl WorthQueryIntentAdmissionCompileFailTarget {
    pub(crate) const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionGoldenTranscript {
    path: &'static str,
}

impl WorthQueryIntentAdmissionGoldenTranscript {
    pub(crate) const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionCrateDocExampleTarget {
    label: &'static str,
    path: &'static str,
}

impl WorthQueryIntentAdmissionCrateDocExampleTarget {
    pub(crate) const fn new(label: &'static str, path: &'static str) -> Self {
        Self { label, path }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_TARGETS: [WorthQueryIntentAdmissionCompileFailTarget; 36] = [
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/runtime_intent_admission_review_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/admitted_runtime_intent_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/raw_request_cannot_mint_admitted_plan.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/family/family_inventory_row_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/family/family_inventory_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/support/support_row_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/support/support_matrix_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/coverage/coverage_row_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/coverage/coverage_inventory_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_plan_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_plan_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_handoff_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_handoff_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_execution_binding_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_execution_binding_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/advisory_decision_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/violation_decision_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/advisory_stop_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/violation_stop_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/trace/decision_trace_row_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/trace/decision_trace_envelope_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/consumer/intent_consumer_inspection_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/bundle/intent_admission_certification_bundle_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/audits/intent_admission_public_boundary_audit_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/audits/intent_admission_proof_shape_audit_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/audits/intent_admission_topology_audit_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/bundle/intent_admission_certification_output_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/parity/intent_admission_oracle_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/parity/intent_admission_legacy_parity_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/parity/intent_admission_support_traceability_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/parity/intent_admission_slope_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/parity/intent_admission_width_run_row_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/artifacts/intent_admission_representative_output_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/artifacts/intent_admission_representative_family_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/artifacts/intent_admission_doc_example_report_constructor_private.rs",
    ),
    WorthQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/reports/artifacts/intent_admission_seeded_certification_report_constructor_private.rs",
    ),
];

const GOLDEN_TRANSCRIPTS: [WorthQueryIntentAdmissionGoldenTranscript; 5] = [
    WorthQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_common_path_golden_transcript_compiles.rs",
    ),
    WorthQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_advanced_path_golden_transcript_compiles.rs",
    ),
    WorthQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_consumer_lane_golden_transcript_compiles.rs",
    ),
    WorthQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_basis_projection_golden_transcript_compiles.rs",
    ),
    WorthQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_read_mutation_inspection_routing_golden_transcript_compiles.rs",
    ),
];

const CRATE_DOC_EXAMPLE_TARGETS: [WorthQueryIntentAdmissionCrateDocExampleTarget; 5] = [
    WorthQueryIntentAdmissionCrateDocExampleTarget::new(
        "common_path",
        "tests/ui/intent_admission/docs/intent_admission_doc_common_path_compiles.rs",
    ),
    WorthQueryIntentAdmissionCrateDocExampleTarget::new(
        "advanced_path",
        "tests/ui/intent_admission/docs/intent_admission_doc_advanced_path_compiles.rs",
    ),
    WorthQueryIntentAdmissionCrateDocExampleTarget::new(
        "consumer_lane",
        "tests/ui/intent_admission/docs/intent_admission_doc_consumer_lane_compiles.rs",
    ),
    WorthQueryIntentAdmissionCrateDocExampleTarget::new(
        "basis_projection",
        "tests/ui/intent_admission/docs/intent_admission_doc_basis_projection_compiles.rs",
    ),
    WorthQueryIntentAdmissionCrateDocExampleTarget::new(
        "read_mutation_inspection_routing",
        "tests/ui/intent_admission/docs/intent_admission_doc_read_mutation_inspection_routing_compiles.rs",
    ),
];

pub fn worth_query_intent_admission_compile_fail_targets(
) -> &'static [WorthQueryIntentAdmissionCompileFailTarget] {
    &COMPILE_FAIL_TARGETS
}

pub fn worth_query_intent_admission_golden_transcripts(
) -> &'static [WorthQueryIntentAdmissionGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
}

pub fn worth_query_intent_admission_crate_doc_example_targets(
) -> &'static [WorthQueryIntentAdmissionCrateDocExampleTarget] {
    &CRATE_DOC_EXAMPLE_TARGETS
}

pub(super) fn compile_fail_boundary_digest() -> String {
    hash_parts(
        &COMPILE_FAIL_TARGETS
            .iter()
            .map(|target| format!("compile-fail:{}", target.path()))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn golden_transcript_digest() -> String {
    hash_parts(
        &GOLDEN_TRANSCRIPTS
            .iter()
            .map(|target| format!("golden:{}", target.path()))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn crate_doc_example_target_digest() -> String {
    hash_parts(
        &CRATE_DOC_EXAMPLE_TARGETS
            .iter()
            .map(|target| format!("{}:{}", target.label(), target.path()))
            .collect::<Vec<_>>(),
    )
}
