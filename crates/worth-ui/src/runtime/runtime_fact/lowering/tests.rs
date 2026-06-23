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
