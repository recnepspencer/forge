use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiAllocatedChildReceipt, WorthUiBoxEdges, WorthUiFlowLayoutCrossAlign,
    WorthUiHostMeasurementReadinessPosture, WorthUiLayoutAllocationContainerPolicyReceipt,
    WorthUiLayoutAllocationCounters, WorthUiLayoutAllocationDenial,
    WorthUiLayoutAllocationDenialReason, WorthUiLayoutAllocationFrame,
    WorthUiLayoutAllocationReceipt, WorthUiLayoutAllocationRequest,
    WorthUiMeasuredProductViewReceipt, WorthUiMountedCompositionTreeReceipt,
    WorthUiMountedFlowKind, WorthUiMountedNodeReceipt, WorthUiRuntimeHost,
};

use super::solver::{allocate_participants, participant_from_row_and_node};

impl WorthUiRuntimeHost {
    pub fn allocate_mounted_product_view(
        &self,
        measured: &WorthUiMeasuredProductViewReceipt,
        request: WorthUiLayoutAllocationRequest,
    ) -> Result<WorthUiLayoutAllocationReceipt, WorthUiLayoutAllocationDenial> {
        WorthUiLayoutAllocationPlan::from_measured_view(measured, request).allocate()
    }
}

struct WorthUiLayoutAllocationPlan<'a> {
    measured: &'a WorthUiMeasuredProductViewReceipt,
    request: WorthUiLayoutAllocationRequest,
}

struct WorthUiLayoutAllocationBuilder<'a> {
    tree: &'a WorthUiMountedCompositionTreeReceipt,
    measured: &'a WorthUiMeasuredProductViewReceipt,
    container_policies: Vec<WorthUiLayoutAllocationContainerPolicyReceipt>,
    children: Vec<WorthUiAllocatedChildReceipt>,
    visited_containers: BTreeSet<String>,
    container_count: usize,
}

struct WorthUiContainerAllocationPolicy {
    kind: WorthUiMountedFlowKind,
    gap: f32,
    cross_align: WorthUiFlowLayoutCrossAlign,
    inner_bounds: WorthUiLayoutAllocationFrame,
    receipt: WorthUiLayoutAllocationContainerPolicyReceipt,
}

impl<'a> WorthUiLayoutAllocationPlan<'a> {
    fn from_measured_view(
        measured: &'a WorthUiMeasuredProductViewReceipt,
        request: WorthUiLayoutAllocationRequest,
    ) -> Self {
        Self { measured, request }
    }

    fn allocate(self) -> Result<WorthUiLayoutAllocationReceipt, WorthUiLayoutAllocationDenial> {
        let root_bounds = self.root_bounds()?;
        let tree = self.measured.mounted_product_view().composition_tree();
        let root_node = self.root_node(tree)?;
        if !can_allocate_children(root_node) {
            return Err(WorthUiLayoutAllocationDenial::new(
                WorthUiLayoutAllocationDenialReason::NonContainerAllocationRoot,
                self.request.root_node_id(),
            ));
        }
        let mut builder = WorthUiLayoutAllocationBuilder::new(tree, self.measured);
        builder.allocate_container(self.request.root_node_id(), root_node, root_bounds)?;
        let counters = builder.counters();
        let container_policies = builder.container_policies;
        let children = builder.children;
        Ok(WorthUiLayoutAllocationReceipt::new(
            self.request.root_node_id(),
            self.measured.host_observations().receipt_digest(),
            container_policies,
            children,
            self.measured.consumed_facts().to_vec(),
            counters,
        ))
    }

    fn root_bounds(&self) -> Result<WorthUiLayoutAllocationFrame, WorthUiLayoutAllocationDenial> {
        if self.measured.host_observations().readiness()
            != WorthUiHostMeasurementReadinessPosture::Ready
        {
            return Err(WorthUiLayoutAllocationDenial::new(
                WorthUiLayoutAllocationDenialReason::MissingAvailableBounds,
                self.request.root_node_id(),
            ));
        }
        self.measured
            .host_observations()
            .available_bounds()
            .iter()
            .find(|row| row.node_id() == self.request.root_node_id())
            .map(|row| {
                WorthUiLayoutAllocationFrame::new(0.0, 0.0, row.width_points(), row.height_points())
            })
            .ok_or_else(|| {
                WorthUiLayoutAllocationDenial::new(
                    WorthUiLayoutAllocationDenialReason::MissingAvailableBounds,
                    self.request.root_node_id(),
                )
            })
    }

