use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet};
use crate::policy_narrowing::{PolicyNarrowingFailureClass, PolicyNarrowingWorkBudget};
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};

use super::super::super::lowering::narrow_policy_query_with_budget;
use super::{admitted, canonical_query, mask_snapshot, secret_salary_key};

#[test]
fn relationship_proof_host_callback_is_forbidden_before_truth_touch() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query_with_budget(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![
                RelationshipProofDescriptor::direct_edge(
                    "manager",
                    admitted.bundle().policy_digest(),
                ),
                RelationshipProofDescriptor::host_callback_forbidden("authz"),
            ],
            RelationshipProofBudget::bounded(2, 1),
        ),
        PolicyNarrowingWorkBudget::bounded(16, 16, 16, 2, 1, 8, 64),
    )
    .expect_err("host callbacks must not be relationship proof authority");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::RelationshipProofDenied(
            crate::relationship_proof::RelationshipProofFailureClass::HostCallbackForbidden
        )
    );
    assert_eq!(
        error
            .counters()
            .relationship_proof()
            .forbidden_host_callback_proof_count(),
        1
    );
    assert_eq!(error.counters().relationship_proof().truth_touch_count(), 0);
}
