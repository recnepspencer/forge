use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet};
use crate::policy_narrowing::{narrow_policy_query, PolicyNarrowingFailureClass};
use crate::relationship_proof::RelationshipProofDescriptorSet;

use super::{
    admitted, canonical_with_masked_ordering, canonical_with_masked_predicate, mask_snapshot,
    secret_salary_key,
};

#[test]
fn masked_predicate_denies_before_narrowed_artifact_construction() {
    let canonical = canonical_with_masked_predicate();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("masked predicate must deny before narrowing");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::AuthorizedProjectionDenied(
            crate::authorized_projection::AuthorizedProjectionFailureClass::MaskedPredicateInfluence
        )
    );
    assert_eq!(
        error
            .counters()
            .authorized_projection()
            .hidden_predicate_denial_count(),
        1
    );
}

#[test]
fn masked_ordering_denies_before_optimizer_input_exists() {
    let canonical = canonical_with_masked_ordering();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("non-disclosing ordering still leaks hidden truth");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::AuthorizedProjectionDenied(
            crate::authorized_projection::AuthorizedProjectionFailureClass::MaskedOrderingInfluence
        )
    );
    assert_eq!(
        error
            .counters()
            .authorized_projection()
            .hidden_ordering_denial_count(),
        1
    );
}
