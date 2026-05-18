use crate::identity::hash_parts;

use super::super::fixtures::{legacy_delegation_parity_fixture, LegacyDelegationParityFixture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionLegacyParityLane {
    AuthoritativeExecution,
    EffectExecution,
    ReadExecutionCurrent,
    ReadExecutionInBasisContext,
}

impl ForgeQueryIntentAdmissionLegacyParityLane {
    fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeExecution => "authoritative_execution",
            Self::EffectExecution => "effect_execution",
            Self::ReadExecutionCurrent => "read_execution_current",
            Self::ReadExecutionInBasisContext => "read_execution_in_basis_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionLegacyParityRow {
    lane: ForgeQueryIntentAdmissionLegacyParityLane,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionLegacyParityRow {
    pub fn lane(&self) -> ForgeQueryIntentAdmissionLegacyParityLane {
        self.lane
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionLegacyParityReport {
    rows: Vec<ForgeQueryIntentAdmissionLegacyParityRow>,
    legacy_delegation_parity_digest: String,
}

impl ForgeQueryIntentAdmissionLegacyParityReport {
    pub fn rows(&self) -> &[ForgeQueryIntentAdmissionLegacyParityRow] {
        &self.rows
    }

    pub fn legacy_delegation_parity_digest(&self) -> &str {
        &self.legacy_delegation_parity_digest
    }
}

pub fn forge_query_intent_admission_legacy_parity_report(
) -> ForgeQueryIntentAdmissionLegacyParityReport {
    let fixture = legacy_delegation_parity_fixture();
    let rows = vec![
        authoritative_execution_row(&fixture),
        effect_execution_row(&fixture),
        read_execution_current_row(&fixture),
        read_execution_basis_context_row(&fixture),
    ];
    let legacy_delegation_parity_digest = hash_row_digests(&rows);
    ForgeQueryIntentAdmissionLegacyParityReport {
        rows,
        legacy_delegation_parity_digest,
    }
}

fn authoritative_execution_row(
    fixture: &LegacyDelegationParityFixture,
) -> ForgeQueryIntentAdmissionLegacyParityRow {
    parity_row(
        ForgeQueryIntentAdmissionLegacyParityLane::AuthoritativeExecution,
        [
            digest_equality(
                "decision",
                fixture.authoritative_legacy.admission_decision_digest(),
                fixture.authoritative_canonical.admission_decision_digest(),
            ),
            digest_equality(
                "handoff",
                fixture.authoritative_legacy.execution_handoff_digest(),
                fixture.authoritative_canonical.execution_handoff_digest(),
            ),
            digest_equality(
                "binding",
                fixture.authoritative_legacy.execution_binding_digest(),
                fixture.authoritative_canonical.execution_binding_digest(),
            ),
            digest_equality(
                "provenance",
                fixture
                    .authoritative_legacy
                    .execution_provenance_chain_digest(),
                fixture
                    .authoritative_canonical
                    .execution_provenance_chain_digest(),
            ),
            digest_equality(
                "result",
                fixture.authoritative_legacy.receipt_digest(),
                fixture.authoritative_canonical.receipt_digest(),
            ),
        ],
    )
}

fn effect_execution_row(
    fixture: &LegacyDelegationParityFixture,
) -> ForgeQueryIntentAdmissionLegacyParityRow {
    parity_row(
        ForgeQueryIntentAdmissionLegacyParityLane::EffectExecution,
        [
            digest_equality(
                "decision",
                fixture
                    .effect_legacy
                    .intent_receipt()
                    .admission_decision_digest(),
                fixture
                    .effect_canonical
                    .intent_receipt()
                    .admission_decision_digest(),
            ),
            digest_equality(
                "handoff",
                fixture
                    .effect_legacy
                    .intent_receipt()
                    .execution_handoff_digest(),
                fixture
                    .effect_canonical
                    .intent_receipt()
                    .execution_handoff_digest(),
            ),
            digest_equality(
                "binding",
                fixture
                    .effect_legacy
                    .intent_receipt()
                    .execution_binding_digest(),
                fixture
                    .effect_canonical
                    .intent_receipt()
                    .execution_binding_digest(),
            ),
            digest_equality(
                "provenance",
                fixture.effect_legacy.execution_provenance_chain_digest(),
                fixture.effect_canonical.execution_provenance_chain_digest(),
            ),
            digest_equality(
                "result",
                fixture.effect_legacy.intent_receipt().receipt_digest(),
                fixture.effect_canonical.intent_receipt().receipt_digest(),
            ),
        ],
    )
}

fn read_execution_current_row(
    fixture: &LegacyDelegationParityFixture,
) -> ForgeQueryIntentAdmissionLegacyParityRow {
    parity_row(
        ForgeQueryIntentAdmissionLegacyParityLane::ReadExecutionCurrent,
        [
            optional_digest_equality(
                "trace",
                fixture
                    .read_current_legacy
                    .receipt()
                    .decision_trace_envelope()
                    .map(|trace| trace.trace_digest()),
                fixture
                    .read_current_canonical
                    .receipt()
                    .decision_trace_envelope()
                    .map(|trace| trace.trace_digest()),
            ),
            optional_digest_equality(
                "provenance",
                fixture
                    .read_current_legacy
                    .receipt()
                    .execution_provenance_chain_digest(),
                fixture
                    .read_current_canonical
                    .receipt()
                    .execution_provenance_chain_digest(),
            ),
            digest_equality(
                "result",
                fixture.read_current_legacy.receipt().result_digest(),
                fixture.read_current_canonical.receipt().result_digest(),
            ),
        ],
    )
}

fn read_execution_basis_context_row(
    fixture: &LegacyDelegationParityFixture,
) -> ForgeQueryIntentAdmissionLegacyParityRow {
    parity_row(
        ForgeQueryIntentAdmissionLegacyParityLane::ReadExecutionInBasisContext,
        [
            optional_digest_equality(
                "trace",
                fixture
                    .read_basis_legacy
                    .receipt()
                    .decision_trace_envelope()
                    .map(|trace| trace.trace_digest()),
                fixture
                    .read_basis_canonical
                    .receipt()
                    .decision_trace_envelope()
                    .map(|trace| trace.trace_digest()),
            ),
            optional_digest_equality(
                "provenance",
                fixture
                    .read_basis_legacy
                    .receipt()
                    .execution_provenance_chain_digest(),
                fixture
                    .read_basis_canonical
                    .receipt()
                    .execution_provenance_chain_digest(),
            ),
            digest_equality(
                "result",
                fixture.read_basis_legacy.receipt().result_digest(),
                fixture.read_basis_canonical.receipt().result_digest(),
            ),
        ],
    )
}

fn parity_row<const N: usize>(
    lane: ForgeQueryIntentAdmissionLegacyParityLane,
    checks: [String; N],
) -> ForgeQueryIntentAdmissionLegacyParityRow {
    ForgeQueryIntentAdmissionLegacyParityRow {
        lane,
        row_digest: hash_parts(&[
            "forge_query_intent_admission_legacy_parity_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            hash_parts(&checks),
        ]),
    }
}

fn digest_equality(label: &str, left: &str, right: &str) -> String {
    format!("{label}:{}", left == right)
}

fn optional_digest_equality(label: &str, left: Option<&str>, right: Option<&str>) -> String {
    format!("{label}:{}", left == right)
}

fn hash_row_digests(rows: &[ForgeQueryIntentAdmissionLegacyParityRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}
