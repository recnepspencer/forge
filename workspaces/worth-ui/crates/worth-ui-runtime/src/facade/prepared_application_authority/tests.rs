use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

use crate::capability::WorthUiQueryViewRegistration;
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifactPosture;
use crate::facade::{WorthUi, WorthUiRustAuthoredDeclarationFixture};
use crate::graph::{UiGraphSessionLabel, UiGraphWorldProfile};
use crate::runtime::observation::{UiObservationProfile, UiObservationProfileInput};
use crate::runtime::rebind::{UiChangeProfile, UiRebindProfile};

#[test]
fn declaration_and_graph_drift_change_prepared_generation_identity() {
    let baseline = app_with_package("prepared-baseline", "ui.prepared.baseline")
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("baseline should prepare");
    let declaration_drift = app_with_package("prepared-drift", "ui.prepared.drift")
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("declaration drift should prepare");
    let graph_drift = app_with_package("prepared-baseline", "ui.prepared.baseline")
        .with_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            UiGraphSessionLabel::new("prepared-preview").expect("valid graph session label"),
        ))
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("graph drift should prepare");

    assert_eq!(
        baseline.capabilities().digest(),
        declaration_drift.capabilities().digest()
    );
    assert_eq!(
        baseline.capabilities().digest(),
        graph_drift.capabilities().digest()
    );
    assert_ne!(
        baseline.generation_identity(),
        declaration_drift.generation_identity()
    );
    assert_ne!(
        baseline.generation_identity(),
        graph_drift.generation_identity()
    );
    assert_eq!(
        baseline.prepared_authority().application_artifact_posture(),
        WorthUiPreparedApplicationArtifactPosture::SourceBacked
    );
}

#[test]
fn query_drift_changes_identity_while_host_selection_remains_outside_it() {
    let left_query = query_app("prepared-query-left");
    let right_query = query_app("prepared-query-right");
    let headless = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("headless app should prepare");
    let alternate_host = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(|application| {
            crate::facade::entry::WorthUiCertificationApplicationTransition::activate_test_host(
                application,
                AlternatePlanAdapter,
            )
        })
        .expect("alternate host app should prepare");

    assert_eq!(
        left_query.capabilities().digest(),
        right_query.capabilities().digest()
    );
    assert_ne!(
        left_query.generation_identity(),
        right_query.generation_identity()
    );
    assert_eq!(
        headless.capabilities().digest(),
        alternate_host.capabilities().digest()
    );
    assert_eq!(
        headless.generation_identity(),
        alternate_host.generation_identity()
    );
}

#[derive(Default)]
struct AlternatePlanAdapter;

impl worth_ui_host_contract::WorthUiMeasurementHostAdapter for AlternatePlanAdapter {
    fn observe_measurement(
        &self,
        _request: &worth_ui_host_contract::UiHostMeasurementRequest,
    ) -> worth_ui_host_contract::UiHostMeasurementObservationValue {
        unreachable!("prepared identity test never enters native observation")
    }
}

impl crate::host::adapter::WorthUiOperationalHostAdapter for AlternatePlanAdapter {
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract {
        worth_ui_host_contract::WorthUiHostContract::diagnostics_only()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport {
        worth_ui_host_contract::WorthUiHostCapabilityReport::available(vec![
            worth_ui_host_contract::WorthUiHostCapability::ViewportObservation,
        ])
    }

    fn release_host_session(
        &self,
        authority: &crate::host::adapter::UiHostAdapterSessionAuthority,
    ) -> crate::host::adapter::UiHostSessionReleaseOutcome {
        crate::host::adapter::UiHostSessionReleaseOutcome::Released(
            crate::host::adapter::UiHostSessionReleaseReceipt::released(
                authority.host_session_identity(),
                0,
            ),
        )
    }
}

#[test]
fn every_prepared_derived_index_rebuilds_from_owned_authority() {
    let mut app = app_with_package("prepared-rebuild", "ui.prepared.rebuild")
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("rebuild fixture should prepare");
    let before = (
        app.authored_evidence_index().clone(),
        app.graph_node_evidence_index().clone(),
        app.graph_aspect_evidence_indexes().clone(),
        app.prepared_authority().consumed_fact_index().clone(),
        app.prepared_authority()
            .graph_snapshot()
            .core_indexes()
            .published_aspects()
            .clone(),
        app.prepared_authority()
            .graph_snapshot()
            .core_indexes()
            .consumed_aspects()
            .clone(),
    );

    app.rebuild_prepared_derived_indexes();

    assert_eq!(before.0, *app.authored_evidence_index());
    assert_eq!(before.1, *app.graph_node_evidence_index());
    assert_eq!(before.2, *app.graph_aspect_evidence_indexes());
    assert_eq!(before.3, *app.prepared_authority().consumed_fact_index());
    assert_eq!(
        before.4,
        *app.prepared_authority()
            .graph_snapshot()
            .core_indexes()
            .published_aspects()
    );
    assert_eq!(
        before.5,
        *app.prepared_authority()
            .graph_snapshot()
            .core_indexes()
            .consumed_aspects()
    );
}

#[test]
fn exact_change_profile_is_generation_identity_and_prepared_authority() {
    let observation = UiObservationProfile::bounded(UiObservationProfileInput {
        admitted_per_turn: 2,
        retained_bytes_per_turn: 2_048,
        queued_during_effecting_rebind: 1,
    })
    .expect("smaller observation profile should be valid");
    let custom = UiChangeProfile::new(observation, UiRebindProfile::platform_pulse());
    let baseline = WorthUi::app()
        .with_change_profile(UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("baseline should prepare");
    let configured = WorthUi::app()
        .with_change_profile(custom)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("configured app should prepare");

    assert_ne!(
        baseline.generation_identity(),
        configured.generation_identity()
    );
    assert_eq!(configured.prepared_authority().change_profile(), custom);
}

fn app_with_package(
    package_name: &str,
    semantic_key: &str,
) -> crate::facade::entry::WorthUiCertificationApplicationBuilder {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(package_name).with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new(semantic_key),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::rust_authored("prepared/application", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:prepared")),
            ),
        )
}

fn query_app(installed_domain: &str) -> crate::facade::WorthUiApp {
    let installed =
        worth_ui_query_binding::certification::worth_ui_installed_test_domain(installed_domain);
    let view = installed
        .live_measurement_view("workspace.view_binding.prepared")
        .expect("installed query view should admit");
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(WorthUiQueryViewRegistration::new(view))
        .expect("query view should register")
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("query app should prepare")
}
