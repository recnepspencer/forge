#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobResumeCounterSnapshot {
    declarations: u64,
    admissions: u64,
    chunk_appends: u64,
    durable_chunks: u64,
    integrity_admissions: u64,
    frontier_checkpoints: u64,
    root_candidates: u64,
    root_ready: u64,
    closes: u64,
    abandons: u64,
    reclaims: u64,
    replays: u64,
    denials: u64,
}

impl BlobResumeCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            declarations: 0,
            admissions: 0,
            chunk_appends: 0,
            durable_chunks: 0,
            integrity_admissions: 0,
            frontier_checkpoints: 0,
            root_candidates: 0,
            root_ready: 0,
            closes: 0,
            abandons: 0,
            reclaims: 0,
            replays: 0,
            denials: 0,
        }
    }

    pub const fn declared(self) -> Self {
        Self {
            declarations: self.declarations + 1,
            ..self
        }
    }
    pub const fn admitted(self) -> Self {
        Self {
            admissions: self.admissions + 1,
            ..self
        }
    }
    pub const fn append_started(self) -> Self {
        Self {
            chunk_appends: self.chunk_appends + 1,
            ..self
        }
    }
    pub const fn bytes_durable(self) -> Self {
        Self {
            durable_chunks: self.durable_chunks + 1,
            ..self
        }
    }
    pub const fn integrity_admitted(self) -> Self {
        Self {
            integrity_admissions: self.integrity_admissions + 1,
            ..self
        }
    }
    pub const fn checkpointed(self) -> Self {
        Self {
            frontier_checkpoints: self.frontier_checkpoints + 1,
            ..self
        }
    }
    pub const fn root_candidate(self) -> Self {
        Self {
            root_candidates: self.root_candidates + 1,
            ..self
        }
    }
    pub const fn root_ready(self) -> Self {
        Self {
            root_ready: self.root_ready + 1,
            ..self
        }
    }
    pub const fn closed(self) -> Self {
        Self {
            closes: self.closes + 1,
            ..self
        }
    }
    pub const fn abandoned(self) -> Self {
        Self {
            abandons: self.abandons + 1,
            ..self
        }
    }
    pub const fn reclaimed(self) -> Self {
        Self {
            reclaims: self.reclaims + 1,
            ..self
        }
    }
    pub const fn replayed(self) -> Self {
        Self {
            replays: self.replays + 1,
            ..self
        }
    }
    pub const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn declarations(self) -> u64 {
        self.declarations
    }
    pub const fn admissions(self) -> u64 {
        self.admissions
    }
    pub const fn frontier_checkpoints(self) -> u64 {
        self.frontier_checkpoints
    }
    pub const fn reclaims(self) -> u64 {
        self.reclaims
    }
    pub const fn replays(self) -> u64 {
        self.replays
    }
    pub const fn denials(self) -> u64 {
        self.denials
    }
}
