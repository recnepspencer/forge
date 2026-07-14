#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharpCheckpointCertificationMode {
    _sealed: (),
}

impl SharpCheckpointCertificationMode {
    pub const fn certified() -> Self {
        Self { _sealed: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuzzyCheckpointCertificationModeDenialKind {
    MissingBeginEndRecords,
    MissingDirtyPageTableEvidence,
    MissingS5InterleavingAssumptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuzzyCheckpointCertificationModeDenial {
    kind: FuzzyCheckpointCertificationModeDenialKind,
}

impl FuzzyCheckpointCertificationModeDenial {
    pub const fn missing_begin_end_records() -> Self {
        Self {
            kind: FuzzyCheckpointCertificationModeDenialKind::MissingBeginEndRecords,
        }
    }

    pub const fn missing_dirty_page_table_evidence() -> Self {
        Self {
            kind: FuzzyCheckpointCertificationModeDenialKind::MissingDirtyPageTableEvidence,
        }
    }

    pub const fn missing_physical_isolation_interleaving_assumptions() -> Self {
        Self {
            kind: FuzzyCheckpointCertificationModeDenialKind::MissingS5InterleavingAssumptions,
        }
    }

    pub const fn kind(self) -> FuzzyCheckpointCertificationModeDenialKind {
        self.kind
    }
}
