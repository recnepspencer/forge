use crate::authorization::{
    RelationalAuthorizationEntityAnchor, RelationalAuthorizationObservationFreshness,
    RelationalAuthorizationObservationPlan, RelationalAuthorizationPathPlan,
};
use crate::facade::history::BranchId;
use crate::tests::support::{
    create_entity, create_relation, delete_relation_on_branch, runtime_with_test_schema,
};

use super::{forward_traversal, ENTITY_KIND};

#[test]
fn exact_next_hop_anchor_ignores_same_source_population() {
    let mut runtime = runtime_with_test_schema();
    let principal = create_entity(&mut runtime, "anchored-principal");
    let anchor = create_entity(&mut runtime, "anchored-role");
    let scope = create_entity(&mut runtime, "anchored-scope");
    create_relation(&mut runtime, principal, anchor, "anchored-principal-role");
    create_relation(&mut runtime, anchor, scope, "anchored-role-scope");

    let baseline = observe_anchored_path(&mut runtime, principal, anchor, scope);
    for ordinal in 0..64 {
        let unrelated = create_entity(&mut runtime, &format!("same-source-grant-{ordinal}"));
        create_relation(
            &mut runtime,
            principal,
            unrelated,
            &format!("same-source-edge-{ordinal}"),
        );
    }
    let populated = observe_anchored_path(&mut runtime, principal, anchor, scope);

    assert_eq!(populated.counters(), baseline.counters());
    assert_eq!(
        populated.paths()[0].witness(),
        baseline.paths()[0].witness()
    );
    assert_eq!(populated.counters().adjacency_edges_inspected, 2);
}

#[test]
fn exact_next_hop_anchor_retains_revocation_currentness() {
    let mut runtime = runtime_with_test_schema();
    let principal = create_entity(&mut runtime, "revoked-anchor-principal");
    let anchor = create_entity(&mut runtime, "revoked-anchor-role");
    let scope = create_entity(&mut runtime, "revoked-anchor-scope");
    let exact = create_relation(&mut runtime, principal, anchor, "revoked-principal-role");
    create_relation(&mut runtime, anchor, scope, "revoked-role-scope");
    let admitted = observe_anchored_path(&mut runtime, principal, anchor, scope);

    let revoked = delete_relation_on_branch(&mut runtime, exact, BranchId("main".to_owned()));

    assert_eq!(
        runtime.compare_authorization_observation(&admitted, revoked.snapshot.clone()),
        RelationalAuthorizationObservationFreshness::Stale,
    );
}

fn observe_anchored_path(
    runtime: &crate::runtime::RelationalRuntime,
    principal: crate::identity::data::EntityId,
    anchor: crate::identity::data::EntityId,
    scope: crate::identity::data::EntityId,
) -> crate::authorization::RelationalAuthorizationObservationEvidence {
    let path = RelationalAuthorizationPathPlan::new([forward_traversal(), forward_traversal()], [])
        .with_entity_anchors([RelationalAuthorizationEntityAnchor::new(
            1,
            ENTITY_KIND,
            anchor,
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
    .unwrap();
    runtime.observe_authorization(plan).unwrap()
}
