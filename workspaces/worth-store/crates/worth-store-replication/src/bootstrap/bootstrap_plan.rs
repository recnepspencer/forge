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
    counters: ReplicaBootstrapExecutionCounters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaBootstrapExecutionCounters {
    source_bytes_read: u64,
    output_bytes_written: u64,
    backend_requests: u64,
    maximum_resident_buffer_bytes: u64,
}

impl ReplicaBootstrapExecutionCounters {
    pub const fn measured(
        source_bytes_read: u64,
        output_bytes_written: u64,
        backend_requests: u64,
        maximum_resident_buffer_bytes: u64,
    ) -> Option<Self> {
        if backend_requests == 0 || maximum_resident_buffer_bytes == 0 {
            return None;
        }
        Some(Self {
            source_bytes_read,
            output_bytes_written,
            backend_requests,
            maximum_resident_buffer_bytes,
        })
    }

    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }
    pub const fn output_bytes_written(self) -> u64 {
        self.output_bytes_written
    }
    pub const fn backend_requests(self) -> u64 {
        self.backend_requests
    }
    pub const fn maximum_resident_buffer_bytes(self) -> u64 {
        self.maximum_resident_buffer_bytes
    }
}

impl ReplicaBootstrapExecutionReport {
    pub const fn from_replication_owner(
        source_lease_identity: [u8; 32],
        reached_frontier: ReplicaRecoveryFrontier,
        durable_target_identity: [u8; 32],
        counters: ReplicaBootstrapExecutionCounters,
    ) -> Self {
        Self {
            source_lease_identity,
            reached_frontier,
            durable_target_identity,
            counters,
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
    execution_counters: ReplicaBootstrapExecutionCounters,
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

    pub const fn execution_counters(&self) -> ReplicaBootstrapExecutionCounters {
        self.execution_counters
    }

    pub fn receipt_identity(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-replica-bootstrap-receipt-v1");
        digest.update(self.plan_fingerprint);
        digest.update(self.target_peer.as_str().as_bytes());
        digest.update(self.reached_frontier.observed_lsn().to_be_bytes());
        digest.update(self.reached_frontier.durable_lsn().to_be_bytes());
        digest.update(
            self.reached_frontier
                .client_acknowledged_lsn()
                .to_be_bytes(),
        );
        digest.update(
            self.reached_frontier
                .replication_acknowledged_lsn()
                .to_be_bytes(),
        );
        digest.update(self.reached_frontier.authority_epoch().to_be_bytes());
        digest.update(self.durable_target_identity);
        digest.update(self.retained_source_lease_identity);
        digest.update(self.execution_counters.source_bytes_read.to_be_bytes());
        digest.update(self.execution_counters.output_bytes_written.to_be_bytes());
        digest.update(self.execution_counters.backend_requests.to_be_bytes());
        digest.update(
            self.execution_counters
                .maximum_resident_buffer_bytes
                .to_be_bytes(),
        );
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
            execution_counters: report.counters,
        };
        Ok((receipt, plan.source_lease))
    }

    pub fn recover_recorded(
        plan: LoweredReplicaBootstrapPlan,
        recorded_receipt_identity: [u8; 32],
        durable_target_identity: [u8; 32],
        source_lease_identity: [u8; 32],
        execution_counters: ReplicaBootstrapExecutionCounters,
    ) -> Result<(ReplicaBootstrapReceipt, BootstrapReachabilityLease), ReplicaBootstrapDenial> {
        if source_lease_identity != plan.source_lease.binding_fingerprint() {
            return Err(ReplicaBootstrapDenial::SourceLeaseMismatch);
        }
        let receipt = ReplicaBootstrapReceipt {
            plan_fingerprint: plan.fingerprint,
            target_peer: plan.intent.target_peer.clone(),
            reached_frontier: plan.intent.expected_frontier,
            durable_target_identity,
            retained_source_lease_identity: source_lease_identity,
            execution_counters,
        };
        if receipt.receipt_identity() != recorded_receipt_identity {
            return Err(ReplicaBootstrapDenial::ExecutionFailed);
        }
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

    pub fn into_retained_source_lease(self) -> BootstrapReachabilityLease {
        self.source_lease
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
    digest.update(
        intent
            .expected_frontier
            .client_acknowledged_lsn()
            .to_be_bytes(),
    );
    digest.update(
        intent
            .expected_frontier
            .replication_acknowledged_lsn()
            .to_be_bytes(),
    );
    digest.update(intent.expected_frontier.authority_epoch().to_be_bytes());
    digest.finalize().into()
}
