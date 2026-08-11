use crate::authorized_projection::PolicyInfluenceSet;
use crate::policy_narrowing::{narrow_policy_query, optimizer_input_from_narrowed_policy_query};

use super::{
    admitted, canonical_query, manager_relationship_proof, mask_snapshot, native_field_pair,
    native_field_pairs, secret_salary_key,
};

#[test]
fn optimizer_input_is_derived_from_narrowed_artifact_only() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("narrowing should admit before optimizer input");

    let optimizer = optimizer_input_from_narrowed_policy_query(&narrowed);

    assert_eq!(
        optimizer.source_narrowed_artifact_digest(),
        narrowed.digest()
    );
    assert_eq!(
        native_field_pairs(optimizer.visible_field_paths()),
        vec![
            native_field_pair("identity", "id"),
            native_field_pair("profile", "display_name")
        ]
    );
    assert!(!native_field_pairs(optimizer.visible_field_paths())
        .iter()
        .any(|field| field == &native_field_pair("secret", "salary")));
    assert_eq!(
        optimizer.authorized_projection_digest(),
        narrowed.authorized_projection().identity().as_str()
    );
}
