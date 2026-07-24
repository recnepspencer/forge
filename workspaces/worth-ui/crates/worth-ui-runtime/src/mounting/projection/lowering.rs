use std::collections::BTreeMap;

use super::frame_storage::{UiMountedProjectionNodeRecord, UiMountedProjectionSurface};
use super::geometry::lower_allocation;
use super::node_receipt::UiMountedNodeReceiptInput;
use super::participation::lower_participation;
use super::{UiMountedNodeReceipt, UiMountedProjectionDenial, UiMountedProjectionFrame};

pub(crate) struct UiPreparedMountedProjection {
    plan_digest: u64,
    nodes: Vec<UiMountedProjectionNodeDraft>,
    surfaces: Vec<UiMountedProjectionSurface>,
    ordinary: Option<crate::runtime::WorthUiOrdinaryLaneFrameReceipt>,
    virtualized: Option<crate::runtime::WorthUiVirtualizedDataFrameReceipt>,
    canvas: Option<(crate::runtime::WorthUiCanvasSpatialFrameReceipt, u64)>,
    realtime: Option<crate::runtime::WorthUiRealtimeFrameReceipt>,
    preview: Option<UiMountedPreviewProjectionInput>,
}

pub(crate) struct UiMountedProjectionInput<'input, 'graph> {
    pub(crate) graph: crate::graph::UiGraphAuthority<'graph>,
    pub(crate) plan_digest: u64,
    pub(crate) plan_rows: &'input [(u64, u32)],
    pub(crate) allocation_receipts: &'input [crate::runtime::UiAllocationReceipt],
    pub(crate) requested_surfaces: &'input [worth_ui_host_contract::UiSemanticSurfaceIdentity],
    pub(crate) preview: Option<UiMountedPreviewProjectionInput>,
}

#[derive(Clone, Copy)]
pub(crate) struct UiMountedPreviewProjectionInput {
    pub(crate) mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    pub(crate) graph_node: crate::graph::UiGraphNodeIdentity,
    pub(crate) frame_epoch: u64,
    pub(crate) extent_subpixels: u32,
    pub(crate) candidate_count: u16,
    pub(crate) all_candidates_admitted: bool,
}

struct UiMountedNodeLoweringContext<'input, 'graph> {
    graph: crate::graph::UiGraphAuthority<'graph>,
    plan_by_provenance: &'input BTreeMap<u64, u32>,
    allocation_by_node: &'input BTreeMap<
        crate::graph::UiGraphNodeIdentity,
        &'input crate::runtime::UiAllocationReceipt,
    >,
    plan_digest: u64,
}

struct UiMountedProjectionNodeDraft {
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    graph_node: crate::graph::UiGraphNodeIdentity,
    semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    incarnation: worth_ui_host_contract::UiMountIncarnation,
    plan_digest: u64,
    role: worth_ui_host_contract::UiMountedMechanicalRole,
    participation: worth_ui_host_contract::UiMountedParticipation,
    allocation: worth_ui_host_contract::UiMountedAllocationProjection,
    plan_index: Option<u32>,
}

#[derive(Clone)]
pub struct UiProjectedMountedFrameCandidate {
    pub(in crate::mounting) frame: UiMountedProjectionFrame,
    pub(in crate::mounting) identity_candidate:
        super::super::identity_state::UiMountedIdentityFrameCandidate,
}

impl UiProjectedMountedFrameCandidate {
    pub fn frame(&self) -> &UiMountedProjectionFrame {
        &self.frame
    }

    pub fn is_unpublished(&self) -> bool {
        let _ = &self.identity_candidate;
        true
    }

    pub(crate) fn presented_receipts(
        &self,
    ) -> impl Iterator<
        Item = (
            worth_ui_host_contract::UiMountedInstanceIdentity,
            worth_ui_host_contract::UiMountedNodeReceiptIdentity,
        ),
    > + '_ {
        self.identity_candidate.presented_receipts()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiMountedProjectionFrame,
        super::super::identity_state::UiMountedIdentityFrameCandidate,
    ) {
        (self.frame, self.identity_candidate)
    }
}

