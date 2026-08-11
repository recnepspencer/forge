use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

use super::PhysicalRecoveryFreshnessAuthority;

static REGISTERED_SESSIONS: OnceLock<Mutex<BTreeSet<[u8; 16]>>> = OnceLock::new();

/// Store-owned proof that the sampled recovery identity is registered in this
/// process for one live recovery session.
#[derive(Debug)]
pub struct PhysicalRecoveryRegisteredSessionAuthority {
    freshness: PhysicalRecoveryFreshnessAuthority,
}

impl PhysicalRecoveryFreshnessAuthority {
    pub fn register_session(self) -> Option<PhysicalRecoveryRegisteredSessionAuthority> {
        let identity = self.sample_identity();
        let mut registered = registered_sessions().lock().ok()?;
        registered
            .insert(identity)
            .then_some(PhysicalRecoveryRegisteredSessionAuthority { freshness: self })
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
