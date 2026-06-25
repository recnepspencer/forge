use std::collections::HashMap;

mod reconciliation;

pub use reconciliation::{
    WorthUiLiveViewCompositionSubjectReconciliationPosture,
    WorthUiLiveViewCompositionSubjectReconciliationReceipt,
    WorthUiLiveViewCompositionSubjectReconciliationRow, WorthUiMountedGraphChildSelectionCounters,
};

use crate::runtime::{
    WorthUiCompositionContextPropagationReceipt, WorthUiCompositionGraphAccessReceipt,
    WorthUiCompositionGraphChildAccessRow, WorthUiCompositionNodeKind,
    WorthUiLiveViewCompositionChildBindingReceipt, WorthUiLiveViewCompositionChildSubjectKind,
    WorthUiLiveViewProjectionRenderControl, WorthUiLiveViewProjectionRenderInteraction,
    WorthUiLiveViewProjectionRenderPlan,
};

use super::{
    WorthUiMountedControlNodeReceipt, WorthUiMountedInteractionNodeReceipt,
    WorthUiMountedNodeReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedGraphChildSelectionReceipt {
    nodes: Vec<WorthUiMountedNodeReceipt>,
    reconciliation: WorthUiLiveViewCompositionSubjectReconciliationReceipt,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum GraphChildSubjectKind {
    Control,
    Interaction,
}

pub(super) fn select_mounted_graph_children(
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
    context_propagation: &WorthUiCompositionContextPropagationReceipt,
    graph_access: &WorthUiCompositionGraphAccessReceipt,
) -> WorthUiMountedGraphChildSelectionReceipt {
    let controls_by_id = controls_by_id(render_plan);
    let interactions_by_id = interactions_by_id(render_plan);
    let mut mounted_nodes = Vec::new();
    let mut reconciliation_rows = Vec::new();
    let mut graph_subject_counts: HashMap<(GraphChildSubjectKind, String), usize> = HashMap::new();
    let mut counters = initial_selection_counters(render_plan, graph_access);

    for row in graph_access.child_rows() {
        select_graph_child_row(
            row,
            &controls_by_id,
            &interactions_by_id,
            context_propagation,
            &mut graph_subject_counts,
            &mut mounted_nodes,
            &mut reconciliation_rows,
            &mut counters,
        );
    }

    append_declared_unmounted_control_rows(
        &controls_by_id,
        &graph_subject_counts,
        &mut reconciliation_rows,
        &mut counters,
    );
    append_declared_unmounted_interaction_rows(
        &interactions_by_id,
        &graph_subject_counts,
        &mut reconciliation_rows,
        &mut counters,
    );

    WorthUiMountedGraphChildSelectionReceipt {
        nodes: mounted_nodes,
        reconciliation: WorthUiLiveViewCompositionSubjectReconciliationReceipt::new(
            reconciliation_rows,
            counters,
            graph_access.child_rows(),
        ),
    }
}

fn initial_selection_counters(
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
    graph_access: &WorthUiCompositionGraphAccessReceipt,
) -> WorthUiMountedGraphChildSelectionCounters {
    WorthUiMountedGraphChildSelectionCounters {
        graph_child_row_count: graph_access.child_rows().len(),
        projection_control_scan_count: usize::from(!render_plan.controls().is_empty()),
        projection_interaction_scan_count: usize::from(!render_plan.interactions().is_empty()),
        ..Default::default()
    }
}

fn select_graph_child_row(
    row: &WorthUiCompositionGraphChildAccessRow,
    controls_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderControl>,
    interactions_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderInteraction>,
    context_propagation: &WorthUiCompositionContextPropagationReceipt,
    graph_subject_counts: &mut HashMap<(GraphChildSubjectKind, String), usize>,
    mounted_nodes: &mut Vec<WorthUiMountedNodeReceipt>,
    reconciliation_rows: &mut Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: &mut WorthUiMountedGraphChildSelectionCounters,
) {
    let Some(subject_kind) = GraphChildSubjectKind::from_node_kind(row.node().kind()) else {
        return;
    };
    let subject_id = row.node().authority_identity();
    if record_duplicate_graph_subject(subject_kind, subject_id, graph_subject_counts) {
        counters.duplicate_subject_count += 1;
        reconciliation_rows.push(reconciliation_row(
            subject_kind,
            subject_id,
            row,
            WorthUiLiveViewCompositionSubjectReconciliationPosture::DuplicateGraphSubject,
        ));
        return;
    }
    match subject_kind {
        GraphChildSubjectKind::Control => select_control_graph_child(
            subject_id,
            row,
            controls_by_id,
            mounted_nodes,
            reconciliation_rows,
            counters,
        ),
        GraphChildSubjectKind::Interaction => select_interaction_graph_child(
            subject_id,
            row,
            interactions_by_id,
            context_propagation,
            mounted_nodes,
            reconciliation_rows,
            counters,
        ),
    }
}

fn record_duplicate_graph_subject(
    subject_kind: GraphChildSubjectKind,
    subject_id: &str,
    graph_subject_counts: &mut HashMap<(GraphChildSubjectKind, String), usize>,
) -> bool {
    let key = (subject_kind, subject_id.to_owned());
    let seen_count = graph_subject_counts.entry(key).or_default();
    *seen_count += 1;
    *seen_count > 1
}

fn select_control_graph_child(
    subject_id: &str,
    row: &WorthUiCompositionGraphChildAccessRow,
    controls_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderControl>,
    mounted_nodes: &mut Vec<WorthUiMountedNodeReceipt>,
    reconciliation_rows: &mut Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: &mut WorthUiMountedGraphChildSelectionCounters,
) {
    counters.control_payload_lookup_count += 1;
    if let Some(control) = controls_by_id.get(subject_id) {
        let binding = WorthUiLiveViewCompositionChildBindingReceipt::from_admitted_child_row(row)
            .expect("control graph child row has a child binding subject");
        mounted_nodes.push(mounted_control_node(binding, control));
        counters.mounted_subject_count += 1;
        reconciliation_rows.push(reconciliation_row(
            GraphChildSubjectKind::Control,
            subject_id,
            row,
            WorthUiLiveViewCompositionSubjectReconciliationPosture::Mounted,
        ));
    } else {
        counters.missing_payload_count += 1;
        reconciliation_rows.push(reconciliation_row(
            GraphChildSubjectKind::Control,
            subject_id,
            row,
            WorthUiLiveViewCompositionSubjectReconciliationPosture::GraphChildMissingProjectionPayload,
        ));
    }
}

fn select_interaction_graph_child(
    subject_id: &str,
    row: &WorthUiCompositionGraphChildAccessRow,
    interactions_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderInteraction>,
    context_propagation: &WorthUiCompositionContextPropagationReceipt,
    mounted_nodes: &mut Vec<WorthUiMountedNodeReceipt>,
    reconciliation_rows: &mut Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: &mut WorthUiMountedGraphChildSelectionCounters,
) {
    counters.interaction_payload_lookup_count += 1;
    if let Some(interaction) = interactions_by_id.get(subject_id) {
        let binding = WorthUiLiveViewCompositionChildBindingReceipt::from_admitted_child_row(row)
            .expect("interaction graph child row has a child binding subject");
        mounted_nodes.push(mounted_interaction_node(
            binding,
            interaction,
            context_propagation,
        ));
        counters.mounted_subject_count += 1;
        reconciliation_rows.push(reconciliation_row(
            GraphChildSubjectKind::Interaction,
            subject_id,
            row,
            WorthUiLiveViewCompositionSubjectReconciliationPosture::Mounted,
        ));
    } else {
        counters.missing_payload_count += 1;
        reconciliation_rows.push(reconciliation_row(
            GraphChildSubjectKind::Interaction,
            subject_id,
            row,
            WorthUiLiveViewCompositionSubjectReconciliationPosture::GraphChildMissingProjectionPayload,
        ));
    }
}

fn controls_by_id(
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
) -> HashMap<&str, &WorthUiLiveViewProjectionRenderControl> {
    render_plan
        .controls()
        .iter()
        .map(|row| (row.control().control_id(), row))
        .collect()
}

fn interactions_by_id(
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
) -> HashMap<&str, &WorthUiLiveViewProjectionRenderInteraction> {
    render_plan
        .interactions()
        .iter()
        .map(|row| (row.interaction().interaction_id(), row))
        .collect()
}

fn mounted_control_node(
    binding: WorthUiLiveViewCompositionChildBindingReceipt,
    control: &WorthUiLiveViewProjectionRenderControl,
) -> WorthUiMountedNodeReceipt {
    WorthUiMountedNodeReceipt::Control(WorthUiMountedControlNodeReceipt::from_parts(
        binding,
        control.control().binding().clone(),
        control.host_frame().clone(),
    ))
}

fn mounted_interaction_node(
    binding: WorthUiLiveViewCompositionChildBindingReceipt,
    interaction: &WorthUiLiveViewProjectionRenderInteraction,
    context_propagation: &WorthUiCompositionContextPropagationReceipt,
) -> WorthUiMountedNodeReceipt {
    let context = context_propagation.context_for_node(binding.composition_node_id());
    WorthUiMountedNodeReceipt::Interaction(
        WorthUiMountedInteractionNodeReceipt::from_parts_with_context(
            binding,
            interaction.interaction().clone(),
            interaction.posture(),
            context.cloned(),
        ),
    )
}

fn reconciliation_row(
    subject_kind: GraphChildSubjectKind,
    subject_id: &str,
    row: &WorthUiCompositionGraphChildAccessRow,
    posture: WorthUiLiveViewCompositionSubjectReconciliationPosture,
) -> WorthUiLiveViewCompositionSubjectReconciliationRow {
    WorthUiLiveViewCompositionSubjectReconciliationRow::new(
        subject_kind.public_kind(),
        subject_id,
        Some(row.node().node_id().as_str()),
        Some(row.parent_id()),
        posture,
    )
}

fn append_declared_unmounted_control_rows(
    controls_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderControl>,
    graph_subject_counts: &HashMap<(GraphChildSubjectKind, String), usize>,
    rows: &mut Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: &mut WorthUiMountedGraphChildSelectionCounters,
) {
    for control_id in controls_by_id.keys() {
        if !graph_subject_counts
            .contains_key(&(GraphChildSubjectKind::Control, (*control_id).to_owned()))
        {
            counters.declared_unmounted_count += 1;
            rows.push(WorthUiLiveViewCompositionSubjectReconciliationRow::new(
                WorthUiLiveViewCompositionChildSubjectKind::Control,
                control_id,
                None,
                None,
                WorthUiLiveViewCompositionSubjectReconciliationPosture::DeclaredButUnmounted,
            ));
        }
    }
}

fn append_declared_unmounted_interaction_rows(
    interactions_by_id: &HashMap<&str, &WorthUiLiveViewProjectionRenderInteraction>,
    graph_subject_counts: &HashMap<(GraphChildSubjectKind, String), usize>,
    rows: &mut Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: &mut WorthUiMountedGraphChildSelectionCounters,
) {
    for interaction_id in interactions_by_id.keys() {
        if !graph_subject_counts.contains_key(&(
            GraphChildSubjectKind::Interaction,
            (*interaction_id).to_owned(),
        )) {
            counters.declared_unmounted_count += 1;
            rows.push(WorthUiLiveViewCompositionSubjectReconciliationRow::new(
                WorthUiLiveViewCompositionChildSubjectKind::Interaction,
                interaction_id,
                None,
                None,
                WorthUiLiveViewCompositionSubjectReconciliationPosture::DeclaredButUnmounted,
            ));
        }
    }
}

impl WorthUiMountedGraphChildSelectionReceipt {
    pub fn nodes(&self) -> &[WorthUiMountedNodeReceipt] {
        &self.nodes
    }

    pub fn reconciliation(&self) -> &WorthUiLiveViewCompositionSubjectReconciliationReceipt {
        &self.reconciliation
    }
}

impl GraphChildSubjectKind {
    const fn from_node_kind(kind: WorthUiCompositionNodeKind) -> Option<Self> {
        match kind {
            WorthUiCompositionNodeKind::Control => Some(Self::Control),
            WorthUiCompositionNodeKind::Interaction => Some(Self::Interaction),
            _ => None,
        }
    }

    const fn public_kind(self) -> WorthUiLiveViewCompositionChildSubjectKind {
        match self {
            Self::Control => WorthUiLiveViewCompositionChildSubjectKind::Control,
            Self::Interaction => WorthUiLiveViewCompositionChildSubjectKind::Interaction,
        }
    }
}
