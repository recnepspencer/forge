use worth_ui_host_contract::{
    UiHostMeasurementSchemaVersion, UiHostProtocolAgreement, UiHostProtocolContract,
    UiHostProtocolDenial, UiHostProtocolIdentity, UiHostProtocolNegotiation,
    UiHostProtocolSchemaFamily, UiHostProtocolVersion, UiHostSurfaceIdentity,
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationMode, UiMountedFrameConsumptionInput,
    UiMountedFrameSchemaVersion, UiMountedPaintOrderIntegrity,
    UiMountedPresentationAttemptIdentity, UiMountedPresentationInitial,
    UiMountedPresentationInitialInput, UiMountedPresentationSchemaVersion,
    UiMountedPresentationWorkView, UiMountedProjectionView, UiMountedSurfaceBindingRequirement,
    WorthUiHostCapabilityObservationGeneration,
};
use worth_ui_test_support::{
    semantic_text_projection_for_certification as projection,
    UiSemanticTextProjectionCertificationMutation as Mutation,
};

#[test]
fn validated_agreement_semantic_text_consumes_and_mixed_contract_stops_before_consumer() {
    let projection = projection(Mutation::Exact);
    let prepared = consume_projection(&projection, current_protocol(), super::prepare).unwrap();

    assert_eq!(prepared.len(), 1);
    assert_eq!(prepared[0].origin, egui::pos2(8.0, 12.0));
    assert_eq!(prepared[0].text.as_ref(), "ONLINE");
    assert_eq!(prepared[0].font.size, 14.0);
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
        Mutation::ForeignInstance,
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
    for mutation in [
        Mutation::MissingReference,
        Mutation::DuplicateReference,
        Mutation::UnreferencedRow,
    ] {
        let projection = projection(mutation);
        assert!(matches!(
            consume_projection(&projection, current_protocol(), super::prepare),
            Err(UiHostSurfacePresentationDenial::MalformedProjection)
        ));
    }
}

fn consume_projection<T>(
    projection: &UiMountedProjectionView,
    protocol: UiHostProtocolAgreement,
    consume: impl FnOnce(&worth_ui_host_contract::UiMountedFrameConsumptionView<'_>) -> T,
) -> T {
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
        });
    let view = worth_ui_host_contract::UiMountedFrameConsumptionView::from_inert_mechanics(
        UiMountedFrameConsumptionInput {
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

fn current_protocol() -> UiHostProtocolAgreement {
    compatible(UiHostProtocolContract::current())
}

fn mounted_frame_revision_two() -> UiHostProtocolContract {
    UiHostProtocolContract::new(
        UiHostProtocolIdentity::worth_ui(),
        UiHostProtocolVersion::new(4),
        UiMountedFrameSchemaVersion::new(2),
        UiMountedPresentationSchemaVersion::new(3),
        UiHostProtocolContract::current().observation(),
        UiHostMeasurementSchemaVersion::new(3),
    )
}

fn compatible(contract: UiHostProtocolContract) -> UiHostProtocolAgreement {
    match contract.negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(denial) => panic!("{denial:?}"),
    }
}
