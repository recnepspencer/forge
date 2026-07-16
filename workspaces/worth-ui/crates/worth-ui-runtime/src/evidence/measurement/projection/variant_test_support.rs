#![cfg(test)]

use worth_ui_query_binding::certification::{
    worth_ui_query_prerequisite_fixture, WorthUiQueryCertificationProjection,
};

use crate::graph::UiGraphWorldProfile;

pub(crate) fn display_field_plus_entity_identity_projection_context(
    lane_label: &str,
) -> (
    worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::WorthUiQueryAuthorityHandle,
    UiGraphWorldProfile,
) {
    let (prerequisites, authority) = worth_ui_query_prerequisite_fixture(
        lane_label,
        WorthUiQueryCertificationProjection::DisplayFieldAndEntityIdentities,
    );
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(prerequisites.clone());
    (prerequisites, authority, world_profile)
}
