#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLayoutAccessFamily {
    Page,
    Frame,
    Segment,
    Extent,
    RootManifest,
    ManifestIndex,
    Allocation,
    FreeSpace,
    Fragmentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLayoutAccessPattern {
    PointLookup,
    RangeLookup,
    BoundedScan,
    FullScan,
    StreamingRead,
    DegradedExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPageLayoutRule {
    _private: (),
}

impl AdmittedPageLayoutRule {
    pub(crate) const fn internal_phase19() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase19-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase19() -> Self {
        Self::internal_phase19()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedFrameLayoutRule {
    _private: (),
}

impl AdmittedFrameLayoutRule {
    pub(crate) const fn internal_phase19() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase19-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase19() -> Self {
        Self::internal_phase19()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedSegmentLayoutRule {
    _private: (),
}

impl AdmittedSegmentLayoutRule {
    pub(crate) const fn internal_phase19() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase19-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase19() -> Self {
        Self::internal_phase19()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedExtentLayoutRule {
    _private: (),
}

impl AdmittedExtentLayoutRule {
    pub(crate) const fn internal_phase19() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase19-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase19() -> Self {
        Self::internal_phase19()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedRootManifestLayoutRule {
    _private: (),
}

impl AdmittedRootManifestLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedManifestIndexLayoutRule {
    _private: (),
}

impl AdmittedManifestIndexLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedAllocationLayoutRule {
    _private: (),
}

impl AdmittedAllocationLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedFreeSpaceLayoutRule {
    _private: (),
}

impl AdmittedFreeSpaceLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedFragmentationLayoutRule {
    _private: (),
}

impl AdmittedFragmentationLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}
