use crate::canonicalization::{CanonicalBasisDomain, CanonicalMismatchBasis};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalExportComparisonOutcome {
    Equivalent,
    Mismatched(CanonicalMismatchBasis),
    ManifestMismatch(CanonicalExportManifestMismatch),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalExportManifestMismatch {
    kind: CanonicalExportManifestMismatchKind,
    left_domain: Option<CanonicalBasisDomain>,
    right_domain: Option<CanonicalBasisDomain>,
}

impl CanonicalExportManifestMismatch {
    pub(super) fn new(
        kind: CanonicalExportManifestMismatchKind,
        left_domain: Option<CanonicalBasisDomain>,
        right_domain: Option<CanonicalBasisDomain>,
    ) -> Self {
        Self {
            kind,
            left_domain,
            right_domain,
        }
    }

    pub const fn kind(&self) -> CanonicalExportManifestMismatchKind {
        self.kind
    }

    pub const fn left_domain(&self) -> Option<CanonicalBasisDomain> {
        self.left_domain
    }

    pub const fn right_domain(&self) -> Option<CanonicalBasisDomain> {
        self.right_domain
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalExportManifestMismatchKind {
    MissingManifestRow,
    AdditionalManifestRow,
    DomainMismatch,
    RuleVersionMismatch,
    ProducerShapeMismatch,
    EquivalenceBasisMismatch,
    EntryCountMismatch,
    CostMismatch,
}
