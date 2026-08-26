//! Stable logical width for a retained canonical query bundle.

use crate::binding::QueryBindingSubject;
use crate::diagnostics::{CanonicalizationWarning, NormalizationEvent};

use super::{CanonicalQueryBundle, WorthQueryPortableCanonicalQueryBundleRecord};

const LOGICAL_LIST_COUNT_BYTES: u64 = 8;
const LOGICAL_COUNTER_BYTES: u64 = 8;
const LOGICAL_QUERY_LISTS: u64 = 5;
const LOGICAL_RESULT_LISTS: u64 = 1;
const LOGICAL_REPORT_LISTS: u64 = 2;
const LOGICAL_CANONICALIZATION_COUNTERS: u64 = 10;
const LOGICAL_REPORT_COUNTERS: u64 = 3;

impl CanonicalQueryBundle {
    /// Canonical semantic bytes carried when this bundle enters a portable record.
    pub fn portable_record_logical_bytes(&self) -> u64 {
        WorthQueryPortableCanonicalQueryBundleRecord::project(self).portable_record_logical_bytes()
    }
}

impl WorthQueryPortableCanonicalQueryBundleRecord {
    /// Aggregate retained list entries that reconstruction must traverse.
    pub fn portable_record_nested_entries(&self) -> u64 {
        let query = self.query();
        let result = self.result_shape();
        [
            query.projection().len(),
            query.predicates().len(),
            query.ordering().len(),
            query.traversal().len(),
            query.identity_bindings().len(),
            result.fields().len(),
            self.report().warnings().len(),
            self.report().events().len(),
        ]
        .into_iter()
        .try_fold(0_u64, |total, count| {
            total.checked_add(u64::try_from(count).unwrap_or(u64::MAX))
        })
        .unwrap_or(u64::MAX)
    }

    /// Stable logical width of this authority-free descriptive projection.
    pub fn portable_record_logical_bytes(&self) -> u64 {
        let query = self.query();
        let result = self.result_shape();
        let report = self.report();
        let fixed_list_bytes = (LOGICAL_QUERY_LISTS + LOGICAL_RESULT_LISTS + LOGICAL_REPORT_LISTS)
            * LOGICAL_LIST_COUNT_BYTES;
        let fixed_counter_bytes =
            (LOGICAL_CANONICALIZATION_COUNTERS + LOGICAL_REPORT_COUNTERS) * LOGICAL_COUNTER_BYTES;
        let mut bytes = 3_u64
            + fixed_list_bytes
            + fixed_counter_bytes
            + text(query.digest().as_str())
            + text(query.root().as_str())
            + text(result.digest().as_str());
        bytes += query
            .projection()
            .iter()
            .map(|entry| text(&entry.digest_part()))
            .fold(0, u64::saturating_add);
        bytes += query
            .predicates()
            .iter()
            .map(|entry| text(&entry.digest_part()))
            .fold(0, u64::saturating_add);
        bytes += query
            .ordering()
            .iter()
            .map(|entry| text(&entry.digest_part()))
            .fold(0, u64::saturating_add);
        bytes += query
            .traversal()
            .iter()
            .map(|entry| text(&entry.digest_part()))
            .fold(0, u64::saturating_add);
        bytes += query
            .identity_bindings()
            .iter()
            .map(|binding| {
                binding_subject_bytes(binding.subject())
                    .saturating_add(text(binding.slot().as_str()))
            })
            .fold(0, u64::saturating_add);
        bytes += result
            .fields()
            .iter()
            .map(|field| text(&field.digest_part()))
            .fold(0, u64::saturating_add);
        bytes += report
            .warnings()
            .iter()
            .map(warning_bytes)
            .fold(0, u64::saturating_add);
        bytes = bytes.saturating_add(
            report
                .events()
                .iter()
                .map(event_bytes)
                .fold(0, u64::saturating_add),
        );
        bytes = bytes.saturating_add(text(&report.identity_freeze().query_digest));
        bytes.saturating_add(text(&report.identity_freeze().result_shape_digest))
    }
}

const fn binding_subject_bytes(subject: &QueryBindingSubject) -> u64 {
    match subject {
        QueryBindingSubject::RootEntity | QueryBindingSubject::TraversalRoot => 1,
    }
}

fn warning_bytes(warning: &CanonicalizationWarning) -> u64 {
    match warning {
        CanonicalizationWarning::DuplicateProjectionCollapsed { aspect, field } => {
            1 + text(aspect) + text(field)
        }
        CanonicalizationWarning::DuplicateTraversalCollapsed { relation, .. } => 2 + text(relation),
        CanonicalizationWarning::DuplicateResultFieldCollapsed { delivered_name } => {
            1 + text(delivered_name)
        }
        CanonicalizationWarning::NonIdentityBindingMetadataIgnored { key } => 1 + text(key),
    }
}

fn event_bytes(event: &NormalizationEvent) -> u64 {
    match event {
        NormalizationEvent::ProjectionRetained { aspect, field }
        | NormalizationEvent::ProjectionCollapsedDuplicate { aspect, field } => {
            1 + text(aspect) + text(field)
        }
        NormalizationEvent::TraversalRetained { relation, .. }
        | NormalizationEvent::TraversalCollapsedDuplicate { relation, .. } => 2 + text(relation),
        NormalizationEvent::ResultFieldRetained {
            source_aspect,
            source_field,
            delivered_name,
        } => 1 + text(source_aspect) + text(source_field) + text(delivered_name),
        NormalizationEvent::ResultFieldCollapsedDuplicate { delivered_name } => {
            1 + text(delivered_name)
        }
        NormalizationEvent::IdentityBindingRetained { slot }
        | NormalizationEvent::IdentityBindingCollapsedDuplicate { slot } => 1 + text(slot),
        NormalizationEvent::NonIdentityBindingIgnored { key } => 1 + text(key),
        NormalizationEvent::CompatibilityEstablished => 1,
        NormalizationEvent::IdentityFrozen {
            query_digest,
            result_shape_digest,
        } => 1 + text(query_digest) + text(result_shape_digest),
    }
}

fn text(value: &str) -> u64 {
    8_u64.saturating_add(u64::try_from(value.len()).unwrap_or(u64::MAX))
}
