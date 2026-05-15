use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCompileFailTarget {
    path: &'static str,
}

impl ForgeQueryIntentAdmissionCompileFailTarget {
    pub(crate) const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionGoldenTranscript {
    path: &'static str,
}

impl ForgeQueryIntentAdmissionGoldenTranscript {
    pub(crate) const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

const COMPILE_FAIL_TARGETS: [ForgeQueryIntentAdmissionCompileFailTarget; 35] = [
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/runtime_intent_admission_review_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/admitted_runtime_intent_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/authoring/raw_request_cannot_mint_admitted_plan.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/family/family_inventory_row_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/family/family_inventory_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/support/support_row_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/support/support_matrix_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/coverage/coverage_row_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/inventory/coverage/coverage_inventory_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_plan_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_plan_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_handoff_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_handoff_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/authoritative/authoritative_execution_binding_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/execution/effect/effect_execution_binding_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/advisory_decision_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/violation_decision_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/advisory_stop_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/outcomes/violation_stop_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/trace/decision_trace_row_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/trace/decision_trace_envelope_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/consumer/intent_consumer_inspection_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_certification_bundle_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_public_boundary_audit_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_proof_shape_audit_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_topology_audit_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_certification_output_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_oracle_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_legacy_parity_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_support_traceability_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_slope_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_representative_output_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_representative_family_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_doc_example_report_constructor_private.rs",
    ),
    ForgeQueryIntentAdmissionCompileFailTarget::new(
        "tests/ui/intent_admission/certification/intent_admission_seeded_certification_report_constructor_private.rs",
    ),
];

const GOLDEN_TRANSCRIPTS: [ForgeQueryIntentAdmissionGoldenTranscript; 3] = [
    ForgeQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_common_path_golden_transcript_compiles.rs",
    ),
    ForgeQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_advanced_path_golden_transcript_compiles.rs",
    ),
    ForgeQueryIntentAdmissionGoldenTranscript::new(
        "tests/ui/intent_admission/golden/intent_admission_consumer_lane_golden_transcript_compiles.rs",
    ),
];

pub fn forge_query_intent_admission_compile_fail_targets(
) -> &'static [ForgeQueryIntentAdmissionCompileFailTarget] {
    &COMPILE_FAIL_TARGETS
}

pub fn forge_query_intent_admission_golden_transcripts(
) -> &'static [ForgeQueryIntentAdmissionGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
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
