use std::sync::Arc;

use super::{admit_current, component_fixture};

use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation, ProductBranchReferenceSnapshot,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::ProductBranchReferenceGeneration;
use crate::lifecycle::{
    owner::RuntimeWorldOwnerConstructionContract, RuntimeWorldCancellationSource,
};
use crate::publication::{
    lower_component_plans, CompositeAttemptProgress, CompositeComponentIntent,
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, CompositePublicationReady, ProductBranchIntent,
    ResolvedExpectedProductHead, SignalComponentPlanPosture,
};
use crate::retention::{ComponentBasisObligationTransferDestination, RuntimeWorldRetentionOwner};

#[test]
fn publication_transfers_its_own_prospective_obligation_to_the_new_head() {
    let fixture = component_fixture();
    let mut owner = RuntimeWorldOwnerConstructionContract::new().expect("World owner construction");
    let budgets = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 2,
            history_metadata_bytes: 1,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 2,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 1,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 1,
            retained_partial_metadata_bytes: 1,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 2,
            in_flight_pin_acquisition_reservations: 16,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("publication limits are valid");
    let retention = RuntimeWorldRetentionOwner::new(
        owner.owner_identity(),
        budgets.unique_exact_component_pins(),
        budgets.in_flight_pin_acquisition_reservations(),
    );
    let basis = admit_current(
        owner.issuer(),
        &fixture.relational_port,
        &fixture.signal_port,
        &fixture.correspondence_port,
        fixture.relational,
        fixture.signal,
        fixture.correspondence,
    )
    .expect("the real component tuple is admitted");
    let root = Arc::new(
        CompositeRuntimeWorldCommit::from_root_bootstrap(
            owner
                .issuer_mut()
                .composite_commit()
                .expect("root commit identity"),
            basis.clone(),
            owner
                .issuer_mut()
                .bootstrap_attempt()
                .expect("bootstrap provenance"),
            None,
        )
        .expect("root commit is coherent"),
    );
    let branch = owner
        .issuer_mut()
        .product_branch()
        .expect("product branch identity");
    let lifecycle = owner
        .issuer_mut()
        .branch_lifecycle()
        .expect("branch lifecycle identity");
    let successor = Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            owner
                .issuer_mut()
                .composite_commit()
                .expect("successor commit identity"),
            &root,
            basis.clone(),
            owner
                .issuer_mut()
                .publication_attempt()
                .expect("publication provenance"),
            CompositeOwnerExecutionResults::retained(),
            None,
        )
        .expect("successor commit is coherent"),
    );
    let expected = ProductBranchObservation::owner_issued(
        ProductBranchReferenceSnapshot::owner_issued(
            owner.owner_identity(),
            branch.clone(),
            lifecycle,
            ProductBranchReferenceGeneration::initial(),
            root,
        )
        .expect("expected reference snapshot is owner-coherent"),
        retention
            .issue_observation(&basis)
            .expect("expected head observation obligation"),
    )
    .expect("expected observation retention is basis-coherent");
    let next_generation = expected
        .reference_generation()
        .advance()
        .expect("reference generation advances without wrapping");
    let new_head = ProductBranchObservation::owner_issued(
        ProductBranchReferenceSnapshot::owner_issued(
            owner.owner_identity(),
            branch,
            lifecycle,
            next_generation,
            successor.clone(),
        )
        .expect("new reference snapshot is owner-coherent"),
        retention
            .issue_observation(&basis)
            .expect("new head observation obligation"),
    )
    .expect("new observation retention is basis-coherent");
    let ready = CompositePublicationReady::new(
        owner
            .issuer_mut()
            .publication_attempt()
            .expect("attempt identity"),
        expected,
        successor,
        CompositeAttemptProgress::untouched(),
        retention
            .issue_publication(&basis)
            .expect("prospective publication obligation"),
    );

    let performed = ready
        .publish(
            new_head,
            CompositeOwnerExecutionResults::retained(),
            CompositeLateCancellationPosture::NotRequested,
            CompositePublicationCostCounters::zero(),
        )
        .expect("the coherent owner result performs publication");
    assert_eq!(
        performed.retention_transfer().destination(),
        ComponentBasisObligationTransferDestination::ProductBranchHead
    );
    assert_eq!(
        performed
            .retention_transfer()
            .obligation()
            .relational()
            .dependency(),
        crate::retention::ComponentBasisDependencyClass::ProductBranchHead
    );
    assert_eq!(
        retention.active_component_obligation_count(),
        6,
        "two observations and the transferred publication pair remain live"
    );
    drop(performed);
    assert_eq!(retention.active_component_obligation_count(), 0);
}

