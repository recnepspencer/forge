//! Fresh Query-owned admission for authority-free canonical bundle records.

use std::collections::{BTreeMap, BTreeSet};

use crate::result_shape::family_matches_query;

use super::errors::QueryCanonicalizationError as Error;
use super::portable_bundle::{
    WorthQueryPortableCanonicalQueryBundleRecord, WorthQueryPortableCanonicalQueryRecord,
    WorthQueryPortableCanonicalResultShapeRecord,
};
use super::query_artifact::build_query_artifact;
use super::readmission_limits::WorthQueryPortableCanonicalQueryReadmissionLimits;
use super::result_shape_artifact::build_result_shape_from_canonical_fields;
use super::CanonicalQueryBundle;

#[cfg(test)]
#[path = "fresh_readmission_tests.rs"]
mod tests;

/// Re-admits portable query meaning through current Query validation.
///
/// Stored digests remain descriptive claims. Query rebuilds both identities
/// from the carried canonical fields and exposes canonical authority only after
/// exact-form and bundle-invariant checks succeed.
pub fn validate_portable_query_bundle_freshly(
    record: WorthQueryPortableCanonicalQueryBundleRecord,
    limits: WorthQueryPortableCanonicalQueryReadmissionLimits,
) -> Result<CanonicalQueryBundle, Error> {
    validate_portable_query_bundle_freshly_with_work(record, limits)
        .map(|readmission| readmission.into_bundle())
}

#[derive(Debug)]
pub struct WorthQueryPortableCanonicalQueryReadmission {
    bundle: CanonicalQueryBundle,
    logical_work_bytes: u64,
}

impl WorthQueryPortableCanonicalQueryReadmission {
    pub fn into_bundle(self) -> CanonicalQueryBundle {
        self.bundle
    }

    pub const fn logical_work_bytes(&self) -> u64 {
        self.logical_work_bytes
    }
}

pub fn validate_portable_query_bundle_freshly_with_work(
    record: WorthQueryPortableCanonicalQueryBundleRecord,
    limits: WorthQueryPortableCanonicalQueryReadmissionLimits,
) -> Result<WorthQueryPortableCanonicalQueryReadmission, Error> {
    validate_work_budget(&record, limits.narrowed())?;
    validate_exact_canonical_form(&record)?;
    let logical_work_bytes = record.portable_record_logical_bytes();
    let parts = record.into_parts();
    let stored_query_digest = parts.query_digest;
    let stored_result_shape_digest = parts.result_shape_digest;

    let query = build_query_artifact(
        parts.query_family,
        parts.query_root,
        parts.projection,
        parts.predicates,
        parts.ordering,
        parts.traversal,
        parts.identity_bindings,
    );
    if query.digest() != &stored_query_digest {
        return Err(Error::DigestBasisInconsistency { artifact: "query" });
    }

    let result_shape = build_result_shape_from_canonical_fields(
        parts.result_shape_family,
        parts.result_shape_fields,
    );
    if result_shape.digest() != &stored_result_shape_digest {
        return Err(Error::DigestBasisInconsistency {
            artifact: "result_shape",
        });
    }

    let bundle = CanonicalQueryBundle {
        query,
        result_shape,
        report: parts.report,
        counters: parts.counters,
    };
    bundle.check_invariants()?;
    Ok(WorthQueryPortableCanonicalQueryReadmission {
        bundle,
        logical_work_bytes,
    })
}

fn validate_work_budget(
    record: &WorthQueryPortableCanonicalQueryBundleRecord,
    limits: WorthQueryPortableCanonicalQueryReadmissionLimits,
) -> Result<(), Error> {
    let observed_entries = record.portable_record_nested_entries();
    if observed_entries > u64::from(limits.maximum_entries()) {
        return Err(Error::PortableRecordEntryBudgetExceeded {
            observed: observed_entries,
            maximum: limits.maximum_entries(),
        });
    }
    let observed_bytes = record.portable_record_logical_bytes();
    if observed_bytes > limits.maximum_logical_bytes() {
        return Err(Error::PortableRecordLogicalBytesBudgetExceeded {
            observed: observed_bytes,
            maximum: limits.maximum_logical_bytes(),
        });
    }
    Ok(())
}

fn validate_exact_canonical_form(
    record: &WorthQueryPortableCanonicalQueryBundleRecord,
) -> Result<(), Error> {
    let query = record.query();
    let result_shape = record.result_shape();
    if query.projection().is_empty() {
        return Err(Error::EmptyProjectionSet);
    }
    if result_shape.fields().is_empty() {
        return Err(Error::EmptyResultShapeFieldSet);
    }
    if !family_matches_query(query.family(), result_shape.family()) {
        return Err(Error::QueryShapeFamilyMismatch {
            query_family: query.family().clone(),
            result_shape_family: result_shape.family().clone(),
        });
    }
    require_strict_order(query.projection(), "query_projection")?;
    require_strict_order(query.predicates(), "query_predicates")?;
    require_strict_order(query.ordering(), "query_ordering")?;
    require_strict_order(query.traversal(), "query_traversal")?;
    require_strict_order(result_shape.fields(), "result_shape_fields")?;
    if let Some(traversal) = query.traversal().iter().find(|entry| entry.depth == 0) {
        return Err(Error::UnsupportedTraversalDepth {
            relation: traversal.relation.to_string(),
            depth: traversal.depth,
        });
    }
    if !query
        .identity_bindings()
        .windows(2)
        .all(|pair| pair[0].slot() < pair[1].slot())
    {
        return Err(Error::InvalidCanonicalOrderingBasis {
            artifact: "query_identity_bindings",
        });
    }
    validate_result_shape_sources(query, result_shape)
}

fn require_strict_order<T: Ord>(values: &[T], artifact: &'static str) -> Result<(), Error> {
    if values.windows(2).all(|pair| pair[0] < pair[1]) {
        Ok(())
    } else {
        Err(Error::InvalidCanonicalOrderingBasis { artifact })
    }
}

fn validate_result_shape_sources(
    query: &WorthQueryPortableCanonicalQueryRecord,
    result_shape: &WorthQueryPortableCanonicalResultShapeRecord,
) -> Result<(), Error> {
    let projected = query
        .projection()
        .iter()
        .map(|entry| entry.field_key().clone())
        .collect::<BTreeSet<_>>();
    let mut delivered_sources = BTreeMap::new();
    for field in result_shape.fields() {
        let source = field.source_field_key();
        if !projected.contains(source) {
            return Err(Error::UnprojectedShapeField {
                source_aspect: source.aspect().to_string(),
                source_field: source.field().to_string(),
                delivered_name: field.delivered_name.to_string(),
            });
        }
        match delivered_sources.insert(field.delivered_name.clone(), source.clone()) {
            Some(first) if first != *source => {
                return Err(Error::AmbiguousShapeAliasIdentity {
                    delivered_name: field.delivered_name.to_string(),
                    first_source_aspect: first.aspect().to_string(),
                    first_source_field: first.field().to_string(),
                    second_source_aspect: source.aspect().to_string(),
                    second_source_field: source.field().to_string(),
                });
            }
            _ => {}
        }
    }
    Ok(())
}
