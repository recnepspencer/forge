use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, Weak};

/// Drop-governed publication authority owned directly by one runtime.
#[derive(Debug)]
pub(in crate::runtime) struct RelationalRuntimePublicationOwner {
    binding: RelationalRuntimePublicationBinding,
}

/// Cloneable runtime binding carried by independently borrowable publication artifacts.
#[derive(Debug, Clone)]
pub(crate) struct RelationalRuntimePublicationBinding {
    pub(super) lifecycle: Arc<RelationalRuntimePublicationLifecycle>,
}

#[derive(Debug)]
pub(super) struct RelationalRuntimePublicationLifecycle {
    accepting_publication: AtomicBool,
    in_flight: RwLock<()>,
    next_candidate_id: AtomicU64,
    candidates: Mutex<HashMap<u64, RegisteredCandidate>>,
    pub(super) deferred_settlements: Mutex<
        HashMap<
            crate::history::data::CommitId,
            crate::publication::data::DeferredPublicationSettlement,
        >,
    >,
}

#[derive(Debug)]
struct RegisteredCandidate {
    expires_at: std::time::Instant,
    payload: Weak<Mutex<Option<crate::mvcc::publication::CandidatePayload>>>,
}

pub(crate) enum RelationalCandidateRegistrationDenial {
    CapacityExhausted,
    IdentityExhausted,
}

pub(crate) struct AdmittedRelationalRuntimePublication<'lifecycle> {
    _in_flight: RwLockReadGuard<'lifecycle, ()>,
}

impl RelationalRuntimePublicationOwner {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            binding: RelationalRuntimePublicationBinding {
                lifecycle: Arc::new(RelationalRuntimePublicationLifecycle {
                    accepting_publication: AtomicBool::new(true),
                    in_flight: RwLock::new(()),
                    next_candidate_id: AtomicU64::new(1),
                    candidates: Mutex::new(HashMap::new()),
                    deferred_settlements: Mutex::new(HashMap::new()),
                }),
            },
        }
    }

    pub(super) fn binding(&self) -> RelationalRuntimePublicationBinding {
        self.binding.clone()
    }

    pub(super) fn close(&self) {
        self.binding
            .lifecycle
            .accepting_publication
            .store(false, Ordering::Release);
        drop(
            self.binding
                .lifecycle
                .in_flight
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
        );
        self.binding.clear_deferred_settlements();
    }
}

impl RelationalRuntimePublicationBinding {
    pub(crate) fn register_candidate(
        &self,
        expires_at: std::time::Instant,
        maximum_candidates: usize,
        payload: &Arc<Mutex<Option<crate::mvcc::publication::CandidatePayload>>>,
    ) -> Result<u64, RelationalCandidateRegistrationDenial> {
        let mut candidates = self
            .lifecycle
            .candidates
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if candidates.len() >= maximum_candidates {
            return Err(RelationalCandidateRegistrationDenial::CapacityExhausted);
        }
        let id = self
            .lifecycle
            .next_candidate_id
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                (current != 0).then(|| current.checked_add(1).unwrap_or(0))
            })
            .map_err(|_| RelationalCandidateRegistrationDenial::IdentityExhausted)?;
        assert_ne!(id, 0, "candidate identity zero is reserved");
        assert!(
            !candidates.contains_key(&id),
            "candidate identity allocator collided with a live candidate"
        );
        let replaced = candidates.insert(
            id,
            RegisteredCandidate {
                expires_at,
                payload: Arc::downgrade(payload),
            },
        );
        debug_assert!(replaced.is_none());
        Ok(id)
    }

    pub(crate) fn take_candidate(
        &self,
        candidate_id: u64,
    ) -> Option<crate::mvcc::publication::CandidatePayload> {
        let registered = self
            .lifecycle
            .candidates
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&candidate_id)?;
        registered.payload.upgrade()?.lock().ok()?.take()
    }

    pub(crate) fn discard_candidate(&self, candidate_id: u64) {
        let registered = self
            .lifecycle
            .candidates
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&candidate_id);
        if let Some(payload) = registered.and_then(|entry| entry.payload.upgrade()) {
            payload
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take();
        }
    }

    pub(crate) fn reap_expired_candidates(&self) -> usize {
        let mut candidates = self
            .lifecycle
            .candidates
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        reap_expired_candidates(&mut candidates, std::time::Instant::now())
    }

    pub(crate) fn belongs_to_same_owner(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.lifecycle, &other.lifecycle)
    }

    pub(crate) fn admit(&self) -> Option<AdmittedRelationalRuntimePublication<'_>> {
        if !self.lifecycle.accepting_publication.load(Ordering::Acquire) {
            return None;
        }
        let in_flight = self
            .lifecycle
            .in_flight
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if !self.lifecycle.accepting_publication.load(Ordering::Acquire) {
            return None;
        }
        Some(AdmittedRelationalRuntimePublication {
            _in_flight: in_flight,
        })
    }
}

fn reap_expired_candidates(
    candidates: &mut HashMap<u64, RegisteredCandidate>,
    now: std::time::Instant,
) -> usize {
    let expired = candidates
        .iter()
        .filter_map(|(id, entry)| {
            (now >= entry.expires_at || entry.payload.strong_count() == 0).then_some(*id)
        })
        .collect::<Vec<_>>();
    for id in &expired {
        if let Some(payload) = candidates
            .remove(id)
            .and_then(|entry| entry.payload.upgrade())
        {
            payload
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take();
        }
    }
    expired.len()
}
