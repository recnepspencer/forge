#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobOrphanReclaimCounterSnapshot {
    barriers: u64,
    proofs: u64,
    denials: u64,
}

impl BlobOrphanReclaimCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            barriers: 0,
            proofs: 0,
            denials: 0,
        }
    }

    pub const fn with_barrier(self) -> Self {
        Self {
            barriers: self.barriers + 1,
            ..self
        }
    }

    pub const fn with_proof(self) -> Self {
        Self {
            proofs: self.proofs + 1,
            ..self
        }
    }

    pub const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn barriers(self) -> u64 {
        self.barriers
    }

    pub const fn proofs(self) -> u64 {
        self.proofs
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}