use crate::facade::history::BranchId;
use crate::identity::data::{KindId, PartitionId};
use crate::tests::support::{create_entity, create_relation, delete_relation_on_branch};
use crate::transactions::data::{
    AspectFieldPatch, CreateIntent, EntityReference, MutationIntent, RelationSpec,
    WorkerIntentBatch,
};

use super::{allow_plan, authorization_fixture, role_and_direct_path_plan};
use crate::authorization::RelationalAuthorizationObservationFreshness;

#[test]
fn exact_authorization_observation_stales_after_membership_revocation() {
    let mut fixture = authorization_fixture();
    let admitted_snapshot = fixture.runtime.visibility_authority().snapshot();
    let admitted = fixture
        .runtime
        .observe_authorization(allow_plan(
            admitted_snapshot,
            fixture.principal,
            fixture.scope,
            [],
        ))
        .unwrap();
    let unchanged = fixture.runtime.visibility_authority().snapshot();
    assert_eq!(
        fixture
            .runtime
            .compare_authorization_observation(&admitted, unchanged),
        RelationalAuthorizationObservationFreshness::Fresh
    );

    let revoked = delete_relation_on_branch(
        &mut fixture.runtime,
        fixture.role_scope_relation,
        BranchId("main".to_string()),
    );
    assert_eq!(
        fixture
            .runtime
            .compare_authorization_observation(&admitted, revoked.snapshot.clone()),
        RelationalAuthorizationObservationFreshness::Stale
    );
}

#[test]
fn unrelated_relation_kind_does_not_widen_authorization_causality() {
    let mut fixture = authorization_fixture();
    let admitted_snapshot = fixture.runtime.visibility_authority().snapshot();
    let admitted = fixture
        .runtime
        .observe_authorization(allow_plan(
            admitted_snapshot,
            fixture.principal,
            fixture.scope,
            [],
        ))
        .unwrap();
    let unrelated = create_entity(&mut fixture.runtime, "unrelated-kind-target");
    create_relation_of_kind(
        &mut fixture.runtime,
        fixture.principal,
        unrelated,
        KindId(99),
    );
    let current = fixture.runtime.visibility_authority().snapshot();

    assert_eq!(
        fixture
            .runtime
            .compare_authorization_observation(&admitted, current),
        RelationalAuthorizationObservationFreshness::Fresh
    );
}

#[test]
fn newly_matching_parallel_path_stales_the_exact_observation() {
    let mut fixture = authorization_fixture();
    let admitted_snapshot = fixture.runtime.visibility_authority().snapshot();
    let admitted = fixture
        .runtime
        .observe_authorization(role_and_direct_path_plan(
            admitted_snapshot,
            fixture.principal,
            fixture.scope,
        ))
        .unwrap();
    assert!(!admitted.paths()[1].matched());

    create_relation(
        &mut fixture.runtime,
        fixture.principal,
        fixture.scope,
        "newly-matching-deny",
    );
    let current = fixture.runtime.visibility_authority().snapshot();
    assert_eq!(
        fixture
            .runtime
            .compare_authorization_observation(&admitted, current),
        RelationalAuthorizationObservationFreshness::Stale
    );
}

fn create_relation_of_kind(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    source: crate::identity::data::EntityId,
    target: crate::identity::data::EntityId,
    kind_id: KindId,
) {
    let mut transaction = runtime.begin_transaction(Default::default());
    transaction.push_batch(
        WorkerIntentBatch::new("unrelated-kind").push(MutationIntent::Create(
            CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id,
                client_key: crate::symbols::data::ClientKey::raw("unrelated-kind"),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(target),
                fields: AspectFieldPatch::default(),
            }),
        )),
    );
    transaction.commit().unwrap();
}
