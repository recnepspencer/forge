use super::{
    prepare_mounted_semantic_text, UiMountedEventTimeDpiAuthority,
    UiNativeTextPresentationPreparation, UiNativeTextPresentationReadiness,
};
use crate::certification_support::{
    initial_presentation_mechanics_for_certification, semantic_text_projection_for_certification,
    UiSemanticTextProjectionCertificationMutation,
};
use crate::mounting::qualified_text_test_support::inert_qualified_layout;
use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationMode, UiMountedPresentationWorkView,
    UiMountedSurfaceBindingRequirement, WorthUiHostCapabilityObservationGeneration,
};

fn requirement(
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> UiMountedSurfaceBindingRequirement {
    UiMountedSurfaceBindingRequirement::new(
        projection.surface(),
        UiHostSurfaceIdentity::mint_unbound().unwrap(),
        projection.binding(),
        WorthUiHostCapabilityObservationGeneration::new(7),
        11,
        UiHostSurfacePresentationMode::NativeDisplay,
    )
}

#[test]
fn demand_preparation_stops_at_typed_atlas_plan_boundary_without_raster_work() {
    let projection = semantic_text_projection_for_certification(
        UiSemanticTextProjectionCertificationMutation::Exact,
    );
    let requirement = requirement(&projection);
    let initial = initial_presentation_mechanics_for_certification(&projection, requirement);
    let layout = inert_qualified_layout("ONLINE");
    let dpi = UiMountedEventTimeDpiAuthority::from_requirement(requirement).unwrap();
    let preparation = prepare_mounted_semantic_text(
        UiMountedPresentationWorkView::Initial(&initial),
        dpi,
        |_| Some(layout.as_ref()),
    )
    .unwrap();
    let UiNativeTextPresentationPreparation::Prepared(prepared) = preparation else {
        panic!("exact mounted text must prepare a native transaction");
    };
    assert_eq!(prepared.layout_count(), 1);
    assert_eq!(prepared.demand_batches().len(), 1);
    let planning = prepared.planning_inspection().unwrap();
    assert_eq!(planning.demand_batches(), 1);
    assert_eq!(
        usize::try_from(planning.demand_records()).unwrap(),
        prepared.demand_batches()[0].records().len()
    );
    assert_eq!(planning.key_checks(), planning.demand_records());
    assert_eq!(prepared.raster_work().rasterized_glyphs(), 0);
    assert_eq!(prepared.raster_work().rasterized_texels(), 0);
    assert_eq!(prepared.raster_work().produced_bytes(), 0);
    assert_eq!(prepared.paint_span_count(), 1);
}

#[test]
fn consumer_layout_substitution_is_rejected_before_layout_admission() {
    let projection = semantic_text_projection_for_certification(
        UiSemanticTextProjectionCertificationMutation::Exact,
    );
    let requirement = requirement(&projection);
    let initial = initial_presentation_mechanics_for_certification(&projection, requirement);
    let substituted = inert_qualified_layout("SYSTEM FONT SUBSTITUTE");
    let dpi = UiMountedEventTimeDpiAuthority::from_requirement(requirement).unwrap();
    let preparation = prepare_mounted_semantic_text(
        UiMountedPresentationWorkView::Initial(&initial),
        dpi,
        |_| Some(substituted.as_ref()),
    )
    .unwrap();
    let UiNativeTextPresentationPreparation::Denied(denial) = preparation else {
        panic!("a substituted layout must be denied before preparation");
    };
    assert_eq!(
        denial.readiness(),
        UiNativeTextPresentationReadiness::SemanticTextLayoutMismatch
    );
}