#[test]
fn serial_reservation_chain_reaches_only_typed_pre_effect_cancellation() {
    let fixture = component_fixture();
    let mut owner = RuntimeWorldOwnerConstructionContract::new().expect("World owner construction");
    let budgets = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 2,
            history_metadata_bytes: 1,
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
            unique_exact_component_pins: 2,
            in_flight_pin_acquisition_reservations: 4,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("named publication limits are valid");
    let retention = RuntimeWorldRetentionOwner::new(
        owner.owner_identity(),
        budgets.unique_exact_component_pins(),
        budgets.in_flight_pin_acquisition_reservations(),
    );
    let basis = admit_current(
        owner.issuer(),
        &fixture.relational_port,
        &fixture.signal_port,
        &fixture.correspondence_port,
        fixture.relational,
        fixture.signal,
        fixture.correspondence,
    )
    .expect("the real component tuple is admitted");
    let root = Arc::new(
        CompositeRuntimeWorldCommit::from_root_bootstrap(
            owner
                .issuer_mut()
                .composite_commit()
                .expect("root commit identity"),
            basis.clone(),
            owner
                .issuer_mut()
                .bootstrap_attempt()
                .expect("root bootstrap provenance"),
            None,
        )
        .expect("root commit is coherent"),
    );
    let branch = owner
        .issuer_mut()
        .product_branch()
        .expect("product branch identity");
    let lifecycle = owner
        .issuer_mut()
        .branch_lifecycle()
        .expect("branch lifecycle identity");
    let expected = ProductBranchObservation::owner_issued(
        ProductBranchReferenceSnapshot::owner_issued(
            owner.owner_identity(),
            branch,
            lifecycle,
            ProductBranchReferenceGeneration::initial(),
            root.clone(),
        )
        .expect("expected reference snapshot is owner-coherent"),
        retention
            .issue_observation(&basis)
            .expect("expected head observation obligation"),
    )
    .expect("expected observation retention is basis-coherent");

    let component_intent = CompositeComponentIntent::signal_only();
    let resolved = ResolvedExpectedProductHead::new(
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("main").expect("branch name is valid"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            component_intent.clone(),
        ),
        expected,
    );
    let plan = lower_component_plans(resolved, component_intent);
    assert_eq!(
        plan.signal().posture(),
        SignalComponentPlanPosture::AdvanceExact
    );

    let cancellation = RuntimeWorldCancellationSource::new();
    let token = cancellation.token();
    let reserved = plan
        .reserve(
            owner
                .issuer_mut()
                .publication_attempt()
                .expect("reservation attempt identity"),
            owner
                .issuer_mut()
                .composite_commit()
                .expect("reserved commit identity"),
            &budgets,
            &token,
            retention
                .issue_publication(&basis)
                .expect("prospective publication obligation"),
            None,
        )
        .expect("lowered plan reserves before owner effects");

    assert_eq!(
        reserved.cancellation_posture(),
        crate::publication::CompositeAttemptCancellationPosture::Open
    );
    assert_eq!(
        reserved.progress().signal_posture(),
        crate::publication::SignalAttemptProgressPosture::Untouched
    );
    let no_effect = reserved.cancel();
    assert_eq!(
        no_effect.cause(),
        crate::publication::NoEffectCause::CancelledBeforeEffect
    );
    assert_eq!(
        no_effect
            .expected_head()
            .expect("cancellation retains the expected head")
            .selected_commit(),
        root.identity()
    );
}
