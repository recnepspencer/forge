use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

use super::PhysicalRecoveryFreshnessAuthority;

static REGISTERED_SESSIONS: OnceLock<Mutex<BTreeSet<[u8; 16]>>> = OnceLock::new();

/// Store-owned proof that the sampled recovery identity is registered in this
/// process for one live recovery session.
#[derive(Debug)]
pub struct PhysicalRecoveryRegisteredSessionAuthority {
    freshness: PhysicalRecoveryFreshnessAuthority,
    cleanup_media: Option<crate::physical_runtime::media_ownership::RecoveryCleanupMediaOwner>,
}

impl PhysicalRecoveryFreshnessAuthority {
    pub fn register_session(mut self) -> Option<PhysicalRecoveryRegisteredSessionAuthority> {
        let identity = self.sample_identity();
        let cleanup_media = self.take_cleanup_media()?;
        let mut registered = registered_sessions().lock().ok()?;
        registered
            .insert(identity)
            .then_some(PhysicalRecoveryRegisteredSessionAuthority {
                freshness: self,
                cleanup_media: Some(cleanup_media),
            })
    }
}

impl PhysicalRecoveryRegisteredSessionAuthority {
    pub const fn session_identity_bytes(&self) -> [u8; 16] {
        self.freshness.sample_identity()
    }

    pub(in crate::physical_runtime) const fn freshness(
        &self,
    ) -> &PhysicalRecoveryFreshnessAuthority {
        &self.freshness
    }

    pub(in crate::physical_runtime) fn cleanup_media(
        &self,
    ) -> &crate::physical_runtime::media_ownership::RecoveryCleanupMediaOwner {
        self.cleanup_media
            .as_ref()
            .expect("registered recovery session retains cleanup media")
    }

    pub(in crate::physical_runtime) fn take_cleanup_media(
        &mut self,
    ) -> crate::physical_runtime::media_ownership::RecoveryCleanupMediaOwner {
        self.cleanup_media
            .take()
            .expect("coordination consumes cleanup media exactly once")
    }
}

impl Drop for PhysicalRecoveryRegisteredSessionAuthority {
    fn drop(&mut self) {
        if let Ok(mut registered) = registered_sessions().lock() {
            registered.remove(&self.session_identity_bytes());
        }
    }
}

fn registered_sessions() -> &'static Mutex<BTreeSet<[u8; 16]>> {
    REGISTERED_SESSIONS.get_or_init(|| Mutex::new(BTreeSet::new()))
}
