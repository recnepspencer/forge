use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet};
use crate::policy_narrowing::narrow_policy_query;

use super::{
    admitted, canonical_query, manager_relationship_proof, mask_snapshot, secret_salary_key,
};

#[test]
fn validation_report_digest_binds_authorized_projection_identity() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let visible = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(&admitted, PolicyAspectMask::allow_all()),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("visible projection should narrow");
    let masked = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("masked projection should narrow");

    assert_ne!(
        visible.validation_report().digest(),
        masked.validation_report().digest()
    );
}
