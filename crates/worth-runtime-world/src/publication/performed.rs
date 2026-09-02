use worth_proof::AuthorityWitness;

use crate::branch::ProductBranchObservation;
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::CompositePublicationAttemptIdentity;

worth_proof::authority_marker!(pub(crate) CompositePublicationAuthorityMarker);

/// Linear proof that one immutable composite commit won the exact product
/// compare-and-publish transition.
#[must_use = "a performed publication must be handed to the product owner"]
pub struct PerformedCompositePublication {
    commit: CompositeRuntimeWorldCommit,
    product_head: ProductBranchObservation,
    attempt_identity: CompositePublicationAttemptIdentity,
    _authority: AuthorityWitness<CompositePublicationAuthorityMarker>,
}

impl std::fmt::Debug for PerformedCompositePublication {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PerformedCompositePublication")
            .field("commit", &self.commit.identity())
            .field("product_head", &self.product_head)
            .field("attempt_identity", &self.attempt_identity)
            .finish_non_exhaustive()
    }
}

impl PerformedCompositePublication {
    pub(crate) fn new(
        commit: CompositeRuntimeWorldCommit,
        product_head: ProductBranchObservation,
        attempt_identity: CompositePublicationAttemptIdentity,
    ) -> Self {
        Self {
            commit,
            product_head,
            attempt_identity,
            _authority: AuthorityWitness::from_authority_marker(
                CompositePublicationAuthorityMarker::seal(),
            ),
        }
    }

    pub fn commit(&self) -> &CompositeRuntimeWorldCommit {
        &self.commit
    }

    pub fn product_head(&self) -> &ProductBranchObservation {
        &self.product_head
    }

    pub fn attempt_identity(&self) -> &CompositePublicationAttemptIdentity {
        &self.attempt_identity
    }
}
