use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) enum UiMountedThemeValueSource {
    ActiveCurrent {
        values: Arc<BTreeMap<crate::capability::ThemeTokenId, crate::capability::ThemeTokenValue>>,
        changed_graph_nodes: Arc<[crate::graph::UiGraphNodeIdentity]>,
    },
    ReplacementCandidateFrozenPlan,
    PreviewOnly,
}

impl UiMountedThemeValueSource {
    pub(crate) fn current(
        values: Arc<BTreeMap<crate::capability::ThemeTokenId, crate::capability::ThemeTokenValue>>,
        changed_graph_nodes: impl IntoIterator<Item = crate::graph::UiGraphNodeIdentity>,
    ) -> Self {
        let mut changed_graph_nodes = changed_graph_nodes.into_iter().collect::<Vec<_>>();
        changed_graph_nodes.sort();
        changed_graph_nodes.dedup();
        Self::ActiveCurrent {
            values,
            changed_graph_nodes: changed_graph_nodes.into(),
        }
    }

    pub(crate) const fn replacement_candidate_frozen_plan() -> Self {
        Self::ReplacementCandidateFrozenPlan
    }

    pub(crate) const fn preview_only() -> Self {
        Self::PreviewOnly
    }

    pub(crate) fn current_value(
        &self,
        token: &crate::capability::ThemeTokenId,
    ) -> Option<&crate::capability::ThemeTokenValue> {
        match self {
            Self::ActiveCurrent { values, .. } => values.get(token),
            Self::ReplacementCandidateFrozenPlan | Self::PreviewOnly => None,
        }
    }

    pub(crate) const fn uses_frozen_plan(&self) -> bool {
        matches!(self, Self::ReplacementCandidateFrozenPlan)
    }

    pub(crate) fn changed_graph_nodes(&self) -> &[crate::graph::UiGraphNodeIdentity] {
        match self {
            Self::ActiveCurrent {
                changed_graph_nodes,
                ..
            } => changed_graph_nodes,
            Self::ReplacementCandidateFrozenPlan | Self::PreviewOnly => &[],
        }
    }

    pub(crate) fn changes_graph_node(&self, graph_node: crate::graph::UiGraphNodeIdentity) -> bool {
        self.changed_graph_nodes()
            .binary_search(&graph_node)
            .is_ok()
    }
}
