use super::ForgeQueryIdentityBoundaryHostileMatrixRow;
#[path = "identity_boundary_hostile_intent_receipt_fixtures.rs"]
mod identity_boundary_hostile_intent_receipt_fixtures;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
};
use crate::session_label::ForgeQuerySessionLabel;
use identity_boundary_hostile_intent_receipt_fixtures::{
    authoritative_intent_receipt_identity_fixture, preview_intent_admission_identity_fixture,
    preview_intent_receipt_identity_fixture, preview_intent_receipt_inspection_basis_fixture,
    preview_intent_receipt_inspection_identity_fixture, sample_identity,
};

pub(super) fn evidence_identity_delimiter_collision_resistance_row(
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let authority = ForgeQueryRuntimeEvidenceAuthority::new();
    let left = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let certified = left.admission_identity() != right.admission_identity();
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
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
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let left = authoritative_intent_receipt_identity_fixture("intent|receipt", "strategy");
    let right = authoritative_intent_receipt_identity_fixture("intent", "receipt|strategy");
    let certified = left != right;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
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
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let left_intent_receipt =
        authoritative_intent_receipt_identity_fixture("intent|receipt", "strategy");
    let right_intent_receipt =
        authoritative_intent_receipt_identity_fixture("intent", "receipt|strategy");
    let left = forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("effect_name"), "effect|receipt")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::EffectTriggerCommitIdentity,
                "trigger|commit",
            ),
        )
        .field_shape(ForgeQueryEvidenceTag::new("trigger_source_kind"), "write")
        .field_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger_digest"),
            "write-adjacent-trigger",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pending_intent_target"),
            "pending|target",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            "effect-triggered",
        )
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), "workspace")
        .field_shape(ForgeQueryEvidenceTag::new("effect_policy"), "authoritative")
        .field_value_sequence(ForgeQueryEvidenceTag::new("phase"), ["pending", "executed"])
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            "trigger-commit",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            "pending-intent-receipt",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            &left_intent_receipt,
        )
        .seal();
    let right = forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("effect_name"), "effect")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_evidence_identity"),
            &sample_identity(
                ForgeQueryEvidenceScope::EffectTriggerCommitIdentity,
                "receipt|commit",
            ),
        )
        .field_shape(ForgeQueryEvidenceTag::new("trigger_source_kind"), "write")
        .field_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger_digest"),
            "write-adjacent-trigger",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pending_intent_target"),
            "pending|target",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            "effect-triggered",
        )
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), "workspace")
        .field_shape(ForgeQueryEvidenceTag::new("effect_policy"), "authoritative")
        .field_value_sequence(ForgeQueryEvidenceTag::new("phase"), ["pending", "executed"])
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            "trigger-commit",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("idempotence"),
            "pending-intent-receipt",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("intent_receipt_identity"),
            &right_intent_receipt,
        )
        .seal();
    let certified = left != right;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
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
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let left =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentExecutionProvenanceChain)
            .field_shape(ForgeQueryEvidenceTag::new("family"), "intent|family")
            .field_shape(ForgeQueryEvidenceTag::new("entrypoint"), "entrypoint")
            .field_shape(ForgeQueryEvidenceTag::new("seam"), "seam")
            .field_identity(
                ForgeQueryEvidenceTag::new("admission_decision_digest"),
                "decision",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_handoff_digest"),
                "handoff",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_binding_digest"),
                "binding",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_outcome_digest"),
                "outcome",
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot_token"),
                &sample_identity(
                    ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
                    "snapshot|token",
                ),
            )
            .seal();
    let right =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentExecutionProvenanceChain)
            .field_shape(ForgeQueryEvidenceTag::new("family"), "intent")
            .field_shape(ForgeQueryEvidenceTag::new("entrypoint"), "entrypoint")
            .field_shape(ForgeQueryEvidenceTag::new("seam"), "seam")
            .field_identity(
                ForgeQueryEvidenceTag::new("admission_decision_digest"),
                "decision",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_handoff_digest"),
                "handoff",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_binding_digest"),
                "binding",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("execution_outcome_digest"),
                "family|outcome",
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot_token"),
                &sample_identity(
                    ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
                    "snapshot|token",
                ),
            )
            .seal();
    let certified = left != right;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
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
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
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
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
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
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let left = preview_intent_receipt_inspection_basis_fixture("preview|intent", "alpha");
    let right = preview_intent_receipt_inspection_basis_fixture("preview", "intent|alpha");
    let certified = left != right;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "preview-intent-receipt-inspection-basis-identity-delimiter-boundaries",
        certified,
        witness_digest(
            "preview-intent-receipt-inspection-basis-identity-delimiter-boundaries",
            certified,
            [left.as_str(), right.as_str()],
        ),
    )
}

fn test_session_label(label: &str) -> ForgeQuerySessionLabel {
    ForgeQuerySessionLabel::scoped_strs("forge-query-identity-boundary", [label]).expect("label")
}

fn witness_digest<'a>(
    row_name: &'static str,
    certified: bool,
    evidence: impl IntoIterator<Item = &'a str>,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_shape(ForgeQueryEvidenceTag::new("row_name"), row_name)
        .field_bool(ForgeQueryEvidenceTag::new("certified"), certified)
        .field_identity_sequence(ForgeQueryEvidenceTag::new("evidence"), evidence)
        .seal()
        .as_str()
        .to_string()
}
