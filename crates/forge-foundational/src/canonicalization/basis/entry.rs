use super::{CanonicalBasisDomain, CanonicalBasisEntryKind, CanonicalBasisLocus};
use crate::canonicalization::basis::CanonicalBasisValue;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalBasisEntry {
    domain: CanonicalBasisDomain,
    locus: CanonicalBasisLocus,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
}

impl CanonicalBasisEntry {
    pub fn new(
        domain: CanonicalBasisDomain,
        locus: CanonicalBasisLocus,
        kind: CanonicalBasisEntryKind,
        value: CanonicalBasisValue,
    ) -> Self {
        Self {
            domain,
            locus,
            kind,
            value,
        }
    }

    pub const fn domain(&self) -> CanonicalBasisDomain {
        self.domain
    }

    pub const fn locus(&self) -> &CanonicalBasisLocus {
        &self.locus
    }

    pub const fn kind(&self) -> CanonicalBasisEntryKind {
        self.kind
    }

    pub const fn value(&self) -> &CanonicalBasisValue {
        &self.value
    }
}
