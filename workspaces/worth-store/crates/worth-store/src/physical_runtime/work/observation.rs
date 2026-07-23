use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, OnceLock,
};

use super::PhysicalWorkIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkTerminalStage {
    Declared,
    Ready,
    Queued,
    Dispatched,
    Settling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkTerminalDisposition {
    ClosedBeforeReadiness,
    AbortedBeforeReadiness,
    DroppedBeforeReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkTerminalObservation {
    identity: PhysicalWorkIdentity,
    stage: PhysicalWorkTerminalStage,
    disposition: PhysicalWorkTerminalDisposition,
}

impl PhysicalWorkTerminalObservation {
    pub(super) const fn active(
        identity: PhysicalWorkIdentity,
        stage: PhysicalWorkTerminalStage,
        disposition: PhysicalWorkTerminalDisposition,
    ) -> Self {
        Self {
            identity,
            stage,
            disposition,
        }
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn stage(self) -> PhysicalWorkTerminalStage {
        self.stage
    }

    pub const fn disposition(self) -> PhysicalWorkTerminalDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkShutdownObservation {
    declared: u64,
    ready: u64,
    queued: u64,
    dispatched: u64,
    settling: u64,
    terminal: Box<[PhysicalWorkTerminalObservation]>,
    residual: u64,
    unaccounted_terminal: u64,
}

#[derive(Clone)]
pub struct PhysicalWorkObservation {
    terminal: Arc<OnceLock<PhysicalWorkShutdownObservation>>,
}

pub(super) struct PhysicalWorkObservationOwner {
    terminal: Arc<OnceLock<PhysicalWorkShutdownObservation>>,
}

impl PhysicalWorkObservationOwner {
    pub(super) fn new() -> Self {
        Self {
            terminal: Arc::new(OnceLock::new()),
        }
    }

    pub(super) fn handle(&self) -> PhysicalWorkObservation {
        PhysicalWorkObservation {
            terminal: Arc::clone(&self.terminal),
        }
    }

    pub(super) fn publish(&self, observation: PhysicalWorkShutdownObservation) {
        let _ = self.terminal.set(observation);
    }
}

impl PhysicalWorkObservation {
    pub fn terminal(&self) -> Option<&PhysicalWorkShutdownObservation> {
        self.terminal.get()
    }
}

impl PhysicalWorkShutdownObservation {
    pub(super) fn from_active(
        declared_total: u64,
        safe_pre_effect_terminal: u64,
        active: impl IntoIterator<Item = (PhysicalWorkIdentity, PhysicalWorkTerminalStage)>,
        disposition: PhysicalWorkTerminalDisposition,
    ) -> Self {
        let terminal: Box<[_]> = active
            .into_iter()
            .map(|(identity, stage)| {
                PhysicalWorkTerminalObservation::active(identity, stage, disposition)
            })
            .collect();
        let terminal_total = terminal.len() as u64;
        let mut ready = 0;
        let mut queued = 0;
        let mut dispatched = 0;
        let mut settling = 0;
        for observation in &terminal {
            match observation.stage {
                PhysicalWorkTerminalStage::Declared => {}
                PhysicalWorkTerminalStage::Ready => ready += 1,
                PhysicalWorkTerminalStage::Queued => queued += 1,
                PhysicalWorkTerminalStage::Dispatched => dispatched += 1,
                PhysicalWorkTerminalStage::Settling => settling += 1,
            }
        }
        let accounted = safe_pre_effect_terminal.saturating_add(terminal_total);
        Self {
            declared: declared_total,
            ready,
            queued,
            dispatched,
            settling,
            terminal,
            residual: declared_total.saturating_sub(accounted),
            unaccounted_terminal: accounted.saturating_sub(declared_total),
        }
    }

    pub const fn declared(&self) -> u64 {
        self.declared
    }

    pub const fn ready(&self) -> u64 {
        self.ready
    }

    pub const fn queued(&self) -> u64 {
        self.queued
    }

    pub const fn dispatched(&self) -> u64 {
        self.dispatched
    }

    pub const fn settling(&self) -> u64 {
        self.settling
    }

    pub const fn terminal(&self) -> &[PhysicalWorkTerminalObservation] {
        &self.terminal
    }

    pub const fn residual(&self) -> u64 {
        self.residual
    }

    pub const fn unaccounted_terminal(&self) -> u64 {
        self.unaccounted_terminal
    }
}

pub(super) struct PhysicalWorkAccounting {
    declared: AtomicU64,
    safe_pre_effect_terminal: AtomicU64,
}

impl PhysicalWorkAccounting {
    pub(super) const fn new() -> Self {
        Self {
            declared: AtomicU64::new(0),
            safe_pre_effect_terminal: AtomicU64::new(0),
        }
    }

    pub(super) fn record_declared(&self) {
        let _ = self
            .declared
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            });
    }

    pub(super) fn declared(&self) -> u64 {
        self.declared.load(Ordering::Acquire)
    }

    pub(super) fn record_safe_pre_effect_terminal(&self) {
        let _ = self.safe_pre_effect_terminal.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |value| value.checked_add(1),
        );
    }

    pub(super) fn safe_pre_effect_terminal(&self) -> u64 {
        self.safe_pre_effect_terminal.load(Ordering::Acquire)
    }
}
