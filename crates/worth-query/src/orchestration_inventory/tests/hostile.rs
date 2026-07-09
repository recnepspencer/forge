use std::collections::BTreeSet;

use crate::application::WorthQueryDeclarationEntryContributionCategoryFamily;
use crate::orchestration_inventory::{
    WorthQueryOrchestrationAspectPosture, WorthQueryOrchestrationBindingProjection,
    WorthQueryOrchestrationContributionCompatibility, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceInventory, WorthQueryOrchestrationSurfaceVisibility,
};

#[test]
fn continuation_prepare_and_execute_rows_stay_distinct() {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let target_rows =
        inventory.rows_for_family(WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget);
    let context_rows =
        inventory.rows_for_family(WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext);
    let execute_rows =
        inventory.rows_for_family(WorthQueryOrchestrationSurfaceFamily::ContinuationExecute);

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
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();

    let signal = inventory
        .row_for_public_name("orchestrate_signal_compatibility")
        .expect("signal row should exist");
    let contribution = inventory
        .row_for_public_name("orchestrate_declaration_with_contributions")
        .expect("contribution row should exist");

    assert_eq!(
        signal.family(),
        WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
    );
    assert_eq!(
        contribution.family(),
        WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
    );
}

#[test]
fn outcome_rows_are_first_class_inventory_rows() {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let outcome_names = inventory
        .rows()
        .iter()
        .filter(|row| row.visibility() == WorthQueryOrchestrationSurfaceVisibility::OrdinaryOutcome)
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
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();

    for family in [
        WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget,
        WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext,
        WorthQueryOrchestrationSurfaceFamily::ContinuationExecute,
    ] {
        for row in inventory.rows_for_family(family) {
            assert_eq!(
                row.binding_projection(),
                WorthQueryOrchestrationBindingProjection::SharedContinuationBinding
            );
        }
    }

    for row in inventory
        .rows_for_family(WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration)
    {
        assert_eq!(
            row.binding_projection(),
            WorthQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding
        );
    }

    for row in inventory
        .rows_for_family(WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration)
    {
        assert_eq!(
            row.binding_projection(),
            WorthQueryOrchestrationBindingProjection::SharedContributionBinding
        );
    }
}

#[test]
fn semantic_attachments_stay_visible_for_signal_and_contribution_rows() {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();

    let signal = inventory
        .row_for_public_name("orchestrate_signal_compatibility")
        .expect("signal row should exist");
    assert_eq!(
        signal.aspect_posture(),
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage
    );
    assert!(signal.strategy_attachment().is_merge_strategy_aware());

    let contribution = inventory
        .row_for_public_name("orchestrate_declaration_with_contributions")
        .expect("contribution row should exist");
    assert_eq!(
        contribution.aspect_posture(),
        WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition
    );
    assert!(contribution
        .strategy_attachment()
        .foundational_materialization_profile_relevant());
    assert!(contribution
        .contribution_compatibility()
        .supports(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview));
}

#[test]
fn grouped_rows_advertise_grouped_neighborhood_contribution_posture_honestly() {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let grouped = inventory
        .row_for_public_name("orchestrate_local_neighborhood_for_active_face_selection")
        .expect("grouped helper row should exist");

    assert_eq!(
        grouped.family(),
        WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration
    );
    assert_eq!(
        grouped.contribution_compatibility().kind().as_str(),
        "grouped_neighborhood"
    );
    assert!(!grouped
        .contribution_compatibility()
        .supports(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview));
}

#[test]
fn declaration_scoped_contribution_compatibility_canonicalizes_family_order() {
    let left = WorthQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
        WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
        WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
        WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
    ]);
    let right = WorthQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
        WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
        WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
    ]);

    assert_eq!(left.supported_families(), right.supported_families());
    assert_eq!(left.as_digest_fragment(), right.as_digest_fragment());
}
