#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedDedupeLayoutRule {
    _private: (),
}

impl AdmittedDedupeLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReachabilityLayoutRule {
    _private: (),
}

impl AdmittedReachabilityLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRetentionLayoutRule {
    _private: (),
}

impl AdmittedRetentionLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReclaimLayoutRule {
    _private: (),
}

impl AdmittedReclaimLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCompactionLayoutRule {
    _private: (),
}

impl AdmittedCompactionLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedQuarantineLayoutRule {
    _private: (),
}

impl AdmittedQuarantineLayoutRule {
    pub(crate) const fn phase25() -> Self {
        Self { _private: () }
    }
}
