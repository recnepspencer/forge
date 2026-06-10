use super::super::super::support::*;

pub(in super::super) fn assert_canonical_evidence_identity_token(token: impl AsRef<str>) {
    let token = token.as_ref();
    assert!(
        token.starts_with("forge.query.evidence-identity.v1:forge.test.stable-digest-v1:"),
        "expected canonical evidence identity token, got {token}"
    );
}

pub(in super::super) fn assert_phase_one_surface_has_no_digest_folklore(source: &str) {
    for forbidden in [
        "hash_parts(",
        "digest_owned_parts(",
        ".join(\"|\")",
        "format!(\"{}|",
        "format!(\"{:?}\"",
        "format!(\"{:?}|",
        "format!(\"{:#?}\"",
        "format!(\"{:#?}|",
    ] {
        assert!(
            !source.contains(forbidden),
            "phase-1-covered surface must not retain digest folklore pattern {forbidden}"
        );
    }
}

pub(in super::super) fn compose_basis_admission_identity(
    scope: crate::ForgeQueryEvidenceScope,
    label: &ForgeQuerySessionLabel,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: impl IntoIterator<Item = impl Into<String>>,
) -> crate::ForgeQueryEvidenceIdentity {
    let evidence_rows = ForgeQueryBasisAdmissionEvidenceRow::rows_from_values(evidence);
    crate::ForgeQueryEvidenceIdentity::compose(scope)
        .field_identity(
            crate::ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().as_str(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("effect_policy"),
            effect_policy.as_str(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("authority_lane"),
            authority_lane.as_str(),
        )
        .field_identity_sequence(
            crate::ForgeQueryEvidenceTag::new("evidence_row"),
            evidence_rows
                .iter()
                .map(|row| row.row_digest().as_str()),
        )
        .seal()
}

pub(in super::super) fn compose_receipt_identity(
    scope: crate::ForgeQueryEvidenceScope,
    admission_digest: impl AsRef<str>,
    posture: &str,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(scope)
        .field_identity(
            crate::ForgeQueryEvidenceTag::new("admission_digest"),
            admission_digest,
        )
        .field_shape(crate::ForgeQueryEvidenceTag::new("posture"), posture)
        .seal()
}

pub(in super::super) fn compose_denial_evidence_identity(
    evidence: &ForgeQueryIntentDenialEvidence,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(crate::ForgeQueryEvidenceScope::IntentDenialEvidence)
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("intent_name"),
            evidence.intent_name(),
        )
        .field_shape(crate::ForgeQueryEvidenceTag::new("stage"), evidence.stage())
        .field_value(
            crate::ForgeQueryEvidenceTag::new("message"),
            evidence.message(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("strategy_identity"),
            evidence.strategy_identity(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("strategy_version"),
            evidence.strategy_version(),
        )
        .optional_value(
            crate::ForgeQueryEvidenceTag::new("returned_strategy_identity"),
            evidence.returned_strategy_identity(),
        )
        .optional_value(
            crate::ForgeQueryEvidenceTag::new("returned_strategy_version"),
            evidence.returned_strategy_version(),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("returned_strategy_descriptor_digest"),
            evidence.returned_strategy_descriptor_digest(),
        )
        .field_identity(
            crate::ForgeQueryEvidenceTag::new("canonical_input_digest"),
            evidence.canonical_input_digest(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("source_lane"),
            evidence.source_lane().as_str(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("target_lane"),
            evidence.target_lane().as_str(),
        )
        .optional_shape(
            crate::ForgeQueryEvidenceTag::new("execution_kind"),
            evidence
                .execution_kind()
                .map(ForgeQueryIntentExecutionKind::as_str),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("attempt_digest"),
            evidence.attempt_digest(),
        )
        .field_identity_sequence(
            crate::ForgeQueryEvidenceTag::new("invariant_evidence"),
            evidence.invariant_evidence().iter().map(String::as_str),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("snapshot_token"),
            evidence.snapshot_token(),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("execution_provenance"),
            evidence
                .execution_provenance()
                .map(ForgeQueryIntentExecutionProvenance::execution_provenance_chain_digest),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("decision_trace_digest"),
            evidence
                .decision_trace_envelope()
                .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest),
        )
        .seal()
}
