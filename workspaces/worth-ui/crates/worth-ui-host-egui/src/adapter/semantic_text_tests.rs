use worth_ui_host_contract::{
    UiHostProtocolAgreement, UiHostProtocolContract, UiHostProtocolDenial, UiHostProtocolIdentity,
    UiHostProtocolNegotiation, UiHostProtocolSchemaFamily, UiHostSurfaceIdentity,
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationMode, UiMountedFrameConsumptionInput,
    UiMountedFrameSchemaVersion, UiMountedPaintOrderIntegrity,
    UiMountedPresentationAttemptIdentity, UiMountedPresentationInitial,
    UiMountedPresentationInitialInput, UiMountedPresentationWorkView, UiMountedProjectionView,
    UiMountedSurfaceBindingRequirement, WorthUiHostCapabilityObservationGeneration,
};
use worth_ui_test_support::{
    semantic_text_projection_for_certification as projection,
    UiSemanticTextProjectionCertificationMutation as Mutation,
};

#[test]
fn validated_agreement_semantic_text_consumes_and_mixed_contract_stops_before_consumer() {
    let projection = projection(Mutation::Exact);
    consume_projection(&projection, current_protocol(), super::prepare).unwrap();

    let adapter = include_str!("semantic_text.rs");
    assert!(!adapter.contains("FontId"));
    assert!(!adapter.contains("Painter::text"));
    assert!(matches!(
        mounted_frame_revision_two().negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooOld(
            UiHostProtocolSchemaFamily::MountedFrame
        ))
    ));
}

#[test]
fn foreign_runtime_bases_are_rejected_before_paint() {
    for mutation in [
        Mutation::ForeignFrame,
        Mutation::ForeignSurface,
        Mutation::ForeignBinding,
        Mutation::ForeignContentGeneration,
        Mutation::ForeignNodeReceipt,
        Mutation::ForeignAllocation,
        Mutation::ForeignCapabilityGeneration,
        Mutation::ForeignCapabilityProfile,
        Mutation::WithheldPaint,
    ] {
        let projection = projection(mutation);
        assert!(
            matches!(
                consume_projection(&projection, current_protocol(), super::prepare),
                Err(UiHostSurfacePresentationDenial::MalformedProjection)
            ),
            "mutation {mutation:?} must stop before paint"
        );
    }
}

#[test]
fn missing_duplicate_and_old_protocol_references_are_rejected() {
    let projection = projection(Mutation::UnreferencedRow);
    assert!(matches!(
        consume_projection(&projection, current_protocol(), super::prepare),
        Err(UiHostSurfacePresentationDenial::MalformedProjection)
    ));
}

#[test]
fn impossible_authored_sources_stop_before_a_projection_view_exists() {
    for mutation in [
        Mutation::ForeignInstance,
        Mutation::MissingReference,
        Mutation::DuplicateReference,
    ] {
        let result = std::panic::catch_unwind(|| projection(mutation));
        assert!(
            result.is_err(),
            "mutation {mutation:?} must stop at authored-source admission"
        );
    }
}

fn consume_projection<T>(
    projection: &UiMountedProjectionView,
    protocol: UiHostProtocolAgreement,
    consume: impl FnOnce(&worth_ui_host_contract::UiMountedFrameConsumptionView<'_>) -> T,
) -> T {
    let row = &projection.semantic_text().rows()[0];
    let qualified_text = ExactLayoutResolver {
        view: worth_ui_host_contract::UiQualifiedTextLayoutView::from_text_mechanics(
            worth_ui_host_contract::UiQualifiedTextLayoutViewInput {
                request_identity: row.qualified_layout_request(),
                identity: row.qualified_layout_identity(),
                source: row.text(),
                graphemes: &[],
                word_boundaries: &[],
                styles: &[],
                logical_runs: &[],
                glyphs: &[],
                lines: &[],
                visual_runs: &[],
                positioned_glyphs: &[],
                logical_bounds: Default::default(),
                ink_bounds: Default::default(),
                carets: &[],
                coverage: &[],
                cost: Default::default(),
                profile: row.qualified_layout_profile(),
                font_collection: row.qualified_layout_fonts(),
                text_scale: row.qualified_layout_scale(),
            },
        ),
    };
    let generation = WorthUiHostCapabilityObservationGeneration::new(7);
    let requirement = UiMountedSurfaceBindingRequirement::new(
        projection.surface(),
        UiHostSurfaceIdentity::mint_unbound().unwrap(),
        projection.binding(),
        generation,
        11,
        UiHostSurfacePresentationMode::NativeDisplay,
    );
    let presentation_work =
        UiMountedPresentationInitial::from_inert_mechanics(UiMountedPresentationInitialInput {
            successor: projection.frame(),
            surface: projection.surface(),
            binding: projection.binding(),
            content: projection.content_generation(),
            baseline: requirement.baseline(),
            projection: projection.clone(),
            commands: Vec::new(),
            order: Vec::new(),
            order_integrity: UiMountedPaintOrderIntegrity::for_order(&[]),
            damage: Vec::new(),
            production_cost: Default::default(),
        });
    let view = worth_ui_host_contract::UiMountedFrameConsumptionView::from_inert_mechanics(
        UiMountedFrameConsumptionInput {
            qualified_text: &qualified_text,
            authority: std::rc::Rc::new(()),
            host_session_identity: 13,
            protocol,
            capability_generation: generation,
            capability_profile_digest: 11,
            attempt: UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
            deadline: worth_ui_host_contract::UiPresentationDeadline::at_tick(20),
            requirement,
            presentation_work: UiMountedPresentationWorkView::Initial(&presentation_work),
        },
    );
    consume(&view)
}

struct ExactLayoutResolver<'layout> {
    view: worth_ui_host_contract::UiQualifiedTextLayoutView<'layout>,
}

impl worth_ui_host_contract::UiMountedQualifiedTextResolver for ExactLayoutResolver<'_> {
    fn resolve(
        &self,
        identity: worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    ) -> Option<worth_ui_host_contract::UiQualifiedTextLayoutView<'_>> {
        (identity == self.view.identity()).then_some(self.view)
    }
}

fn current_protocol() -> UiHostProtocolAgreement {
    compatible(UiHostProtocolContract::current())
}

fn mounted_frame_revision_two() -> UiHostProtocolContract {
    let current = UiHostProtocolContract::current();
    UiHostProtocolContract::new(
        UiHostProtocolIdentity::worth_ui(),
        current.protocol(),
        UiMountedFrameSchemaVersion::new(2),
        current.mounted_presentation(),
        current.observation(),
        current.measurement(),
    )
}

fn compatible(contract: UiHostProtocolContract) -> UiHostProtocolAgreement {
    match contract.negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(denial) => panic!("{denial:?}"),
    }
}
