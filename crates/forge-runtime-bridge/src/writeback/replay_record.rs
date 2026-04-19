use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackReplayRecordIdentityTag};

use super::{BridgeWritebackCounters, BridgeWritebackFailureClass, BridgeWritebackReplayBundle};

pub type BridgeWritebackReplayRecordIdentity = BridgeIdentity<WritebackReplayRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayRecord {
    record_identity: BridgeWritebackReplayRecordIdentity,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    expected_replay_digest: Arc<str>,
    replayed_replay_digest: Arc<str>,
    expected_semantic_digest: Arc<str>,
    replayed_semantic_digest: Arc<str>,
    expected_causality_digest: Arc<str>,
    replayed_causality_digest: Arc<str>,
    failure_class: Option<BridgeWritebackFailureClass>,
    counters: BridgeWritebackCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackReplayRecord {
    pub fn new(
        expected: &BridgeWritebackReplayBundle,
        replayed: &BridgeWritebackReplayBundle,
        failure_class: Option<BridgeWritebackFailureClass>,
        counters: BridgeWritebackCounters,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-replay-record|family:{:?}|expected-replay={}|replayed-replay={}|expected-semantic={}|replayed-semantic={}|expected-causality={}|replayed-causality={}|failure-class={}|counter-digest={}",
            expected.family_kind(),
            expected.digest(),
            replayed.digest(),
            expected.semantic_digest(),
            replayed.semantic_digest(),
            expected.causality_digest(),
            replayed.causality_digest(),
            failure_class
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "none".to_string()),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            record_identity: BridgeWritebackReplayRecordIdentity::new(format!(
                "bridge-writeback-replay-record:sha256:{digest:x}"
            )),
            family_kind: expected.family_kind(),
            expected_replay_digest: Arc::from(expected.digest().to_owned()),
            replayed_replay_digest: Arc::from(replayed.digest().to_owned()),
            expected_semantic_digest: Arc::from(expected.semantic_digest().to_owned()),
            replayed_semantic_digest: Arc::from(replayed.semantic_digest().to_owned()),
            expected_causality_digest: Arc::from(expected.causality_digest().to_owned()),
            replayed_causality_digest: Arc::from(replayed.causality_digest().to_owned()),
            failure_class,
            counters,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-replay-record:sha256:{digest:x}")),
        }
    }

    pub fn record_identity(&self) -> &BridgeWritebackReplayRecordIdentity {
        &self.record_identity
    }

    pub fn expected_replay_digest(&self) -> &str {
        self.expected_replay_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn replayed_replay_digest(&self) -> &str {
        self.replayed_replay_digest.as_ref()
    }

    pub fn expected_semantic_digest(&self) -> &str {
        self.expected_semantic_digest.as_ref()
    }

    pub fn replayed_semantic_digest(&self) -> &str {
        self.replayed_semantic_digest.as_ref()
    }

    pub fn expected_causality_digest(&self) -> &str {
        self.expected_causality_digest.as_ref()
    }

    pub fn replayed_causality_digest(&self) -> &str {
        self.replayed_causality_digest.as_ref()
    }

    pub fn failure_class(&self) -> Option<BridgeWritebackFailureClass> {
        self.failure_class
    }

    pub fn counters(&self) -> &BridgeWritebackCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
