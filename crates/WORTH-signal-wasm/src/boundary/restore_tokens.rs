use std::cell::RefCell;
use std::collections::BTreeMap;

use worth_signal::facade::history::RuntimeSnapshot;

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::core::ExactRuntimeRestoreArtifact;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

thread_local! {
    static RESTORE_TOKENS: RefCell<RestoreTokenRegistry> = RefCell::new(RestoreTokenRegistry::default());
}

#[derive(Default)]
struct RestoreTokenRegistry {
    next_token: u64,
    runtime_envelopes: BTreeMap<String, ExactRuntimeRestoreArtifact>,
    snapshot_envelopes: BTreeMap<String, RuntimeSnapshotEnvelope>,
    snapshots: BTreeMap<String, RuntimeSnapshot>,
}

impl RestoreTokenRegistry {
    fn next_key(&mut self, prefix: &str) -> String {
        self.next_token = self.next_token.saturating_add(1);
        format!("{prefix}:{}", self.next_token)
    }
}

pub fn store_runtime_envelope(value: ExactRuntimeRestoreArtifact) -> String {
    RESTORE_TOKENS.with(|registry| {
        let mut registry = registry.borrow_mut();
        let key = registry.next_key("runtimeEnvelope");
        registry.runtime_envelopes.insert(key.clone(), value);
        key
    })
}

pub fn load_runtime_envelope(
    token: &str,
) -> Result<ExactRuntimeRestoreArtifact, WORTHSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow()
            .runtime_envelopes
            .get(token)
            .cloned()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "unknown runtime envelope restore token `{token}`"
                ))
            })
    })
}

pub fn store_snapshot_envelope(value: RuntimeSnapshotEnvelope) -> String {
    RESTORE_TOKENS.with(|registry| {
        let mut registry = registry.borrow_mut();
        let key = registry.next_key("snapshotEnvelope");
        registry.snapshot_envelopes.insert(key.clone(), value);
        key
    })
}

pub fn load_snapshot_envelope(token: &str) -> Result<RuntimeSnapshotEnvelope, WORTHSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow()
            .snapshot_envelopes
            .get(token)
            .cloned()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "unknown snapshot envelope restore token `{token}`"
                ))
            })
    })
}

pub fn store_snapshot(value: RuntimeSnapshot) -> String {
    RESTORE_TOKENS.with(|registry| {
        let mut registry = registry.borrow_mut();
        let key = registry.next_key("snapshot");
        registry.snapshots.insert(key.clone(), value);
        key
    })
}

pub fn load_snapshot(token: &str) -> Result<RuntimeSnapshot, WORTHSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow()
            .snapshots
            .get(token)
            .cloned()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "unknown snapshot restore token `{token}`"
                ))
            })
    })
}
