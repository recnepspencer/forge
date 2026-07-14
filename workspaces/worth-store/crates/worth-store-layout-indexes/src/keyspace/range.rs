use super::comparator::ComparatorLaw;
use super::declaration::PhysicalKeyDomain;
use super::encoding::{exclusive_bound_sentinel, CanonicalKeyBytes};
use super::value::ConcretePhysicalKeyWitness;
use crate::catalog::ArtifactFamilyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeBoundBehavior {
    InclusiveStartExclusiveEnd,
    InclusiveStartInclusiveEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeBoundLawWitness {
    comparator: ComparatorLaw,
    behavior: RangeBoundBehavior,
}

impl RangeBoundLawWitness {
    pub(crate) const fn new(comparator: ComparatorLaw, behavior: RangeBoundBehavior) -> Self {
        Self {
            comparator,
            behavior,
        }
    }

    pub const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }

    pub const fn behavior(self) -> RangeBoundBehavior {
        self.behavior
    }
}

pub(crate) const fn require_range_bound_law(
    comparator: ComparatorLaw,
) -> Result<RangeBoundLawWitness, ArtifactFamilyDenial> {
    let behavior = match comparator.encoding().domain().domain() {
        PhysicalKeyDomain::PageAddressKey
        | PhysicalKeyDomain::SegmentAddressKey
        | PhysicalKeyDomain::ExtentAddressKey
        | PhysicalKeyDomain::PhysicalReferenceKey
        | PhysicalKeyDomain::WalRecordKey
        | PhysicalKeyDomain::BlobIdentityKey => RangeBoundBehavior::InclusiveStartExclusiveEnd,
        PhysicalKeyDomain::RootManifestKey => {
            return Err(ArtifactFamilyDenial::PhysicalKeyDomainDoesNotSupportRangeBounds);
        }
    };

    Ok(RangeBoundLawWitness::new(comparator, behavior))
}

pub(crate) fn range_start_bytes_for_key(
    law: RangeBoundLawWitness,
    key: ConcretePhysicalKeyWitness,
) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
    super::comparator::canonical_bytes_for_key(law.comparator(), key)
}

pub(crate) fn range_end_bytes_for_key(
    law: RangeBoundLawWitness,
    key: ConcretePhysicalKeyWitness,
) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
    let start = super::comparator::canonical_bytes_for_key(law.comparator(), key)?;
    let bytes = match law.behavior() {
        RangeBoundBehavior::InclusiveStartExclusiveEnd => {
            let mut bytes = start.as_bytes().to_vec();
            bytes.push(exclusive_bound_sentinel(start.encoding()));
            bytes
        }
        RangeBoundBehavior::InclusiveStartInclusiveEnd => start.as_bytes().to_vec(),
    };
    Ok(CanonicalKeyBytes::new(law.comparator().encoding(), bytes))
}
