use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::time::Duration;

use worth_query_declaration::facade::{
    application_query::ApplicationQueryParameterSet, application_schema::TypedApplicationValue,
};
use worth_relational::facade::{
    history::BranchId,
    transactions::{
        AspectFieldPatch, EntityMutationIntent, MutationIntent, TransactionOptions,
        UpdateEntityFieldsIntent, WorkerIntentBatch,
    },
};
use worth_runtime_bridge::facade::{TruthBranchIdentity, TruthCommitIdentity};

use super::super::fixture::{
    installed_authorization_world, live_scope, status_parameter, AccountLabel, AccountStatus,
};
use super::{current_controls, installed_query};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryBasisPosture,
    WorthQueryApplicationQueryConsistency, WorthQueryApplicationQueryControls,
    WorthQueryApplicationQueryFreshness, WorthQueryPrincipalResolutionMode,
};
use worth_query_admission::facade::application_query::WorthQueryApplicationQueryLane;

#[test]
fn historical_and_current_share_meaning_but_bind_distinct_truth() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let historical_head = branch_head(&world, "main");
    change_account_label(&world, account.entity_id(), "changed", "main");
    let historical_basis = world
        .application
        .admit_application_historical_basis(
            crate::domain_computation::primary_graph::WorthQueryApplicationHistoricalRead::at_commit(
                TruthBranchIdentity::from_relational_branch_id("main"),
                TruthCommitIdentity::from_relational_commit_id(historical_head.commit_id.0),
            ),
            &request,
        )
        .unwrap();
    assert_eq!(historical_basis.version_id(), historical_head.version_id);
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let historical_plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::historical(
                historical_basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    let historical = world
        .application
        .execute_application_query_historical(historical_plan)
        .unwrap();
    let current_plan = world
        .application
        .admit_application_query(&query, &access, parameters(), current_controls(&request))
        .unwrap();
    let current = world
        .application
        .execute_application_query_one_shot(current_plan)
        .unwrap();

    assert_eq!(historical.rows()[0].label(), "primary");
    assert_eq!(current.rows()[0].label(), "changed");
    assert_eq!(
        historical.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Historical
    );
    assert_eq!(
        historical.receipt().lane(),
        WorthQueryApplicationQueryLane::Historical
    );
    assert_eq!(
        historical.receipt().consistency(),
        WorthQueryApplicationQueryConsistency::HistoricalSnapshot
    );
    assert_eq!(
        historical.receipt().freshness(),
        WorthQueryApplicationQueryFreshness::Historical
    );
    assert!(historical.receipt().basis_released());
}

#[test]
fn application_preview_retains_its_declared_snapshot_after_main_advances() {
    let world = installed_authorization_world(true);
    let request = live_scope();
    let (principal, account) = principal_and_account(&world, &request);
    let session = world
        .application
        .open_application_preview_session(&request)
        .unwrap();
    change_account_label(&world, account.entity_id(), "changed-after-preview", "main");
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();
    let basis = world
        .application
        .admit_application_preview_basis(&session, &request)
        .unwrap();
    assert_eq!(observer.observe().active(), before.active() + 1);
    let query = installed_query(&world);
    let access = WorthQueryApplicationQueryAccessContext::new(&principal, &account);
    let plan = world
        .application
        .admit_application_query(
            &query,
            &access,
            parameters(),
            WorthQueryApplicationQueryControls::preview(
                basis,
                NonZeroUsize::new(10).unwrap(),
                NonZeroUsize::new(10_000).unwrap(),
                &request,
            ),
        )
        .unwrap();
    let result = world
        .application
        .execute_application_query_preview(plan)
        .unwrap();

    assert_eq!(result.rows()[0].label(), "primary");
    assert_eq!(
        result.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Preview
    );
    assert_eq!(
        result.receipt().lane(),
        WorthQueryApplicationQueryLane::Preview
    );
    assert_eq!(
        result.receipt().consistency(),
        WorthQueryApplicationQueryConsistency::PreviewSnapshot
    );
    assert_eq!(
        result.receipt().freshness(),
        WorthQueryApplicationQueryFreshness::PreviewAtAdmission
    );
    assert_eq!(observer.observe().active(), before.active());
    let current_plan = world
        .application
        .admit_application_query(&query, &access, parameters(), current_controls(&request))
        .unwrap();
    let current = world
        .application
        .execute_application_query_one_shot(current_plan)
        .unwrap();
    assert_eq!(current.rows()[0].label(), "changed-after-preview");
    assert_eq!(
        result.receipt().query_identity(),
        current.receipt().query_identity()
    );
    assert!(session.discard().unwrap().discarded());
}

pub(super) fn principal_and_account(
    world: &super::super::fixture::AuthorizationWorld,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
) -> (
    crate::domain_computation::primary_graph::WorthQueryAuthenticatedPrincipal<
        super::super::fixture::IdentityExecutionSchema,
        super::super::fixture::Principal,
        u64,
    >,
    crate::domain_computation::primary_graph::WorthQueryApplicationEntityIdentity<
        super::super::fixture::IdentityExecutionSchema,
        super::super::fixture::Account,
    >,
) {
    let external = world.authenticate("alice", Duration::from_secs(60), request);
    let principal = world
        .application
        .resolve_authenticated_principal(
            &world.binding,
            external,
            request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    let account = world
        .application
        .resolve_entity(
            AccountStatus::reference(),
            "open".to_string(),
            request,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .unwrap();
    (principal, account)
}

pub(super) fn parameters(
) -> ApplicationQueryParameterSet<super::super::fixture::AccountSummaryQuery> {
    ApplicationQueryParameterSet::new().bind(status_parameter(), "open".to_string())
}

pub(super) fn branch_head(
    world: &super::super::fixture::AuthorizationWorld,
    branch: &str,
) -> worth_relational::facade::history::CommitReference {
    let graph = world.application.runtime.primary_graph().unwrap();
    graph.integration_handle().with_runtime(|runtime| {
        runtime
            .history()
            .branch_head(&BranchId(branch.to_string()))
            .unwrap()
            .clone()
    })
}

fn change_account_label(
    world: &super::super::fixture::AuthorizationWorld,
    account: worth_relational::facade::identity::EntityId,
    label: &str,
    branch: &str,
) {
    let graph = world.application.runtime.primary_graph().unwrap();
    let locator = graph
        .layout
        .field_locator(
            AccountLabel::reference().entity(),
            AccountLabel::reference().aspect(),
            AccountLabel::reference().field(),
        )
        .unwrap()
        .clone();
    let handle = graph.integration_handle();
    handle.with_runtime_mut(|runtime| {
        let fields = AspectFieldPatch::from(BTreeMap::from([(
            locator,
            label.to_string().into_foundational_value(),
        )]));
        let mut transaction = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId(branch.to_string())),
            ..TransactionOptions::default()
        });
        transaction.push_batch(WorkerIntentBatch::new("query-lane-label").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: account,
                    fields,
                },
            )),
        ));
        transaction.commit().unwrap();
        let branch = runtime.config().history.main_branch.clone();
        handle
            .ensure_primary_indexes_current(runtime, &branch)
            .unwrap();
    });
}
