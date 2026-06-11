use crate::authoring::{QueryFamily, RawAuthoredQuery, RawAuthoredResultShape};
use crate::binding::QueryBindingDescriptor;
use crate::canonicalization::CanonicalQueryBundle;
use crate::composition::counters::CompositionCounters;
use crate::composition::digests::ScopeLineageDigest;
use crate::composition::errors::QueryCompositionError;
use crate::composition::ScopeFamily;

use super::descriptor::QueryScopeDescriptor;
use super::evidence::BasisScopeEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpandedScopeArtifact {
    query_family: QueryFamily,
    scope_lineage_digest: ScopeLineageDigest,
    basis_evidence: Option<BasisScopeEvidence>,
    counters: CompositionCounters,
}

impl ExpandedScopeArtifact {
    pub fn query_family(&self) -> QueryFamily {
        self.query_family.clone()
    }

    pub fn scope_lineage_digest(&self) -> &ScopeLineageDigest {
        &self.scope_lineage_digest
    }

    pub fn basis_evidence(&self) -> Option<&BasisScopeEvidence> {
        self.basis_evidence.as_ref()
    }

    pub fn counters(&self) -> &CompositionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScopeExpansionResult {
    pub(crate) query: RawAuthoredQuery,
    pub(crate) result_shape: RawAuthoredResultShape,
    pub(crate) bindings: QueryBindingDescriptor,
    pub(crate) artifact: ExpandedScopeArtifact,
}

pub(crate) fn expand_scopes(
    mut query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
    bindings: QueryBindingDescriptor,
    scopes: &[QueryScopeDescriptor],
) -> Result<ScopeExpansionResult, QueryCompositionError> {
    let mut width = 0usize;
    let mut basis_evidence = None;
    let mut lineage_parts = Vec::with_capacity(scopes.len() * 3);

    for scope in scopes {
        lineage_parts.push(format!(
            "scope:{}:{}",
            scope.family().as_str(),
            scope.label()
        ));
        match scope {
            QueryScopeDescriptor::Predicate(descriptor) => {
                width += descriptor.predicates().len();
                for predicate in descriptor.predicates() {
                    query = query.with_predicate(predicate.clone());
                }
            }
            QueryScopeDescriptor::Ordering(descriptor) => {
                width += descriptor.ordering().len();
                for ordering in descriptor.ordering() {
                    query = query.with_ordering(ordering.clone());
                }
            }
            QueryScopeDescriptor::Projection(descriptor) => {
                width += descriptor.projection().len();
                for projection in descriptor.projection() {
                    query = query.with_projection(projection.clone());
                }
            }
            QueryScopeDescriptor::TraversalBound(descriptor) => {
                width += descriptor.traversal().len();
                query = apply_traversal_scope(query, descriptor, scopes.len(), width)?;
            }
            QueryScopeDescriptor::BasisAware(descriptor) => {
                width += 1;
                if basis_evidence.is_some() {
                    return Err(QueryCompositionError::duplicate_basis_aware_scope(
                        CompositionCounters::for_scope_expansion(scopes.len(), width),
                        "basis-aware scope cannot be applied more than once in a single composition lane",
                    ));
                }
                lineage_parts.push(format!(
                    "basis:{}:{}",
                    descriptor.evidence().basis_family().as_str(),
                    descriptor.evidence().basis_digest()
                ));
                basis_evidence = Some(descriptor.evidence().clone());
            }
            #[cfg(test)]
            QueryScopeDescriptor::Unsupported(_) => {
                return Err(QueryCompositionError::unsupported_scope(
                    ScopeFamily::UnsupportedScope,
                    CompositionCounters::for_scope_expansion(scopes.len(), width),
                    "unsupported scope family remains denied in Phase 1",
                ));
            }
        }
    }

    let counters = CompositionCounters::for_scope_expansion(scopes.len(), width);
    let scope_lineage_digest = ScopeLineageDigest::from_parts(&lineage_parts);
    let query_family = query.family();

    Ok(ScopeExpansionResult {
        query,
        result_shape,
        bindings,
        artifact: ExpandedScopeArtifact {
            query_family,
            scope_lineage_digest,
            basis_evidence,
            counters,
        },
    })
}

fn apply_traversal_scope(
    mut query: RawAuthoredQuery,
    descriptor: &super::descriptor::TraversalBoundScopeDescriptor,
    scope_count: usize,
    width: usize,
) -> Result<RawAuthoredQuery, QueryCompositionError> {
    for traversal in descriptor.traversal() {
        if traversal.depth() > descriptor.max_depth() {
            return Err(QueryCompositionError::invalid_scope(
                ScopeFamily::TraversalBoundScope,
                CompositionCounters::for_scope_expansion(scope_count, width),
                format!(
                    "traversal scope '{}' declared max depth {} but attempted depth {} on relation '{}'",
                    descriptor.label(),
                    descriptor.max_depth(),
                    traversal.depth(),
                    traversal.relation()
                ),
            ));
        }
        query = query.with_traversal(traversal.clone());
    }

    Ok(query)
}

pub(crate) fn validate_basis_evidence_for_canonical_query(
    basis_evidence: Option<&BasisScopeEvidence>,
    canonical: &CanonicalQueryBundle,
    family: ScopeFamily,
    counters: CompositionCounters,
) -> Result<(), QueryCompositionError> {
    let Some(evidence) = basis_evidence else {
        return Ok(());
    };

    if evidence.expected_canonical_query_digest() != canonical.query().digest().as_str() {
        return Err(QueryCompositionError::basis_query_mismatch(
            family,
            counters,
            evidence.expected_canonical_query_digest(),
            canonical.query().digest().as_str(),
        ));
    }

    Ok(())
}
