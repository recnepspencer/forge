use crate::runtime::live_view::digest::digest_parts;
mod composition_bridge;
mod composition_tree;
mod content_node;
mod control_node;
mod evidence_node;
mod flow_node;
mod graph_child_selection;
mod interaction_node;
mod placeholder_node_getters;
mod product_view;
mod surface_node;

pub use composition_tree::{
    WorthUiMountedCompositionChildReceipt, WorthUiMountedCompositionTraversalCounters,
    WorthUiMountedCompositionTreeReceipt,
};
pub use content_node::WorthUiMountedContentNodeReceipt;
pub use control_node::WorthUiMountedControlNodeReceipt;
pub use evidence_node::{WorthUiMountedEvidenceNodeReceipt, WorthUiMountedEvidenceRowReceipt};
pub use flow_node::{
    WorthUiMountedFlowAlign, WorthUiMountedFlowContainerNodeReceipt, WorthUiMountedFlowKind,
};
pub use graph_child_selection::{
    WorthUiLiveViewCompositionSubjectReconciliationPosture,
    WorthUiLiveViewCompositionSubjectReconciliationReceipt,
    WorthUiLiveViewCompositionSubjectReconciliationRow, WorthUiMountedGraphChildSelectionCounters,
};
pub use interaction_node::{
    WorthUiMountedContextualEventPostureReceipt, WorthUiMountedInteractionNodeReceipt,
    WorthUiMountedInteractionStyleReceipt,
};
pub use product_view::{
    WorthUiMountedProductRootEntryReceipt, WorthUiMountedProductViewCounters,
    WorthUiMountedProductViewReceipt, WorthUiMountedProductViewSemanticSlice,
};
pub use surface_node::WorthUiMountedSurfaceNodeReceipt;

