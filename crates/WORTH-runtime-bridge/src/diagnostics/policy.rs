use std::sync::Arc;

use crate::policy::{
    BridgePolicyProvenanceReport, BridgePolicyProvenanceReportRow, BridgePolicyRejection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyExplanationRow {
    label: Arc<str>,
    semantic_policy_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    provenance_digest: Arc<str>,
    replay_digest: Arc<str>,
}

impl BridgePolicyExplanationRow {
    pub fn from_report_row(row: &BridgePolicyProvenanceReportRow) -> Self {
        Self {
            label: Arc::from(row.label().to_owned()),
            semantic_policy_digest: Arc::from(row.semantic_policy_digest().to_owned()),
            lowered_policy_digest: Arc::from(row.lowered_policy_digest().to_owned()),
            provenance_digest: Arc::from(row.provenance_digest().to_owned()),
            replay_digest: Arc::from(row.replay_digest().to_owned()),
        }
    }

    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
    pub fn semantic_policy_digest(&self) -> &str {
        self.semantic_policy_digest.as_ref()
    }
    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }
    pub fn provenance_digest(&self) -> &str {
        self.provenance_digest.as_ref()
    }
    pub fn replay_digest(&self) -> &str {
        self.replay_digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyExplanation {
    report_digest: Arc<str>,
    rows: Arc<[BridgePolicyExplanationRow]>,
}

impl BridgePolicyExplanation {
    pub fn from_report(report: &BridgePolicyProvenanceReport) -> Self {
        Self {
            report_digest: Arc::from(report.digest().to_owned()),
            rows: Arc::from(
                report
                    .rows()
                    .iter()
                    .map(BridgePolicyExplanationRow::from_report_row)
                    .collect::<Vec<_>>(),
            ),
        }
    }

    pub fn report_digest(&self) -> &str {
        self.report_digest.as_ref()
    }
    pub fn rows(&self) -> &[BridgePolicyExplanationRow] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyRejectionExplanation {
    rejection_digest: Arc<str>,
    detail: Arc<str>,
}

impl BridgePolicyRejectionExplanation {
    pub fn from_rejection(rejection: &BridgePolicyRejection) -> Self {
        Self {
            rejection_digest: Arc::from(rejection.digest().to_owned()),
            detail: Arc::from(rejection.detail().to_owned()),
        }
    }

    pub fn rejection_digest(&self) -> &str {
        self.rejection_digest.as_ref()
    }
    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}
