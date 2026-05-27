use std::collections::BTreeSet;

use crate::orchestration_inventory::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryOrchestrationSurfaceVisibility,
};

#[test]
fn continuation_prepare_and_execute_rows_stay_distinct() {
    let inventory = ForgeQueryOrchestrationSurfaceInventory::current();
    let target_rows =
        inventory.rows_for_family(ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget);
    let context_rows =
        inventory.rows_for_family(ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext);
    let execute_rows =
        inventory.rows_for_family(ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute);

    assert!(!target_rows.is_empty());
    assert!(!context_rows.is_empty());
    assert!(!execute_rows.is_empty());
    assert_ne!(
        target_rows[0].canonical_base_name(),
        execute_rows[0].canonical_base_name()
    );
    assert_ne!(
        context_rows[0].canonical_base_name(),
        execute_rows[0].canonical_base_name()
    );
}

#[test]
fn signal_and_contribution_rows_do_not_collapse_into_declaration_entry() {
    let inventory = ForgeQueryOrchestrationSurfaceInventory::current();

    let signal = inventory
        .row_for_public_name("orchestrate_signal_compatibility")
        .expect("signal row should exist");
    let contribution = inventory
        .row_for_public_name("orchestrate_declaration_with_contributions")
        .expect("contribution row should exist");

    assert_eq!(
        signal.family(),
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
    );
    assert_eq!(
        contribution.family(),
        ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
    );
}

#[test]
fn outcome_rows_are_first_class_inventory_rows() {
    let inventory = ForgeQueryOrchestrationSurfaceInventory::current();
    let outcome_names = inventory
        .rows()
        .iter()
        .filter(|row| row.visibility() == ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome)
        .map(|row| row.public_name())
        .collect::<BTreeSet<_>>();

    assert!(outcome_names.contains("orchestrate_declaration_entry_outcome"));
    assert!(outcome_names.contains("prepare_continuation_from_target_outcome"));
    assert!(outcome_names.contains("prepare_continuation_from_context_outcome"));
    assert!(outcome_names.contains("execute_prepared_continuation_outcome"));
    assert!(outcome_names.contains("orchestrate_signal_compatibility_outcome"));
    assert!(outcome_names.contains("orchestrate_declaration_with_contributions_outcome"));
}

#[test]
fn shared_binding_projection_stays_explicit_for_composed_families() {
    let inventory = ForgeQueryOrchestrationSurfaceInventory::current();

    for family in [
        ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget,
        ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext,
        ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute,
    ] {
        for row in inventory.rows_for_family(family) {
            assert_eq!(
                row.binding_projection(),
                ForgeQueryOrchestrationBindingProjection::SharedContinuationBinding
            );
        }
    }

    for row in inventory
        .rows_for_family(ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration)
    {
        assert_eq!(
            row.binding_projection(),
            ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding
        );
    }

    for row in inventory
        .rows_for_family(ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration)
    {
        assert_eq!(
            row.binding_projection(),
            ForgeQueryOrchestrationBindingProjection::SharedContributionBinding
        );
    }
}
