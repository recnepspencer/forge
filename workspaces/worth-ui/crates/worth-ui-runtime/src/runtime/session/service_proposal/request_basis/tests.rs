use core::num::NonZeroU64;

use super::{
    UiServiceCancellationIdentity, UiServiceRequestBasis, UiServiceRequestBasisDenial,
    UiServiceRequestBasisInput, UiServiceRequestIdentity, UiServiceRequestOrigin,
    UiServiceRequestOriginAuthority, UiServiceResourceBudgetIdentity, UiServiceSourceOrder,
    UiServiceSurfaceBasis,
};

struct FixtureIntentAuthority;

impl UiServiceRequestOriginAuthority for FixtureIntentAuthority {
    fn service_request_origin(&self) -> UiServiceRequestOrigin {
        UiServiceRequestOrigin::AdmittedIntent
    }
}

impl super::sealed::Sealed for FixtureIntentAuthority {}

pub(crate) fn fixture_service_request_coherence(seed: u64) -> super::UiServiceRequestCoherence {
    worth_proof::Binding::new(super::UiServiceRequestCoherenceAxes {
        application: fixture_application_generation(seed),
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound()
            .unwrap(),
        host_surface: worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        presentation: None,
        causal_root: UiServiceRequestIdentity(NonZeroU64::new(seed).unwrap()),
        cancellation: UiServiceCancellationIdentity(NonZeroU64::new(seed).unwrap()),
        resource_budget: UiServiceResourceBudgetIdentity(NonZeroU64::new(seed).unwrap()),
    })
}

fn fixture_application_generation(
    seed: u64,
) -> crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
    fixture_application_generation_in_session(seed, seed)
}

pub(crate) fn fixture_application_generation_in_session(
    session_seed: u64,
    generation_seed: u64,
) -> crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
    use worth_ui_dsl::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken,
    };

    let app = crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            crate::facade::WorthUiRustAuthoredDeclarationFixture::named(format!(
                "service-basis-fixture-{generation_seed}"
            ))
            .with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new(format!("ui.service.basis.fixture.{generation_seed}")),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::rust_authored("service/basis", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:service-basis")),
            ),
        )
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("service-basis fixture prepares");
    let session =
        crate::lifecycle::WorthUiActiveApplicationSessionIdentity::from_host_session_value(
            session_seed,
        );
    crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity::current(
        session,
        app.generation_identity(),
    )
}

#[test]
fn common_identity_roles_remain_distinct_types() {
    let value = NonZeroU64::new(7).unwrap();
    let request = UiServiceRequestIdentity(value);
    let source_order = UiServiceSourceOrder(value);
    let cancellation = UiServiceCancellationIdentity(value);
    let budget = UiServiceResourceBudgetIdentity(value);

    fn request_identity(_: UiServiceRequestIdentity) {}
    fn source_order_identity(_: UiServiceSourceOrder) {}
    fn cancellation_identity(_: UiServiceCancellationIdentity) {}
    fn budget_identity(_: UiServiceResourceBudgetIdentity) {}

    request_identity(request);
    source_order_identity(source_order);
    cancellation_identity(cancellation);
    budget_identity(budget);
    assert_eq!(request.0, value);
}

#[test]
fn origin_is_derived_from_the_carried_concrete_authority() {
    let authority = FixtureIntentAuthority;
    assert_eq!(
        authority.service_request_origin(),
        UiServiceRequestOrigin::AdmittedIntent
    );
    assert_eq!(
        [
            UiServiceRequestOrigin::AdmittedIntent,
            UiServiceRequestOrigin::HostObservation,
            UiServiceRequestOrigin::Rebind,
            UiServiceRequestOrigin::ServiceContinuation,
            UiServiceRequestOrigin::RuntimePolicy,
            UiServiceRequestOrigin::Teardown,
        ]
        .len(),
        6
    );
}

