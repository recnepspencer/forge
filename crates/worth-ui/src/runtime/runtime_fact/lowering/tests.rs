use super::WorthUiAuthoredStructuralRuntimeFactLowering;
use crate::runtime::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture,
    WorthUiAuthoredDeltaCounters, WorthUiAuthoredDeltaDigest, WorthUiAuthoredDeltaSummary,
    WorthUiAuthoredSemanticSubject, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiSemanticSliceId, WorthUiTouchedAuthoredDeclarationRow,
    WorthUiTouchedAuthoredSemanticSliceRow,
};

#[test]
fn content_slot_assignment_lowers_into_exact_structural_families() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::ContentSlotAssignment,
            WorthUiAuthoredSemanticSubject::PageSlot {
                page_name: "HeaderProofPage".to_owned(),
                slot_name: "proof".to_owned(),
            },
        )]),
    );
    let changed_facts = rows[0].changed_facts();

    assert_eq!(rows[0].changed_fact_count(), 2);
    assert_eq!(
        rows[0].changed_fact_families(),
        &[
            WorthUiRuntimeFactFamily::ContentMount,
            WorthUiRuntimeFactFamily::PageContentSlot,
        ]
    );
    assert!(
        changed_facts.contains_exact(&WorthUiRuntimeFactId::content_mount(
            "HeaderProofPage.proof",
        ))
    );
}

#[test]
fn lowering_returns_empty_changed_facts_for_subject_slice_mismatch() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::AuthoredMountComponentSelection,
            WorthUiAuthoredSemanticSubject::Page {
                page_name: "HeaderProofPage".to_owned(),
            },
        )]),
    );

    assert_eq!(rows.len(), 1);
    assert!(rows[0].changed_facts().is_empty());
    assert!(rows[0].changed_fact_families().is_empty());
}

#[test]
fn authored_surface_props_lower_into_surface_prop_fact_family() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
            WorthUiAuthoredSemanticSubject::Surface {
                surface_id: "worth.surface.preview.dashboard.hero".to_owned(),
            },
        )]),
    );

    assert_eq!(
        rows[0].changed_fact_families(),
        &[WorthUiRuntimeFactFamily::AuthoredSurfaceProps]
    );
    assert!(rows[0]
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::AuthoredSurfaceProps));
}

#[test]
fn primitive_content_changes_also_touch_primitive_construction_graph() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::PrimitiveContent,
            WorthUiAuthoredSemanticSubject::Surface {
                surface_id: "worth.surface.preview.primitive.proof".to_owned(),
            },
        )]),
    );

    assert_eq!(
        rows[0].changed_fact_families(),
        &[
            WorthUiRuntimeFactFamily::PrimitiveConstruction,
            WorthUiRuntimeFactFamily::PrimitiveContent,
            WorthUiRuntimeFactFamily::PrimitiveDrawPlan,
            WorthUiRuntimeFactFamily::PrimitiveEventRegion,
        ]
    );
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_construction(
            "worth.surface.preview.primitive.proof"
        )));
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_content(
            "worth.surface.preview.primitive.proof"
        )));
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_draw_plan(
            "worth.surface.preview.primitive.proof"
        )));
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_region(
            "worth.surface.preview.primitive.proof"
        )));
}

#[test]
fn primitive_flow_layout_changes_touch_draw_plan_and_event_region_facts() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::PrimitiveFlowLayout,
            WorthUiAuthoredSemanticSubject::Surface {
                surface_id: "worth.surface.preview.primitive.proof".to_owned(),
            },
        )]),
    );

    assert_eq!(
        rows[0].changed_fact_families(),
        &[
            WorthUiRuntimeFactFamily::PrimitiveConstruction,
            WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
            WorthUiRuntimeFactFamily::PrimitiveDrawPlan,
            WorthUiRuntimeFactFamily::PrimitiveEventRegion,
        ]
    );
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_draw_plan(
            "worth.surface.preview.primitive.proof"
        )));
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_region(
            "worth.surface.preview.primitive.proof"
        )));
}

#[test]
fn primitive_event_geometry_changes_touch_event_region_fact() {
    let rows = WorthUiAuthoredStructuralRuntimeFactLowering::from_authored_delta_summary(
        &authored_summary(vec![semantic_row(
            WorthUiSemanticSliceId::PrimitiveEventGeometry,
            WorthUiAuthoredSemanticSubject::Surface {
                surface_id: "worth.surface.preview.primitive.proof".to_owned(),
            },
        )]),
    );

    assert_eq!(
        rows[0].changed_fact_families(),
        &[
            WorthUiRuntimeFactFamily::PrimitiveConstruction,
            WorthUiRuntimeFactFamily::PrimitiveEventGeometry,
            WorthUiRuntimeFactFamily::PrimitiveEventRegion,
        ]
    );
    assert!(rows[0]
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_region(
            "worth.surface.preview.primitive.proof"
        )));
}

fn authored_summary(
    semantic_slice_rows: Vec<WorthUiTouchedAuthoredSemanticSliceRow>,
) -> WorthUiAuthoredDeltaSummary {
    WorthUiAuthoredDeltaSummary::new(
        WorthUiAuthoredDeltaDigest::from_basis(&["phase-24".to_owned()]),
        WorthUiAuthoredDeltaCounters::default(),
        vec![WorthUiTouchedAuthoredDeclarationRow::new(
            WorthUiAuthoredDeclarationKind::Content,
            "HeaderProofPage",
            WorthUiAuthoredDeltaChangePosture::Changed,
        )],
        semantic_slice_rows,
    )
}

fn semantic_row(
    slice_id: WorthUiSemanticSliceId,
    subject: WorthUiAuthoredSemanticSubject,
) -> WorthUiTouchedAuthoredSemanticSliceRow {
    WorthUiTouchedAuthoredSemanticSliceRow::new(
        slice_id,
        subject,
        WorthUiAuthoredDeltaChangePosture::Changed,
    )
}
