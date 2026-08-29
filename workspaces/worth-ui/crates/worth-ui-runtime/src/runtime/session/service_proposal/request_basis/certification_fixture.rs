use core::num::NonZeroU64;

/// One coherence inside an already-established application generation. The scale
/// world uses this so its neighborhoods are genuinely sibling surfaces of one
/// application rather than unrelated applications that could never interact.
pub(in crate::runtime::session::service_proposal) fn fixture_service_request_coherence_in(
    application: &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    seed: u64,
) -> super::UiServiceRequestCoherence {
    worth_proof::Binding::new(super::UiServiceRequestCoherenceAxes {
        application: application.clone(),
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound()
            .unwrap(),
        host_surface: worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        presentation: None,
        causal_root: super::UiServiceRequestIdentity(NonZeroU64::new(seed).unwrap()),
        cancellation: super::UiServiceCancellationIdentity(NonZeroU64::new(seed).unwrap()),
        resource_budget: super::UiServiceResourceBudgetIdentity(NonZeroU64::new(seed).unwrap()),
    })
}

pub(in crate::runtime::session::service_proposal) fn fixture_application_generation(
    seed: u64,
) -> crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
    use worth_ui_dsl::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken,
    };
    let app = crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            crate::facade::WorthUiRustAuthoredDeclarationFixture::named(format!(
                "service-scale-{seed}"
            ))
            .with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new(format!("ui.service.scale.{seed}")),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::rust_authored("service/scale", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:service-scale")),
            ),
        )
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("service scale generation prepares");
    crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity::current(
        crate::lifecycle::WorthUiActiveApplicationSessionIdentity::from_host_session_value(seed),
        app.generation_identity(),
    )
}
