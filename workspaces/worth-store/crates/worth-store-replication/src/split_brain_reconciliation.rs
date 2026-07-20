use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::{
    DivergentReplicaHistoryReport, OldPrimaryDivergenceDisposition, OldPrimaryRejoinReceipt,
    ReplicaHistoryClassification, ReplicaPromotionDenial, ReplicaPromotionReceipt,
    ReplicaPromotionRejectionReceipt, ReplicationPeerId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationPartitionWindow {
    partition_identity: [u8; 32],
    began_at_tick: u64,
    healed_at_tick: u64,
    isolated_peers: BTreeSet<ReplicationPeerId>,
    surviving_peers: BTreeSet<ReplicationPeerId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionSurvivorObservation {
    peer: ReplicationPeerId,
    observed_at_tick: u64,
    history_identity: [u8; 32],
    classification: ReplicaHistoryClassification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitBrainReconciliationReceipt {
    independent_survivors: u64,
    old_primary_excluded_at_tick: u64,
    receipt_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitBrainReconciliationDenial {
    InvalidWindow,
    EmptyPartitionSide,
    PeerOnBothSides,
    ObservationOutsidePartition,
    ObservationNotFromSurvivor,
    DuplicateSurvivor,
    InsufficientIndependentSurvivors,
    ReconciledBeforeHeal,
    OldPrimaryLeaseStillValid,
    CandidateWasNotDivergent,
    PromotionBasisMismatch,
    RejoinNotForensicallyRetained,
    RejoinNotRebootstrapped,
}

impl ReplicationPartitionWindow {
    pub fn admit(
        partition_identity: [u8; 32],
        began_at_tick: u64,
        healed_at_tick: u64,
        isolated_peers: impl IntoIterator<Item = ReplicationPeerId>,
        surviving_peers: impl IntoIterator<Item = ReplicationPeerId>,
    ) -> Result<Self, SplitBrainReconciliationDenial> {
        if partition_identity == [0; 32] || began_at_tick >= healed_at_tick {
            return Err(SplitBrainReconciliationDenial::InvalidWindow);
        }
        let isolated_peers = isolated_peers.into_iter().collect::<BTreeSet<_>>();
        let surviving_peers = surviving_peers.into_iter().collect::<BTreeSet<_>>();
        if isolated_peers.is_empty() || surviving_peers.is_empty() {
            return Err(SplitBrainReconciliationDenial::EmptyPartitionSide);
        }
        if isolated_peers
            .iter()
            .any(|peer| surviving_peers.contains(peer))
        {
            return Err(SplitBrainReconciliationDenial::PeerOnBothSides);
        }
        Ok(Self {
            partition_identity,
            began_at_tick,
            healed_at_tick,
            isolated_peers,
            surviving_peers,
        })
    }

    pub fn observe_survivor(
        &self,
        history: &DivergentReplicaHistoryReport,
        observed_at_tick: u64,
    ) -> Result<PartitionSurvivorObservation, SplitBrainReconciliationDenial> {
        if observed_at_tick < self.began_at_tick || observed_at_tick > self.healed_at_tick {
            return Err(SplitBrainReconciliationDenial::ObservationOutsidePartition);
        }
        let peer = history.observation().peer();
        if !self.surviving_peers.contains(peer) {
            return Err(SplitBrainReconciliationDenial::ObservationNotFromSurvivor);
        }
        Ok(PartitionSurvivorObservation {
            peer: peer.clone(),
            observed_at_tick,
            history_identity: history.stable_fingerprint(),
            classification: history.classification(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconcile(
        &self,
        observations: impl IntoIterator<Item = PartitionSurvivorObservation>,
        old_primary_lease_valid_until_tick: u64,
        reconciled_at_tick: u64,
        rejected: &ReplicaPromotionRejectionReceipt,
        promoted: &ReplicaPromotionReceipt,
        rejoin: &OldPrimaryRejoinReceipt,
    ) -> Result<SplitBrainReconciliationReceipt, SplitBrainReconciliationDenial> {
        if reconciled_at_tick < self.healed_at_tick {
            return Err(SplitBrainReconciliationDenial::ReconciledBeforeHeal);
        }
        if reconciled_at_tick <= old_primary_lease_valid_until_tick {
            return Err(SplitBrainReconciliationDenial::OldPrimaryLeaseStillValid);
        }
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by(|left, right| left.peer.cmp(&right.peer));
        let mut peers = BTreeSet::new();
        for observation in &observations {
            if !self.surviving_peers.contains(&observation.peer)
                || observation.observed_at_tick < self.began_at_tick
                || observation.observed_at_tick > self.healed_at_tick
            {
                return Err(SplitBrainReconciliationDenial::ObservationOutsidePartition);
            }
            if !peers.insert(observation.peer.clone()) {
                return Err(SplitBrainReconciliationDenial::DuplicateSurvivor);
            }
        }
        if peers.len() < 2 {
            return Err(SplitBrainReconciliationDenial::InsufficientIndependentSurvivors);
        }
        if rejected.denial() != ReplicaPromotionDenial::DivergentHistory {
            return Err(SplitBrainReconciliationDenial::CandidateWasNotDivergent);
        }
        if rejected.current_frontier() != promoted.promoted_frontier() {
            return Err(SplitBrainReconciliationDenial::PromotionBasisMismatch);
        }
        if rejoin.disposition()
            != OldPrimaryDivergenceDisposition::RebootstrapAfterForensicRetention
            || rejoin.forensic_retention_identity().is_none()
        {
            return Err(SplitBrainReconciliationDenial::RejoinNotForensicallyRetained);
        }
        if rejoin.rebootstrap_target_identity().is_none() {
            return Err(SplitBrainReconciliationDenial::RejoinNotRebootstrapped);
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-split-brain-reconciliation-v1");
        digest.update(self.partition_identity);
        digest.update(self.began_at_tick.to_be_bytes());
        digest.update(self.healed_at_tick.to_be_bytes());
        digest.update(old_primary_lease_valid_until_tick.to_be_bytes());
        digest.update(reconciled_at_tick.to_be_bytes());
        digest.update(rejected.receipt_identity());
        digest.update(promoted.receipt_identity());
        digest.update(rejoin.receipt_identity());
        for peer in &self.isolated_peers {
            digest.update(b"isolated-peer");
            digest.update((peer.as_str().len() as u64).to_be_bytes());
            digest.update(peer.as_str().as_bytes());
        }
        for peer in &self.surviving_peers {
            digest.update(b"surviving-peer");
            digest.update((peer.as_str().len() as u64).to_be_bytes());
            digest.update(peer.as_str().as_bytes());
        }
        for observation in &observations {
            digest.update(observation.peer.as_str().as_bytes());
            digest.update(observation.observed_at_tick.to_be_bytes());
            digest.update(observation.history_identity);
            digest.update([observation.classification as u8]);
        }
        Ok(SplitBrainReconciliationReceipt {
            independent_survivors: peers.len() as u64,
            old_primary_excluded_at_tick: reconciled_at_tick,
            receipt_identity: digest.finalize().into(),
        })
    }
}

impl SplitBrainReconciliationReceipt {
    pub const fn independent_survivors(self) -> u64 {
        self.independent_survivors
    }
    pub const fn old_primary_excluded_at_tick(self) -> u64 {
        self.old_primary_excluded_at_tick
    }
    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(name: &str) -> ReplicationPeerId {
        ReplicationPeerId::from_declared_peer(name).unwrap()
    }

    #[test]
    fn partition_sides_must_be_nonempty_and_disjoint() {
        assert_eq!(
            ReplicationPartitionWindow::admit([1; 32], 10, 20, [peer("a")], [peer("a")])
                .unwrap_err(),
            SplitBrainReconciliationDenial::PeerOnBothSides
        );
        assert_eq!(
            ReplicationPartitionWindow::admit([1; 32], 10, 20, [], [peer("b")]).unwrap_err(),
            SplitBrainReconciliationDenial::EmptyPartitionSide
        );
        assert_eq!(
            ReplicationPartitionWindow::admit([1; 32], 20, 20, [peer("a")], [peer("b")])
                .unwrap_err(),
            SplitBrainReconciliationDenial::InvalidWindow
        );
    }
}
