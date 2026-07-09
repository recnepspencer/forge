use crate::authoring::QueryFamily;
use crate::binding::IdentityBindingDescriptor;
use crate::canonicalization::CanonicalQueryArtifact;
use crate::identity::{CanonicalQueryDigest, SchemaBasisDigest, ValidatedQueryDigest};

use super::{
    ValidatedOrderingSet, ValidatedPredicateSet, ValidatedProjectionEntry, ValidatedTraversalEntry,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedQueryArtifact {
    digest: ValidatedQueryDigest,
    canonical_query_digest: CanonicalQueryDigest,
    schema_basis: SchemaBasisDigest,
    family: QueryFamily,
    identity_bindings: Vec<IdentityBindingDescriptor>,
    projection: Vec<ValidatedProjectionEntry>,
    traversal: Vec<ValidatedTraversalEntry>,
    predicates: ValidatedPredicateSet,
    ordering: ValidatedOrderingSet,
}

impl ValidatedQueryArtifact {
    pub fn digest(&self) -> &ValidatedQueryDigest {
        &self.digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn family(&self) -> &QueryFamily {
        &self.family
    }

    pub fn projection(&self) -> &[ValidatedProjectionEntry] {
        &self.projection
    }

    pub fn identity_bindings(&self) -> &[IdentityBindingDescriptor] {
        &self.identity_bindings
    }

    pub fn traversal(&self) -> &[ValidatedTraversalEntry] {
        &self.traversal
    }

    pub fn predicates(&self) -> &ValidatedPredicateSet {
        &self.predicates
    }

    pub fn ordering(&self) -> &ValidatedOrderingSet {
        &self.ordering
    }
}

pub(crate) fn build_validated_query_artifact(
    canonical_query: &CanonicalQueryArtifact,
    schema_basis: &SchemaBasisDigest,
    projection: Vec<ValidatedProjectionEntry>,
    traversal: Vec<ValidatedTraversalEntry>,
    predicates: ValidatedPredicateSet,
    ordering: ValidatedOrderingSet,
) -> ValidatedQueryArtifact {
    let mut parts = vec![
        format!("family:{:?}", canonical_query.family()),
        format!("root:{}", canonical_query.root().as_str()),
        format!("schema_basis:{}", schema_basis.as_str()),
    ];
    parts.extend(canonical_query.identity_bindings().iter().map(|binding| {
        format!(
            "binding:{}:{:?}",
            binding.slot().as_str(),
            binding.subject()
        )
    }));
    parts.extend(projection.iter().map(ValidatedProjectionEntry::digest_part));
    parts.extend(traversal.iter().map(ValidatedTraversalEntry::digest_part));
    parts.extend(predicates.digest_parts());
    parts.extend(ordering.digest_parts());

    ValidatedQueryArtifact {
        digest: ValidatedQueryDigest::from_parts(&parts),
        canonical_query_digest: canonical_query.digest().clone(),
        schema_basis: schema_basis.clone(),
        family: canonical_query.family().clone(),
        identity_bindings: canonical_query.identity_bindings().to_vec(),
        projection,
        traversal,
        predicates,
        ordering,
    }
}
