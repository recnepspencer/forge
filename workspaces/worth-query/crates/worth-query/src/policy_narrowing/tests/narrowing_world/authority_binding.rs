use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet, PolicyMaskSnapshot};
use crate::policy_narrowing::{narrow_policy_query, PolicyNarrowingFailureClass};
use crate::relationship_proof::RelationshipProofDescriptorSet;

use super::{admitted, canonical_query, secret_salary_key};

#[test]
fn policy_mask_snapshot_must_match_admitted_policy_authority() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted,
        PolicyMaskSnapshot::synthetic_authority(
            "wrong-policy-digest",
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("mask snapshots must be bound to the admitted policy digest");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::PolicyMaskAuthorityMismatch
    );
}
