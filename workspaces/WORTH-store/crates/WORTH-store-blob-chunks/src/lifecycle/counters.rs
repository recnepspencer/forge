use worth_store_budgets::CounterEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobLifecycleCounterSnapshot {
    strength: CounterEvidenceStrength,
    declarations: u64,
    authority_resolutions: u64,
    lowered_plans: u64,
    scoped_chunks: u64,
    reachability_admissions: u64,
    placement_admissions: u64,
    execution_ready: u64,
    executed_receipts: u64,
    denials: u64,
}

impl BlobLifecycleCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            declarations: 1,
            authority_resolutions: 0,
            lowered_plans: 0,
            scoped_chunks: 0,
            reachability_admissions: 0,
            placement_admissions: 0,
            execution_ready: 0,
            executed_receipts: 0,
            denials: 0,
        }
    }

    pub(crate) const fn record_authority_resolution(self) -> Self {
        Self {
            authority_resolutions: self.authority_resolutions + 1,
            ..self
        }
    }

    pub(crate) const fn record_lowered_plan(self) -> Self {
        Self {
            lowered_plans: self.lowered_plans + 1,
            ..self
        }
    }

    pub(crate) const fn record_scoped_chunk(self) -> Self {
        Self {
            scoped_chunks: self.scoped_chunks + 1,
            ..self
        }
    }

    pub(crate) const fn record_reachability_admission(self) -> Self {
        Self {
            reachability_admissions: self.reachability_admissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_placement_admission(self) -> Self {
        Self {
            placement_admissions: self.placement_admissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_execution_ready(self) -> Self {
        Self {
            execution_ready: self.execution_ready + 1,
            ..self
        }
    }

    pub(crate) const fn record_executed_receipt(self) -> Self {
        Self {
            executed_receipts: self.executed_receipts + 1,
            ..self
        }
    }

    pub(crate) const fn record_denial(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn declarations(self) -> u64 {
        self.declarations
    }

    pub const fn authority_resolutions(self) -> u64 {
        self.authority_resolutions
    }

    pub const fn lowered_plans(self) -> u64 {
        self.lowered_plans
    }

    pub const fn scoped_chunks(self) -> u64 {
        self.scoped_chunks
    }

    pub const fn reachability_admissions(self) -> u64 {
        self.reachability_admissions
    }

    pub const fn placement_admissions(self) -> u64 {
        self.placement_admissions
    }

    pub const fn execution_ready(self) -> u64 {
        self.execution_ready
    }

    pub const fn executed_receipts(self) -> u64 {
        self.executed_receipts
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
