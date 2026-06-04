use crate::facade::BridgeBulkPlanningFailure;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(super) enum ExpectedRetainedCausalDigestArtifact {
    BulkPlanningCounters,
    BulkPlanningFailures,
    BulkPlanningRecord,
    ContinuityRecord,
    HistoricalEvaluationCounters,
    HistoricalEvaluationFailureRecord,
    HistoricalEvaluationRecord,
    MergeRecord,
    PreviewDiscardRecord,
    PreviewExecutionRecord,
    PreviewPromotionRecord,
    RouteRecord,
    SourceFailureRecord,
    SourceMaterializationRecord,
    StreamCheckpointRecord,
    StreamProtocolCounters,
    StreamReplayRecord,
    StructuralBranchComparisonRecord,
    StructuralRemapRecord,
}

impl ExpectedRetainedCausalDigestArtifact {
    fn digest_domain(self) -> &'static str {
        match self {
            Self::BulkPlanningCounters => "bridge-bulk-planning-counters",
            Self::BulkPlanningFailures => "bridge-bulk-planning-failures",
            Self::BulkPlanningRecord => "bridge-causal-retained-bulk-planning-record",
            Self::ContinuityRecord => "bridge-causal-retained-continuity-record",
            Self::HistoricalEvaluationCounters => "bridge-historical-evaluation-counters",
            Self::HistoricalEvaluationFailureRecord => {
                "bridge-causal-retained-historical-evaluation-failure-record"
            }
            Self::HistoricalEvaluationRecord => "bridge-causal-retained-historical-record",
            Self::MergeRecord => "bridge-causal-retained-merge-record",
            Self::PreviewDiscardRecord => "bridge-causal-retained-preview-discard-record",
            Self::PreviewExecutionRecord => "bridge-causal-retained-preview-execution-record",
            Self::PreviewPromotionRecord => "bridge-causal-retained-preview-promotion-record",
            Self::RouteRecord => "bridge-causal-retained-route-record",
            Self::SourceFailureRecord => "bridge-causal-retained-source-failure-record",
            Self::SourceMaterializationRecord => {
                "bridge-causal-retained-source-materialization-record"
            }
            Self::StreamCheckpointRecord => "bridge-causal-retained-stream-checkpoint-record",
            Self::StreamProtocolCounters => "bridge-stream-protocol-counters",
            Self::StreamReplayRecord => "bridge-causal-retained-stream-replay-record",
            Self::StructuralBranchComparisonRecord => {
                "bridge-causal-retained-structural-branch-comparison-record"
            }
            Self::StructuralRemapRecord => "bridge-causal-retained-structural-remap-record",
        }
    }
}

pub(super) fn expected_retained_causal_digest(
    artifact: ExpectedRetainedCausalDigestArtifact,
    parts: &[&str],
) -> String {
    expected_retained_causal_digest_for_parts(artifact, parts.iter().copied())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExpectedRetainedCausalDigestBasis {
    entries: Arc<[ExpectedRetainedCausalDigestBasisEntry]>,
}

impl ExpectedRetainedCausalDigestBasis {
    pub(super) fn from_counter_values(entries: impl IntoIterator<Item = String>) -> Self {
        Self::from_owned_entries(entries)
    }

    pub(super) fn from_bulk_planning_failure_records(
        failures: &[BridgeBulkPlanningFailure],
    ) -> Self {
        Self::from_borrowed_entries(failures.iter().map(BridgeBulkPlanningFailure::digest))
    }

    fn from_owned_entries(entries: impl IntoIterator<Item = String>) -> Self {
        let entries = entries
            .into_iter()
            .map(ExpectedRetainedCausalDigestBasisEntry::from_owned_entry)
            .collect::<Vec<_>>();
        Self {
            entries: Arc::from(entries),
        }
    }

    fn from_borrowed_entries<'a>(entries: impl IntoIterator<Item = &'a str>) -> Self {
        let entries = entries
            .into_iter()
            .map(ExpectedRetainedCausalDigestBasisEntry::from_borrowed_entry)
            .collect::<Vec<_>>();
        Self {
            entries: Arc::from(entries),
        }
    }

    fn entries(&self) -> &[ExpectedRetainedCausalDigestBasisEntry] {
        &self.entries
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExpectedRetainedCausalDigestBasisEntry {
    value: Arc<str>,
}

impl ExpectedRetainedCausalDigestBasisEntry {
    fn from_owned_entry(value: String) -> Self {
        Self {
            value: Arc::from(value),
        }
    }

    fn from_borrowed_entry(value: &str) -> Self {
        Self {
            value: Arc::from(value),
        }
    }

    fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

pub(super) fn expected_retained_causal_digest_for_basis(
    artifact: ExpectedRetainedCausalDigestArtifact,
    basis: &ExpectedRetainedCausalDigestBasis,
) -> String {
    expected_retained_causal_digest_for_parts(
        artifact,
        basis.entries().iter().map(|entry| entry.as_str()),
    )
}

fn expected_retained_causal_digest_for_parts<'a>(
    artifact: ExpectedRetainedCausalDigestArtifact,
    parts: impl IntoIterator<Item = &'a str>,
) -> String {
    use sha2::{Digest, Sha256};

    let digest_domain = artifact.digest_domain();
    let mut canonical = String::from(digest_domain);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{digest_domain}:sha256:{digest:x}")
}