    fn root_node<'b>(
        &self,
        tree: &'b WorthUiMountedCompositionTreeReceipt,
    ) -> Result<&'b WorthUiMountedNodeReceipt, WorthUiLayoutAllocationDenial> {
        tree.graph_access()
            .child_rows()
            .iter()
            .find(|row| row.node().node_id().as_str() == self.request.root_node_id())
            .and_then(|row| {
                mounted_child_for_node(tree, row.parent_id(), self.request.root_node_id())
            })
            .ok_or_else(|| {
                WorthUiLayoutAllocationDenial::new(
                    WorthUiLayoutAllocationDenialReason::UnknownAllocationRoot,
                    self.request.root_node_id(),
                )
            })
    }
}

impl<'a> WorthUiLayoutAllocationBuilder<'a> {
    fn new(
        tree: &'a WorthUiMountedCompositionTreeReceipt,
        measured: &'a WorthUiMeasuredProductViewReceipt,
    ) -> Self {
        Self {
            tree,
            measured,
            container_policies: Vec::new(),
            children: Vec::new(),
            visited_containers: BTreeSet::new(),
            container_count: 0,
        }
    }

    fn allocate_container(
        &mut self,
        node_id: &str,
        mounted_node: &WorthUiMountedNodeReceipt,
        bounds: WorthUiLayoutAllocationFrame,
    ) -> Result<(), WorthUiLayoutAllocationDenial> {
        if !self.visited_containers.insert(node_id.to_owned()) {
            return Ok(());
        }
        self.container_count += 1;
        let policy = container_layout(node_id, mounted_node, bounds)?;
        self.container_policies.push(policy.receipt.clone());
        let rows = self.tree.graph_access().ordered_children(node_id);
        let participants = rows
            .iter()
            .map(|row| {
                mounted_child_for_node(self.tree, row.parent_id(), row.node().node_id().as_str())
                    .map(|node| {
                        participant_from_row_and_node(row, node, self.measured.host_observations())
                    })
                    .ok_or_else(|| {
                        WorthUiLayoutAllocationDenial::new(
                            WorthUiLayoutAllocationDenialReason::MissingMountedChild,
                            row.node().node_id().as_str(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let allocated = allocate_participants(
            policy.kind,
            &participants,
            policy.inner_bounds.x(),
            policy.inner_bounds.y(),
            policy.inner_bounds.width(),
            policy.inner_bounds.height(),
            policy.gap,
            policy.cross_align,
        );
        let child_frames = allocated
            .iter()
            .map(|child| (child.child_node_id().to_owned(), child.frame()))
            .collect::<Vec<_>>();
        self.children.extend(allocated);
        for (child_node_id, child_frame) in child_frames {
            if let Some(child) = mounted_child_for_node(self.tree, node_id, &child_node_id) {
                if can_allocate_children(child) {
                    self.allocate_container(&child_node_id, child, child_frame)?;
                }
            }
        }
        Ok(())
    }

    fn counters(&self) -> WorthUiLayoutAllocationCounters {
        let participating = self
            .children
            .iter()
            .filter(|child| child.participation().participates_in_layout())
            .count();
        let absent = self.children.len().saturating_sub(participating);
        let fill = self
            .children
            .iter()
            .filter(|child| child.sizing().fill_weight().is_some())
            .count();
        let hug = self.children.len().saturating_sub(fill);
        WorthUiLayoutAllocationCounters::new(self.container_count, participating, absent, hug, fill)
    }
}

fn container_layout(
    node_id: &str,
    node: &WorthUiMountedNodeReceipt,
    bounds: WorthUiLayoutAllocationFrame,
) -> Result<WorthUiContainerAllocationPolicy, WorthUiLayoutAllocationDenial> {
    match node {
        WorthUiMountedNodeReceipt::Surface(surface) => {
            let padding_edges = surface.padding_edges();
            let kind = WorthUiMountedFlowKind::Column;
            Ok(WorthUiContainerAllocationPolicy {
                kind,
                gap: 0.0,
                cross_align: WorthUiFlowLayoutCrossAlign::Start,
                inner_bounds: inset_frame(bounds, padding_edges),
                receipt: WorthUiLayoutAllocationContainerPolicyReceipt::new(
                    node_id,
                    flow_kind_token(kind),
                    "none",
                    0.0,
                    surface.padding_token(),
                    padding_edges,
                    WorthUiFlowLayoutCrossAlign::Start,
                ),
            })
        }
        WorthUiMountedNodeReceipt::FlowContainer(flow) => match flow.kind() {
            WorthUiMountedFlowKind::Grid | WorthUiMountedFlowKind::Spacer => {
                Err(WorthUiLayoutAllocationDenial::new(
                    WorthUiLayoutAllocationDenialReason::UnsupportedFlowKind,
                    node_id,
                ))
            }
            kind => {
                let padding_edges = flow.padding_edges();
                Ok(WorthUiContainerAllocationPolicy {
                    kind,
                    gap: flow.gap_points(),
                    cross_align: flow.cross_align(),
                    inner_bounds: inset_frame(bounds, padding_edges),
                    receipt: WorthUiLayoutAllocationContainerPolicyReceipt::new(
                        node_id,
                        flow_kind_token(kind),
                        flow.gap_token(),
                        flow.gap_points(),
                        flow.padding_token(),
                        padding_edges,
                        flow.cross_align(),
                    ),
                })
            }
        },
        WorthUiMountedNodeReceipt::DiagnosticPanel(_) => {
            let kind = WorthUiMountedFlowKind::Column;
            let padding_edges = WorthUiBoxEdges::uniform(0.0);
            Ok(WorthUiContainerAllocationPolicy {
                kind,
                gap: 0.0,
                cross_align: WorthUiFlowLayoutCrossAlign::Start,
                inner_bounds: bounds,
                receipt: WorthUiLayoutAllocationContainerPolicyReceipt::new(
                    node_id,
                    flow_kind_token(kind),
                    "none",
                    0.0,
                    "none",
                    padding_edges,
                    WorthUiFlowLayoutCrossAlign::Start,
                ),
            })
        }
        _ => Err(WorthUiLayoutAllocationDenial::new(
            WorthUiLayoutAllocationDenialReason::NonContainerAllocationRoot,
            node_id,
        )),
    }
}

fn inset_frame(
    frame: WorthUiLayoutAllocationFrame,
    padding: WorthUiBoxEdges,
) -> WorthUiLayoutAllocationFrame {
    WorthUiLayoutAllocationFrame::new(
        frame.x() + padding.left(),
        frame.y() + padding.top(),
        (frame.width() - padding.horizontal()).max(0.0),
        (frame.height() - padding.vertical()).max(0.0),
    )
}

fn flow_kind_token(kind: WorthUiMountedFlowKind) -> &'static str {
    match kind {
        WorthUiMountedFlowKind::Row => "row",
        WorthUiMountedFlowKind::Column => "column",
        WorthUiMountedFlowKind::Inline => "inline",
        WorthUiMountedFlowKind::Stack => "stack",
        WorthUiMountedFlowKind::Grid => "grid",
        WorthUiMountedFlowKind::Spacer => "spacer",
    }
}

fn can_allocate_children(node: &WorthUiMountedNodeReceipt) -> bool {
    matches!(
        node,
        WorthUiMountedNodeReceipt::Surface(_)
            | WorthUiMountedNodeReceipt::FlowContainer(_)
            | WorthUiMountedNodeReceipt::DiagnosticPanel(_)
    )
}

fn mounted_child_for_node<'a>(
    tree: &'a WorthUiMountedCompositionTreeReceipt,
    parent_id: &str,
    node_id: &str,
) -> Option<&'a WorthUiMountedNodeReceipt> {
    tree.ordered_children(parent_id)
        .iter()
        .find(|child| child.node_id() == node_id)
        .map(|child| child.mounted_node())
}
