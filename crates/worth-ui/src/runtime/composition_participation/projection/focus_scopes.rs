use super::digest::digest_parts;
use super::receipt::WorthUiFocusNodeParticipationReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFocusScopeParticipationReceipt {
    focus_scope_id: String,
    owner_node_id: String,
    focus_nodes: Vec<WorthUiFocusNodeParticipationReceipt>,
    receipt_digest: u64,
}

pub(super) fn focus_scopes(
    focus_nodes: &[WorthUiFocusNodeParticipationReceipt],
) -> Vec<WorthUiFocusScopeParticipationReceipt> {
    let mut scope_ids = focus_nodes
        .iter()
        .map(|node| node.focus_scope_id().to_owned())
        .collect::<Vec<_>>();
    scope_ids.sort();
    scope_ids.dedup();
    scope_ids
        .into_iter()
        .map(|scope_id| {
            let scoped_nodes = focus_nodes
                .iter()
                .filter(|node| node.focus_scope_id() == scope_id)
                .cloned()
                .collect();
            WorthUiFocusScopeParticipationReceipt::new(scope_id.clone(), scope_id, scoped_nodes)
        })
        .collect()
}

impl WorthUiFocusScopeParticipationReceipt {
    fn new(
        focus_scope_id: impl Into<String>,
        owner_node_id: impl Into<String>,
        mut focus_nodes: Vec<WorthUiFocusNodeParticipationReceipt>,
    ) -> Self {
        let focus_scope_id = focus_scope_id.into();
        let owner_node_id = owner_node_id.into();
        focus_nodes.sort_by_key(|node| node.graph_order());
        let receipt_digest = digest_parts(
            [
                "focus_scope_participation".to_owned(),
                focus_scope_id.clone(),
                owner_node_id.clone(),
            ]
            .into_iter()
            .chain(
                focus_nodes
                    .iter()
                    .map(|node| node.receipt_digest().to_string()),
            ),
        );
        Self {
            focus_scope_id,
            owner_node_id,
            focus_nodes,
            receipt_digest,
        }
    }

    pub fn focus_scope_id(&self) -> &str {
        &self.focus_scope_id
    }

    pub fn owner_node_id(&self) -> &str {
        &self.owner_node_id
    }

    pub fn focus_nodes(&self) -> &[WorthUiFocusNodeParticipationReceipt] {
        &self.focus_nodes
    }

    pub fn tab_order_node_ids(&self) -> Vec<&str> {
        self.focus_nodes.iter().map(|node| node.node_id()).collect()
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
