use crate::authoring::{AuthoredQueryBundleRequest, RawAuthoredQuery, RawAuthoredResultShape};
use crate::binding::QueryBindingDescriptor;
use crate::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CompatibilityEvidence,
    IdentityFreezeEvidence, NormalizationEvent,
};
use crate::result_shape::family_matches_query;

use super::admission::enforce_admitted_authored_boundary;
use super::bindings::canonicalize_bindings;
use super::bundle_state::CanonicalQueryBundle;
use super::errors::QueryCanonicalizationError;
use super::ordering::canonicalize_ordering;
use super::predicates::canonicalize_predicates;
use super::projection::canonicalize_projection;
use super::query_artifact::build_query_artifact;
use super::result_shape_artifact::build_result_shape_artifact;
use super::traversal::canonicalize_traversal;

pub struct QueryCanonicalizer;

impl QueryCanonicalizer {
    pub fn canonicalize_request(
        request: AuthoredQueryBundleRequest,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        let (query, result_shape, bindings) = request.into_parts();
        Self::canonicalize_bundle(query, result_shape, bindings)
    }

    pub fn canonicalize_bundle(
        query: RawAuthoredQuery,
        result_shape: RawAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        enforce_admitted_authored_boundary(&query, &result_shape)?;

        let mut warnings = Vec::new();
        let mut events = Vec::new();
        let mut counters = CanonicalizationCounters {
            raw_clause_count: query.projection().len()
                + query.predicates().len()
                + query.ordering().len()
                + query.traversal().len(),
            result_shape_field_count: result_shape.fields().len(),
            binding_descriptor_count: bindings.identity().len() + bindings.non_identity().len(),
            ..CanonicalizationCounters::default()
        };

        if !family_matches_query(&query.family(), &result_shape.family()) {
            return Err(QueryCanonicalizationError::QueryShapeFamilyMismatch {
                query_family: query.family(),
                result_shape_family: result_shape.family(),
            });
        }

        let projection = canonicalize_projection(
            query.projection(),
            &mut warnings,
            &mut events,
            &mut counters,
        );
        let predicates = canonicalize_predicates(query.predicates());
        let ordering = canonicalize_ordering(query.ordering());
        let traversal =
            canonicalize_traversal(query.traversal(), &mut warnings, &mut events, &mut counters);
        let identity_bindings = canonicalize_bindings(
            bindings.identity(),
            bindings.non_identity(),
            &mut warnings,
            &mut events,
            &mut counters,
        )?;

        let canonical_query = build_query_artifact(
            query.family(),
            query.root().clone(),
            projection,
            predicates,
            ordering,
            traversal,
            identity_bindings,
        );

        let canonical_result_shape = build_result_shape_artifact(
            result_shape.family(),
            result_shape.fields(),
            &query.projection_field_set(),
            &mut warnings,
            &mut events,
            &mut counters,
        )?;

        validate_canonical_ordering(&canonical_query, &canonical_result_shape)?;
        validate_digest_basis_consistency(&canonical_query, &canonical_result_shape)?;

        counters.normalized_clause_count = canonical_query.projection.len()
            + canonical_query.predicates.len()
            + canonical_query.ordering.len()
            + canonical_query.traversal.len();
        counters.projection_entry_count = canonical_query.projection.len();
        counters.traversal_clause_count = canonical_query.traversal.len();
        counters.canonicalization_warning_count = warnings.len();

        events.push(NormalizationEvent::CompatibilityEstablished);
        let identity_freeze = IdentityFreezeEvidence {
            query_digest: canonical_query.digest().as_str().to_string(),
            result_shape_digest: canonical_result_shape.digest().as_str().to_string(),
        };
        events.push(NormalizationEvent::IdentityFrozen {
            query_digest: identity_freeze.query_digest.clone(),
            result_shape_digest: identity_freeze.result_shape_digest.clone(),
        });

        Ok(CanonicalQueryBundle {
            report: CanonicalizationReport::new(
                warnings,
                events,
                CompatibilityEvidence::Compatible,
                canonical_query.projection.len(),
                canonical_query.traversal.len(),
                canonical_result_shape.fields.len(),
                identity_freeze,
            ),
            query: canonical_query,
            result_shape: canonical_result_shape,
            counters,
        })
    }
}

fn validate_canonical_ordering(
    query: &super::artifacts::CanonicalQueryArtifact,
    result_shape: &super::artifacts::CanonicalResultShapeArtifact,
) -> Result<(), QueryCanonicalizationError> {
    if !query
        .projection()
        .windows(2)
        .all(|window| window[0] <= window[1])
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_projection",
        });
    }

    if !query
        .predicates()
        .windows(2)
        .all(|window| window[0] <= window[1])
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_predicates",
        });
    }

    if !query
        .ordering()
        .windows(2)
        .all(|window| window[0] <= window[1])
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_ordering",
        });
    }

    if !query
        .traversal()
        .windows(2)
        .all(|window| window[0] <= window[1])
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_traversal",
        });
    }

    if !query
        .identity_bindings()
        .windows(2)
        .all(|window| window[0].slot() <= window[1].slot())
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_identity_bindings",
        });
    }

    if !result_shape
        .fields()
        .windows(2)
        .all(|window| window[0] <= window[1])
    {
        return Err(QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "result_shape_fields",
        });
    }

    Ok(())
}

fn validate_digest_basis_consistency(
    query: &super::artifacts::CanonicalQueryArtifact,
    result_shape: &super::artifacts::CanonicalResultShapeArtifact,
) -> Result<(), QueryCanonicalizationError> {
    let mut query_digest_parts = vec![
        format!("family:{:?}", query.family()),
        format!("root:{}", query.root().as_str()),
    ];
    query_digest_parts.extend(query.projection().iter().map(|entry| entry.digest_part()));
    query_digest_parts.extend(query.predicates().iter().map(|entry| entry.digest_part()));
    query_digest_parts.extend(query.ordering().iter().map(|entry| entry.digest_part()));
    query_digest_parts.extend(query.traversal().iter().map(|entry| entry.digest_part()));
    query_digest_parts.extend(query.identity_bindings().iter().map(|binding| {
        format!(
            "binding:{}:{:?}",
            binding.slot().as_str(),
            binding.subject()
        )
    }));
    let expected_query_digest =
        crate::identity::CanonicalQueryDigest::from_parts(&query_digest_parts);
    if &expected_query_digest != query.digest() {
        return Err(QueryCanonicalizationError::DigestBasisInconsistency { artifact: "query" });
    }

    let expected_result_shape_digest = super::result_shape_artifact::derive_result_shape_digest(
        result_shape.family(),
        result_shape.fields(),
    );
    if &expected_result_shape_digest != result_shape.digest() {
        return Err(QueryCanonicalizationError::DigestBasisInconsistency {
            artifact: "result_shape",
        });
    }

    Ok(())
}

pub fn canonicalize_request(
    request: AuthoredQueryBundleRequest,
) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
    QueryCanonicalizer::canonicalize_request(request)
}
