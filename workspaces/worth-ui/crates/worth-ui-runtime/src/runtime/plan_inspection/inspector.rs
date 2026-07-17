use crate::runtime::plan_equivalence::WorthUiExecutionPlanDigestor;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiArtifactToPlanProvenance, WorthUiExecutionPlan,
    WorthUiExecutionPlanInspection, WorthUiLaneInspection, WorthUiPlanInspectionCounters,
    WorthUiPlanInspectionDenial, WorthUiPlanInspectionDenialReason, WorthUiPlanNode,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanNodeInspection,
    WorthUiPlanProvenanceSource, WorthUiQueryInspectionLinks, WorthUiRuntimeHandleAllocationBasis,
};

pub(crate) struct WorthUiExecutionPlanInspector;

impl WorthUiExecutionPlanInspector {
    pub(crate) fn inspect(
        plan: &WorthUiExecutionPlan,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> Result<WorthUiExecutionPlanInspection, WorthUiPlanInspectionDenial> {
        let lowering_basis = allocation_planning
            .lowering_basis()
            .expect("admitted allocation planning must expose lowered basis");
        let node_inputs = allocation_planning
            .node_inputs()
            .expect("admitted allocation planning must expose lowered node inputs");
        let mut counters = WorthUiPlanInspectionCounters::default();
        counters.record_inspection();
        validate_plan_input_alignment(plan, allocation_planning, counters)?;

        let mut provenance = Vec::with_capacity(node_inputs.len());
        let mut nodes = Vec::with_capacity(plan.topology().traversal_order().len());
        for (node, node_input) in plan.topology().traversal_order().iter().zip(node_inputs) {
            let node_provenance = provenance_for_node(node, node_input, &mut counters);
            counters.record_provenance_link();
            counters.record_node_inspection();
            nodes.push(WorthUiPlanNodeInspection::new(
                node.runtime_handle().plan_index(),
                node.runtime_handle(),
                node.family(),
                node.child_range(),
                node.region_structure(),
                node.egui_boundary().cloned(),
                node.render_resource_ref(),
                node_provenance.clone(),
            ));
            provenance.push(node_provenance);
        }

        let lanes = plan
            .lane_partitions()
            .iter()
            .map(|lane| {
                counters.record_lane_inspection();
                WorthUiLaneInspection::new(
                    lane.lane(),
                    lane.plan_indexes().to_vec(),
                    lane.node_count(),
                )
            })
            .collect();
        counters.record_plan_digest();
        let active_artifact_digest = lowering_basis.active_artifact_digest();
        let handle_basis_digest = plan.handle_receipt().basis_digest();
        let plan_digest = WorthUiExecutionPlanDigestor::digest(plan).0;

        Ok(WorthUiExecutionPlanInspection::new(
            active_artifact_digest,
            handle_basis_digest,
            plan_digest,
            nodes,
            lanes,
            provenance,
            counters,
        ))
    }
}

fn validate_plan_input_alignment(
    plan: &WorthUiExecutionPlan,
    allocation_planning: &WorthUiAllocationPlanning,
    counters: WorthUiPlanInspectionCounters,
) -> Result<(), WorthUiPlanInspectionDenial> {
    let node_inputs = allocation_planning
        .node_inputs()
        .expect("admitted allocation planning must expose lowered node inputs");
    let allocation_basis =
        WorthUiRuntimeHandleAllocationBasis::from_allocation_planning(allocation_planning);
    if !plan.handle_receipt().certifies_basis(&allocation_basis) {
        return Err(denial(
            WorthUiPlanInspectionDenialReason::PlanInputReceiptMismatch,
            counters,
        ));
    }
    if plan.topology().traversal_order().len() != node_inputs.len() {
        return Err(denial(
            WorthUiPlanInspectionDenialReason::PlanInputNodeCountMismatch,
            counters,
        ));
    }
    for (position, (node, node_input)) in plan
        .topology()
        .traversal_order()
        .iter()
        .zip(node_inputs)
        .enumerate()
    {
        let plan_index = u32::try_from(position).map_err(|_| {
            denial(
                WorthUiPlanInspectionDenialReason::RuntimeHandlePlanIndexMismatch,
                counters,
            )
        })?;
        if node.runtime_handle().plan_index() != plan_index {
            return Err(denial(
                WorthUiPlanInspectionDenialReason::RuntimeHandlePlanIndexMismatch,
                counters,
            ));
        }
        if node.family().input_family() != node_input.family() {
            return Err(denial(
                WorthUiPlanInspectionDenialReason::PlanNodeFamilyMismatch,
                counters,
            ));
        }
    }
    Ok(())
}

fn provenance_for_node(
    node: &WorthUiPlanNode,
    node_input: &WorthUiPlanNodeInput,
    counters: &mut WorthUiPlanInspectionCounters,
) -> WorthUiArtifactToPlanProvenance {
    let query_links = query_links_for_node_input(node_input, counters);
    WorthUiArtifactToPlanProvenance::new(
        node.runtime_handle().plan_index(),
        node_input.identity_basis().to_owned(),
        node_input.authored_provenance_digest(),
        node_input.family(),
        provenance_source_for_node_input(node_input),
        capability_reference_for_node_input(node_input),
        query_links,
    )
}

fn query_links_for_node_input(
    node_input: &WorthUiPlanNodeInput,
    counters: &mut WorthUiPlanInspectionCounters,
) -> Option<WorthUiQueryInspectionLinks> {
    let identity = node_input.query_binding_identity()?.clone();
    let posture = node_input.query_binding_posture()?;
    counters.record_query_link_preservation();
    counters.record_projection_consumption_link();
    Some(WorthUiQueryInspectionLinks::from_query_posture(
        identity,
        posture.clone(),
        node_input.query_preservation_receipt(),
        node_input.query_required_surfaces().to_vec(),
    ))
}

fn capability_reference_for_node_input(node_input: &WorthUiPlanNodeInput) -> Option<String> {
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

fn provenance_source_for_node_input(
    node_input: &WorthUiPlanNodeInput,
) -> WorthUiPlanProvenanceSource {
    if node_input.query_binding_identity().is_some() && node_input.query_binding_posture().is_some()
    {
        return WorthUiPlanProvenanceSource::QueryBinding;
    }
    provenance_source_for_family(node_input.family())
}

fn provenance_source_for_family(family: WorthUiPlanNodeInputFamily) -> WorthUiPlanProvenanceSource {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => {
            WorthUiPlanProvenanceSource::ComponentLoweringHook
        }
        WorthUiPlanNodeInputFamily::LanePartitionRef => WorthUiPlanProvenanceSource::LaneBoundary,
        WorthUiPlanNodeInputFamily::DiagnosticsRef
        | WorthUiPlanNodeInputFamily::EguiBoundaryRef => WorthUiPlanProvenanceSource::Diagnostics,
        WorthUiPlanNodeInputFamily::RenderResourceRef => {
            WorthUiPlanProvenanceSource::RenderResource
        }
        _ => WorthUiPlanProvenanceSource::ReplacementClassification,
    }
}

fn denial(
    reason: WorthUiPlanInspectionDenialReason,
    counters: WorthUiPlanInspectionCounters,
) -> WorthUiPlanInspectionDenial {
    WorthUiPlanInspectionDenial::new(reason, counters)
}
