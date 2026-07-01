use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{TopologyQueryBackedConsumerCutover, TopologyReadRequestFamily};

impl TopologyQueryBackedConsumerCutover {
    pub fn with_test_family_fallback_counts(
        mut self,
        family: TopologyReadRequestFamily,
        row_scan_fallback_count: usize,
        whole_view_fallback_count: usize,
    ) -> Self {
        let row = self
            .family_rows
            .iter_mut()
            .find(|row| row.request_family == family)
            .expect("requested family row should exist");
        row.row_scan_fallback_count = row_scan_fallback_count;
        row.whole_view_fallback_count = whole_view_fallback_count;
        row.refresh_row_digest();
        self.closeout_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &self
                .family_rows
                .iter()
                .map(|row| format!("family-row:{}", row.row_digest()))
                .chain(std::iter::once(format!(
                    "handle:{}",
                    self.handle_identity_digest
                )))
                .chain(std::iter::once(format!(
                    "support-snapshot:{}",
                    self.support_snapshot_digest
                )))
                .chain(std::iter::once(format!(
                    "operating-context:{}",
                    self.operating_context_identity_digest
                )))
                .chain(std::iter::once(format!(
                    "parity-verified:{}",
                    self.parity_verified_count
                )))
                .chain(std::iter::once(
                    "worth-topo:query-backed-consumer-cutover:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        self
    }
}
