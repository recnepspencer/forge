use std::cell::RefCell;
use std::collections::BTreeMap;

use worth_signal::facade::history::RuntimeSnapshot;

use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::core::ExactRuntimeRestoreArtifact;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

const MAXIMUM_PENDING_RESTORE_TOKENS: usize = 64;

thread_local! {
    static RESTORE_TOKENS: RefCell<RestoreTokenRegistry> = RefCell::new(RestoreTokenRegistry::default());
}

struct RestoreTokenRegistry {
    next_token: u64,
    maximum_pending_tokens: usize,
    artifacts: BTreeMap<String, RestoreArtifact>,
}

enum RestoreArtifact {
    RuntimeEnvelope(ExactRuntimeRestoreArtifact),
    SnapshotEnvelope(RuntimeSnapshotEnvelope),
    Snapshot(RuntimeSnapshot),
    #[cfg(test)]
    Marker,
}

impl Default for RestoreTokenRegistry {
    fn default() -> Self {
        Self {
            next_token: 0,
            maximum_pending_tokens: MAXIMUM_PENDING_RESTORE_TOKENS,
            artifacts: BTreeMap::new(),
        }
    }
}

impl RestoreTokenRegistry {
    fn ensure_capacity(&self) -> Result<(), WorthSignalJsError> {
        if self.artifacts.len() >= self.maximum_pending_tokens {
            return Err(WorthSignalJsError::restore_token_capacity_exhausted(
                self.maximum_pending_tokens,
            ));
        }
        Ok(())
    }

    fn store(
        &mut self,
        prefix: &str,
        artifact: RestoreArtifact,
    ) -> Result<String, WorthSignalJsError> {
        self.ensure_capacity()?;
        self.next_token = self.next_token.checked_add(1).ok_or_else(|| {
            WorthSignalJsError::internal("restore token identity space exhausted")
        })?;
        let key = format!("{prefix}:{}", self.next_token);
        self.artifacts.insert(key.clone(), artifact);
        Ok(key)
    }

    fn take(
        &mut self,
        token: &str,
        expected_prefix: &str,
    ) -> Result<RestoreArtifact, WorthSignalJsError> {
        if !token.starts_with(&format!("{expected_prefix}:")) {
            return Err(unknown_token(expected_prefix, token));
        }
        self.artifacts
            .remove(token)
            .ok_or_else(|| unknown_token(expected_prefix, token))
    }

    fn discard(&mut self, token: &str) -> bool {
        self.artifacts.remove(token).is_some()
    }
}

pub fn ensure_restore_token_capacity_available() -> Result<(), WorthSignalJsError> {
    RESTORE_TOKENS.with(|registry| registry.borrow().ensure_capacity())
}

pub fn store_runtime_envelope(
    value: ExactRuntimeRestoreArtifact,
) -> Result<String, WorthSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow_mut()
            .store("runtimeEnvelope", RestoreArtifact::RuntimeEnvelope(value))
    })
}

pub fn load_runtime_envelope(
    token: &str,
) -> Result<ExactRuntimeRestoreArtifact, WorthSignalJsError> {
    RESTORE_TOKENS.with(
        |registry| match registry.borrow_mut().take(token, "runtimeEnvelope")? {
            RestoreArtifact::RuntimeEnvelope(artifact) => Ok(artifact),
            _ => unreachable!("runtime envelope token prefix must identify its artifact kind"),
        },
    )
}

pub fn store_snapshot_envelope(
    value: RuntimeSnapshotEnvelope,
) -> Result<String, WorthSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow_mut()
            .store("snapshotEnvelope", RestoreArtifact::SnapshotEnvelope(value))
    })
}

pub fn load_snapshot_envelope(token: &str) -> Result<RuntimeSnapshotEnvelope, WorthSignalJsError> {
    RESTORE_TOKENS.with(
        |registry| match registry.borrow_mut().take(token, "snapshotEnvelope")? {
            RestoreArtifact::SnapshotEnvelope(artifact) => Ok(artifact),
            _ => unreachable!("snapshot envelope token prefix must identify its artifact kind"),
        },
    )
}

pub fn store_snapshot(value: RuntimeSnapshot) -> Result<String, WorthSignalJsError> {
    RESTORE_TOKENS.with(|registry| {
        registry
            .borrow_mut()
            .store("snapshot", RestoreArtifact::Snapshot(value))
    })
}

pub fn load_snapshot(token: &str) -> Result<RuntimeSnapshot, WorthSignalJsError> {
    RESTORE_TOKENS.with(
        |registry| match registry.borrow_mut().take(token, "snapshot")? {
            RestoreArtifact::Snapshot(artifact) => Ok(artifact),
            _ => unreachable!("snapshot token prefix must identify its artifact kind"),
        },
    )
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = discardRestoreToken)]
pub fn discard_restore_token(token: String) -> bool {
    RESTORE_TOKENS.with(|registry| registry.borrow_mut().discard(&token))
}

fn unknown_token(expected_prefix: &str, token: &str) -> WorthSignalJsError {
    WorthSignalJsError::invalid_input(format!("unknown {expected_prefix} restore token `{token}`"))
}

#[cfg(test)]
mod tests {
    use super::{RestoreArtifact, RestoreTokenRegistry};

    #[test]
    fn pending_restore_tokens_are_bounded_consumable_and_discardable() {
        let mut registry = RestoreTokenRegistry {
            next_token: 0,
            maximum_pending_tokens: 2,
            artifacts: Default::default(),
        };
        let first = registry.store("test", RestoreArtifact::Marker).unwrap();
        let second = registry.store("test", RestoreArtifact::Marker).unwrap();
        let denial = registry.store("test", RestoreArtifact::Marker).unwrap_err();
        assert_eq!(denial.code, "restoreTokenCapacityExhausted");

        assert!(registry.take(&first, "other").is_err());
        assert!(matches!(
            registry.take(&first, "test").unwrap(),
            RestoreArtifact::Marker
        ));
        assert!(registry.take(&first, "test").is_err());
        assert!(registry.discard(&second));
        assert!(!registry.discard(&second));
        registry
            .store("test", RestoreArtifact::Marker)
            .expect("consumption and disposal should reclaim capacity");
    }
}