#[test]
fn basis_contract_exposes_existing_authority_identities_without_erasure() {
    fn contract<Authority>(basis: &super::UiServiceRequestBasis<Authority>)
    where
        Authority: UiServiceRequestOriginAuthority,
    {
        let _: &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity =
            basis.application();
        let _: worth_ui_host_contract::UiSemanticSurfaceIdentity =
            basis.surface().semantic_surface();
        let _: worth_ui_host_contract::UiHostSurfaceIdentity = basis.surface().host_surface();
        let _: worth_ui_host_contract::UiSurfaceBindingGeneration = basis.surface().binding();
        let _: Option<worth_ui_host_contract::UiHostObservationPresentationBasis> =
            basis.presentation();
        let _: UiServiceRequestOrigin = basis.origin();
    }

    let _ = contract::<FixtureIntentAuthority>;
}

#[test]
fn causal_basis_rejects_root_drift_and_self_parenting() {
    let root = UiServiceRequestIdentity(NonZeroU64::new(1).unwrap());
    let child = UiServiceRequestIdentity(NonZeroU64::new(2).unwrap());

    assert_eq!(super::validate_causal_basis(root, None, root), Ok(()));
    assert_eq!(
        super::validate_causal_basis(child, Some(root), root),
        Ok(())
    );
    assert_eq!(
        super::validate_causal_basis(root, None, child),
        Err(UiServiceRequestBasisDenial::RootIdentityMismatch)
    );
    assert_eq!(
        super::validate_causal_basis(root, Some(child), root),
        Err(UiServiceRequestBasisDenial::ChildRootIdentityMismatch)
    );
    assert_eq!(
        super::validate_causal_basis(child, Some(child), root),
        Err(UiServiceRequestBasisDenial::SelfParent)
    );
}

#[test]
fn presentation_basis_must_match_the_exact_surface_binding() {
    let surface = worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let host_surface = worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let foreign_binding =
        worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
    let surface_basis = UiServiceSurfaceBasis {
        semantic_surface: surface,
        host_surface,
        binding,
    };

    assert_eq!(
        super::validate_presentation_binding(
            surface_basis,
            Some(
                worth_ui_host_contract::UiHostObservationPresentationBasis::new(
                    host_surface,
                    frame,
                    binding,
                    worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
                )
            ),
        ),
        Ok(())
    );
    assert_eq!(
        super::validate_presentation_binding(
            surface_basis,
            Some(
                worth_ui_host_contract::UiHostObservationPresentationBasis::new(
                    host_surface,
                    frame,
                    foreign_binding,
                    worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
                )
            ),
        ),
        Err(UiServiceRequestBasisDenial::PresentationBindingChanged)
    );

    let foreign_surface = worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap();
    assert_eq!(
        super::validate_presentation_binding(
            surface_basis,
            Some(
                worth_ui_host_contract::UiHostObservationPresentationBasis::new(
                    foreign_surface,
                    frame,
                    binding,
                    worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
                )
            ),
        ),
        Err(UiServiceRequestBasisDenial::PresentationSurfaceChanged)
    );
}

#[test]
fn basis_is_constructed_only_through_the_sealing_court() {
    let root = UiServiceRequestIdentity(NonZeroU64::new(1).unwrap());
    let surface = worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let host_surface = worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let sealed = UiServiceRequestBasis::seal(UiServiceRequestBasisInput {
        identity: root,
        causal_parent: None,
        causal_root: root,
        application: fixture_application_generation(1),
        surface: UiServiceSurfaceBasis {
            semantic_surface: surface,
            host_surface,
            binding,
        },
        presentation: None,
        source_order: UiServiceSourceOrder(NonZeroU64::new(1).unwrap()),
        cancellation: UiServiceCancellationIdentity(NonZeroU64::new(1).unwrap()),
        resource_budget: UiServiceResourceBudgetIdentity(NonZeroU64::new(1).unwrap()),
        authority: FixtureIntentAuthority,
    })
    .expect("coherent root request seals");

    assert_eq!(sealed.identity(), root);
    assert_eq!(sealed.causal_parent(), None);
    assert_eq!(sealed.origin(), UiServiceRequestOrigin::AdmittedIntent);
}