use crate::runtime::{
    admit_composition_graph_access, WorthUiAdmittedCompositionGraphReceipt,
    WorthUiAdmittedCompositionRootSetReceipt, WorthUiCompositionContextPropagationReceipt,
    WorthUiCompositionGraphAccessRequest, WorthUiCompositionNodeKind,
    WorthUiCompositionRootMountAuthoritySet, WorthUiCompositionRootMountReport,
    WorthUiCompositionRootSetDefinition, WorthUiGraphBackedLiveViewProjectionReceipt,
    WorthUiLiveViewEditReceipt, WorthUiLiveViewInteractionActivationDenial,
    WorthUiLiveViewInteractionSubmissionReceipt, WorthUiLiveViewProjectionRenderPlan,
    WorthUiLiveViewStateEditDenial, WorthUiPageHostPlan, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedViewReceipt {
    live_view_id: String,
    nodes: Vec<WorthUiMountedNodeReceipt>,
    composition_graph: WorthUiAdmittedCompositionGraphReceipt,
    context_propagation: WorthUiCompositionContextPropagationReceipt,
    child_reconciliation: WorthUiLiveViewCompositionSubjectReconciliationReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiMountedNodeReceipt {
    Surface(WorthUiMountedSurfaceNodeReceipt),
    FlowContainer(WorthUiMountedFlowContainerNodeReceipt),
    Content(WorthUiMountedContentNodeReceipt),
    Control(WorthUiMountedControlNodeReceipt),
    Interaction(WorthUiMountedInteractionNodeReceipt),
    Evidence(WorthUiMountedEvidenceNodeReceipt),
    Text(WorthUiMountedTextNodeReceipt),
    Icon(WorthUiMountedIconNodeReceipt),
    DiagnosticPanel(WorthUiMountedDiagnosticPanelNodeReceipt),
    PortalHost(WorthUiMountedPortalHostNodeReceipt),
    MosaicRegion(WorthUiMountedMosaicRegionNodeReceipt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedTextNodeReceipt {
    node_id: String,
    text: String,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedIconNodeReceipt {
    node_id: String,
    icon_name: String,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedDiagnosticPanelNodeReceipt {
    node_id: String,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedPortalHostNodeReceipt {
    node_id: String,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedMosaicRegionNodeReceipt {
    node_id: String,
    semantic_slice: &'static str,
    receipt_digest: u64,
}

impl WorthUiRuntimeHost {
    fn mount_live_view_projection(
        &self,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
    ) -> WorthUiMountedViewReceipt {
        WorthUiMountedViewReceipt::from_projection(self, projection)
    }

    fn mount_live_view_projection_with_context(
        &self,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        context_propagation: WorthUiCompositionContextPropagationReceipt,
    ) -> WorthUiMountedViewReceipt {
        WorthUiMountedViewReceipt::from_projection_with_context(
            self,
            projection,
            Some(context_propagation),
        )
    }

    pub fn mount_live_view_product_projection_for_page(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
    ) -> Result<WorthUiMountedProductViewReceipt, WorthUiCompositionRootMountReport> {
        let mounted_view = self.mount_live_view_projection(projection);
        let root_set = WorthUiCompositionRootSetDefinition::from_graphs([mounted_view
            .composition_graph()
            .clone()])
        .admit()?;
        let mosaic_legality = self.admit_mosaic_placement_for_page(page_host_plan);
        let authorities = WorthUiCompositionRootMountAuthoritySet::from_page_plan(
            page_host_plan.clone(),
            mosaic_legality,
        );
        self.mount_product_composition_roots(projection, mounted_view, &root_set, &authorities)
    }

    pub fn mount_live_view_product_projection_for_page_with_context(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        context_propagation: WorthUiCompositionContextPropagationReceipt,
    ) -> Result<WorthUiMountedProductViewReceipt, WorthUiCompositionRootMountReport> {
        let mounted_view =
            self.mount_live_view_projection_with_context(projection, context_propagation);
        let root_set = WorthUiCompositionRootSetDefinition::from_graphs([mounted_view
            .composition_graph()
            .clone()])
        .admit()?;
        let mosaic_legality = self.admit_mosaic_placement_for_page(page_host_plan);
        let authorities = WorthUiCompositionRootMountAuthoritySet::from_page_plan(
            page_host_plan.clone(),
            mosaic_legality,
        );
        self.mount_product_composition_roots(projection, mounted_view, &root_set, &authorities)
    }

    pub fn mount_product_composition_roots(
        &self,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        mounted_view: WorthUiMountedViewReceipt,
        root_set: &WorthUiAdmittedCompositionRootSetReceipt,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
    ) -> Result<WorthUiMountedProductViewReceipt, WorthUiCompositionRootMountReport> {
        let root_mounts = root_set
            .roots()
            .iter()
            .map(|root| self.admit_composition_root_mount_with_authority(authorities, root.root()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(WorthUiMountedProductViewReceipt::from_live_view_projection(
            self,
            projection,
            mounted_view,
            root_mounts,
        ))
    }

    pub fn mount_live_view_product_projection_with_roots(
        &self,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        root_set: &WorthUiAdmittedCompositionRootSetReceipt,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
    ) -> Result<WorthUiMountedProductViewReceipt, WorthUiCompositionRootMountReport> {
        let mounted_view = self.mount_live_view_projection(projection);
        self.mount_product_composition_roots(projection, mounted_view, root_set, authorities)
    }

    pub fn mount_live_view_observation_evidence(
        &self,
        last_edit: Option<&WorthUiLiveViewEditReceipt>,
        last_edit_denial: Option<&WorthUiLiveViewStateEditDenial>,
        last_submission: Option<&WorthUiLiveViewInteractionSubmissionReceipt>,
        last_submission_denial: Option<&WorthUiLiveViewInteractionActivationDenial>,
        last_source_denial: Option<&str>,
    ) -> WorthUiMountedEvidenceNodeReceipt {
        WorthUiMountedEvidenceNodeReceipt::from_live_view_observations(
            last_edit,
            last_edit_denial,
            last_submission,
            last_submission_denial,
            last_source_denial,
        )
    }
}

impl WorthUiMountedViewReceipt {
    fn from_projection(
        runtime: &WorthUiRuntimeHost,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
    ) -> Self {
        Self::from_projection_with_context(runtime, projection, None)
    }

    fn from_projection_with_context(
        runtime: &WorthUiRuntimeHost,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        context_propagation: Option<WorthUiCompositionContextPropagationReceipt>,
    ) -> Self {
        let render_plan = runtime.plan_live_view_projection_render(projection.projection());
        let composition_graph = composition_bridge::admitted_live_view_composition(projection);
        let child_binding_access = admit_composition_graph_access(
            &composition_graph,
            WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
        )
        .expect("graph-backed live-view projection must admit mounted child access");
        let context_propagation =
            context_propagation.unwrap_or_else(|| projection.context_propagation().clone());
        let mut nodes = vec![WorthUiMountedNodeReceipt::Surface(
            WorthUiMountedSurfaceNodeReceipt::from_receipts(
                projection.live_view_id(),
                projection.view_flow_layout(),
                projection.view_appearance(),
            ),
        )];
        nodes.extend(flow_nodes_for_projection(projection));
        nodes.extend(content_node::static_content_nodes_for_projection(
            projection,
        ));
        let graph_child_selection = graph_child_selection::select_mounted_graph_children(
            &render_plan,
            &context_propagation,
            &child_binding_access,
        );
        nodes.extend(graph_child_selection.nodes().iter().cloned());
        nodes.push(WorthUiMountedNodeReceipt::Evidence(
            WorthUiMountedEvidenceNodeReceipt::from_live_view_projection(
                projection.projection(),
                &render_plan,
            ),
        ));
        let mut consumed_facts = mounted_view_consumed_facts(
            projection,
            &render_plan,
            &composition_graph,
            &context_propagation,
        );
        consumed_facts.extend(
            graph_child_selection
                .reconciliation()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            nodes
                .iter()
                .map(WorthUiMountedNodeReceipt::receipt_digest)
                .map(|digest| digest.to_string())
                .chain(std::iter::once(
                    composition_graph.receipt_digest().to_string(),
                ))
                .chain(std::iter::once(
                    context_propagation.receipt_digest().to_string(),
                ))
                .chain(std::iter::once(
                    graph_child_selection
                        .reconciliation()
                        .receipt_digest()
                        .to_string(),
                ))
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            live_view_id: projection.live_view_id().to_owned(),
            nodes,
            composition_graph,
            context_propagation,
            child_reconciliation: graph_child_selection.reconciliation().clone(),
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn nodes(&self) -> &[WorthUiMountedNodeReceipt] {
        &self.nodes
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn composition_graph(&self) -> &WorthUiAdmittedCompositionGraphReceipt {
        &self.composition_graph
    }

    pub fn context_propagation(&self) -> &WorthUiCompositionContextPropagationReceipt {
        &self.context_propagation
    }

    pub fn child_reconciliation(&self) -> &WorthUiLiveViewCompositionSubjectReconciliationReceipt {
        &self.child_reconciliation
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn flow_nodes_for_projection(
    projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
) -> Vec<WorthUiMountedNodeReceipt> {
    let graph = projection.composition_graph();
    graph
        .nodes()
        .iter()
        .filter(|node| node.kind() == WorthUiCompositionNodeKind::Container)
        .map(|node| {
            WorthUiMountedNodeReceipt::FlowContainer(
                WorthUiMountedFlowContainerNodeReceipt::from_flow_layout_node(
                    node.node_id().as_str(),
                    projection.live_view_id(),
                    projection.view_flow_layout(),
                ),
            )
        })
        .collect()
}

impl WorthUiMountedNodeReceipt {
    pub fn receipt_digest(&self) -> u64 {
        match self {
            Self::Surface(node) => node.receipt_digest(),
            Self::FlowContainer(node) => node.receipt_digest(),
            Self::Content(node) => node.receipt_digest(),
            Self::Control(node) => node.receipt_digest(),
            Self::Interaction(node) => node.receipt_digest(),
            Self::Evidence(node) => node.receipt_digest(),
            Self::Text(node) => node.receipt_digest(),
            Self::Icon(node) => node.receipt_digest(),
            Self::DiagnosticPanel(node) => node.receipt_digest(),
            Self::PortalHost(node) => node.receipt_digest(),
            Self::MosaicRegion(node) => node.receipt_digest(),
        }
    }
}

fn mounted_view_consumed_facts(
    projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
    composition_graph: &WorthUiAdmittedCompositionGraphReceipt,
    context_propagation: &WorthUiCompositionContextPropagationReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = vec![WorthUiRuntimeFactId::live_view_declaration(
        projection.live_view_id(),
    )];
    facts.extend(
        render_plan
            .controls()
            .iter()
            .flat_map(|row| row.host_frame().consumed_facts().iter().cloned()),
    );
    facts.extend(
        projection
            .content_receipts()
            .iter()
            .map(|receipt| receipt.dependency_fact().clone()),
    );
    facts.extend(render_plan.interactions().iter().map(|row| {
        WorthUiRuntimeFactId::live_view_interaction_intent(format!(
            "{}:{}",
            row.interaction().live_view_id(),
            row.interaction().interaction_id()
        ))
    }));
    facts.extend(composition_graph.consumed_facts().iter().cloned());
    facts.extend(context_propagation.consumed_facts().iter().cloned());
    facts
}
