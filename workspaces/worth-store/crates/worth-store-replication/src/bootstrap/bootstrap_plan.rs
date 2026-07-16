use sha2::{Digest, Sha256};
use worth_store_physical_isolation::BootstrapReachabilityLease;

use crate::{ReplicaRecoveryFrontier, ReplicationPeerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaBootstrapIntent {
    target_peer: ReplicationPeerId,
    expected_frontier: ReplicaRecoveryFrontier,
}

impl ReplicaBootstrapIntent {
    pub fn target_peer(&self) -> &ReplicationPeerId {
        &self.target_peer
    }

    pub const fn expected_frontier(&self) -> ReplicaRecoveryFrontier {
        self.expected_frontier
    }
}

#[derive(Debug)]
pub struct LoweredReplicaBootstrapPlan {
    intent: ReplicaBootstrapIntent,
    source_lease: BootstrapReachabilityLease,
    fingerprint: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaBootstrapDenial {
    InvalidTarget,
    SourceFrontierMismatch,
    SourceLeaseMismatch,
    ExecutionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaBootstrapExecutionReport {
    source_lease_identity: [u8; 32],
    reached_frontier: ReplicaRecoveryFrontier,
    durable_target_identity: [u8; 32],
}

impl ReplicaBootstrapExecutionReport {
    pub const fn from_replication_owner(
        source_lease_identity: [u8; 32],
        reached_frontier: ReplicaRecoveryFrontier,
        durable_target_identity: [u8; 32],
    ) -> Self {
        Self {
            source_lease_identity,
            reached_frontier,
            durable_target_identity,
        }
    }
}

pub trait ReplicaBootstrapExecutionPort {
    fn execute_replica_bootstrap(
        &mut self,
        plan: &LoweredReplicaBootstrapPlan,
    ) -> Result<ReplicaBootstrapExecutionReport, ReplicaBootstrapDenial>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaBootstrapReceipt {
    plan_fingerprint: [u8; 32],
    target_peer: ReplicationPeerId,
    reached_frontier: ReplicaRecoveryFrontier,
    durable_target_identity: [u8; 32],
    retained_source_lease_identity: [u8; 32],
}

impl ReplicaBootstrapReceipt {
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }

    pub const fn reached_frontier(&self) -> ReplicaRecoveryFrontier {
        self.reached_frontier
    }

    pub const fn retained_source_lease_identity(&self) -> [u8; 32] {
        self.retained_source_lease_identity
    }

    pub fn target_peer(&self) -> &ReplicationPeerId {
        &self.target_peer
    }

    pub const fn durable_target_identity(&self) -> [u8; 32] {
        self.durable_target_identity
    }

    pub fn receipt_identity(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-replica-bootstrap-receipt-v1");
        digest.update(self.plan_fingerprint);
        digest.update(self.target_peer.as_str().as_bytes());
        digest.update(self.reached_frontier.observed_lsn().to_be_bytes());
        digest.update(self.reached_frontier.durable_lsn().to_be_bytes());
        digest.update(self.reached_frontier.client_acknowledged_lsn().to_be_bytes());
        digest.update(self.reached_frontier.replication_acknowledged_lsn().to_be_bytes());
        digest.update(self.reached_frontier.authority_epoch().to_be_bytes());
        digest.update(self.durable_target_identity);
        digest.update(self.retained_source_lease_identity);
        digest.finalize().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReplicaBootstrapOwner;

impl ReplicaBootstrapOwner {
    pub fn intent(
        target_peer: ReplicationPeerId,
        expected_frontier: ReplicaRecoveryFrontier,
    ) -> Result<ReplicaBootstrapIntent, ReplicaBootstrapDenial> {
        if target_peer.as_str().is_empty() {
            return Err(ReplicaBootstrapDenial::InvalidTarget);
        }
        Ok(ReplicaBootstrapIntent {
            target_peer,
            expected_frontier,
        })
    }

    pub fn lower(
        intent: ReplicaBootstrapIntent,
        source_lease: BootstrapReachabilityLease,
    ) -> Result<LoweredReplicaBootstrapPlan, ReplicaBootstrapDenial> {
        let fingerprint = bootstrap_plan_fingerprint(&intent, &source_lease);
        Ok(LoweredReplicaBootstrapPlan {
            intent,
            source_lease,
            fingerprint,
        })
    }

    pub fn execute(
        plan: LoweredReplicaBootstrapPlan,
        port: &mut impl ReplicaBootstrapExecutionPort,
    ) -> Result<(ReplicaBootstrapReceipt, BootstrapReachabilityLease), ReplicaBootstrapDenial> {
        let report = port.execute_replica_bootstrap(&plan)?;
        if report.source_lease_identity != plan.source_lease.binding_fingerprint() {
            return Err(ReplicaBootstrapDenial::SourceLeaseMismatch);
        }
        if report.reached_frontier != plan.intent.expected_frontier {
            return Err(ReplicaBootstrapDenial::SourceFrontierMismatch);
        }
        let receipt = ReplicaBootstrapReceipt {
            plan_fingerprint: plan.fingerprint,
            target_peer: plan.intent.target_peer,
            reached_frontier: report.reached_frontier,
            durable_target_identity: report.durable_target_identity,
            retained_source_lease_identity: plan.source_lease.binding_fingerprint(),
        };
        Ok((receipt, plan.source_lease))
    }
}

impl LoweredReplicaBootstrapPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    pub fn target_peer(&self) -> &ReplicationPeerId {
        &self.intent.target_peer
    }

    pub fn source_lease_identity(&self) -> [u8; 32] {
        self.source_lease.binding_fingerprint()
    }
}

fn bootstrap_plan_fingerprint(
    intent: &ReplicaBootstrapIntent,
    lease: &BootstrapReachabilityLease,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-bootstrap-plan-v1");
    digest.update(intent.target_peer.as_str().as_bytes());
    digest.update(lease.binding_fingerprint());
    digest.update(intent.expected_frontier.observed_lsn().to_be_bytes());
    digest.update(intent.expected_frontier.durable_lsn().to_be_bytes());
    digest.update(intent.expected_frontier.client_acknowledged_lsn().to_be_bytes());
    digest.update(intent.expected_frontier.replication_acknowledged_lsn().to_be_bytes());
    digest.update(intent.expected_frontier.authority_epoch().to_be_bytes());
    digest.finalize().into()
}
