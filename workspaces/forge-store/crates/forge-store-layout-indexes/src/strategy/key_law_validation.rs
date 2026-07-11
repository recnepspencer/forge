use super::{S8AdmittedLayoutStrategy, S8LayoutStrategyFamily, S8StrategyDenial};
use crate::keyspace::{
    declare_comparator_law, require_canonical_key_encoding, require_prefix_law,
    require_range_bound_law, CanonicalKeyEncoding, ComparatorLaw, PhysicalKeyDomainWitness,
    PrefixLawWitness, RangeBoundLawWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct S8DeclaredKeyLawPosture {
    encoding: CanonicalKeyEncoding,
    comparator: ComparatorLaw,
    prefix: Option<PrefixLawWitness>,
    range: Option<RangeBoundLawWitness>,
}

impl S8DeclaredKeyLawPosture {
    pub(super) const fn encoding(self) -> CanonicalKeyEncoding {
        self.encoding
    }
    pub(super) const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }
    pub(super) const fn prefix(self) -> Option<PrefixLawWitness> {
        self.prefix
    }
    pub(super) const fn range(self) -> Option<RangeBoundLawWitness> {
        self.range
    }
}

pub(super) fn admit_strategy_key_laws(
    family: S8LayoutStrategyFamily,
    key_domain: PhysicalKeyDomainWitness,
) -> Result<S8DeclaredKeyLawPosture, S8StrategyDenial> {
    let encoding = require_canonical_key_encoding(key_domain);
    let comparator = declare_comparator_law(encoding);
    let prefix = require_prefix_law(encoding).ok();
    let range = require_range_bound_law(comparator).ok();
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => Ok(S8DeclaredKeyLawPosture {
            encoding,
            comparator,
            prefix: Some(prefix.ok_or(S8StrategyDenial::RangeOrPrefixLawRequired)?),
            range: Some(range.ok_or(S8StrategyDenial::RangeOrPrefixLawRequired)?),
        }),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => Ok(S8DeclaredKeyLawPosture {
            encoding,
            comparator,
            prefix: None,
            range: None,
        }),
        _ => Err(S8StrategyDenial::UnsupportedFamily),
    }
}

impl S8AdmittedLayoutStrategy {
    pub const fn canonical_key_encoding(&self) -> Option<CanonicalKeyEncoding> {
        self.declaration.canonical_key_encoding()
    }
    pub const fn comparator_law(&self) -> Option<ComparatorLaw> {
        self.declaration.comparator_law()
    }
    pub const fn prefix_law(&self) -> Option<PrefixLawWitness> {
        self.declaration.prefix_law()
    }
    pub const fn range_bound_law(&self) -> Option<RangeBoundLawWitness> {
        self.declaration.range_bound_law()
    }
}
