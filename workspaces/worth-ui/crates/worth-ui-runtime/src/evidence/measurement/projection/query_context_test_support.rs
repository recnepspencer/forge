use worth_ui_query_binding::certification::{
    worth_ui_query_prerequisite_fixture, WorthUiQueryCertificationProjection,
};

use crate::graph::UiGraphWorldProfile;

pub(crate) fn display_field_projection_consumption(
    lane_label: &str,
) -> (
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
) {
    worth_ui_query_prerequisite_fixture(
        lane_label,
        WorthUiQueryCertificationProjection::DisplayField,
    )
}

pub(crate) fn display_field_projection_context(
    lane_label: &str,
) -> (
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
    UiGraphWorldProfile,
) {
    projection_context(
        lane_label,
        WorthUiQueryCertificationProjection::DisplayField,
    )
}

pub(crate) fn entity_identity_projection_context(
    lane_label: &str,
) -> (
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
    UiGraphWorldProfile,
) {
    projection_context(
        lane_label,
        WorthUiQueryCertificationProjection::EntityIdentities,
    )
}

fn projection_context(
    lane_label: &str,
    projection: WorthUiQueryCertificationProjection,
) -> (
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
    UiGraphWorldProfile,
) {
    let (prerequisites, authority) = worth_ui_query_prerequisite_fixture(lane_label, projection);
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(prerequisites.clone());
    (prerequisites, authority, world_profile)
}
