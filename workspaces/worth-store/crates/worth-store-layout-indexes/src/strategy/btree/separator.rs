use crate::keyspace::{CanonicalKeyBytes, ComparatorLaw, PrefixLawWitness, RangeBoundLawWitness};
use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeLookupBranch {
    Left,
    RightOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeSeparatorLaw {
    comparator: ComparatorLaw,
    prefix: PrefixLawWitness,
    range: RangeBoundLawWitness,
}

impl BTreeSeparatorLaw {
    pub(crate) const fn new(
        comparator: ComparatorLaw,
        prefix: PrefixLawWitness,
        range: RangeBoundLawWitness,
    ) -> Self {
        Self {
            comparator,
            prefix,
            range,
        }
    }

    pub const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }

    pub const fn prefix(self) -> PrefixLawWitness {
        self.prefix
    }

    pub const fn range(self) -> RangeBoundLawWitness {
        self.range
    }

    pub fn route_lookup(
        self,
        probe: &CanonicalKeyBytes,
        separator: &CanonicalKeyBytes,
    ) -> Result<BTreeLookupBranch, StrategyDenial> {
        self.ensure_same_encoding(probe, separator)?;
        Ok(if probe.as_bytes() < separator.as_bytes() {
            BTreeLookupBranch::Left
        } else {
            BTreeLookupBranch::RightOrEqual
        })
    }

    pub fn verify_separator_partition(
        self,
        left_max: &CanonicalKeyBytes,
        separator: &CanonicalKeyBytes,
        right_min: &CanonicalKeyBytes,
    ) -> Result<(), StrategyDenial> {
        self.ensure_same_encoding(left_max, separator)?;
        self.ensure_same_encoding(separator, right_min)?;
        if left_max.as_bytes() < separator.as_bytes()
            && separator.as_bytes() <= right_min.as_bytes()
        {
            return Ok(());
        }
        Err(StrategyDenial::ComparatorOrderViolation)
    }

    fn ensure_same_encoding(
        self,
        left: &CanonicalKeyBytes,
        right: &CanonicalKeyBytes,
    ) -> Result<(), StrategyDenial> {
        if left.encoding() != self.comparator.encoding()
            || right.encoding() != self.comparator.encoding()
        {
            return Err(StrategyDenial::RangeOrPrefixLawRequired);
        }
        Ok(())
    }
}
