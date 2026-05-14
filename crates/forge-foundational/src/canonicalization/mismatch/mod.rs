use super::equivalence::{CanonicalComparisonInput, CanonicalEquivalenceBasis};
use super::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalMismatchBasis {
    kind: CanonicalMismatchKind,
    equivalence_basis: CanonicalEquivalenceBasis,
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    left_domain: CanonicalBasisDomain,
    right_domain: CanonicalBasisDomain,
    left_locus: Option<CanonicalBasisLocus>,
    right_locus: Option<CanonicalBasisLocus>,
    left_entry_kind: Option<CanonicalBasisEntryKind>,
    right_entry_kind: Option<CanonicalBasisEntryKind>,
}

impl CanonicalMismatchBasis {
    pub(crate) fn from_input(
        input: &CanonicalComparisonInput,
        kind: CanonicalMismatchKind,
        left: Option<&CanonicalBasisEntry>,
        right: Option<&CanonicalBasisEntry>,
    ) -> Self {
        Self {
            kind,
            equivalence_basis: input.equivalence_basis(),
            left_version: input.left().payload().version().clone(),
            right_version: input.right().payload().version().clone(),
            left_domain: input.left().payload().domain(),
            right_domain: input.right().payload().domain(),
            left_locus: left.map(|entry| entry.locus().clone()),
            right_locus: right.map(|entry| entry.locus().clone()),
            left_entry_kind: left.map(|entry| entry.kind()),
            right_entry_kind: right.map(|entry| entry.kind()),
        }
    }

    pub(crate) fn from_export_entries(
        kind: CanonicalMismatchKind,
        context: CanonicalExportMismatchContext,
        left: Option<&CanonicalBasisEntry>,
        right: Option<&CanonicalBasisEntry>,
    ) -> Self {
        Self {
            kind,
            equivalence_basis: context.equivalence_basis,
            left_version: context.left_version,
            right_version: context.right_version,
            left_domain: context.left_domain,
            right_domain: context.right_domain,
            left_locus: left.map(|entry| entry.locus().clone()),
            right_locus: right.map(|entry| entry.locus().clone()),
            left_entry_kind: left.map(|entry| entry.kind()),
            right_entry_kind: right.map(|entry| entry.kind()),
        }
    }

    pub const fn kind(&self) -> CanonicalMismatchKind {
        self.kind
    }

    pub const fn equivalence_basis(&self) -> CanonicalEquivalenceBasis {
        self.equivalence_basis
    }

    pub fn left_version(&self) -> &CanonicalizationRuleVersion {
        &self.left_version
    }

    pub fn right_version(&self) -> &CanonicalizationRuleVersion {
        &self.right_version
    }

    pub const fn left_domain(&self) -> CanonicalBasisDomain {
        self.left_domain
    }

    pub const fn right_domain(&self) -> CanonicalBasisDomain {
        self.right_domain
    }

    pub fn left_locus(&self) -> Option<&CanonicalBasisLocus> {
        self.left_locus.as_ref()
    }

    pub fn right_locus(&self) -> Option<&CanonicalBasisLocus> {
        self.right_locus.as_ref()
    }

    pub const fn left_entry_kind(&self) -> Option<CanonicalBasisEntryKind> {
        self.left_entry_kind
    }

    pub const fn right_entry_kind(&self) -> Option<CanonicalBasisEntryKind> {
        self.right_entry_kind
    }
}

pub(crate) struct CanonicalExportMismatchContext {
    equivalence_basis: CanonicalEquivalenceBasis,
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    left_domain: CanonicalBasisDomain,
    right_domain: CanonicalBasisDomain,
}

impl CanonicalExportMismatchContext {
    pub(crate) fn new(
        equivalence_basis: CanonicalEquivalenceBasis,
        left_version: CanonicalizationRuleVersion,
        right_version: CanonicalizationRuleVersion,
        left_domain: CanonicalBasisDomain,
        right_domain: CanonicalBasisDomain,
    ) -> Self {
        Self {
            equivalence_basis,
            left_version,
            right_version,
            left_domain,
            right_domain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalMismatchKind {
    MissingEntry,
    AdditionalEntry,
    EntryKindMismatch,
    ValueMismatch,
    OrderingMismatch,
    EquivalenceBasisMismatch,
    VersionMismatch,
    UnsupportedComparison,
}
