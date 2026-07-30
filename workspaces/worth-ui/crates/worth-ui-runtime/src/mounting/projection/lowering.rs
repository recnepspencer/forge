use super::frame_storage::{
    UiMountedProjectionNodeRecord, UiMountedProjectionSurface, UiMountedSemanticProjection,
};
use super::geometry::lower_allocation;
use super::mechanical_role::mechanical_role;
use super::node_receipt::UiMountedNodeReceiptInput;
use super::participation::lower_participation;
use super::prepared_projection::{UiPreparedMountedProjection, UiPreparedMountedProjectionInput};
use super::{UiMountedNodeReceipt, UiMountedProjectionDenial};

mod delta;

pub(crate) struct UiMountedProjectionInput<'input, 'graph> {
    pub(crate) graph: crate::graph::UiGraphAuthority<'graph>,
    pub(crate) plan_digest: u64,
    pub(crate) plan: super::super::UiMountedPlanProjectionSource<'input>,
    pub(crate) allocation_source: &'input crate::runtime::UiMountedAllocationProjectionSource,
    pub(crate) requested_surfaces: &'input [worth_ui_host_contract::UiSemanticSurfaceIdentity],
    pub(crate) preview: Option<UiMountedPreviewProjectionInput>,
    pub(crate) visual_overlay: Option<super::super::UiMountedVisualOverlayProjectionInput>,
    pub(crate) semantic_content: &'input super::super::UiMountedSemanticContentInput,
    pub(in crate::mounting) semantic_predecessor: Option<&'input UiMountedSemanticProjection>,
    pub(crate) capability_generation:
        worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    pub(crate) capability_profile_digest: u64,
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
    semantic_content: &'input super::super::UiMountedSemanticContentInput,
    predecessor: Option<&'input UiMountedSemanticProjection>,
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
    semantic_text: Option<super::semantic_text::UiMountedSemanticTextSeed>,
    hit_test: Option<super::hit_test::UiMountedHitTestSeed>,
}

struct UiMountedFullProjectionInput<'basis, 'input, 'graph> {
    state: &'basis super::super::UiMountedIdentityState,
    lowering: &'basis UiMountedNodeLoweringContext<'input, 'graph>,
    requested_surfaces: &'basis [worth_ui_host_contract::UiSemanticSurfaceIdentity],
    has_published_frame: bool,
    changes: &'basis super::super::UiMountedProjectionChangeSnapshot,
}

struct UiMountedProjectionBuild {
    semantic: UiMountedSemanticProjection,
    cost: super::cost_accounting::UiMountedProjectionCostInput,
    replaced_order_rows: usize,
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
        semantic_content: input.semantic_content,
        predecessor: input.semantic_predecessor,
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
        None => build_full_projection(UiMountedFullProjectionInput {
            state,
            lowering: &lowering,
            requested_surfaces: input.requested_surfaces,
            has_published_frame: state.has_published_frame(),
            changes: &projection_changes,
        })?,
    };
    let counters = begin_build_counters(build.cost, build.replaced_order_rows)?;
    Ok(UiPreparedMountedProjection::new(
        UiPreparedMountedProjectionInput {
            plan_digest: input.plan_digest,
            semantic: build.semantic,
            preview: input.preview,
            visual_overlay: input.visual_overlay,
            projection_changes,
            counters,
            capability_generation: input.capability_generation,
            capability_profile_digest: input.capability_profile_digest,
        },
    ))
}

fn begin_build_counters(
    cost: super::cost_accounting::UiMountedProjectionCostInput,
    replaced_order_rows: usize,
) -> Result<super::super::UiMountStageCounters, UiMountedProjectionDenial> {
    let mut counters = super::cost_accounting::begin_projection_cost(cost)?;
    if replaced_order_rows > 0 {
        counters
            .replace_rows::<worth_ui_host_contract::UiMountedInstanceIdentity>(replaced_order_rows)
            .map_err(|_| UiMountedProjectionDenial::CostCounterOverflow)?;
    }
    Ok(counters)
}

fn build_full_projection(
    input: UiMountedFullProjectionInput<'_, '_, '_>,
) -> Result<UiMountedProjectionBuild, UiMountedProjectionDenial> {
    let instances = input.state.projection_instances(input.requested_surfaces);
    let nodes = instances
        .iter()
        .map(|instance| {
            input
                .lowering
                .lower(instance)
                .map(UiMountedProjectionNodeDraft::materialize)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let surfaces = projection_surfaces(input.state, input.requested_surfaces)?;
    let node_count = nodes.len();
    let surface_count = surfaces.len();
    let work_class = if input.has_published_frame {
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
            changed_bindings: usize::from(!input.has_published_frame) * surface_count,
            reused: 0,
            retired: 0,
            coalesced: input.changes.coalesced(),
            overflowed: input.changes.overflowed(),
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
        let semantic_text_style =
            super::semantic_text::lower_semantic_text_style(self.plan, plan_index)?;
        let predecessor = self
            .predecessor
            .and_then(|semantic| semantic.node(instance.identity()))
            .and_then(|node| node.semantic_text.as_ref());
        let semantic_text = super::semantic_text::lower_semantic_text_seed(
            self.semantic_content.get(instance.graph_node_identity()),
            predecessor,
            semantic_text_style,
        )?;
        let hit_test = super::hit_test::lower_hit_test_seed(self.plan, plan_index)?;
        let participation = lower_participation(
            graph_node.participation_posture(),
            static_paint.is_some() || semantic_text.is_some(),
            hit_test.is_some(),
        );
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
            semantic_text,
            hit_test,
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
            semantic_text: self.semantic_text,
            hit_test: self.hit_test,
        }
    }
}
