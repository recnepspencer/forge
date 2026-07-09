use super::WorthQueryIdentityBoundaryHostileMatrixRow;
#[path = "identity_boundary_hostile_intent_receipt_fixtures.rs"]
mod identity_boundary_hostile_intent_receipt_fixtures;

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryEffectPolicy, WorthQueryPreviewBasisAdmission, WorthQueryRuntimeEvidenceAuthority,
};
use crate::session_label::WorthQuerySessionLabel;
use identity_boundary_hostile_intent_receipt_fixtures::{
    authoritative_intent_receipt_identity_fixture, preview_intent_admission_identity_fixture,
    preview_intent_receipt_identity_fixture, preview_intent_receipt_inspection_basis_fixture,
    preview_intent_receipt_inspection_identity_fixture, sample_identity,
};

pub(super) fn evidence_identity_delimiter_collision_resistance_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let left = WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let certified = left.admission_identity() != right.admission_identity();
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "evidence-identity-delimiter-collision-resistance",
        certified,
        witness_digest(
            "evidence-identity-delimiter-collision-resistance",
            certified,
            [
                left.admission_identity().as_str(),
                right.admission_identity().as_str(),
            ],
        ),
    )
}

pub(super) fn authoritative_intent_receipt_identity_delimiter_boundaries_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left = authoritative_intent_receipt_identity_fixture("intent|receipt", "strategy");
    let right = authoritative_intent_receipt_identity_fixture("intent", "receipt|strategy");
    let certified = left != right;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "authoritative-intent-receipt-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "authoritative-intent-receipt-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

pub(super) fn effect_intent_receipt_identity_delimiter_boundaries_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left_intent_receipt =
        authoritative_intent_receipt_identity_fixture("intent|receipt", "strategy");
    let right_intent_receipt =
        authoritative_intent_receipt_identity_fixture("intent", "receipt|strategy");
    let left = worth_query_evidence_identity(WorthQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(WorthQueryEvidenceTag::new("effect_name"), "effect|receipt")
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            &sample_identity(
                WorthQueryEvidenceScope::EffectTriggerCommitIdentity,
                "trigger|commit",
            ),
        )
        .field_shape(WorthQueryEvidenceTag::new("trigger_source_kind"), "write")
        .field_value(
            WorthQueryEvidenceTag::new("write_adjacent_trigger_digest"),
            "write-adjacent-trigger",
        )
        .field_value(
            WorthQueryEvidenceTag::new("pending_intent_target"),
            "pending|target",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source_lane"),
            "effect-triggered",
        )
        .field_shape(WorthQueryEvidenceTag::new("target_lane"), "workspace")
        .field_shape(WorthQueryEvidenceTag::new("effect_policy"), "authoritative")
        .field_value_sequence(WorthQueryEvidenceTag::new("phase"), ["pending", "executed"])
        .field_shape(
            WorthQueryEvidenceTag::new("loop_prevention"),
            "trigger-commit",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("idempotence"),
            "pending-intent-receipt",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("intent_receipt_identity"),
            &left_intent_receipt,
        )
        .seal();
    let right = worth_query_evidence_identity(WorthQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(WorthQueryEvidenceTag::new("effect_name"), "effect")
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            &sample_identity(
                WorthQueryEvidenceScope::EffectTriggerCommitIdentity,
                "receipt|commit",
            ),
        )
        .field_shape(WorthQueryEvidenceTag::new("trigger_source_kind"), "write")
        .field_value(
            WorthQueryEvidenceTag::new("write_adjacent_trigger_digest"),
            "write-adjacent-trigger",
        )
        .field_value(
            WorthQueryEvidenceTag::new("pending_intent_target"),
            "pending|target",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source_lane"),
            "effect-triggered",
        )
        .field_shape(WorthQueryEvidenceTag::new("target_lane"), "workspace")
        .field_shape(WorthQueryEvidenceTag::new("effect_policy"), "authoritative")
        .field_value_sequence(WorthQueryEvidenceTag::new("phase"), ["pending", "executed"])
        .field_shape(
            WorthQueryEvidenceTag::new("loop_prevention"),
            "trigger-commit",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("idempotence"),
            "pending-intent-receipt",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("intent_receipt_identity"),
            &right_intent_receipt,
        )
        .seal();
    let certified = left != right;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "effect-intent-receipt-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "effect-intent-receipt-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

