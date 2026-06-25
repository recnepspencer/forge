use super::digest::digest_parts;
use crate::runtime::{
    WorthUiBoxEdges, WorthUiFlowLayoutCrossAlign, WorthUiLayoutAllocationCounters,
    WorthUiLayoutAllocationFrame, WorthUiLayoutParticipationPosture, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLayoutAllocationReceipt {
    root_node_id: String,
    host_measurement_basis_digest: u64,
    container_policies: Vec<WorthUiLayoutAllocationContainerPolicyReceipt>,
    children: Vec<WorthUiAllocatedChildReceipt>,
    participating_child_ids: Vec<String>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiLayoutAllocationCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAllocatedChildReceipt {
    parent_id: String,
    child_node_id: String,
    order: u32,
    sizing: crate::runtime::WorthUiLayoutAllocatedChildSizing,
    sizing_token: String,
    participation: WorthUiLayoutParticipationPosture,
    natural_width_points: f32,
    natural_height_points: f32,
    baseline_points: f32,
    natural_metric_basis: String,
    frame: WorthUiLayoutAllocationFrame,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLayoutAllocationContainerPolicyReceipt {
    node_id: String,
    kind_token: String,
    gap_token: String,
    gap_points: f32,
    padding_token: String,
    padding_edges: WorthUiBoxEdges,
    cross_align: WorthUiFlowLayoutCrossAlign,
    receipt_digest: u64,
}

impl WorthUiLayoutAllocationReceipt {
    pub(super) fn new(
        root_node_id: impl Into<String>,
        host_measurement_basis_digest: u64,
        container_policies: Vec<WorthUiLayoutAllocationContainerPolicyReceipt>,
        children: Vec<WorthUiAllocatedChildReceipt>,
        mut consumed_facts: Vec<WorthUiRuntimeFactId>,
        counters: WorthUiLayoutAllocationCounters,
    ) -> Self {
        let root_node_id = root_node_id.into();
        let participating_child_ids = children
            .iter()
            .filter(|child| child.participation().participates_in_layout())
            .map(|child| child.child_node_id().to_owned())
            .collect::<Vec<_>>();
        consumed_facts.sort();
        consumed_facts.dedup();
        let allocation_fact_identity = format!("{root_node_id}:{host_measurement_basis_digest}");
        consumed_facts.push(WorthUiRuntimeFactId::layout_allocation(
            allocation_fact_identity,
        ));
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            [
                "layout_allocation".to_owned(),
                root_node_id.clone(),
                host_measurement_basis_digest.to_string(),
            ]
            .into_iter()
            .chain(
                children
                    .iter()
                    .map(|child| child.receipt_digest().to_string()),
            )
            .chain(
                container_policies
                    .iter()
                    .map(|policy| policy.receipt_digest().to_string()),
            )
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            root_node_id,
            host_measurement_basis_digest,
            container_policies,
            children,
            participating_child_ids,
            consumed_facts,
            counters,
            receipt_digest,
        }
    }

    pub fn root_node_id(&self) -> &str {
        &self.root_node_id
    }

    pub fn host_measurement_basis_digest(&self) -> u64 {
        self.host_measurement_basis_digest
    }

    pub fn children(&self) -> &[WorthUiAllocatedChildReceipt] {
        &self.children
    }

    pub fn participating_child_ids(&self) -> &[String] {
        &self.participating_child_ids
    }

    pub fn container_policies(&self) -> &[WorthUiLayoutAllocationContainerPolicyReceipt] {
        &self.container_policies
    }

    pub fn child_frame(&self, child_node_id: &str) -> Option<WorthUiLayoutAllocationFrame> {
        self.children
            .iter()
            .find(|child| child.child_node_id() == child_node_id)
            .map(WorthUiAllocatedChildReceipt::frame)
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn counters(&self) -> WorthUiLayoutAllocationCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiAllocatedChildReceipt {
    pub(super) fn new(
        parent_id: impl Into<String>,
        child_node_id: impl Into<String>,
        order: u32,
        sizing: crate::runtime::WorthUiLayoutAllocatedChildSizing,
        sizing_token: impl Into<String>,
        participation: WorthUiLayoutParticipationPosture,
        natural_width_points: f32,
        natural_height_points: f32,
        baseline_points: f32,
        natural_metric_basis: impl Into<String>,
        frame: WorthUiLayoutAllocationFrame,
    ) -> Self {
        let parent_id = parent_id.into();
        let child_node_id = child_node_id.into();
        let sizing_token = sizing_token.into();
        let natural_metric_basis = natural_metric_basis.into();
        let receipt_digest = digest_parts([
            "allocated_child".to_owned(),
            parent_id.clone(),
            child_node_id.clone(),
            order.to_string(),
            sizing_token.clone(),
            participation.token().to_owned(),
            natural_width_points.to_string(),
            natural_height_points.to_string(),
            baseline_points.to_string(),
            natural_metric_basis.clone(),
            frame.x().to_string(),
            frame.y().to_string(),
            frame.width().to_string(),
            frame.height().to_string(),
        ]);
        Self {
            parent_id,
            child_node_id,
            order,
            sizing,
            sizing_token,
            participation,
            natural_width_points,
            natural_height_points,
            baseline_points,
            natural_metric_basis,
            frame,
            receipt_digest,
        }
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn child_node_id(&self) -> &str {
        &self.child_node_id
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn sizing_token(&self) -> &str {
        &self.sizing_token
    }

    pub fn sizing(&self) -> crate::runtime::WorthUiLayoutAllocatedChildSizing {
        self.sizing
    }

    pub fn participation(&self) -> WorthUiLayoutParticipationPosture {
        self.participation
    }

    pub fn natural_width_points(&self) -> f32 {
        self.natural_width_points
    }

    pub fn natural_height_points(&self) -> f32 {
        self.natural_height_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }

    pub fn natural_metric_basis(&self) -> &str {
        &self.natural_metric_basis
    }

    pub fn frame(&self) -> WorthUiLayoutAllocationFrame {
        self.frame
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiLayoutAllocationContainerPolicyReceipt {
    pub(super) fn new(
        node_id: impl Into<String>,
        kind_token: impl Into<String>,
        gap_token: impl Into<String>,
        gap_points: f32,
        padding_token: impl Into<String>,
        padding_edges: WorthUiBoxEdges,
        cross_align: WorthUiFlowLayoutCrossAlign,
    ) -> Self {
        let node_id = node_id.into();
        let kind_token = kind_token.into();
        let gap_token = gap_token.into();
        let padding_token = padding_token.into();
        let receipt_digest = digest_parts([
            "layout_allocation_container_policy".to_owned(),
            node_id.clone(),
            kind_token.clone(),
            gap_token.clone(),
            gap_points.to_string(),
            padding_token.clone(),
            padding_edges.digest_basis(),
            format!("{cross_align:?}"),
        ]);
        Self {
            node_id,
            kind_token,
            gap_token,
            gap_points,
            padding_token,
            padding_edges,
            cross_align,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn kind_token(&self) -> &str {
        &self.kind_token
    }

    pub fn gap_token(&self) -> &str {
        &self.gap_token
    }

    pub fn gap_points(&self) -> f32 {
        self.gap_points
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn padding_edges(&self) -> WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn cross_align(&self) -> WorthUiFlowLayoutCrossAlign {
        self.cross_align
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
