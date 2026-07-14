use crate::authoring::{QueryFamily, RootEntityKey};
use crate::binding::IdentityBindingDescriptor;
use crate::identity::{CanonicalEquivalence, CanonicalQueryDigest};
use crate::identity_authority::QueryCanonicalAuthority;

use super::entries::{
    CanonicalOrderingEntry, CanonicalPredicateEntry, CanonicalProjectionEntry,
    CanonicalTraversalEntry,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalQueryArtifact {
    pub(crate) digest: CanonicalQueryDigest,
    pub(crate) family: QueryFamily,
    pub(crate) root: RootEntityKey,
    pub(crate) projection: Vec<CanonicalProjectionEntry>,
    pub(crate) predicates: Vec<CanonicalPredicateEntry>,
    pub(crate) ordering: Vec<CanonicalOrderingEntry>,
    pub(crate) traversal: Vec<CanonicalTraversalEntry>,
    pub(crate) identity_bindings: Vec<IdentityBindingDescriptor>,
}

impl CanonicalQueryArtifact {
    pub fn authority(&self) -> QueryCanonicalAuthority {
        QueryCanonicalAuthority::from_query_artifact(&self.digest)
    }

    pub fn digest(&self) -> &CanonicalQueryDigest {
        &self.digest
    }

    pub fn family(&self) -> &QueryFamily {
        &self.family
    }

    pub fn root(&self) -> &RootEntityKey {
        &self.root
    }

    pub fn projection(&self) -> &[CanonicalProjectionEntry] {
        &self.projection
    }

    pub fn predicates(&self) -> &[CanonicalPredicateEntry] {
        &self.predicates
    }

    pub fn ordering(&self) -> &[CanonicalOrderingEntry] {
        &self.ordering
    }

    pub fn traversal(&self) -> &[CanonicalTraversalEntry] {
        &self.traversal
    }

    pub fn identity_bindings(&self) -> &[IdentityBindingDescriptor] {
        &self.identity_bindings
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.family == other.family
            && self.root == other.root
            && self.projection == other.projection
            && self.predicates == other.predicates
            && self.ordering == other.ordering
            && self.traversal == other.traversal
            && self.identity_bindings == other.identity_bindings
            && self.digest == other.digest
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    #[cfg(test)]
    pub(crate) fn reverse_projection_for_test(&mut self) {
        self.projection.reverse();
    }

    #[cfg(test)]
    pub(crate) fn corrupt_digest_for_test(&mut self, marker: &str) {
        self.digest = CanonicalQueryDigest::from_parts(&[marker.to_string()]);
    }
}
