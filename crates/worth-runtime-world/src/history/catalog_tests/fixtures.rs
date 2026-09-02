use std::sync::Arc;

use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::lifecycle::owner::RuntimeWorldOwnerConstructionContract;
use crate::publication::CompositeOwnerExecutionResults;

use super::super::RuntimeWorldHistoryCatalogContract;

pub(super) fn history_contract(
    maximum_commits: u64,
    maximum_metadata_bytes: u64,
) -> RuntimeWorldHistoryCatalogContract {
    let budgets = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: maximum_commits,
            history_metadata_bytes: maximum_metadata_bytes,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 1,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 1,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 1,
            retained_partial_metadata_bytes: 1,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 1,
            in_flight_pin_acquisition_reservations: 1,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("test history budgets are valid");
    RuntimeWorldHistoryCatalogContract::installed(
        budgets.retained_composite_commits(),
        budgets.history_metadata_bytes(),
    )
}

pub(super) fn linear_history(
    length: usize,
) -> (
    RuntimeWorldOwnerConstructionContract,
    Vec<Arc<CompositeRuntimeWorldCommit>>,
) {
    assert!(length > 0);
    let mut owner = RuntimeWorldOwnerConstructionContract::new().expect("World owner");
    let basis =
        crate::basis::AdmittedCompositeRuntimeWorldBasis::admit_test_fixture(owner.issuer())
            .expect("real owner basis admission");
    let root = Arc::new(
        CompositeRuntimeWorldCommit::from_root_bootstrap(
            owner
                .issuer_mut()
                .composite_commit()
                .expect("root identity"),
            basis.clone(),
            owner
                .issuer_mut()
                .bootstrap_attempt()
                .expect("root attempt"),
            None,
        )
        .expect("root commit"),
    );
    let mut commits = vec![root];
    for _ in 1..length {
        let predecessor = commits.last().expect("linear predecessor");
        let commit = Arc::new(
            CompositeRuntimeWorldCommit::from_ordinary_publication(
                owner
                    .issuer_mut()
                    .composite_commit()
                    .expect("ordinary identity"),
                predecessor.as_ref(),
                basis.clone(),
                owner
                    .issuer_mut()
                    .publication_attempt()
                    .expect("ordinary attempt"),
                CompositeOwnerExecutionResults::retained(),
                None,
            )
            .expect("ordinary commit"),
        );
        commits.push(commit);
    }
    (owner, commits)
}
