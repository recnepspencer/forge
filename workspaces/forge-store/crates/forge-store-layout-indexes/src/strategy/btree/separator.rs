use crate::keyspace::{CanonicalKeyBytes, ComparatorLaw, PrefixLawWitness, RangeBoundLawWitness};
use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BTreeLookupBranch {
    Left,
    RightOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeSeparatorLaw {
    comparator: ComparatorLaw,
    prefix: PrefixLawWitness,
    range: RangeBoundLawWitness,
}

impl S8BTreeSeparatorLaw {
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
    ) -> Result<S8BTreeLookupBranch, S8StrategyDenial> {
        self.ensure_same_encoding(probe, separator)?;
        Ok(if probe.as_bytes() < separator.as_bytes() {
            S8BTreeLookupBranch::Left
        } else {
            S8BTreeLookupBranch::RightOrEqual
        })
    }

    pub fn verify_separator_partition(
        self,
        left_max: &CanonicalKeyBytes,
        separator: &CanonicalKeyBytes,
        right_min: &CanonicalKeyBytes,
    ) -> Result<(), S8StrategyDenial> {
        self.ensure_same_encoding(left_max, separator)?;
        self.ensure_same_encoding(separator, right_min)?;
        if left_max.as_bytes() < separator.as_bytes()
            && separator.as_bytes() <= right_min.as_bytes()
        {
            return Ok(());
        }
        Err(S8StrategyDenial::ComparatorOrderViolation)
    }

    fn ensure_same_encoding(
        self,
        left: &CanonicalKeyBytes,
        right: &CanonicalKeyBytes,
    ) -> Result<(), S8StrategyDenial> {
        if left.encoding() != self.comparator.encoding()
            || right.encoding() != self.comparator.encoding()
        {
            return Err(S8StrategyDenial::RangeOrPrefixLawRequired);
        }
        Ok(())
    }
}
