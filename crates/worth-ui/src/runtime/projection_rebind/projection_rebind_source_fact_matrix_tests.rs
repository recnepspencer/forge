use super::projection_rebind_test_support::{
    header_frame_plan, page_host_plan, page_host_source_text, prepare_projection_rebind_plan,
    projection_rebind_app, runtime_for_source_authored_page_host, validation_activated_for_source,
};
use crate::runtime::{WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus};

#[test]
fn content_slot_edit_rebuilds_page_host_and_preserves_header() {
    assert_source_fact_matrix(
        page_host_source_text().replace(
            "collection -> validation.surface.products.collection",
            "collection -> validation.surface.products.toolbar",
        ),
        WorthUiPageHostRebindStatus::ReboundAfterActivation,
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation,
    );
}

#[test]
fn layout_gap_edit_rebuilds_page_host_and_preserves_header() {
    assert_source_fact_matrix(
        page_host_source_text().replace("column {", "column gap(24) {"),
        WorthUiPageHostRebindStatus::ReboundAfterActivation,
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation,
    );
}

#[test]
fn layout_padding_edit_rebuilds_page_host_and_preserves_header() {
    assert_source_fact_matrix(
        page_host_source_text().replace("column {", "column padding(18) {"),
        WorthUiPageHostRebindStatus::ReboundAfterActivation,
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation,
    );
}

fn assert_source_fact_matrix(
    edited_source: String,
    expected_page_host_status: WorthUiPageHostRebindStatus,
    expected_header_status: WorthUiHeaderFrameRebindStatus,
) {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let header_plan = header_frame_plan(&app);
    let page_plan = page_host_plan(&runtime);
    let evidence = validation_activated_for_source(&mut runtime, edited_source);
    let admitted = runtime.admit_validation_runtime_change(&evidence).unwrap();

    let (_, page_receipt) = runtime
        .rebind_page_host_after_reload(
            &page_plan,
            crate::runtime::WorthUiPageHostRequest::new("ProductsPage"),
            &evidence,
        )
        .expect("page host rebind should run through activated source evidence");
    let header_receipt = match prepare_projection_rebind_plan(&runtime, header_plan, &admitted) {
        crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
            preserved.complete_preserved().1
        }
        crate::runtime::WorthUiProjectionRebindPlan::Rebuild(activated) => {
            activated
                .complete_rebuild(
                    runtime
                        .admit_projection_plan(header_frame_plan(&app))
                        .unwrap(),
                )
                .1
        }
    };

    assert_eq!(page_receipt.status(), expected_page_host_status);
    assert_eq!(
        page_receipt
            .projection_rebind_batch()
            .counters()
            .dependency_intersection_count(),
        1
    );
    assert_eq!(
        header_receipt.rows()[0].status(),
        expected_header_status.into()
    );
    assert_eq!(header_receipt.counters().dependency_intersection_count(), 0);
}

impl From<WorthUiHeaderFrameRebindStatus> for crate::runtime::WorthUiProjectionRebindStatus {
    fn from(status: WorthUiHeaderFrameRebindStatus) -> Self {
        match status {
            WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload => {
                Self::PreservedEquivalentReload
            }
            WorthUiHeaderFrameRebindStatus::PreservedDeniedReload => Self::PreservedDeniedReload,
            WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation => {
                Self::EquivalentAfterActivation
            }
            WorthUiHeaderFrameRebindStatus::ReboundAfterActivation => Self::ReboundAfterActivation,
        }
    }
}
