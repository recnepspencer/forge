use bank_domain::{
    estate::{
        DeathNoticeStatus, EstateCaseStatus, EstateWorkflowStage, LegalAuthorityKind,
        MandatoryReviewKind, MandatoryReviewStatus,
    },
    model::EmployeeRole,
};
use bank_server::{queries, BankApplicationQueryDenial, BankReadControls};
use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryAdmissionDenialKind;
use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryBasisPosture;

use super::estate_fixture::estate_read_world;
use crate::support::request_scope;

#[test]
fn estate_specialist_reads_the_complete_installed_overview_without_fallback() {
    let fixture = estate_read_world("estate-specialist-overview");
    let specialist = fixture.authenticate(1);
    let result = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&specialist)
        .controls(controls())
        .execute()
        .expect("an assigned estate specialist should read the estate overview");

    let overview = &result.rows()[0];
    assert_eq!(overview.id(), fixture.estate);
    assert_eq!(overview.stage(), EstateWorkflowStage::Administration);
    assert_eq!(overview.status(), EstateCaseStatus::Open);
    assert_eq!(overview.branch(), fixture.branch);
    assert_eq!(overview.account().id(), fixture.account);
    assert_eq!(overview.deceased(), fixture.deceased);
    assert_eq!(
        overview.death_notice().status(),
        DeathNoticeStatus::Verified
    );
    assert_eq!(overview.executors(), [fixture.executor]);
    assert_eq!(overview.beneficiaries(), [fixture.executor]);
    assert_eq!(overview.assignments().len(), 1);
    assert_eq!(overview.assignments()[0].assignment(), fixture.assignment);
    assert_eq!(overview.assignments()[0].principal(), fixture.specialist);
    assert_eq!(
        overview.assignments()[0].role(),
        EmployeeRole::EstateSpecialist
    );
    assert_eq!(overview.legal_authorities()[0].id(), fixture.authority);
    assert_eq!(
        overview.legal_authorities()[0].kind(),
        LegalAuthorityKind::CourtAppointment
    );
    assert!(overview.legal_authorities()[0].recognized());
    assert_eq!(overview.reviews()[0].id(), fixture.review);
    assert_eq!(
        overview.reviews()[0].kind(),
        MandatoryReviewKind::EstateRelease
    );
    assert_eq!(
        overview.reviews()[0].status(),
        MandatoryReviewStatus::Completed
    );
    assert_eq!(overview.reviews()[0].reviewer(), Some(fixture.specialist));
    assert_eq!(result.receipt().fallback_count(), 0);
    assert_eq!(result.receipt().per_result_neighbor_lookup_count(), 0);
    assert!(
        result
            .receipt()
            .graph_read_plan()
            .cost_estimate()
            .supported()
            .index_bytes()
            > 5_120
    );
    assert_eq!(
        result
            .receipt()
            .graph_read_plan()
            .budget_check()
            .max_inline_index_bytes(),
        32_768
    );
}

#[test]
fn executor_uses_the_same_query_and_scope_authority() {
    let fixture = estate_read_world("estate-executor-overview");
    let executor = fixture.authenticate(2);
    let result = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&executor)
        .controls(controls())
        .execute()
        .expect("an estate executor should read the same installed overview");

    assert_eq!(result.rows()[0].id(), fixture.estate);
}

#[test]
fn estate_preview_preserves_canonical_query_meaning_and_releases_authority() {
    let fixture = estate_read_world("estate-preview-overview");
    let specialist = fixture.authenticate(1);
    let preview_request = request_scope();
    let session = fixture
        .world
        .runtime
        .open_preview(&preview_request)
        .expect("the installed bank runtime should open a Query-owned preview session");
    let session_identity = session.identity().clone();
    let preview = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&specialist)
        .controls(controls())
        .preview(&session)
        .expect("the estate query should execute through the admitted preview basis");
    let current = fixture
        .world
        .runtime
        .query(queries::estate_case(fixture.estate))
        .as_principal(&specialist)
        .controls(controls())
        .execute()
        .expect("the same estate query should execute at the current basis");

    assert_eq!(preview.rows(), current.rows());
    assert_eq!(
        preview.receipt().query_identity(),
        current.receipt().query_identity()
    );
    assert_eq!(
        preview.receipt().basis_posture(),
        WorthQueryApplicationQueryBasisPosture::Preview
    );
    assert_eq!(preview.receipt().fallback_count(), 0);
    assert_eq!(preview.receipt().per_result_neighbor_lookup_count(), 0);
    assert!(preview.receipt().basis_released());
    let terminal = preview.receipt().read_completion();
    assert_eq!(
        terminal.basis_identity(),
        preview.receipt().basis_identity()
    );
    assert!(terminal.basis_release().released());
    assert_eq!(terminal.release().released_reservation_count(), 1);

    let discard = session
        .discard()
        .expect("the Query-owned preview session should discard cleanly");
    assert_eq!(discard.identity(), &session_identity);
    assert!(discard.discarded());
}

#[test]
fn incomplete_governance_context_fails_closed_before_governance_admission() {
    let fixture = estate_read_world("estate-governance-boundary");
    let specialist = fixture.authenticate(1);
    let denial = fixture
        .world
        .runtime
        .query(queries::estate_governance_context(fixture.estate))
        .as_principal(&specialist)
        .controls(controls())
        .execute();

    let Err(denial) = denial else {
        panic!("the governance context unexpectedly executed")
    };
    match denial {
        BankApplicationQueryDenial::Admission(denial) => assert_eq!(
            denial.kind(),
            WorthQueryApplicationQueryAdmissionDenialKind::DisclosureContractInvalid,
            "{denial:#?}"
        ),
        denial => panic!("unexpected governance boundary denial: {denial:#?}"),
    }
}

fn controls() -> BankReadControls {
    BankReadControls::current(request_scope(), 4, 20_000).unwrap()
}
