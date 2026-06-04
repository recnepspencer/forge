use super::super::{FeedbackInterleavedTruthMatrix, WritebackFeedbackLoopMatrix};
use crate::routing::canonicalization::digest_string;

impl FeedbackInterleavedTruthMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn ordinary_truth_commit_identity(&self) -> &str {
        self.ordinary_truth_commit_identity.as_str()
    }

    pub(in crate::harness::adapter::adapter_impl) fn ordinary_truth_route_digest(&self) -> String {
        digest_string(
            "bridge-writeback-route-digest",
            self.ordinary_truth_route_identity.as_str(),
        )
        .to_string()
    }

    pub(in crate::harness::adapter::adapter_impl) fn bridge_feedback_commit_identity(
        &self,
    ) -> &str {
        self.bridge_feedback_commit_identity.as_str()
    }

    pub(in crate::harness::adapter::adapter_impl) fn ordinary_truth_commit(
        &self,
    ) -> &crate::facade::TruthCommitIdentity {
        &self.ordinary_truth_commit_identity
    }

    pub(in crate::harness::adapter::adapter_impl) fn ordinary_truth_route_identity(
        &self,
    ) -> &crate::facade::BridgeRouteIdentity {
        &self.ordinary_truth_route_identity
    }

    pub(in crate::harness::adapter::adapter_impl) fn bridge_feedback_commit(
        &self,
    ) -> &crate::facade::TruthCommitIdentity {
        &self.bridge_feedback_commit_identity
    }

    pub(in crate::harness::adapter::adapter_impl) fn interleaving_preserved_single_authoritative_commit(
        &self,
    ) -> bool {
        self.interleaving_preserved_single_authoritative_commit
    }
}

impl WritebackFeedbackLoopMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn interleaved_truth_matrix(
        &self,
    ) -> &FeedbackInterleavedTruthMatrix {
        &self.interleaved_truth_matrix
    }
}
