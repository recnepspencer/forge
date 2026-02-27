//! Replay log for reproducible kernel operations.
//!
//! DOMAIN: Operation recording and determinism verification.
//! INVARIANTS: Replay logs are append-only during a draft (D1).
//! DEPENDENCIES: `lineage` (OpSignature).
//!
//! Every topology mutation records its signature, parameters, RNG seed,
//! and pre/post state hashes. This enables:
//! - Replaying exact failure sequences
//! - Verifying determinism (same input → same output)
//! - Extracting minimal reproduction cases

use serde::{Deserialize, Serialize};

use crate::lineage::OpSignature;
use forge_core::DecisionDelta;

/// A single entry in the replay log.
///
/// Captures everything needed to reproduce one step of an operation
/// sequence: what operation ran, with what parameters, under what
/// RNG state, and what the topology looked like before and after.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEntry {
    /// The operation that was executed.
    signature: OpSignature,
    /// Serialized operation parameters (binary, format-agnostic).
    parameters: Vec<u8>,
    /// RNG seed at the start of this operation.
    seed: u64,
    /// Topology hash before this operation.
    pre_hash: u128,
    /// Topology hash after this operation (0 if not yet committed).
    post_hash: u128,
    /// Decision delta relative to the previous step (P3.1).
    decision_delta: Option<DecisionDelta>,
}

impl ReplayEntry {
    /// Create a new replay entry.
    pub fn new(signature: OpSignature, parameters: Vec<u8>, seed: u64, pre_hash: u128) -> Self {
        Self {
            signature,
            parameters,
            seed,
            pre_hash,
            post_hash: 0,
            decision_delta: None,
        }
    }

    /// Set the post-operation hash (called after the operation completes).
    pub fn set_post_hash(&mut self, hash: u128) {
        self.post_hash = hash;
    }

    /// The operation signature.
    pub fn signature(&self) -> &OpSignature {
        &self.signature
    }

    /// The serialized parameters (binary).
    pub fn parameters(&self) -> &[u8] {
        &self.parameters
    }

    /// The RNG seed.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The pre-operation topology hash.
    pub fn pre_hash(&self) -> u128 {
        self.pre_hash
    }

    /// The post-operation topology hash.
    pub fn post_hash(&self) -> u128 {
        self.post_hash
    }

    /// The decision delta relative to the previous step.
    pub fn get_decision_delta(&self) -> Option<&DecisionDelta> {
        self.decision_delta.as_ref()
    }

    /// Set the decision delta for this entry.
    pub fn set_decision_delta(&mut self, delta: DecisionDelta) {
        self.decision_delta = Some(delta);
    }
}

/// Append-only log of operations for replay and determinism verification.
///
/// Built up during a `MutableDraft` via `record()`, then extracted
/// on commit for archival or comparison.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplayLog {
    /// The recorded entries, in execution order.
    entries: Vec<ReplayEntry>,
    /// The build target triple this log was recorded on (e.g., "aarch64-macos").
    #[serde(default)]
    target_triple: Option<String>,
    /// Whether FMA (fused multiply-add) instructions are available.
    #[serde(default)]
    fma_enabled: Option<bool>,
    /// Optimization level ("debug" or "release").
    #[serde(default)]
    opt_level: Option<String>,
}

impl ReplayLog {
    /// Create an empty replay log stamped with the current architecture metadata.
    pub fn new() -> Self {
        Self::with_current_target()
    }

