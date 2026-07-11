#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBlobObjectLayoutRule {
    _private: (),
}

impl AdmittedBlobObjectLayoutRule {
    pub(crate) const fn phase24() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedChunkTreeLayoutRule {
    _private: (),
}

impl AdmittedChunkTreeLayoutRule {
    pub(crate) const fn phase24() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedStreamingLayoutRule {
    _private: (),
}

impl AdmittedStreamingLayoutRule {
    pub(crate) const fn phase24() -> Self {
        Self { _private: () }
    }
}
