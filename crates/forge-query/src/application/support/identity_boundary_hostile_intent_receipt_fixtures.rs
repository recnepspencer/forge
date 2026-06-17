use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

pub(super) fn authoritative_intent_receipt_identity_fixture(
    intent_name: &str,
    strategy_identity: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::AuthoritativeIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
        .field_shape(ForgeQueryEvidenceTag::new("execution_kind"), "mutating")
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            strategy_identity,
        )
        .field_shape(ForgeQueryEvidenceTag::new("strategy_version"), "1.0")
        .field_value(
            ForgeQueryEvidenceTag::new("strategy_descriptor_digest"),
            "strategy-descriptor",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            "canonical-input",
        )
        .field_value(ForgeQueryEvidenceTag::new("outcome_digest"), "outcome")
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            ["invariant|one", "invariant:two"],
        )
        .field_shape(ForgeQueryEvidenceTag::new("source_lane"), "strategy")
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), "workspace")
        .optional_identity(
            ForgeQueryEvidenceTag::new("effect_trigger_digest"),
            Option::<&str>::None,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
                "commit|identity",
            ),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
                "snapshot|identity",
            ),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("affected_live_view_id"),
            ["live|view"],
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("affected_derived_view_id"),
            ["derived|view"],
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("considered_computed_view_count"),
            1,
        )
        .field_usize(ForgeQueryEvidenceTag::new("considered_effect_count"), 1)
        .field_usize(ForgeQueryEvidenceTag::new("delivered_effect_count"), 1)
        .field_usize(ForgeQueryEvidenceTag::new("pending_write_intent_count"), 0)
        .field_usize(ForgeQueryEvidenceTag::new("suppressed_effect_count"), 0)
        .field_usize(
            ForgeQueryEvidenceTag::new("meaningful_effect_suppression_count"),
            0,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_expression_failure_count"),
            0,
        )
        .field_bool(ForgeQueryEvidenceTag::new("refresh_fallback"), false)
        .field_shape(
            ForgeQueryEvidenceTag::new("admission_family"),
            "authoritative",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("covered_entrypoint"),
            "runtime.write",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_seam"),
            "intent-authority",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("admission_decision_digest"),
            "decision",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_handoff_digest"),
            "handoff",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("execution_binding_digest"),
            "binding",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("execution_provenance_chain_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::IntentExecutionProvenanceChain,
                "provenance|chain",
            ),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("decision_trace_digest"),
            "decision-trace",
        )
        .seal()
}

pub(super) fn preview_intent_receipt_inspection_basis_fixture(
    intent_name: &str,
    basis_label: &str,
) -> ForgeQueryEvidenceIdentity {
    let basis_admission_identity =
        sample_identity(ForgeQueryEvidenceScope::PreviewBasisAdmission, basis_label);
    let basis_evidence_identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentBasisEvidence)
            .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
            .field_usize(ForgeQueryEvidenceTag::new("basis_evidence_count"), 2)
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("basis_admission_identity"),
                &basis_admission_identity,
            )
            .seal();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceiptInspectionBasis)
        .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
        .field_usize(ForgeQueryEvidenceTag::new("basis_evidence_count"), 2)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_evidence"),
            &basis_evidence_identity,
        )
        .seal()
}

pub(super) fn preview_intent_admission_identity_fixture(
    intent_name: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentAdmission)
        .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
        .field_shape(ForgeQueryEvidenceTag::new("strategy_identity"), "strategy")
        .field_shape(ForgeQueryEvidenceTag::new("strategy_version"), "1.0")
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            "canonical-input",
        )
        .field_shape(ForgeQueryEvidenceTag::new("source_lane"), "strategy")
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), "preview")
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            "sandboxed-write-intent",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admitted_action"),
            "write-intent",
        )
        .field_shape(ForgeQueryEvidenceTag::new("admitted_lane"), "preview-truth")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_admission_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::PreviewBasisAdmission,
                "preview-basis",
            ),
        )
        .seal()
}

pub(super) fn preview_intent_receipt_identity_fixture(
    admission_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceipt)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            admission_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("posture"),
            "preview-local-staged-no-authoritative-execution",
        )
        .seal()
}

pub(super) fn preview_intent_receipt_inspection_identity_fixture(
    intent_name: &str,
    basis_identity: &ForgeQueryEvidenceIdentity,
    admission_identity: &ForgeQueryEvidenceIdentity,
    receipt_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceiptInspection)
        .field_shape(ForgeQueryEvidenceTag::new("intent_name"), intent_name)
        .field_value(ForgeQueryEvidenceTag::new("strategy_identity"), "strategy")
        .field_shape(ForgeQueryEvidenceTag::new("strategy_version"), "1.0")
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            "canonical-input",
        )
        .field_shape(ForgeQueryEvidenceTag::new("source_lane"), "strategy")
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), "preview")
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            "sandboxed-write-intent",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("basis_identity"), basis_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            admission_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_identity"),
            receipt_identity,
        )
        .seal()
}

pub(super) fn sample_identity(
    scope: ForgeQueryEvidenceScope,
    label: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(scope)
        .field_shape(ForgeQueryEvidenceTag::new("fixture"), label)
        .seal()
}
