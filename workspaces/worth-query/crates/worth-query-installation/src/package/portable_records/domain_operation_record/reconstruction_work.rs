//! Owner-aligned work observation for retained operation records.

use super::*;

impl WorthQueryPortableDomainOperationRecord {
    pub(crate) fn reconstruction_work(&self) -> (u64, u64) {
        let text = crate::package::reconstruction_text_bytes;
        let (canonical_bytes, canonical_entries) =
            crate::domain_operation::canonical_operation_reconstruction_work(
                &self.identity,
                &self.semantics,
            );
        let logical_bytes = canonical_bytes
            .saturating_add(text(&self.canonical_identity))
            .saturating_add(
                self.semantics
                    .canonical_query
                    .portable_record_logical_bytes(),
            );
        let nested_entries = canonical_entries.saturating_add(
            self.semantics
                .canonical_query
                .portable_record_nested_entries(),
        );
        (logical_bytes, nested_entries)
    }
}
