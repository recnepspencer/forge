use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::speculation::BridgePreviewSessionIdentity;

#[derive(Debug)]
struct BridgePreviewSessionLivenessState {
    active: RwLock<bool>,
}

#[derive(Debug)]
pub(crate) struct BridgePreviewSessionLivenessOwner {
    state: Arc<BridgePreviewSessionLivenessState>,
    session_identity: BridgePreviewSessionIdentity,
}

/// Bridge-owned observation of one active preview-session lifecycle.
///
/// The observer does not keep the session alive and cannot perform preview
/// work. An admitted guard holds the lifecycle read boundary while a consumer
/// performs work already authorized by that session.
#[derive(Clone, Debug)]
pub struct BridgePreviewSessionLivenessObserver {
    state: Arc<BridgePreviewSessionLivenessState>,
    session_identity: BridgePreviewSessionIdentity,
}

pub struct BridgePreviewSessionLivenessGuard<'observer> {
    _liveness: RwLockReadGuard<'observer, bool>,
}

impl BridgePreviewSessionLivenessOwner {
    pub(crate) fn new(session_identity: BridgePreviewSessionIdentity) -> Self {
        Self {
            state: Arc::new(BridgePreviewSessionLivenessState {
                active: RwLock::new(true),
            }),
            session_identity,
        }
    }

    pub(crate) fn observer(&self) -> BridgePreviewSessionLivenessObserver {
        BridgePreviewSessionLivenessObserver {
            state: Arc::clone(&self.state),
            session_identity: self.session_identity.clone(),
        }
    }
}

impl Drop for BridgePreviewSessionLivenessOwner {
    fn drop(&mut self) {
        match self.state.active.write() {
            Ok(mut active) => *active = false,
            Err(poisoned) => *poisoned.into_inner() = false,
        }
    }
}

impl BridgePreviewSessionLivenessObserver {
    pub fn session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.session_identity
    }

    pub fn admit_active_session(&self) -> Option<BridgePreviewSessionLivenessGuard<'_>> {
        let liveness = self.state.active.read().ok()?;
        if !*liveness {
            return None;
        }
        Some(BridgePreviewSessionLivenessGuard {
            _liveness: liveness,
        })
    }
}
