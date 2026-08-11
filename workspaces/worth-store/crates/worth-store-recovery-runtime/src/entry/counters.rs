use std::sync::atomic::{AtomicU64, Ordering};

static SESSIONS_ISSUED: AtomicU64 = AtomicU64::new(0);
static SESSIONS_TERMINATED_REFUSED: AtomicU64 = AtomicU64::new(0);
static SESSIONS_TERMINATED_BLOCKED: AtomicU64 = AtomicU64::new(0);
static SESSIONS_TERMINATED_PUBLICATION_INDETERMINATE: AtomicU64 = AtomicU64::new(0);
static SESSIONS_TERMINATED_RECOVERED: AtomicU64 = AtomicU64::new(0);
static NON_TERMINAL_DROPS: AtomicU64 = AtomicU64::new(0);
static ACTIVE_SESSIONS: AtomicU64 = AtomicU64::new(0);
static BINDING_COMPARISONS: AtomicU64 = AtomicU64::new(0);
static BINDING_DENIALS: AtomicU64 = AtomicU64::new(0);
static COORDINATORS_CREATED: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecoveryAdmissionCounters {
    pub sessions_issued: u64,
    pub sessions_terminated_refused: u64,
    pub sessions_terminated_blocked: u64,
    pub sessions_terminated_publication_indeterminate: u64,
    pub sessions_terminated_recovered: u64,
    pub owner_detected_non_terminal_drops: u64,
    pub active_sessions: u64,
    pub entry_binding_comparisons: u64,
    pub entry_binding_denials: u64,
    pub fresh_coordinators_created: u64,
    pub recovery_effects: Option<u64>,
}

pub(crate) fn snapshot(recovery_effects: Option<u64>) -> PhysicalRecoveryAdmissionCounters {
    PhysicalRecoveryAdmissionCounters {
        sessions_issued: SESSIONS_ISSUED.load(Ordering::Relaxed),
        sessions_terminated_refused: SESSIONS_TERMINATED_REFUSED.load(Ordering::Relaxed),
        sessions_terminated_blocked: SESSIONS_TERMINATED_BLOCKED.load(Ordering::Relaxed),
        sessions_terminated_publication_indeterminate:
            SESSIONS_TERMINATED_PUBLICATION_INDETERMINATE.load(Ordering::Relaxed),
        sessions_terminated_recovered: SESSIONS_TERMINATED_RECOVERED.load(Ordering::Relaxed),
        owner_detected_non_terminal_drops: NON_TERMINAL_DROPS.load(Ordering::Relaxed),
        active_sessions: ACTIVE_SESSIONS.load(Ordering::Relaxed),
        entry_binding_comparisons: BINDING_COMPARISONS.load(Ordering::Relaxed),
        entry_binding_denials: BINDING_DENIALS.load(Ordering::Relaxed),
        fresh_coordinators_created: COORDINATORS_CREATED.load(Ordering::Relaxed),
        recovery_effects,
    }
}

pub(crate) fn record_session_issued() {
    SESSIONS_ISSUED.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn record_session_refused() {
    SESSIONS_TERMINATED_REFUSED.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_sub(1, Ordering::Relaxed);
}

pub(crate) fn record_session_blocked() {
    SESSIONS_TERMINATED_BLOCKED.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_sub(1, Ordering::Relaxed);
}

pub(crate) fn record_session_recovered() {
    SESSIONS_TERMINATED_RECOVERED.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_sub(1, Ordering::Relaxed);
}

pub(crate) fn record_session_publication_indeterminate() {
    SESSIONS_TERMINATED_PUBLICATION_INDETERMINATE.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_sub(1, Ordering::Relaxed);
}

pub(crate) fn record_non_terminal_drop() {
    NON_TERMINAL_DROPS.fetch_add(1, Ordering::Relaxed);
    ACTIVE_SESSIONS.fetch_sub(1, Ordering::Relaxed);
}

pub(crate) fn record_binding_comparison() {
    BINDING_COMPARISONS.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn record_binding_denial() {
    BINDING_DENIALS.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn record_coordinator_created() {
    COORDINATORS_CREATED.fetch_add(1, Ordering::Relaxed);
}
