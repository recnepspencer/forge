use crate::authorized_projection::PolicyInfluenceSet;
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};

use super::{
    admitted, canonical_query, mask_snapshot, native_field_pair, native_field_pairs,
    secret_salary_key,
};

#[test]
fn narrowed_artifact_binds_policy_tenant_projection_and_proof() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::direct_edge(
            "manager",
            admitted.bundle().policy_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        descriptors,
    )
    .expect("Phase 2 narrowing should admit bounded direct proof");

    assert_eq!(
        narrowed.canonical_query_digest(),
        canonical.query().digest().as_str()
    );
    assert_eq!(
        narrowed.authorized_projection().visible_field_paths().len(),
        2
    );
    assert_eq!(
        native_field_pairs(
            narrowed
                .authorized_projection()
                .masked_projection()
                .masked_field_paths()
        ),
        vec![native_field_pair("secret", "salary")]
    );
    assert_eq!(narrowed.relationship_proof().descriptor_count(), 1);
    assert_eq!(narrowed.counters().narrowed_artifact_count(), 1);
    assert_eq!(
        narrowed.counters().relationship_proof().truth_touch_count(),
        0
    );
    assert!(!narrowed.digest().is_empty());
}