pub(super) fn intent_provenance_chain_identity_delimiter_boundaries_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left =
        worth_query_evidence_identity(WorthQueryEvidenceScope::IntentExecutionProvenanceChain)
            .field_shape(WorthQueryEvidenceTag::new("family"), "intent|family")
            .field_shape(WorthQueryEvidenceTag::new("entrypoint"), "entrypoint")
            .field_shape(WorthQueryEvidenceTag::new("seam"), "seam")
            .field_value(
                WorthQueryEvidenceTag::new("admission_decision_digest"),
                "decision",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_handoff_digest"),
                "handoff",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_binding_digest"),
                "binding",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_outcome_digest"),
                "outcome",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("snapshot_token"),
                &sample_identity(
                    WorthQueryEvidenceScope::WriteReceiptSnapshotIdentity,
                    "snapshot|token",
                ),
            )
            .seal();
    let right =
        worth_query_evidence_identity(WorthQueryEvidenceScope::IntentExecutionProvenanceChain)
            .field_shape(WorthQueryEvidenceTag::new("family"), "intent")
            .field_shape(WorthQueryEvidenceTag::new("entrypoint"), "entrypoint")
            .field_shape(WorthQueryEvidenceTag::new("seam"), "seam")
            .field_value(
                WorthQueryEvidenceTag::new("admission_decision_digest"),
                "decision",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_handoff_digest"),
                "handoff",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_binding_digest"),
                "binding",
            )
            .field_value(
                WorthQueryEvidenceTag::new("execution_outcome_digest"),
                "family|outcome",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("snapshot_token"),
                &sample_identity(
                    WorthQueryEvidenceScope::WriteReceiptSnapshotIdentity,
                    "snapshot|token",
                ),
            )
            .seal();
    let certified = left != right;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "intent-provenance-chain-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "intent-provenance-chain-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

pub(super) fn preview_intent_receipt_inspection_identity_delimiter_boundaries_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left_basis = preview_intent_receipt_inspection_basis_fixture("preview|intent", "alpha");
    let right_basis = preview_intent_receipt_inspection_basis_fixture("preview", "intent|alpha");
    let admission_identity = preview_intent_admission_identity_fixture("preview|intent");
    let receipt_identity = preview_intent_receipt_identity_fixture(&admission_identity);
    let left = preview_intent_receipt_inspection_identity_fixture(
        "preview|intent",
        &left_basis,
        &admission_identity,
        &receipt_identity,
    );
    let right = preview_intent_receipt_inspection_identity_fixture(
        "preview",
        &right_basis,
        &admission_identity,
        &receipt_identity,
    );
    let certified = left != right;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "preview-intent-receipt-inspection-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "preview-intent-receipt-inspection-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

pub(super) fn preview_intent_receipt_inspection_basis_identity_delimiter_boundaries_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left = preview_intent_receipt_inspection_basis_fixture("preview|intent", "alpha");
    let right = preview_intent_receipt_inspection_basis_fixture("preview", "intent|alpha");
    let certified = left != right;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "preview-intent-receipt-inspection-basis-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "preview-intent-receipt-inspection-basis-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

fn test_session_label(label: &str) -> WorthQuerySessionLabel {
    WorthQuerySessionLabel::scoped_strs("worth-query-identity-boundary", [label]).expect("label")
}

fn witness_digest<'a>(
    row_name: &'static str,
    certified: bool,
    evidence: impl IntoIterator<Item = &'a str>,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_shape(WorthQueryEvidenceTag::new("row_name"), row_name)
        .field_bool(WorthQueryEvidenceTag::new("certified"), certified)
        .field_value_sequence(WorthQueryEvidenceTag::new("evidence"), evidence)
        .seal()
        .as_str()
        .to_string()
}
