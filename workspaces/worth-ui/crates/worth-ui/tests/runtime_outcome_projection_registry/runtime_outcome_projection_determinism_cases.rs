use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        IconDescriptor, IconFamily, IconId, IconSourceDescriptor, RuntimeOutcomeAffordance,
        RuntimeOutcomeFamily, RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor,
        RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference, RuntimeOutcomeTone,
    },
};

use super::runtime_outcome_projection_assertions::assert_registered_runtime_outcome_projection_ids;
use super::runtime_outcome_projection_fixtures::{
    denied_projection, failed_projection, projection_id, ready_projection,
};

#[test]
fn equivalent_outcome_projections_preserve_family_identity() {
    let left = WorthUi::app()
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .freeze()
        .expect("application preparation should succeed");
    let right = WorthUi::app()
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .freeze()
        .expect("application preparation should succeed");

    let left_entry = &left.capabilities().runtime_outcome_projections().entries()[0];
    let right_entry = &right.capabilities().runtime_outcome_projections().entries()[0];

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left_entry.key().runtime_identity_basis(),
        right_entry.key().runtime_identity_basis()
    );
}

#[test]
fn outcome_projection_does_not_change_runtime_meaning() {
    let plain = WorthUi::app()
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .freeze()
        .expect("application preparation should succeed");
    let renamed = WorthUi::app()
        .register_runtime_outcome_projection(
            denied_projection("workspace.outcome.denied").with_presentation(
                RuntimeOutcomePresentation::new()
                    .with_label("Permission required")
                    .with_icon(icon_id("workspace.icon.shield"))
                    .with_tone(RuntimeOutcomeTone::destructive())
                    .with_affordance(RuntimeOutcomeAffordance::inspect()),
            ),
        )
        .register_icon(command_icon("workspace.icon.shield"))
        .freeze()
        .expect("application preparation should succeed");

    let plain_entry = &plain.capabilities().runtime_outcome_projections().entries()[0];
    let renamed_entry = &renamed
        .capabilities()
        .runtime_outcome_projections()
        .entries()[0];

    assert_eq!(
        plain_entry.key().runtime_identity_basis(),
        renamed_entry.key().runtime_identity_basis()
    );
    assert_ne!(
        plain_entry.key().projection_basis(),
        renamed_entry.key().projection_basis()
    );
    assert_ne!(
        plain.capabilities().digest(),
        renamed.capabilities().digest()
    );
}

#[test]
fn async_result_state_family_is_preserved_without_local_bool_flattening() {
    let app = WorthUi::app()
        .register_runtime_outcome_projection(ready_projection("workspace.outcome.ready"))
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .register_runtime_outcome_projection(failed_projection("workspace.outcome.failed"))
        .freeze()
        .expect("application preparation should succeed");

    let runtime_families = app
        .capabilities()
        .runtime_outcome_projections()
        .entries()
        .iter()
        .map(|entry| entry.descriptor().family())
        .collect::<Vec<_>>();

    assert_eq!(
        runtime_families,
        vec![
            &RuntimeOutcomeFamily::denied(),
            &RuntimeOutcomeFamily::failed(),
            &RuntimeOutcomeFamily::ready(),
        ]
    );
}

#[test]
fn ui_outcome_sources_project_distinct_typed_families() {
    let app = WorthUi::app()
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::new(
                projection_id("workspace.outcome.ordinary_rebind"),
                RuntimeOutcomeFamily::recoverable(),
                RuntimeOutcomeSourceReference::new(RuntimeOutcomeFamily::recoverable()),
            )
            .with_recovery_posture(RuntimeOutcomeRecoveryPosture::action_hint()),
        )
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            projection_id("workspace.outcome.runtime_pending"),
            RuntimeOutcomeFamily::loading(),
            RuntimeOutcomeSourceReference::new(RuntimeOutcomeFamily::loading()),
        ))
        .register_runtime_outcome_projection(ready_projection("workspace.outcome.async_ready"))
        .freeze()
        .expect("application preparation should succeed");

    let runtime_families = app
        .capabilities()
        .runtime_outcome_projections()
        .entries()
        .iter()
        .map(|entry| entry.descriptor().family())
        .collect::<Vec<_>>();

    assert_eq!(
        runtime_families,
        vec![
            &RuntimeOutcomeFamily::ready(),
            &RuntimeOutcomeFamily::recoverable(),
            &RuntimeOutcomeFamily::loading(),
        ]
    );
}

#[test]
fn accepted_runtime_outcome_projections_remain_inspectable_after_freeze() {
    let app = WorthUi::app()
        .register_runtime_outcome_projection(failed_projection("workspace.outcome.failed"))
        .register_runtime_outcome_projection(denied_projection("workspace.outcome.denied"))
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_runtime_outcome_projection_ids(
        app.capabilities().runtime_outcome_projections(),
        &["workspace.outcome.denied", "workspace.outcome.failed"],
    );
}

#[test]
fn projection_key_basis_is_not_delimiter_collision_prone_for_presentation_text() {
    let split = denied_projection("workspace.outcome.denied")
        .with_presentation(RuntimeOutcomePresentation::new().with_label("pending"));
    let joined = denied_projection("workspace.outcome.denied")
        .with_presentation(RuntimeOutcomePresentation::new().with_label("pending|retry"));

    let split_app = WorthUi::app()
        .register_runtime_outcome_projection(split)
        .freeze()
        .expect("application preparation should succeed");
    let joined_app = WorthUi::app()
        .register_runtime_outcome_projection(joined)
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        split_app
            .capabilities()
            .runtime_outcome_projections()
            .entries()[0]
            .key()
            .projection_basis(),
        joined_app
            .capabilities()
            .runtime_outcome_projections()
            .entries()[0]
            .key()
            .projection_basis()
    );
}

fn command_icon(id: &str) -> IconDescriptor {
    IconDescriptor::new(
        icon_id(id),
        IconFamily::command(),
        IconSourceDescriptor::symbol(id),
    )
}

fn icon_id(raw_text: &str) -> IconId {
    IconId::new(raw_text).expect("valid icon id")
}
