use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::current_cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use super::current_route_authority::{
    current_completed_split_route_authority, current_lookup_consumed_route_authority,
    current_replay_undo_boundary_route_authority, WorthWorkloadCurrentLookupConsumedRouteAuthority,
    WorthWorkloadCurrentOrdinaryRouteAuthority,
    WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum WorthWorkloadOrdinaryConsumerRouteKind {
    LookupConsumedBatchExecutionCluster,
    CompletedSplitBatchExecutionCluster,
    ReplayUndoBoundaryBatchExecutionCluster,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadOrdinaryConsumerCurrentRouteWitness {
    route_kind: WorthWorkloadOrdinaryConsumerRouteKind,
    route_lineage_digest: String,
    route_authority_digest: String,
    route_authority: WorthWorkloadCurrentOrdinaryRouteAuthority,
}

pub(crate) fn current_lookup_consumed_batch_execution_cluster_witness() -> Result<
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    Ok(WorthWorkloadOrdinaryConsumerCurrentRouteWitness::new(
        WorthWorkloadOrdinaryConsumerRouteKind::LookupConsumedBatchExecutionCluster,
        WorthWorkloadCurrentOrdinaryRouteAuthority::LookupConsumed(
            current_lookup_consumed_route_authority()?,
        ),
    ))
}

pub(crate) fn current_completed_split_batch_execution_cluster_witness() -> Result<
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    Ok(WorthWorkloadOrdinaryConsumerCurrentRouteWitness::new(
        WorthWorkloadOrdinaryConsumerRouteKind::CompletedSplitBatchExecutionCluster,
        WorthWorkloadCurrentOrdinaryRouteAuthority::CompletedSplit(
            current_completed_split_route_authority()?,
        ),
    ))
}

pub(crate) fn current_replay_undo_boundary_batch_execution_cluster_witness() -> Result<
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    Ok(WorthWorkloadOrdinaryConsumerCurrentRouteWitness::new(
        WorthWorkloadOrdinaryConsumerRouteKind::ReplayUndoBoundaryBatchExecutionCluster,
        WorthWorkloadCurrentOrdinaryRouteAuthority::ReplayUndoBoundary(
            current_replay_undo_boundary_route_authority()?,
        ),
    ))
}

impl WorthWorkloadOrdinaryConsumerCurrentRouteWitness {
    fn new(
        route_kind: WorthWorkloadOrdinaryConsumerRouteKind,
        route_authority: WorthWorkloadCurrentOrdinaryRouteAuthority,
    ) -> Self {
        let route_authority_digest = route_authority.route_authority_digest().to_string();
        let route_lineage_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:ordinary-consumer-route-witness:v1".to_string(),
                format!("route:{}", route_kind.as_str()),
                format!("authority:{route_authority_digest}"),
            ],
        );
        Self {
            route_kind,
            route_lineage_digest,
            route_authority_digest,
            route_authority,
        }
    }

    pub(crate) const fn route_kind(&self) -> WorthWorkloadOrdinaryConsumerRouteKind {
        self.route_kind
    }

    pub(crate) fn route_lineage_digest(&self) -> &str {
        &self.route_lineage_digest
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub(crate) fn lookup_route_authority(
        &self,
    ) -> &WorthWorkloadCurrentLookupConsumedRouteAuthority {
        self.route_authority.lookup_route_authority()
    }

    pub(crate) fn replay_undo_route_authority(
        &self,
    ) -> Option<&WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority> {
        match &self.route_authority {
            WorthWorkloadCurrentOrdinaryRouteAuthority::ReplayUndoBoundary(authority) => {
                Some(authority)
            }
            _ => None,
        }
    }

    pub(crate) fn replay_undo_boundary_proof_digest(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.boundary_proof_digest())
    }

    pub(crate) fn replay_undo_route_packet_identity(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.route_packet_identity())
    }

    pub(crate) fn replay_undo_route_family(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.route_family())
    }

    pub(crate) fn transaction_packet_identity(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.transaction_packet_identity())
    }

    pub(crate) fn replay_scope_identity(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.replay_scope_identity())
    }

    pub(crate) fn undo_scope_identity(&self) -> Option<&str> {
        self.replay_undo_route_authority()
            .map(|authority| authority.undo_scope_identity())
    }

    pub(crate) fn require_same_lookup_route_authority(
        witnesses: &[Self],
    ) -> Result<
        WorthWorkloadCurrentLookupConsumedRouteAuthority,
        WorthWorkloadOrdinaryConsumerCutoverError,
    > {
        let Some(first) = witnesses.first() else {
            return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
                WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
                "phase 13 current ordinary cutover proof requires at least one selected-plan route witness",
            ));
        };
        let expected_digest = first.lookup_route_authority().route_authority_digest();
        if witnesses.iter().all(|witness| {
            witness.lookup_route_authority().route_authority_digest() == expected_digest
        }) {
            Ok(first.lookup_route_authority().clone())
        } else {
            Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
                WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
                "phase 13 selected-plan route witnesses must converge on one canonical ordinary-consumer route authority",
            ))
        }
    }
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(super) use test_support::current_replay_undo_boundary_batch_execution_cluster_witness_with_test_override;

impl WorthWorkloadOrdinaryConsumerRouteKind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::LookupConsumedBatchExecutionCluster => "lookup-consumed-batch-execution-cluster",
            Self::CompletedSplitBatchExecutionCluster => "completed-split-batch-execution-cluster",
            Self::ReplayUndoBoundaryBatchExecutionCluster => {
                "replay-undo-boundary-batch-execution-cluster"
            }
        }
    }
}
