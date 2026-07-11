#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedSnapshotLayoutRule {
    _private: (),
}

impl AdmittedSnapshotLayoutRule {
    pub(crate) const fn internal_phase23() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBranchDeltaLayoutRule {
    _private: (),
}

impl AdmittedBranchDeltaLayoutRule {
    pub(crate) const fn internal_phase23() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedStableBasisLayoutRule {
    _private: (),
}

impl AdmittedStableBasisLayoutRule {
    pub(crate) const fn internal_phase23() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedContinuationLayoutRule {
    _private: (),
}

impl AdmittedContinuationLayoutRule {
    pub(crate) const fn internal_phase23() -> Self {
        Self { _private: () }
    }
}
