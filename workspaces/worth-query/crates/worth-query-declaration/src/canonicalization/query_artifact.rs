use crate::authoring::{QueryFamily, RootEntityKey};
use crate::binding::IdentityBindingDescriptor;
use crate::identity::CanonicalQueryDigest;
use crate::identity_authority::QueryCanonicalAuthority;

use super::artifacts::{
    CanonicalOrderingEntry, CanonicalPredicateEntry, CanonicalProjectionEntry,
    CanonicalQueryArtifact, CanonicalTraversalEntry,
};

pub(super) fn build_query_artifact(
    family: QueryFamily,
    root: RootEntityKey,
    projection: Vec<CanonicalProjectionEntry>,
    predicates: Vec<CanonicalPredicateEntry>,
    ordering: Vec<CanonicalOrderingEntry>,
    traversal: Vec<CanonicalTraversalEntry>,
    identity_bindings: Vec<IdentityBindingDescriptor>,
) -> CanonicalQueryArtifact {
    let mut digest_parts = vec![
        format!("family:{family:?}"),
        format!("root:{}", root.as_str()),
    ];
    digest_parts.extend(projection.iter().map(CanonicalProjectionEntry::digest_part));
    digest_parts.extend(predicates.iter().map(CanonicalPredicateEntry::digest_part));
    digest_parts.extend(ordering.iter().map(CanonicalOrderingEntry::digest_part));
    digest_parts.extend(traversal.iter().map(CanonicalTraversalEntry::digest_part));
    digest_parts.extend(identity_bindings.iter().map(|binding| {
        format!(
            "binding:{}:{:?}",
            binding.slot().as_str(),
            binding.subject()
        )
    }));

    let digest = CanonicalQueryDigest::from_parts(&digest_parts);
    CanonicalQueryArtifact {
        authority: QueryCanonicalAuthority::mint(digest.clone()),
        digest,
        family,
        root,
        projection,
        predicates,
        ordering,
        traversal,
        identity_bindings,
    }
}
