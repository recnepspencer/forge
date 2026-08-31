#[test]
fn runtime_can_complete_inert_mechanics_without_publication_authority() {
    use worth_ui_host_contract::{
        UiAppearanceClip, UiAppearanceDamageRegion, UiAppearancePhysicalRadii,
        UiAppearanceProjectionAttribution, UiHostPointerIdentity, UiMountedAppearanceColor,
        UiMountedAppearanceOpacity, UiMountedBackdropCompletionInput, UiMountedBackdropIdentity,
        UiMountedBackdropMechanic, UiMountedFrameIdentity, UiMountedInstanceIdentity,
        UiMountedLayerProjection, UiMountedLayerReference, UiMountedNodeReceiptIssuer,
        UiMountedOutlineAppearanceCompletionInput, UiMountedOutlineAppearanceMechanic,
        UiMountedPointerAffordanceMechanic, UiMountedPortalSurfaceAppearanceMechanic,
        UiMountedPresentationAttemptIdentity, UiMountedSurfaceAppearanceCompletionInput,
        UiMountedSurfaceAppearanceMechanic, UiMountedSurfacePaint,
        UiMountedTextForegroundAppearanceCompletionInput,
        UiMountedTextForegroundAppearanceMechanic, UiMountedTextPaintSpanIdentity,
        UiOverlayParticipantIdentity, UiOverlayPlacementReceipt, UiOverlayStackSnapshot,
        UiPointerAffordanceFamily, UiSemanticSurfaceIdentity,
    };

    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let node_issuer = UiMountedNodeReceiptIssuer::mint_for(frame).unwrap();
    let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let portal = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let pointer = UiHostPointerIdentity::new(9);
    let bounds = UiAppearanceDamageRegion::new(0, 0, 8, 8).unwrap();
    let clip = UiAppearanceClip::new(bounds);
    let color = UiMountedAppearanceColor::from_straight_srgba([12, 34, 56, 255]);
    let projection =
        UiAppearanceProjectionAttribution::from_runtime_mounting(node_issuer, 1, 1).unwrap();
    let surface_mechanic = UiMountedSurfaceAppearanceMechanic::complete_from_runtime_mounting(
        UiMountedSurfaceAppearanceCompletionInput {
            issuer: node_issuer,
            node_receipt: node_issuer.receipt_for(portal),
            bounds,
            clip,
            layer: UiMountedLayerProjection::Layer(UiMountedLayerReference::new(0)),
            visual_bounds: bounds,
            radii: UiAppearancePhysicalRadii::normalize(bounds, [0; 4]),
            paint: UiMountedSurfacePaint::Fill(color),
            opacity: UiMountedAppearanceOpacity::ONE,
            projection,
        },
    )
    .unwrap();
    let _portal_surface = UiMountedPortalSurfaceAppearanceMechanic::complete_from_runtime_mounting(
        portal,
        surface_mechanic,
    )
    .unwrap();
    let _outline = UiMountedOutlineAppearanceMechanic::complete_from_runtime_mounting(
        UiMountedOutlineAppearanceCompletionInput {
            issuer: node_issuer,
            node_receipt: node_issuer.receipt_for(portal),
            clip,
            visual_bounds: bounds,
            color,
            width: 1,
            offset: 0,
            radii: UiAppearancePhysicalRadii::normalize(bounds, [0; 4]),
            opacity: UiMountedAppearanceOpacity::ONE,
            projection,
        },
    )
    .unwrap();
    let _text = UiMountedTextForegroundAppearanceMechanic::complete_from_runtime_mounting(
        UiMountedTextForegroundAppearanceCompletionInput {
            issuer: node_issuer,
            paint_span: UiMountedTextPaintSpanIdentity::from_runtime_mounting([7; 32]),
            foreground: color,
            opacity: UiMountedAppearanceOpacity::ONE,
            projection,
        },
    )
    .unwrap();
    let backdrop_identity =
        UiMountedBackdropIdentity::from_runtime_mounting("dialog.backdrop").unwrap();
    let _backdrop = UiMountedBackdropMechanic::complete_from_runtime_mounting(
        UiMountedBackdropCompletionInput {
            issuer: node_issuer,
            identity: backdrop_identity.clone(),
            semantic_surface: surface,
            placement: UiOverlayPlacementReceipt::from_runtime_overlay_order(4, 1).unwrap(),
            bounds,
            clip,
            background: color,
            opacity: UiMountedAppearanceOpacity::ONE,
            projection,
        },
    )
    .unwrap();

    let affordance = UiMountedPointerAffordanceMechanic::complete_from_runtime_mounting(
        pointer,
        surface,
        portal,
        UiPointerAffordanceFamily::Default,
    );
    assert_eq!(affordance.pointer(), pointer);
    assert_eq!(affordance.target(), portal);

    let order = UiOverlayStackSnapshot::complete_from_runtime_overlay_order(
        surface,
        UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        4,
        7,
        [
            UiOverlayParticipantIdentity::Portal(portal),
            UiOverlayParticipantIdentity::Backdrop(backdrop_identity),
        ],
    )
    .unwrap();
    assert_eq!(order.bottom_to_top().len(), 2);
}