impl UiPreparedMountedProjection {
    pub(crate) fn record_ordinary(
        &mut self,
        receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.ordinary)?;
        self.ordinary = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.virtualized)?;
        self.virtualized = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn record_canvas(
        &mut self,
        receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        resource_content_identity: u64,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.canvas)?;
        self.canvas = Some((receipt.clone(), resource_content_identity));
        Ok(())
    }

    pub(crate) fn record_realtime(
        &mut self,
        receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.realtime)?;
        self.realtime = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn finish(
        self,
        state: &super::super::UiMountedIdentityState,
    ) -> Result<UiProjectedMountedFrameCandidate, UiMountedProjectionDenial> {
        self.validate_capacity()?;
        let identity_candidate = state.prepare_frame_candidate()?;
        let identity_view = state.projection_identity_view(&identity_candidate);
        let frame_receipts = identity_view
            .frame_receipts()
            .iter()
            .map(|receipt| (receipt.mounted_instance_identity(), *receipt))
            .collect::<BTreeMap<_, _>>();
        let nodes = self
            .nodes
            .into_iter()
            .map(|draft| draft.materialize(&frame_receipts))
            .collect::<Result<Vec<_>, UiMountedProjectionDenial>>()?;
        let mut frame = UiMountedProjectionFrame::new(
            identity_candidate.frame(),
            self.plan_digest,
            nodes,
            self.surfaces,
        );
        if let Some(receipt) = self.ordinary.as_ref() {
            frame.record_ordinary(receipt)?;
        }
        if let Some(receipt) = self.virtualized.as_ref() {
            frame.record_virtualized(receipt)?;
        }
        if let Some((receipt, resource)) = self.canvas.as_ref() {
            frame.record_canvas(receipt, *resource)?;
        }
        if let Some(receipt) = self.realtime.as_ref() {
            frame.record_realtime(receipt)?;
        }
        if let Some(preview) = self.preview {
            frame.record_preview(preview)?;
        }
        Ok(UiProjectedMountedFrameCandidate {
            frame,
            identity_candidate,
        })
    }

    fn validate_capacity(&self) -> Result<(), UiMountedProjectionDenial> {
        let paint_rows = usize::from(self.ordinary.is_some())
            + usize::from(self.virtualized.is_some())
            + usize::from(self.canvas.is_some())
            + usize::from(self.realtime.is_some());
        if paint_rows > 2_048 {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        Ok(())
    }
}

pub(crate) fn prepare_projection(
    state: &super::super::UiMountedIdentityState,
    input: UiMountedProjectionInput<'_, '_>,
) -> Result<UiPreparedMountedProjection, UiMountedProjectionDenial> {
    let plan_by_provenance = unique_plan_rows(input.plan_rows)?;
    let allocation_by_node = unique_allocations(input.graph, input.allocation_receipts)?;
    let lowering = UiMountedNodeLoweringContext {
        graph: input.graph,
        plan_by_provenance: &plan_by_provenance,
        allocation_by_node: &allocation_by_node,
        plan_digest: input.plan_digest,
    };
    let identity_view = state.view();
    let nodes = identity_view
        .mounted_instances()
        .iter()
        .filter(|instance| {
            input
                .requested_surfaces
                .contains(&instance.basis().semantic_surface_identity())
        })
        .map(|instance| lowering.lower(instance))
        .collect::<Result<Vec<_>, UiMountedProjectionDenial>>()?;
    let surfaces = identity_view
        .surface_bindings()
        .iter()
        .filter(|binding| {
            input
                .requested_surfaces
                .contains(&binding.semantic_surface_identity())
        })
        .map(|binding| {
            let surface = binding.semantic_surface_identity();
            let audience = state
                .audience_for(surface)
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            Ok(UiMountedProjectionSurface {
                surface,
                binding: binding.binding_generation(),
                audience,
            })
        })
        .collect::<Result<Vec<_>, UiMountedProjectionDenial>>()?;
    Ok(UiPreparedMountedProjection {
        plan_digest: input.plan_digest,
        nodes,
        surfaces,
        ordinary: None,
        virtualized: None,
        canvas: None,
        realtime: None,
        preview: input.preview,
    })
}

