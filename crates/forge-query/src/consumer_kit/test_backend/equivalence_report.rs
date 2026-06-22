use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryWriteReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendEquivalenceRow {
    field: &'static str,
    matched: bool,
    expected: String,
    found: String,
}

impl ForgeQueryTestBackendEquivalenceRow {
    fn present(field: &'static str, value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            field,
            matched: true,
            expected: value.clone(),
            found: value,
        }
    }

    fn compare(field: &'static str, expected: impl Into<String>, found: impl Into<String>) -> Self {
        let expected = expected.into();
        let found = found.into();
        Self {
            field,
            matched: expected == found,
            expected,
            found,
        }
    }

    pub fn field(&self) -> &'static str {
        self.field
    }

    pub fn matched(&self) -> bool {
        self.matched
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn found(&self) -> &str {
        &self.found
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendEquivalenceReport {
    rows: Vec<ForgeQueryTestBackendEquivalenceRow>,
    report_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryTestBackendEquivalenceReport {
    fn new(rows: Vec<ForgeQueryTestBackendEquivalenceRow>) -> Self {
        let report_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerEvidenceReport)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "in-memory-test-backend-equivalence",
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("field"),
                    rows.iter().map(ForgeQueryTestBackendEquivalenceRow::field),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("matched"),
                    rows.iter()
                        .map(|row| if row.matched() { "matched" } else { "mismatch" }),
                )
                .seal();
        Self {
            rows,
            report_identity,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryTestBackendEquivalenceRow] {
        &self.rows
    }

    pub fn matched(&self) -> bool {
        self.rows
            .iter()
            .all(ForgeQueryTestBackendEquivalenceRow::matched)
    }

    pub fn report_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
    }
}

pub fn compare_test_backend_write_receipts(
    in_memory: &ForgeQueryWriteReceipt,
    bridge: &ForgeQueryWriteReceipt,
) -> ForgeQueryTestBackendEquivalenceReport {
    ForgeQueryTestBackendEquivalenceReport::new(vec![
        ForgeQueryTestBackendEquivalenceRow::compare(
            "mutation_family",
            in_memory.mutation_family().as_str(),
            bridge.mutation_family().as_str(),
        ),
        ForgeQueryTestBackendEquivalenceRow::compare(
            "authority_lane",
            format!("{:?}", in_memory.authority_lane()),
            format!("{:?}", bridge.authority_lane()),
        ),
        ForgeQueryTestBackendEquivalenceRow::compare(
            "semantic_deltas",
            semantic_delta_rows(in_memory),
            semantic_delta_rows(bridge),
        ),
        ForgeQueryTestBackendEquivalenceRow::compare(
            "affected_live_view_count",
            in_memory.affected_live_view_ids().len().to_string(),
            bridge.affected_live_view_ids().len().to_string(),
        ),
        ForgeQueryTestBackendEquivalenceRow::present(
            "in_memory_commit_identity_present",
            (!in_memory.commit_identity().is_empty()).to_string(),
        ),
        ForgeQueryTestBackendEquivalenceRow::present(
            "bridge_commit_identity_present",
            (!bridge.commit_identity().is_empty()).to_string(),
        ),
        ForgeQueryTestBackendEquivalenceRow::present(
            "in_memory_snapshot_identity_present",
            (!in_memory
                .snapshot_evidence_identity()
                .terminal_projection_for_reporting()
                .is_empty())
            .to_string(),
        ),
        ForgeQueryTestBackendEquivalenceRow::present(
            "bridge_snapshot_identity_present",
            (!bridge
                .snapshot_evidence_identity()
                .terminal_projection_for_reporting()
                .is_empty())
            .to_string(),
        ),
    ])
}

fn semantic_delta_rows(receipt: &ForgeQueryWriteReceipt) -> String {
    receipt
        .deltas()
        .iter()
        .map(|delta| {
            format!(
                "{}:{:?}:{}",
                delta.collection(),
                delta.kind(),
                delta
                    .aspect_paths()
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}
