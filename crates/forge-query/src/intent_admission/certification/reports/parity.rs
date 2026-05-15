use crate::identity::hash_parts;

use super::super::fixtures::legacy_delegation_parity_fixture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionLegacyParityLane {
    AuthoritativeExecution,
    EffectExecution,
}

impl ForgeQueryIntentAdmissionLegacyParityLane {
    fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeExecution => "authoritative_execution",
            Self::EffectExecution => "effect_execution",
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
        ForgeQueryIntentAdmissionLegacyParityRow {
            lane: ForgeQueryIntentAdmissionLegacyParityLane::AuthoritativeExecution,
            row_digest: hash_parts(&[
                "forge_query_intent_admission_legacy_parity_row_v1".to_string(),
                format!(
                    "lane:{}",
                    ForgeQueryIntentAdmissionLegacyParityLane::AuthoritativeExecution.as_str()
                ),
                format!(
                    "decision:{}",
                    fixture.authoritative_legacy.admission_decision_digest()
                        == fixture.authoritative_canonical.admission_decision_digest()
                ),
                format!(
                    "handoff:{}",
                    fixture.authoritative_legacy.execution_handoff_digest()
                        == fixture.authoritative_canonical.execution_handoff_digest()
                ),
                format!(
                    "binding:{}",
                    fixture.authoritative_legacy.execution_binding_digest()
                        == fixture.authoritative_canonical.execution_binding_digest()
                ),
                format!(
                    "provenance:{}",
                    fixture
                        .authoritative_legacy
                        .execution_provenance_chain_digest()
                        == fixture
                            .authoritative_canonical
                            .execution_provenance_chain_digest()
                ),
                format!(
                    "result:{}",
                    fixture.authoritative_legacy.receipt_digest()
                        == fixture.authoritative_canonical.receipt_digest()
                ),
            ]),
        },
        ForgeQueryIntentAdmissionLegacyParityRow {
            lane: ForgeQueryIntentAdmissionLegacyParityLane::EffectExecution,
            row_digest: hash_parts(&[
                "forge_query_intent_admission_legacy_parity_row_v1".to_string(),
                format!(
                    "lane:{}",
                    ForgeQueryIntentAdmissionLegacyParityLane::EffectExecution.as_str()
                ),
                format!(
                    "decision:{}",
                    fixture
                        .effect_legacy
                        .intent_receipt()
                        .admission_decision_digest()
                        == fixture
                            .effect_canonical
                            .intent_receipt()
                            .admission_decision_digest()
                ),
                format!(
                    "handoff:{}",
                    fixture
                        .effect_legacy
                        .intent_receipt()
                        .execution_handoff_digest()
                        == fixture
                            .effect_canonical
                            .intent_receipt()
                            .execution_handoff_digest()
                ),
                format!(
                    "binding:{}",
                    fixture
                        .effect_legacy
                        .intent_receipt()
                        .execution_binding_digest()
                        == fixture
                            .effect_canonical
                            .intent_receipt()
                            .execution_binding_digest()
                ),
                format!(
                    "provenance:{}",
                    fixture.effect_legacy.execution_provenance_chain_digest()
                        == fixture.effect_canonical.execution_provenance_chain_digest()
                ),
                format!(
                    "result:{}",
                    fixture.effect_legacy.intent_receipt().receipt_digest()
                        == fixture.effect_canonical.intent_receipt().receipt_digest()
                ),
            ]),
        },
    ];
    let legacy_delegation_parity_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionLegacyParityReport {
        rows,
        legacy_delegation_parity_digest,
    }
}
