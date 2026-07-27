use super::frame_storage::{
    UiMountedProjectionNodeRecord, UiMountedProjectionSurface, UiMountedSemanticProjection,
};
use super::geometry::lower_allocation;
use super::mechanical_role::mechanical_role;
use super::node_receipt::UiMountedNodeReceiptInput;
use super::participation::lower_participation;
use super::{UiMountedNodeReceipt, UiMountedProjectionDenial, UiMountedProjectionFrame};

mod delta;

pub(crate) struct UiPreparedMountedProjection {
    plan_digest: u64,
    semantic: UiMountedSemanticProjection,
    ordinary: Option<crate::runtime::WorthUiOrdinaryLaneFrameReceipt>,
    virtualized: Option<crate::runtime::WorthUiVirtualizedDataFrameReceipt>,
    canvas: Option<(crate::runtime::WorthUiCanvasSpatialFrameReceipt, u64)>,
    realtime: Option<crate::runtime::WorthUiRealtimeFrameReceipt>,
    preview: Option<UiMountedPreviewProjectionInput>,
    projection_changes: super::super::UiMountedProjectionChangeSnapshot,
    counters: super::super::UiMountStageCounters,
}

pub(crate) struct UiMountedProjectionInput<'input, 'graph> {
    pub(crate) graph: crate::graph::UiGraphAuthority<'graph>,
    pub(crate) plan_digest: u64,
    pub(crate) plan: super::super::UiMountedPlanProjectionSource<'input>,
    pub(crate) allocation_source: &'input crate::runtime::UiMountedAllocationProjectionSource,
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
    plan: super::super::UiMountedPlanProjectionSource<'input>,
    allocation_source: &'input crate::runtime::UiMountedAllocationProjectionSource,
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
    static_paint: Option<super::static_paint::UiMountedStaticPaintSeed>,
}

struct UiMountedProjectionBuild {
    semantic: UiMountedSemanticProjection,
    cost: super::cost_accounting::UiMountedProjectionCostInput,
    replaced_order_rows: usize,
}

#[derive(Clone)]
pub struct UiProjectedMountedFrameCandidate {
    pub(in crate::mounting) frame: UiMountedProjectionFrame,
    pub(in crate::mounting) identity_candidate:
        super::super::identity_state::UiMountedIdentityFrameCandidate,
    pub(in crate::mounting) projection_changes: super::super::UiMountedProjectionChangeSnapshot,
}

impl UiProjectedMountedFrameCandidate {
    pub fn frame(&self) -> &UiMountedProjectionFrame {
        &self.frame
    }

    pub fn is_unpublished(&self) -> bool {
        let _ = &self.identity_candidate;
        true
    }