    /// Create a replay log stamped with the current build target and platform metadata.
    pub fn with_current_target() -> Self {
        let triple = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);
        let fma = cfg!(target_feature = "fma");
        let opt = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        Self {
            entries: Vec::new(),
            target_triple: Some(triple),
            fma_enabled: Some(fma),
            opt_level: Some(opt.to_string()),
        }
    }

    /// Set the target triple explicitly.
    pub fn set_target_triple(&mut self, triple: &str) {
        self.target_triple = Some(triple.to_string());
    }

    /// The target triple this log was recorded on.
    pub fn get_target_triple(&self) -> Option<&str> {
        self.target_triple.as_deref()
    }

    /// Whether FMA instructions were enabled at build time.
    pub fn get_fma_enabled(&self) -> Option<bool> {
        self.fma_enabled
    }

    /// The optimization level ("debug" or "release").
    pub fn get_opt_level(&self) -> Option<&str> {
        self.opt_level.as_deref()
    }

    /// Verify that this log's target triple matches the current process.
    ///
    /// Returns `Ok(())` if triples match or either is `None` (lenient).
    /// Returns `Err(KernelError)` if triples mismatch.
    pub fn verify_architecture(&self, current_triple: &str) -> Result<(), forge_core::KernelError> {
        match &self.target_triple {
            None => Ok(()),
            Some(recorded) if recorded == current_triple => Ok(()),
            Some(recorded) => Err(forge_core::KernelError::ReplayMismatch {
                expected: recorded.clone(),
                actual: current_triple.to_string(),
                context: None,
            }),
        }
    }

    /// Record an operation entry.
    pub fn record(&mut self, entry: ReplayEntry) {
        self.entries.push(entry);
    }

    /// The recorded entries.
    pub fn entries(&self) -> &[ReplayEntry] {
        &self.entries
    }

    /// The number of recorded entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Update the post-hash of the most recent entry.
    pub fn finalize_last(&mut self, post_hash: u128) {
        if let Some(last) = self.entries.last_mut() {
            last.set_post_hash(post_hash);
        }
    }

    /// Verify determinism by comparing two replay logs entry-by-entry.
    ///
    /// Returns `true` if both logs have the same entries with matching
    /// signatures, seeds, and state hashes.
    pub fn verify_determinism(&self, other: &ReplayLog) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }
        self.entries.iter().zip(other.entries.iter()).all(|(a, b)| {
            a.signature == b.signature
                && a.parameters == b.parameters
                && a.seed == b.seed
                && a.pre_hash == b.pre_hash
                && a.post_hash == b.post_hash
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lineage::OpSignature;

    fn make_op_sig(name: &'static str) -> OpSignature {
        OpSignature::new(name)
    }

    #[test]
    fn empty_log() {
        let log = ReplayLog::new();
        assert!(log.is_empty());
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn record_and_retrieve() {
        let mut log = ReplayLog::new();
        log.record(ReplayEntry::new(make_op_sig("test_op"), vec![], 42, 100));

        assert_eq!(log.len(), 1);
        assert_eq!(log.entries()[0].seed(), 42);
        assert_eq!(log.entries()[0].pre_hash(), 100);
    }

    #[test]
    fn finalize_sets_post_hash() {
        let mut log = ReplayLog::new();
        log.record(ReplayEntry::new(make_op_sig("op"), vec![], 1, 0));
        log.finalize_last(999);
        assert_eq!(log.entries()[0].post_hash(), 999);
    }

    #[test]
    fn verify_identical_logs() {
        let mut a = ReplayLog::new();
        let mut b = ReplayLog::new();

        for i in 0..5 {
            let entry =
                ReplayEntry::new(make_op_sig("op"), vec![i as u8], i as u64, i as u128 * 10);
            a.record(entry.clone());
            b.record(entry);
        }

        assert!(a.verify_determinism(&b));
    }

    #[test]
    fn verify_different_logs() {
        let mut a = ReplayLog::new();
        let mut b = ReplayLog::new();

        a.record(ReplayEntry::new(make_op_sig("op"), vec![], 1, 0));
        b.record(ReplayEntry::new(make_op_sig("op"), vec![], 2, 0));

        assert!(!a.verify_determinism(&b));
    }

    #[test]
    fn verify_different_length_logs() {
        let mut a = ReplayLog::new();
        let b = ReplayLog::new();

        a.record(ReplayEntry::new(make_op_sig("op"), vec![], 1, 0));

        assert!(!a.verify_determinism(&b));
    }
}
