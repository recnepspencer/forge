use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationSupportLane,
};

use super::execution_result_row::WorthQueryGraphObligationExecutionResultRow;
use super::reduction::WorthQueryGraphObligationReduction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDenialProjection {
    reduction_digest: String,
    blocking_count: usize,
    rows: Vec<WorthQueryGraphObligationDenialProjectionRow>,
    projection_digest: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDenialProjectionRow {
    rule_identity_digest: String,
    rule_namespace: String,
    rule_name: String,
    rule_semantic_version: String,
    obligation_kind: WorthQueryGraphObligationKind,
    execution_status: WorthQueryGraphObligationExecutionStatus,
    verdict: String,
    verdict_context: Option<String>,
    support_lane: WorthQueryGraphObligationSupportLane,
    execution_input_digest: String,
    executor_contract_digest: String,
    row_digest: String,
    projection_row_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationDenialProjection {
    pub fn from_reduction(reduction: &WorthQueryGraphObligationReduction) -> Option<Self> {
        let mut rows = reduction
            .rows()
            .iter()
            .filter(|row| row.verdict().is_some_and(|verdict| verdict.is_blocking()))
            .map(WorthQueryGraphObligationDenialProjectionRow::from_execution_row)
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            left.rule_identity_digest()
                .cmp(right.rule_identity_digest())
                .then_with(|| left.row_digest().cmp(right.row_digest()))
        });
        let blocking_count = rows.len();
        if blocking_count == 0 {
            return None;
        }
        let reduction_digest = reduction.reduction_digest().to_string();
        let row_digests = rows
            .iter()
            .map(WorthQueryGraphObligationDenialProjectionRow::projection_row_evidence_digest)
            .collect::<Vec<_>>();
        let projection_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationDenialProjection)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("reduction"),
                    reduction.reduction_evidence_digest(),
                )
                .field_usize(WorthQueryEvidenceTag::new("blocking_count"), blocking_count)
                .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("row"), row_digests)
                .seal();
        Some(Self {
            reduction_digest,
            blocking_count,
            rows,
            projection_digest,
        })
    }

    pub fn reduction_digest(&self) -> &str {
        &self.reduction_digest
    }

    pub fn blocking_count(&self) -> usize {
        self.blocking_count
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationDenialProjectionRow] {
        &self.rows
    }

    pub fn projection_digest(&self) -> &str {
        self.projection_digest.as_str()
    }
}

impl WorthQueryGraphObligationDenialProjectionRow {
    fn from_execution_row(row: &WorthQueryGraphObligationExecutionResultRow) -> Self {
        let input = row.input();
        let registration = input.selected_registration();
        let rule_identity = registration.rule_identity();
        let verdict = row
            .verdict()
            .expect("denial projection row requires a blocking verdict");
        let rule_identity_digest = rule_identity.identity_digest().to_string();
        let rule_namespace = rule_identity.namespace().to_string();
        let rule_name = rule_identity.name().to_string();
        let rule_semantic_version = rule_identity.semantic_version().to_string();
        let obligation_kind = registration.kind();
        let execution_status = row.status();
        let verdict_value = verdict.as_str().to_string();
        let verdict_context = verdict.context().map(str::to_string);
        let support_lane = input.executor_contract().support_lane();
        let execution_input_digest = input.input_digest().to_string();
        let executor_contract_digest = input.executor_contract().contract_digest().to_string();
        let row_digest = row.row_digest().to_string();
        let projection_row_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationDenialProjectionRow,
        )
        .field_value(WorthQueryEvidenceTag::new("rule"), &rule_identity_digest)
        .field_shape(WorthQueryEvidenceTag::new("kind"), obligation_kind.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("status"),
            execution_status.as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("verdict"), &verdict_value)
        .optional_value(
            WorthQueryEvidenceTag::new("verdict_context"),
            verdict_context.as_deref(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("execution_input"),
            &execution_input_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("executor_contract"),
            &executor_contract_digest,
        )
        .field_value(WorthQueryEvidenceTag::new("row"), &row_digest)
        .seal();
        Self {
            rule_identity_digest,
            rule_namespace,
            rule_name,
            rule_semantic_version,
            obligation_kind,
            execution_status,
            verdict: verdict_value,
            verdict_context,
            support_lane,
            execution_input_digest,
            executor_contract_digest,
            row_digest,
            projection_row_digest,
        }
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn rule_namespace(&self) -> &str {
        &self.rule_namespace
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn rule_semantic_version(&self) -> &str {
        &self.rule_semantic_version
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn execution_status(&self) -> WorthQueryGraphObligationExecutionStatus {
        self.execution_status
    }

    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    pub fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub fn support_lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn execution_input_digest(&self) -> &str {
        &self.execution_input_digest
    }

    pub fn executor_contract_digest(&self) -> &str {
        &self.executor_contract_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn projection_row_digest(&self) -> &str {
        self.projection_row_digest.as_str()
    }

    pub(crate) fn projection_row_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.projection_row_digest
    }
}
