use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet};
use crate::policy_narrowing::{PolicyNarrowingFailureClass, PolicyNarrowingWorkBudget};
use crate::relationship_proof::RelationshipProofDescriptorSet;

use super::super::super::lowering::narrow_policy_query_with_budget;
use super::{admitted, canonical_query, mask_snapshot, secret_salary_key};

#[test]
fn digest_part_budget_denies_before_artifact_construction() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let budget = PolicyNarrowingWorkBudget::bounded(16, 16, 16, 0, 0, 8, 1);

    let error = narrow_policy_query_with_budget(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
        budget,
    )
    .expect_err("declared digest-part budget must be enforced");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::DigestPartBudgetExceeded
    );
}
