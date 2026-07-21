use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanInput, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily, WorthUiPlanProvenanceSource, WorthUiQueryInspectionLinks,
};

pub(super) fn expected_provenance_for_node_input(
    plan_index: u32,
    node_input: &WorthUiPlanNodeInput,
) -> WorthUiArtifactToPlanProvenance {
    WorthUiArtifactToPlanProvenance::new(
        plan_index,
        node_input.identity_basis().to_owned(),
        node_input.authored_provenance_digest(),
        node_input.family(),
        expected_provenance_source_for_node_input(node_input),
        expected_capability_reference_for_node_input(node_input),
        expected_query_links_for_node_input(node_input),
    )
}

pub(super) fn expected_query_links_from_plan_input(
    plan_input: &WorthUiExecutionPlanInput,
) -> Vec<WorthUiQueryInspectionLinks> {
    plan_input
        .node_inputs()
        .iter()
        .filter_map(expected_query_links_for_node_input)
        .collect()
}

fn expected_query_links_for_node_input(
    node_input: &WorthUiPlanNodeInput,
) -> Option<WorthUiQueryInspectionLinks> {
    let identity = node_input.query_binding_identity()?.clone();
    let settled_fact_link = node_input.query_settled_fact_link()?.clone();
    Some(WorthUiQueryInspectionLinks::from_settled_fact_link(
        identity,
        settled_fact_link,
        node_input.query_preservation_receipt(),
    ))
}

fn expected_capability_reference_for_node_input(
    node_input: &WorthUiPlanNodeInput,
) -> Option<String> {
    match node_input.family() {
        WorthUiPlanNodeInputFamily::QueryViewBinding => node_input
            .query_binding_identity()
            .map(|identity| identity.view_binding_id().to_owned())
            .or_else(|| Some(node_input.identity_basis().to_owned())),
        WorthUiPlanNodeInputFamily::ComponentInvocation
        | WorthUiPlanNodeInputFamily::Command
        | WorthUiPlanNodeInputFamily::TokenStyle
        | WorthUiPlanNodeInputFamily::LayoutRegion
        | WorthUiPlanNodeInputFamily::RenderResourceRef => {
            Some(node_input.identity_basis().to_owned())
        }
        _ => None,
    }
}

fn expected_provenance_source_for_node_input(
    node_input: &WorthUiPlanNodeInput,
) -> WorthUiPlanProvenanceSource {
    if node_input.query_binding_identity().is_some()
        && node_input.query_settled_fact_link().is_some()
    {
        return WorthUiPlanProvenanceSource::QueryBinding;
    }
    expected_provenance_source_for_family(node_input.family())
}

fn expected_provenance_source_for_family(
    family: WorthUiPlanNodeInputFamily,
) -> WorthUiPlanProvenanceSource {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => {
            WorthUiPlanProvenanceSource::ComponentLoweringHook
        }
        WorthUiPlanNodeInputFamily::LanePartitionRef => WorthUiPlanProvenanceSource::LaneBoundary,
        WorthUiPlanNodeInputFamily::DiagnosticsRef => WorthUiPlanProvenanceSource::Diagnostics,
        WorthUiPlanNodeInputFamily::RenderResourceRef => {
            WorthUiPlanProvenanceSource::RenderResource
        }
        _ => WorthUiPlanProvenanceSource::ReplacementClassification,
    }
}
