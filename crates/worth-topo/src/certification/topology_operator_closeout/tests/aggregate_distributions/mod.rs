use crate::facade::{
    certify_milestone_three_hostile_suite, MilestoneThreeHostileScenario,
    TopologyEditNamingOutcome, TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn hostile_suite_aggregate_distribution_rows_bind_to_scenario_sets() {
    let report = certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.hostile_suite.aggregate_distributions",
    )
    .expect("milestone three hostile suite should certify");

    assert!(report.family_coverage_rows.iter().all(|row| {
        row.scenario_count() == row.scenarios().len()
            && row.scenario_count() > 0
            && row
                .row_digest()
                .starts_with(&format!("family={:?};", row.family()))
            && row.row_digest().contains("scenarios=")
    }));
    let rejection_row = report
        .rejection_distribution_rows
        .iter()
        .find(|row| row.rejection_class() == TopologyEditRejectionClass::InvariantBlocked)
        .expect("invariant-blocked rejection distribution should be emitted");
    assert_eq!(
        rejection_row.scenarios(),
        &[
            MilestoneThreeHostileScenario::BowtieAdjacentRewire,
            MilestoneThreeHostileScenario::BrokenRadialLocalization,
        ]
    );
    assert_eq!(
        rejection_row.row_digest(),
        "rejection_class=InvariantBlocked;case_count=2;scenarios=BowtieAdjacentRewire|BrokenRadialLocalization"
    );
    assert_eq!(
        report.rejection_distribution_rows.len(),
        TopologyEditRejectionClass::ALL.len(),
        "the rejection distribution must expose every closed taxonomy class, even when a class has zero hostile cases"
    );
    for rejection_class in TopologyEditRejectionClass::ALL {
        let row = rejection_distribution_row(&report, rejection_class);
        assert_eq!(row.case_count(), row.scenarios().len());
        assert_eq!(
            row.row_digest(),
            format!(
                "rejection_class={};case_count={};scenarios={}",
                rejection_class.as_str(),
                row.case_count(),
                row.scenarios()
                    .iter()
                    .map(|scenario| scenario.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            )
        );
    }
    assert_eq!(
        rejection_distribution_row(
            &report,
            TopologyEditRejectionClass::ScopeLocalizationUnavailable
        )
        .case_count(),
        0
    );
    assert_eq!(
        rejection_distribution_row(&report, TopologyEditRejectionClass::DerivedFallbackExceeded)
            .case_count(),
        0
    );

    let ambiguous_row = naming_distribution_row(&report, TopologyEditNamingOutcome::Ambiguous);
    assert_eq!(
        ambiguous_row.scenarios(),
        &[
            MilestoneThreeHostileScenario::BowtieAdjacentRewire,
            MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
            MilestoneThreeHostileScenario::BrokenRadialLocalization,
        ]
    );
    assert_eq!(
        ambiguous_row.row_digest(),
        "naming_outcome=Ambiguous;case_count=3;scenarios=BowtieAdjacentRewire|AmbiguousLocalRewireContinuity|BrokenRadialLocalization"
    );

    let rejected_row = naming_distribution_row(&report, TopologyEditNamingOutcome::Rejected);
    assert_eq!(
        rejected_row.scenarios(),
        &[
            MilestoneThreeHostileScenario::CancellationChainParity,
            MilestoneThreeHostileScenario::SplitCollapseChurn,
        ]
    );
    assert_eq!(
        rejected_row.row_digest(),
        "naming_outcome=Rejected;case_count=2;scenarios=CancellationChainParity|SplitCollapseChurn"
    );
}

fn rejection_distribution_row(
    report: &crate::facade::MilestoneThreeHostileSuiteReport,
    rejection_class: TopologyEditRejectionClass,
) -> &crate::facade::MilestoneThreeHostileRejectionDistributionRow {
    report
        .rejection_distribution_rows
        .iter()
        .find(|row| row.rejection_class() == rejection_class)
        .expect("rejection distribution row should exist")
}

fn naming_distribution_row(
    report: &crate::facade::MilestoneThreeHostileSuiteReport,
    outcome: TopologyEditNamingOutcome,
) -> &crate::facade::MilestoneThreeHostileNamingDistributionRow {
    report
        .naming_distribution_rows
        .iter()
        .find(|row| row.continuity_outcome_class() == outcome)
        .expect("naming distribution row should exist")
}




