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
pub struct PhysicalLayoutAccessConstraint {
    family: PhysicalLayoutAccessFamily,
    pattern: PhysicalLayoutAccessPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedPhysicalLayoutAccess {
    family: PhysicalLayoutAccessFamily,
    pattern: PhysicalLayoutAccessPattern,
}

impl PhysicalLayoutAccessConstraint {
    pub const fn admit(
        family: PhysicalLayoutAccessFamily,
        pattern: PhysicalLayoutAccessPattern,
    ) -> Result<Self, UnsupportedPhysicalLayoutAccess> {
        if family.supports(pattern) {
            Ok(Self { family, pattern })
        } else {
            Err(UnsupportedPhysicalLayoutAccess { family, pattern })
        }
    }

    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        self.family
    }
    pub const fn pattern(self) -> PhysicalLayoutAccessPattern {
        self.pattern
    }
}

impl PhysicalLayoutAccessFamily {
    pub const fn supports(self, pattern: PhysicalLayoutAccessPattern) -> bool {
        match self {
            Self::Page | Self::Frame | Self::Segment | Self::Extent => {
                matches!(pattern, PhysicalLayoutAccessPattern::PointLookup)
            }
            Self::RootManifest | Self::Allocation | Self::Fragmentation | Self::FreeSpace => {
                matches!(pattern, PhysicalLayoutAccessPattern::BoundedScan)
            }
            Self::ManifestIndex => matches!(
                pattern,
                PhysicalLayoutAccessPattern::PointLookup
                    | PhysicalLayoutAccessPattern::RangeLookup
                    | PhysicalLayoutAccessPattern::BoundedScan
            ),
        }
    }
}

impl UnsupportedPhysicalLayoutAccess {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        self.family
    }
    pub const fn pattern(self) -> PhysicalLayoutAccessPattern {
        self.pattern
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhysicalLayoutAccessConstraint, PhysicalLayoutAccessFamily, PhysicalLayoutAccessPattern,
    };

    #[test]
    fn grammar_admits_only_mechanics_supported_by_each_family() {
        assert!(PhysicalLayoutAccessConstraint::admit(
            PhysicalLayoutAccessFamily::Page,
            PhysicalLayoutAccessPattern::PointLookup,
        )
        .is_ok());
        assert!(PhysicalLayoutAccessConstraint::admit(
            PhysicalLayoutAccessFamily::Page,
            PhysicalLayoutAccessPattern::FullScan,
        )
        .is_err());
        assert!(PhysicalLayoutAccessConstraint::admit(
            PhysicalLayoutAccessFamily::FreeSpace,
            PhysicalLayoutAccessPattern::BoundedScan,
        )
        .is_ok());
    }
}
