use crate::canonicalization::basis::{
    CanonicalBasisDomain, CanonicalBasisEntryKind, CanonicalBasisLocus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalBasisConstructionDenial {
    EmptySequence,
    DomainMismatch {
        expected: CanonicalBasisDomain,
        actual: CanonicalBasisDomain,
    },
    DuplicateEntry {
        domain: CanonicalBasisDomain,
        locus: CanonicalBasisLocus,
        kind: CanonicalBasisEntryKind,
    },
    BundleRuleVersionMismatch,
    DuplicateBundleDomain {
        domain: CanonicalBasisDomain,
    },
}