impl UiMountedNodeLoweringContext<'_, '_> {
    fn lower(
        &self,
        instance: &super::super::UiMountedInstanceIdentityView,
    ) -> Result<UiMountedProjectionNodeDraft, UiMountedProjectionDenial> {
        let graph_node = self
            .graph
            .lookup()
            .graph_node(instance.graph_node_identity())
            .ok_or(UiMountedProjectionDenial::UnknownGraphNode)?
            .value();
        let provenance = graph_node.authored_provenance_digest();
        let plan_index = self.plan_by_provenance.get(&provenance).copied();
        let allocation = lower_allocation(
            self.allocation_by_node
                .get(&instance.graph_node_identity())
                .copied(),
        )?;
        Ok(UiMountedProjectionNodeDraft {
            mounted_instance: instance.identity(),
            graph_node: instance.graph_node_identity(),
            semantic_surface: instance.basis().semantic_surface_identity(),
            incarnation: instance.mount_incarnation(),
            plan_digest: self.plan_digest,
            role: mechanical_role(graph_node.operator_kind()),
            participation: lower_participation(graph_node.participation_posture()),
            allocation,
            plan_index,
        })
    }
}

impl UiMountedProjectionNodeDraft {
    fn materialize(
        self,
        identities: &BTreeMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            super::super::UiMountedFrameIdentityView,
        >,
    ) -> Result<UiMountedProjectionNodeRecord, UiMountedProjectionDenial> {
        let identity = identities
            .get(&self.mounted_instance)
            .ok_or(UiMountedProjectionDenial::ForeignMountIncarnation)?
            .node_receipt_identity();
        Ok(UiMountedProjectionNodeRecord {
            receipt: UiMountedNodeReceipt::from_input(UiMountedNodeReceiptInput {
                identity,
                mounted_instance: self.mounted_instance,
                graph_node: self.graph_node,
                semantic_surface: self.semantic_surface,
                incarnation: self.incarnation,
                plan_digest: self.plan_digest,
                role: self.role,
                participation: self.participation,
                allocation: self.allocation,
            }),
            plan_index: self.plan_index,
        })
    }
}

fn mechanical_role(
    operator: crate::declaration::UiDeclarationPlanningOperatorKind,
) -> worth_ui_host_contract::UiMountedMechanicalRole {
    use crate::declaration::UiDeclarationPlanningOperatorKind as Operator;
    use worth_ui_host_contract::UiMountedMechanicalRole as Role;

    match operator {
        Operator::PageRoot | Operator::PageSet => Role::Surface,
        Operator::Control => Role::Control,
        Operator::DiagnosticSurface => Role::Diagnostic,
        Operator::PortalAnchor => Role::Portal,
        Operator::Region
        | Operator::Mosaic
        | Operator::LocalComposition
        | Operator::Stack
        | Operator::Row
        | Operator::Grid
        | Operator::Split
        | Operator::Overlay
        | Operator::Scroll => Role::Container,
    }
}

fn require_vacant<T>(slot: &Option<T>) -> Result<(), UiMountedProjectionDenial> {
    slot.is_none()
        .then_some(())
        .ok_or(UiMountedProjectionDenial::DuplicateLaneContribution)
}

fn unique_plan_rows(rows: &[(u64, u32)]) -> Result<BTreeMap<u64, u32>, UiMountedProjectionDenial> {
    let mut by_provenance = BTreeMap::new();
    for (provenance, plan_index) in rows {
        if by_provenance.insert(*provenance, *plan_index).is_some() {
            return Err(UiMountedProjectionDenial::ForeignPlan);
        }
    }
    Ok(by_provenance)
}

fn unique_allocations<'a>(
    graph: crate::graph::UiGraphAuthority<'_>,
    receipts: &'a [crate::runtime::UiAllocationReceipt],
) -> Result<
    BTreeMap<crate::graph::UiGraphNodeIdentity, &'a crate::runtime::UiAllocationReceipt>,
    UiMountedProjectionDenial,
> {
    let mut by_node = BTreeMap::new();
    for receipt in receipts {
        if receipt.generation().neighborhood_generation() != graph.generation() {
            return Err(UiMountedProjectionDenial::ForeignGraphWorld);
        }
        if graph
            .lookup()
            .graph_node(receipt.identity().graph_node_identity())
            .is_none()
        {
            return Err(UiMountedProjectionDenial::ForeignAllocation);
        }
        if by_node
            .insert(receipt.identity().graph_node_identity(), receipt)
            .is_some()
        {
            return Err(UiMountedProjectionDenial::ForeignAllocation);
        }
    }
    Ok(by_node)
}