    pub(crate) fn presented_receipt_basis(&self) -> &super::super::UiMountedNodeReceiptBasis {
        self.identity_candidate.receipt_basis()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiMountedProjectionFrame,
        super::super::identity_state::UiMountedIdentityFrameCandidate,
        super::super::UiMountedProjectionChangeSnapshot,
    ) {
        (self.frame, self.identity_candidate, self.projection_changes)
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
        let identity_candidate = state.prepare_frame_candidate_for(self.semantic.membership())?;
        let mut frame = UiMountedProjectionFrame::new(
            identity_candidate.frame(),
            identity_candidate.receipt_basis().clone(),
            self.plan_digest,
            self.semantic,
            self.counters,
        );
        frame.complete_static_paint()?;
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
            projection_changes: self.projection_changes,
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
    let lowering = UiMountedNodeLoweringContext {
        graph: input.graph,
        plan: input.plan,
        allocation_source: input.allocation_source,
        plan_digest: input.plan_digest,
    };
    let projection_changes = state.projection_change_snapshot();
    let delta_predecessor = state
        .current_projection()
        .filter(|current| current.plan_digest() == input.plan_digest)
        .filter(|current| {
            current
                .semantic_projection()
                .supports_surfaces(input.requested_surfaces)
        });
    let delta = match (delta_predecessor, input.allocation_source.delta()) {
        (
            Some(current),
            crate::runtime::UiMountedAllocationProjectionDelta::Exact(allocation_delta),
        ) => delta::build(delta::UiMountedDeltaProjectionInput {
            state,
            lowering: &lowering,
            predecessor: current.semantic_projection(),
            requested_surfaces: input.requested_surfaces,
            changes: &projection_changes,
            allocation_delta,
        })?,
        _ => None,
    };
    let build = match delta {
        Some(build) => build,
        None => build_full_projection(
            state,
            &lowering,
            input.requested_surfaces,
            state.has_published_frame(),
            &projection_changes,
        )?,
    };
    let mut counters = super::cost_accounting::begin_projection_cost(build.cost)?;
    if build.replaced_order_rows > 0 {
        counters
            .replace_rows::<worth_ui_host_contract::UiMountedInstanceIdentity>(
                build.replaced_order_rows,
            )
            .map_err(|_| UiMountedProjectionDenial::CostCounterOverflow)?;
    }
    Ok(UiPreparedMountedProjection {
        plan_digest: input.plan_digest,
        semantic: build.semantic,
        ordinary: None,
        virtualized: None,
        canvas: None,
        realtime: None,
        preview: input.preview,
        projection_changes,
        counters,
    })
}

fn build_full_projection(
    state: &super::super::UiMountedIdentityState,
    lowering: &UiMountedNodeLoweringContext<'_, '_>,
    requested_surfaces: &[worth_ui_host_contract::UiSemanticSurfaceIdentity],
    has_published_frame: bool,
    changes: &super::super::UiMountedProjectionChangeSnapshot,
) -> Result<UiMountedProjectionBuild, UiMountedProjectionDenial> {
    let instances = state.projection_instances(requested_surfaces);
    let nodes = instances
        .iter()
        .map(|instance| {
            lowering
                .lower(instance)
                .map(UiMountedProjectionNodeDraft::materialize)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let surfaces = projection_surfaces(state, requested_surfaces)?;
    let node_count = nodes.len();
    let surface_count = surfaces.len();
    let work_class = if has_published_frame {
        super::super::UiMountWorkClass::ComparisonRequired
    } else {
        super::super::UiMountWorkClass::InitialMount
    };
    Ok(UiMountedProjectionBuild {
        semantic: UiMountedSemanticProjection::initial(nodes, surfaces),
        cost: super::cost_accounting::UiMountedProjectionCostInput {
            work_class,
            considered: node_count
                .checked_mul(3)
                .and_then(|count| count.checked_add(surface_count))
                .ok_or(UiMountedProjectionDenial::CostCounterOverflow)?,
            index_entries: node_count
                .checked_mul(2)
                .ok_or(UiMountedProjectionDenial::CostCounterOverflow)?,
            projected_instances: node_count,
            surface_instance_pairs: node_count,
            changed_bindings: usize::from(!has_published_frame) * surface_count,
            reused: 0,
            retired: 0,
            coalesced: changes.coalesced(),
            overflowed: changes.overflowed(),
        },
        replaced_order_rows: node_count,
    })
}

fn projection_surfaces(
    state: &super::super::UiMountedIdentityState,
    requested: &[worth_ui_host_contract::UiSemanticSurfaceIdentity],
) -> Result<Vec<UiMountedProjectionSurface>, UiMountedProjectionDenial> {
    requested
        .iter()
        .map(|surface| {
            let (binding, audience) = state
                .projection_surface(*surface)
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            Ok(UiMountedProjectionSurface {
                surface: *surface,
                binding: binding.binding_generation(),
                audience,
            })
        })
        .collect()
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
        let plan_index = self
            .plan
            .plan_index(provenance)
            .map_err(|_| UiMountedProjectionDenial::ForeignPlan)?;
        let allocation = lower_allocation(
            self.allocation_source
                .projection(instance.graph_node_identity()),
        )?;
        let static_paint = super::static_paint::lower_static_paint_seed(self.plan, plan_index)?;
        let participation =
            lower_participation(graph_node.participation_posture(), static_paint.is_some());
        Ok(UiMountedProjectionNodeDraft {
            mounted_instance: instance.identity(),
            graph_node: instance.graph_node_identity(),
            semantic_surface: instance.basis().semantic_surface_identity(),
            incarnation: instance.mount_incarnation(),
            plan_digest: self.plan_digest,
            role: mechanical_role(graph_node.operator_kind()),
            participation,
            allocation,
            plan_index,
            static_paint,
        })
    }
}

impl UiMountedProjectionNodeDraft {
    fn materialize(self) -> UiMountedProjectionNodeRecord {
        UiMountedProjectionNodeRecord {
            receipt: UiMountedNodeReceipt::from_input(UiMountedNodeReceiptInput {
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
            static_paint: self.static_paint,
        }
    }
}

fn require_vacant<T>(slot: &Option<T>) -> Result<(), UiMountedProjectionDenial> {
    slot.is_none()
        .then_some(())
        .ok_or(UiMountedProjectionDenial::DuplicateLaneContribution)
}
