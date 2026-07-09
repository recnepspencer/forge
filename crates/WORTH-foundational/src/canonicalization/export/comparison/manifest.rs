use super::outcome::{CanonicalExportManifestMismatch, CanonicalExportManifestMismatchKind};
use crate::canonicalization::export::bundle::CanonicalExportBundle;

pub(super) fn first_manifest_mismatch(
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
