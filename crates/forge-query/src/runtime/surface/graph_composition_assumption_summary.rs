use crate::identity::hash_parts;
use crate::runtime::{ForgeQueryVerificationReadSetBreadth, ForgeQueryWriteReceipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionAssumptionSummary {
    assumption_snapshot_digests: Vec<String>,
    verified_precondition_digests: Vec<String>,
    verified_step_count: usize,
    verification_read_set_breadth: ForgeQueryVerificationReadSetBreadth,
    counter_snapshot: String,
    aggregate_assumption_snapshot_digest: String,
    aggregate_verified_precondition_digest: String,
    assumption_summary_digest: String,
}

impl ForgeQueryGraphCompositionAssumptionSummary {
    pub(in crate::runtime) fn derive(write_receipts: &[ForgeQueryWriteReceipt]) -> Option<Self> {
        let assumption_sets = write_receipts
            .iter()
            .filter_map(|receipt| {
                receipt
                    .existing_truth_assertion_evidence()
                    .and_then(|evidence| evidence.verified_assumption_set())
            })
            .collect::<Vec<_>>();
        if assumption_sets.is_empty() {
            return None;
        }

        let assumption_snapshot_digests = assumption_sets
            .iter()
            .map(|set| set.assumption_snapshot_digest().to_string())
            .collect::<Vec<_>>();
        let verified_precondition_digests = assumption_sets
            .iter()
            .map(|set| set.verified_precondition_digest().to_string())
            .collect::<Vec<_>>();

        let target_binding_count = assumption_sets
            .iter()
            .map(|set| set.verification_read_set_breadth().target_binding_count())
            .sum();
        let asserted_aspect_count = assumption_sets
            .iter()
            .map(|set| set.verification_read_set_breadth().asserted_aspect_count())
            .sum();
        let asserted_aspect_paths = assumption_sets
            .iter()
            .flat_map(|set| set.asserted_aspect_paths().iter().cloned())
            .collect::<Vec<_>>();
        let cleared_assertion_count = assumption_sets
            .iter()
            .map(|set| {
                set.verification_read_set_breadth()
                    .cleared_assertion_count()
            })
            .sum();
        let verification_read_set_breadth = ForgeQueryVerificationReadSetBreadth::new(
            target_binding_count,
            asserted_aspect_count,
            &asserted_aspect_paths,
            cleared_assertion_count,
        );
        let verified_step_count = assumption_sets.len();
        let counter_snapshot = format!(
            "verified_steps={verified_step_count};{}",
            verification_read_set_breadth.counter_snapshot()
        );
        let aggregate_assumption_snapshot_digest = aggregate_digest(
            "forge_query_graph_composition_assumption_snapshot_digest_v1",
            &assumption_snapshot_digests,
        );
        let aggregate_verified_precondition_digest = aggregate_digest(
            "forge_query_graph_composition_verified_precondition_digest_v1",
            &verified_precondition_digests,
        );
        let assumption_summary_digest = hash_parts(&[
            "forge_query_graph_composition_assumption_summary_v1".to_string(),
            format!("assumption-snapshots:{aggregate_assumption_snapshot_digest}"),
            format!("verified-preconditions:{aggregate_verified_precondition_digest}"),
            format!("counters:{counter_snapshot}"),
        ]);

        Some(Self {
            assumption_snapshot_digests,
            verified_precondition_digests,
            verified_step_count,
            verification_read_set_breadth,
            counter_snapshot,
            aggregate_assumption_snapshot_digest,
            aggregate_verified_precondition_digest,
            assumption_summary_digest,
        })
    }

    pub fn assumption_snapshot_digests(&self) -> &[String] {
        &self.assumption_snapshot_digests
    }

    pub fn verified_precondition_digests(&self) -> &[String] {
        &self.verified_precondition_digests
    }

    pub fn verified_step_count(&self) -> usize {
        self.verified_step_count
    }

    pub fn verification_read_set_breadth(&self) -> &ForgeQueryVerificationReadSetBreadth {
        &self.verification_read_set_breadth
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn aggregate_assumption_snapshot_digest(&self) -> &str {
        &self.aggregate_assumption_snapshot_digest
    }

    pub fn aggregate_verified_precondition_digest(&self) -> &str {
        &self.aggregate_verified_precondition_digest
    }

    pub fn assumption_summary_digest(&self) -> &str {
        &self.assumption_summary_digest
    }
}

fn aggregate_digest(label: &str, digests: &[String]) -> String {
    hash_parts(
        &std::iter::once(label.to_string())
            .chain(digests.iter().map(|digest| format!("digest:{digest}")))
            .collect::<Vec<_>>(),
    )
}
