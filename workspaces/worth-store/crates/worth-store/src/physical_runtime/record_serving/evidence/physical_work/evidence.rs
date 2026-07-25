use super::{
    PhysicalWorkArtifactBinding, PhysicalWorkBackendProfileEvidence, PhysicalWorkCausalEvidence,
    PhysicalWorkCourtroomFinding, PhysicalWorkCourtroomRunBinding, PhysicalWorkCourtroomVerdict,
    PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkShutdownEvidence {
    declared: u64,
    blocked: u64,
    ready: u64,
    queued: u64,
    dispatched: u64,
    settling: u64,
    terminal_observations: u64,
    residual: u64,
    unaccounted_terminal: u64,
    settled: u64,
    cancelled_before_dispatch: u64,
    continued_after_cancellation: u64,
    inspection_required: u64,
    released_before_dispatch: u64,
    drain_residual: u64,
    reconciliation_deferred: u64,
    drain_evidence_overflow: u64,
}

impl PhysicalWorkShutdownEvidence {
    pub const fn declared(self) -> u64 {
        self.declared
    }

    pub const fn residual(self) -> u64 {
        self.residual
    }

    pub const fn unaccounted_terminal(self) -> u64 {
        self.unaccounted_terminal
    }

    pub const fn drain_residual(self) -> u64 {
        self.drain_residual
    }

    pub const fn drain_evidence_overflow(self) -> u64 {
        self.drain_evidence_overflow
    }

    pub const fn stage_counts(self) -> [u64; 6] {
        [
            self.blocked,
            self.ready,
            self.queued,
            self.dispatched,
            self.settling,
            self.terminal_observations,
        ]
    }

    pub const fn drain_counts(self) -> [u64; 6] {
        [
            self.settled,
            self.cancelled_before_dispatch,
            self.continued_after_cancellation,
            self.inspection_required,
            self.released_before_dispatch,
            self.reconciliation_deferred,
        ]
    }

    pub(super) const fn from_parts(parts: PhysicalWorkShutdownEvidenceParts) -> Self {
        Self {
            declared: parts.declared,
            blocked: parts.blocked,
            ready: parts.ready,
            queued: parts.queued,
            dispatched: parts.dispatched,
            settling: parts.settling,
            terminal_observations: parts.terminal_observations,
            residual: parts.residual,
            unaccounted_terminal: parts.unaccounted_terminal,
            settled: parts.settled,
            cancelled_before_dispatch: parts.cancelled_before_dispatch,
            continued_after_cancellation: parts.continued_after_cancellation,
            inspection_required: parts.inspection_required,
            released_before_dispatch: parts.released_before_dispatch,
            drain_residual: parts.drain_residual,
            reconciliation_deferred: parts.reconciliation_deferred,
            drain_evidence_overflow: parts.drain_evidence_overflow,
        }
    }
}

pub(super) struct PhysicalWorkShutdownEvidenceParts {
    pub(super) declared: u64,
    pub(super) blocked: u64,
    pub(super) ready: u64,
    pub(super) queued: u64,
    pub(super) dispatched: u64,
    pub(super) settling: u64,
    pub(super) terminal_observations: u64,
    pub(super) residual: u64,
    pub(super) unaccounted_terminal: u64,
    pub(super) settled: u64,
    pub(super) cancelled_before_dispatch: u64,
    pub(super) continued_after_cancellation: u64,
    pub(super) inspection_required: u64,
    pub(super) released_before_dispatch: u64,
    pub(super) drain_residual: u64,
    pub(super) reconciliation_deferred: u64,
    pub(super) drain_evidence_overflow: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkCourtroomEvidence {
    store: [u8; 16],
    runtime: u64,
    generation: u64,
    backend_profile: Option<PhysicalWorkBackendProfileEvidence>,
    run: PhysicalWorkCourtroomRunBinding,
    causal: Box<[PhysicalWorkCausalEvidence]>,
    causal_overflow: u64,
    shutdown: PhysicalWorkShutdownEvidence,
    artifacts: Box<[PhysicalWorkArtifactBinding]>,
    oracle: PhysicalWorkOracleEvidence,
    mutants: Box<[PhysicalWorkMutantLocalization]>,
    verdict: PhysicalWorkCourtroomVerdict,
}

impl PhysicalWorkCourtroomEvidence {
    pub const fn store(&self) -> [u8; 16] {
        self.store
    }

    pub const fn runtime(&self) -> u64 {
        self.runtime
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn backend_profile(&self) -> Option<PhysicalWorkBackendProfileEvidence> {
        self.backend_profile
    }

    pub const fn run(&self) -> &PhysicalWorkCourtroomRunBinding {
        &self.run
    }

    pub const fn causal(&self) -> &[PhysicalWorkCausalEvidence] {
        &self.causal
    }

    pub const fn causal_overflow(&self) -> u64 {
        self.causal_overflow
    }

    pub const fn shutdown(&self) -> PhysicalWorkShutdownEvidence {
        self.shutdown
    }

    pub const fn artifacts(&self) -> &[PhysicalWorkArtifactBinding] {
        &self.artifacts
    }

    pub const fn oracle(&self) -> &PhysicalWorkOracleEvidence {
        &self.oracle
    }

    pub const fn mutants(&self) -> &[PhysicalWorkMutantLocalization] {
        &self.mutants
    }

    pub const fn verdict(&self) -> &PhysicalWorkCourtroomVerdict {
        &self.verdict
    }

    pub(super) fn from_parts(parts: PhysicalWorkCourtroomEvidenceParts) -> Self {
        let verdict = PhysicalWorkCourtroomVerdict::from_findings(parts.findings);
        Self {
            store: parts.store,
            runtime: parts.runtime,
            generation: parts.generation,
            backend_profile: parts.backend_profile,
            run: parts.run,
            causal: parts.causal,
            causal_overflow: parts.causal_overflow,
            shutdown: parts.shutdown,
            artifacts: parts.artifacts,
            oracle: parts.oracle,
            mutants: parts.mutants,
            verdict,
        }
    }
}

pub(super) struct PhysicalWorkCourtroomEvidenceParts {
    pub(super) store: [u8; 16],
    pub(super) runtime: u64,
    pub(super) generation: u64,
    pub(super) backend_profile: Option<PhysicalWorkBackendProfileEvidence>,
    pub(super) run: PhysicalWorkCourtroomRunBinding,
    pub(super) causal: Box<[PhysicalWorkCausalEvidence]>,
    pub(super) causal_overflow: u64,
    pub(super) shutdown: PhysicalWorkShutdownEvidence,
    pub(super) artifacts: Box<[PhysicalWorkArtifactBinding]>,
    pub(super) oracle: PhysicalWorkOracleEvidence,
    pub(super) mutants: Box<[PhysicalWorkMutantLocalization]>,
    pub(super) findings: Vec<PhysicalWorkCourtroomFinding>,
}
