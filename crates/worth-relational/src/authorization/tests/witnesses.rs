use crate::tests::support::{
    aspect_field_locator, aspect_key, create_entity, create_relation, field_key,
    runtime_with_test_schema,
};

use super::{forward_traversal, ENTITY_KIND};
use crate::authorization::{
    RelationalAuthorizationFieldComparison, RelationalAuthorizationFieldConstraint,
    RelationalAuthorizationFieldOperand, RelationalAuthorizationObservationPlan,
    RelationalAuthorizationPathPlan,
};

#[test]
fn field_comparison_cannot_join_values_from_different_path_witnesses() {
    let mut runtime = runtime_with_test_schema();
    let principal = create_entity(&mut runtime, "witness-principal");
    let left_a = create_entity(&mut runtime, "A");
    let right_b = create_entity(&mut runtime, "B");
    let left_b = create_entity(&mut runtime, "B");
    let right_a = create_entity(&mut runtime, "A");
    let scope = create_entity(&mut runtime, "witness-scope");
    connect_branch(&mut runtime, principal, left_a, right_b, scope, "first");
    connect_branch(&mut runtime, principal, left_b, right_a, scope, "second");

    let evidence = observe_same_witness_names(&mut runtime, principal, scope);
    assert!(
        !evidence.paths()[0].matched(),
        "cross-branch A/A and B/B values must not form a synthetic witness"
    );

    let matching_left = create_entity(&mut runtime, "same");
    let matching_right = create_entity(&mut runtime, "same");
    connect_branch(
        &mut runtime,
        principal,
        matching_left,
        matching_right,
        scope,
        "matching",
    );
    let evidence = observe_same_witness_names(&mut runtime, principal, scope);
    assert!(evidence.paths()[0].matched());
}

fn observe_same_witness_names(
    runtime: &mut crate::runtime::RelationalRuntime,
    principal: crate::identity::data::EntityId,
    scope: crate::identity::data::EntityId,
) -> crate::authorization::RelationalAuthorizationObservationEvidence {
    let name_field = aspect_field_locator(aspect_key("name"), field_key("name"));
    let path = RelationalAuthorizationPathPlan::new(
        [
            forward_traversal(),
            forward_traversal(),
            forward_traversal(),
        ],
        [],
    )
    .with_field_constraints([RelationalAuthorizationFieldConstraint::new(
        RelationalAuthorizationFieldOperand::new(1, ENTITY_KIND, name_field.clone()),
        RelationalAuthorizationFieldComparison::Equal,
        RelationalAuthorizationFieldOperand::new(2, ENTITY_KIND, name_field),
    )]);
    let plan = RelationalAuthorizationObservationPlan::try_new(
        runtime.visibility_authority().snapshot(),
        principal,
        scope,
        ENTITY_KIND,
        ENTITY_KIND,
        [path],
        [],
    )
    .expect("same-witness comparison plan is structurally valid");
    runtime
        .observe_authorization(plan)
        .expect("same-witness graph observation")
}

fn connect_branch(
    runtime: &mut crate::runtime::RelationalRuntime,
    principal: crate::identity::data::EntityId,
    left: crate::identity::data::EntityId,
    right: crate::identity::data::EntityId,
    scope: crate::identity::data::EntityId,
    label: &str,
) {
    create_relation(runtime, principal, left, &format!("{label}-principal-left"));
    create_relation(runtime, left, right, &format!("{label}-left-right"));
    create_relation(runtime, right, scope, &format!("{label}-right-scope"));
}
