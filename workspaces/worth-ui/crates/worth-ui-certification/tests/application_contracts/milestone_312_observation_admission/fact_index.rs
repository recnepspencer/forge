use std::collections::BTreeSet;

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::observation_report::{
    UiHostObservationLoss, UiHostObservationPayload, WorthUiHostObservationSessionExt,
};
use worth_ui::facade::rebind::UiProducedFactFamily;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;
use worth_ui_test_support::{
    UiGraphFactConsumerIdentity, WorthUiActiveSessionCertificationExt,
    WorthUiMountedIdentityCertificationExt,
};

use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

use super::super::host_observation_fixture::{batch, report, source};
use super::super::mounted_application_lifecycle::known_empty_surface_world::profile;
use super::super::mounted_application_lifecycle::published_mounted_world::publish;
use super::validated;

#[test]
fn fact_indexes_join_real_consumption_authority() {
    let host = ScriptedPresentationHost::default();
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            host.clone(),
        )
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("phase-312-fact-index")
                .with_semantic_artifact_spec(appearance_consumer_spec())
                .with_semantic_artifact_spec(content_consumer_spec()),
        )
        .freeze()
        .expect("fact-index application should prepare");
    let graph = app.graph();
    let appearance_artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.identity().authored_semantic_name() == "pulse.appearance.consumer"
        })
        .expect("appearance consumer declaration should be admitted");
    let appearance_node = graph
        .lookup()
        .declaration_instances(appearance_artifact.identity())
        .value()[0];
    let appearance_slot = graph
        .lookup()
        .mount_eligibility_slot_for_node(appearance_node)
        .expect("appearance consumer has a mount-eligibility slot")
        .value()
        .mount_eligibility_identity();
    let graph_generation = graph.generation();

    let mut session = app.launch().expect("fact-index app should launch");
    let surface = session
        .create_semantic_surface()
        .expect("semantic surface should admit");
    let binding = session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .expect("host surface should register")
        .binding_generation();
    let appearance_handle = session
        .mounted_graph_node(appearance_node)
        .expect("appearance consumer is mountable");
    let mounted = session
        .mount_instance(appearance_handle, surface)
        .expect("appearance consumer should mount");
    let current = publish(&mut session, &host, mounted);
    let validated = validated(session.validate_host_observation_batch(batch(
        source(&session, binding, &current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::DeviceScale { micros: 1_250_000 },
            &current,
        )],
    )));
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_host(validated)
        .expect("validated device scale should admit");
    let admitted = turn.seal().unwrap();
    let classified = match session.classify_observations(admitted) {
        Ok(UiChangeClassificationOutcome::Changed(classified)) => classified,
        _ => panic!("device scale must produce one changed classification"),
    };
    let fact = classified
        .facts()
        .iter()
        .find(|fact| fact.family() == UiProducedFactFamily::HostDeviceScale)
        .expect("classification retains the owner-produced device-scale fact");
    let receipt = session
        .lookup_consumed_fact(fact)
        .expect("current production index resolves the classified fact");
    let observed = receipt
        .entries()
        .iter()
        .map(|entry| entry.consumer())
        .collect::<BTreeSet<_>>();

    assert_eq!(receipt.basis().graph_generation(), graph_generation);
    assert_eq!(
        observed,
        BTreeSet::from([
            UiGraphFactConsumerIdentity::GraphNode(appearance_node),
            UiGraphFactConsumerIdentity::MountEligibilitySlot(appearance_slot),
        ])
    );
    assert!(receipt.entries().iter().all(|entry| {
        entry
            .affected_aspect()
            .is_some_and(|aspect| aspect.canonical_label() == "appearance.background")
    }));
    assert_eq!(receipt.cost().index_probes(), 1);
    assert_eq!(receipt.cost().contract_checks(), 2);
    assert_eq!(receipt.cost().selected_consumers(), 2);
    let _ = session.shutdown();
}

fn appearance_consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("pulse.appearance.consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/fact_index_contract.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("region:appearance-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("appearance.background"))
}

fn content_consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("pulse.content.consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/fact_index_contract.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:content-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}
