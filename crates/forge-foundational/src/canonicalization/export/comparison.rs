use super::super::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalEquivalenceBasis, CanonicalMismatchBasis,
    CanonicalMismatchKind, CanonicalizationRuleVersion,
};
use super::bundle::{CanonicalExportBasisBundle, CanonicalExportBundle};
use super::readmission::CanonicalExportReadyArtifact;
use crate::canonicalization::mismatch::CanonicalExportMismatchContext;

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
    fn new(
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

pub fn compare_canonical_exports(
    left: &CanonicalExportReadyArtifact,
    right: &CanonicalExportReadyArtifact,
) -> CanonicalExportComparisonOutcome {
    if let Some(manifest_mismatch) = first_manifest_mismatch(left.payload(), right.payload()) {
        return CanonicalExportComparisonOutcome::ManifestMismatch(manifest_mismatch);
    }

    match first_bundle_mismatch(left.payload().bundle(), right.payload().bundle()) {
        Some(mismatch) => CanonicalExportComparisonOutcome::Mismatched(mismatch),
        None => CanonicalExportComparisonOutcome::Equivalent,
    }
}

fn first_manifest_mismatch(
    left: &CanonicalExportBundle,
    right: &CanonicalExportBundle,
) -> Option<CanonicalExportManifestMismatch> {
    let mut left_rows = left.manifest().rows().iter();
    let mut right_rows = right.manifest().rows().iter();

    loop {
        match (left_rows.next(), right_rows.next()) {
            (Some(left), Some(right)) => {
                if left.domain() != right.domain() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::DomainMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
                if left.rule_version() != right.rule_version() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::RuleVersionMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
                if left.producer_shape() != right.producer_shape() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::ProducerShapeMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
                if left.equivalence_basis() != right.equivalence_basis() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::EquivalenceBasisMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
                if left.expected_entry_count() != right.expected_entry_count() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::EntryCountMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
                if left.expected_cost() != right.expected_cost() {
                    return Some(CanonicalExportManifestMismatch::new(
                        CanonicalExportManifestMismatchKind::CostMismatch,
                        Some(left.domain()),
                        Some(right.domain()),
                    ));
                }
            }
            (Some(left), None) => {
                return Some(CanonicalExportManifestMismatch::new(
                    CanonicalExportManifestMismatchKind::AdditionalManifestRow,
                    Some(left.domain()),
                    None,
                ));
            }
            (None, Some(right)) => {
                return Some(CanonicalExportManifestMismatch::new(
                    CanonicalExportManifestMismatchKind::MissingManifestRow,
                    None,
                    Some(right.domain()),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn first_bundle_mismatch(
    left: &CanonicalExportBasisBundle,
    right: &CanonicalExportBasisBundle,
) -> Option<CanonicalMismatchBasis> {
    let mut left_sequences = left.sequences().iter();
    let mut right_sequences = right.sequences().iter();

    loop {
        match (left_sequences.next(), right_sequences.next()) {
            (Some(left_sequence), Some(right_sequence)) => {
                if let Some(mismatch) = first_sequence_mismatch(
                    left.version().clone(),
                    right.version().clone(),
                    left_sequence.domain(),
                    right_sequence.domain(),
                    left_sequence.entries(),
                    right_sequence.entries(),
                ) {
                    return Some(mismatch);
                }
            }
            (Some(left_sequence), None) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::AdditionalEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left.version().clone(),
                        right.version().clone(),
                        left_sequence.domain(),
                        left_sequence.domain(),
                    ),
                    left_sequence.entries().first(),
                    None,
                ));
            }
            (None, Some(right_sequence)) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::MissingEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left.version().clone(),
                        right.version().clone(),
                        right_sequence.domain(),
                        right_sequence.domain(),
                    ),
                    None,
                    right_sequence.entries().first(),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn first_sequence_mismatch(
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    left_domain: CanonicalBasisDomain,
    right_domain: CanonicalBasisDomain,
    left_entries: &[CanonicalBasisEntry],
    right_entries: &[CanonicalBasisEntry],
) -> Option<CanonicalMismatchBasis> {
    let mut left_iter = left_entries.iter().peekable();
    let mut right_iter = right_entries.iter().peekable();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(left), Some(right))
                if same_entry_locus(left, right) && left.kind() != right.kind() =>
            {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::EntryKindMismatch,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left_version,
                        right_version,
                        left_domain,
                        right_domain,
                    ),
                    left_iter.next(),
                    right_iter.next(),
                ));
            }
            (Some(left), Some(right)) => match compare_entry_keys(left, right) {
                std::cmp::Ordering::Equal => {
                    let left = left_iter.next().expect("peeked left");
                    let right = right_iter.next().expect("peeked right");
                    if left.value() != right.value() {
                        return Some(CanonicalMismatchBasis::from_export_entries(
                            CanonicalMismatchKind::ValueMismatch,
                            CanonicalExportMismatchContext::new(
                                CanonicalEquivalenceBasis::ExactCanonicalBasis,
                                left_version,
                                right_version,
                                left_domain,
                                right_domain,
                            ),
                            Some(left),
                            Some(right),
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    return Some(CanonicalMismatchBasis::from_export_entries(
                        CanonicalMismatchKind::AdditionalEntry,
                        CanonicalExportMismatchContext::new(
                            CanonicalEquivalenceBasis::ExactCanonicalBasis,
                            left_version,
                            right_version,
                            left_domain,
                            right_domain,
                        ),
                        left_iter.next(),
                        None,
                    ));
                }
                std::cmp::Ordering::Greater => {
                    return Some(CanonicalMismatchBasis::from_export_entries(
                        CanonicalMismatchKind::MissingEntry,
                        CanonicalExportMismatchContext::new(
                            CanonicalEquivalenceBasis::ExactCanonicalBasis,
                            left_version,
                            right_version,
                            left_domain,
                            right_domain,
                        ),
                        None,
                        right_iter.next(),
                    ));
                }
            },
            (Some(_), None) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::AdditionalEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left_version,
                        right_version,
                        left_domain,
                        right_domain,
                    ),
                    left_iter.next(),
                    None,
                ));
            }
            (None, Some(_)) => {
                return Some(CanonicalMismatchBasis::from_export_entries(
                    CanonicalMismatchKind::MissingEntry,
                    CanonicalExportMismatchContext::new(
                        CanonicalEquivalenceBasis::ExactCanonicalBasis,
                        left_version,
                        right_version,
                        left_domain,
                        right_domain,
                    ),
                    None,
                    right_iter.next(),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn same_entry_locus(left: &CanonicalBasisEntry, right: &CanonicalBasisEntry) -> bool {
    left.domain() == right.domain() && left.locus() == right.locus()
}

fn compare_entry_keys(
    left: &CanonicalBasisEntry,
    right: &CanonicalBasisEntry,
) -> std::cmp::Ordering {
    (left.domain(), left.locus(), left.kind()).cmp(&(right.domain(), right.locus(), right.kind()))
}
