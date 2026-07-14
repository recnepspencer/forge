use super::entity_seeding::SeededEntityState;
use super::seed_catalog::FintechCaseSeed;
use super::{FintechCaseRole, FintechCasebook, FintechWorkflowCase};

pub(super) fn build_workflow_cases(
    case_seeds: &[FintechCaseSeed],
    seeded: &SeededEntityState,
) -> Vec<FintechWorkflowCase> {
    let mut workflow_cases = Vec::new();
    for (idx, seed) in case_seeds.iter().enumerate() {
        workflow_cases.push(FintechWorkflowCase {
            role: seed.role,
            desk: *seeded
                .desk_map
                .get(seed.desk_name)
                .expect("desk should exist for case seed"),
            book: *seeded
                .book_map
                .get(seed.book_name)
                .expect("book should exist for case seed"),
            account: seeded.accounts[idx],
            counterparty: seeded.counterparties[idx],
            trade: seeded.trades[idx],
            instrument: seeded.instruments[idx],
            market_point: seeded.market_points[idx],
            risk_view: seeded.risk_views[idx],
            settlement: seeded.settlements[idx],
            cash_event: seeded.cash_events[idx],
            limit: seeded.limits[idx],
            breach: seeded.breaches[idx],
            audit_record: seeded.audit_records[idx],
        });
    }
    workflow_cases
}

pub(super) fn build_casebook(workflow_cases: &[FintechWorkflowCase]) -> FintechCasebook {
    FintechCasebook {
        baseline_portfolio: find_case(workflow_cases, FintechCaseRole::BaselinePortfolio),
        late_trade_correction: find_case(workflow_cases, FintechCaseRole::LateTradeCorrection),
        intraday_risk: find_case(workflow_cases, FintechCaseRole::IntradayRisk),
        failed_settlement_repair: find_case(
            workflow_cases,
            FintechCaseRole::FailedSettlementRepair,
        ),
    }
}

fn find_case(cases: &[FintechWorkflowCase], role: FintechCaseRole) -> FintechWorkflowCase {
    cases
        .iter()
        .copied()
        .find(|case| case.role == role)
        .expect("workflow case should exist for seeded role")
}
