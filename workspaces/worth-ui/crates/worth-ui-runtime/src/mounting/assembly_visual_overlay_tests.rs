use super::UiMountedFrameRequest;

#[test]
fn overlay_revision_and_projection_are_exact_request_reuse_dependencies() {
    let overlay = overlay_projection();
    let first = UiMountedFrameRequest::all_bound_surfaces().with_visual_overlay(1, Some(overlay));
    let exact = UiMountedFrameRequest::all_bound_surfaces().with_visual_overlay(1, Some(overlay));
    let revised = UiMountedFrameRequest::all_bound_surfaces().with_visual_overlay(2, Some(overlay));
    let cleared = UiMountedFrameRequest::all_bound_surfaces().with_visual_overlay(3, None);

    assert_eq!(first, exact);
    assert_ne!(first, revised);
    assert_ne!(revised, cleared);
    assert_eq!(first.visual_overlay_revision(), 1);
    assert_eq!(
        crate::mounting::UiMountedFrameReuseContract::canonical_dependency_order().last(),
        Some(&crate::mounting::UiMountedFrameReuseDependency::VisualOverlay)
    );
}

fn overlay_projection() -> crate::mounting::UiMountedVisualOverlayProjectionInput {
    let base_frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
    let instance = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let target_receipt = worth_ui_host_contract::UiMountedNodeReceiptIssuer::mint_for(base_frame)
        .unwrap()
        .receipt_for(instance);
    crate::mounting::UiMountedVisualOverlayProjectionInput {
        overlay_identity: 7,
        base_snapshot: 11,
        base_frame,
        target_receipt,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        coordinate_basis: coordinate_basis(),
        target_region: worth_ui_host_contract::UiMountedClientPhysicalRect::from_runtime_mounting(
            32, 20, 128, 76,
        )
        .unwrap(),
    }
}

fn coordinate_basis() -> worth_ui_host_contract::UiMountedClientCoordinateBasis {
    worth_ui_host_contract::UiMountedClientCoordinateBasis::from_runtime_mounting(
        worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
            worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host(
                [0, 0],
                [160, 96],
            ),
            worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
                [160.0, 96.0],
                [1.0, 1.0],
                [0.0, 0.0],
            ),
            worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
                worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
                worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
            ),
        ),
    )
    .unwrap()
}
