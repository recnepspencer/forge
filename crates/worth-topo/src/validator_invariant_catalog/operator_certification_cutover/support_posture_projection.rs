use std::collections::BTreeMap;

use crate::validator_invariant_catalog::operator_certification_cutover::WorthTopologyOperatorSelectedObligationCloseoutRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorSelectedObligationSupportPostureRow {
    query_support_lane: String,
    query_support_status: String,
    receipt_count: usize,
    query_execution_budget_digests: Vec<String>,
    support_posture_row_digest: String,
}

impl WorthTopologyOperatorSelectedObligationSupportPostureRow {
    pub(in crate::validator_invariant_catalog) fn from_closeout_rows(
        rows: &[WorthTopologyOperatorSelectedObligationCloseoutRow],
    ) -> Vec<Self> {
        let mut grouped = BTreeMap::<(&str, &str), Vec<&str>>::new();
        for row in rows {
            grouped
                .entry((row.query_support_lane(), row.query_support_status()))
                .or_default()
                .push(row.query_execution_budget_digest());
        }
        grouped
            .into_iter()
            .map(|((lane, status), budget_digests)| Self::from_group(lane, status, budget_digests))
            .collect()
    }

    fn from_group(
        query_support_lane: &str,
        query_support_status: &str,
        mut query_execution_budget_digests: Vec<&str>,
    ) -> Self {
        let receipt_count = query_execution_budget_digests.len();
        query_execution_budget_digests.sort_unstable();
        query_execution_budget_digests.dedup();
        let query_execution_budget_digests = query_execution_budget_digests
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut digest_parts = vec![
            "worth-topo-operator-selected-obligation-support-posture-row-v1".to_string(),
            format!("lane:{query_support_lane}"),
            format!("status:{query_support_status}"),
            format!("receipt-count:{receipt_count}"),
        ];
        digest_parts.extend(
            query_execution_budget_digests
                .iter()
                .map(|digest| format!("budget:{digest}")),
        );
        Self {
            query_support_lane: query_support_lane.to_string(),
            query_support_status: query_support_status.to_string(),
            receipt_count,
            query_execution_budget_digests,
            support_posture_row_digest: digest_parts.join("|"),
        }
    }

    pub fn query_support_lane(&self) -> &str {
        &self.query_support_lane
    }

    pub fn query_support_status(&self) -> &str {
        &self.query_support_status
    }

    pub const fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub fn query_execution_budget_digests(&self) -> &[String] {
        &self.query_execution_budget_digests
    }

    pub fn support_posture_row_digest(&self) -> &str {
        &self.support_posture_row_digest
    }
}
