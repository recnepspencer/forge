use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionRootMountDenial,
    WorthUiCompositionRootMountDenialCode, WorthUiCompositionRootMountReport,
    WorthUiCompositionRootReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootSetDefinition {
    graphs: Vec<WorthUiAdmittedCompositionGraphReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedCompositionRootSetReceipt {
    roots: Vec<WorthUiCompositionRootSetReceipt>,
    graphs: Vec<WorthUiAdmittedCompositionGraphReceipt>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootSetReceipt {
    root: WorthUiCompositionRootReceipt,
    graph_digest: u64,
    receipt_digest: u64,
}

impl WorthUiCompositionRootSetDefinition {
    pub fn from_graphs(
        graphs: impl IntoIterator<Item = WorthUiAdmittedCompositionGraphReceipt>,
    ) -> Self {
        Self {
            graphs: graphs.into_iter().collect(),
        }
    }

    pub fn admit(
        self,
    ) -> Result<WorthUiAdmittedCompositionRootSetReceipt, WorthUiCompositionRootMountReport> {
        let mut seen = BTreeSet::new();
        let mut denials = Vec::new();
        for graph in &self.graphs {
            let root = graph.root();
            let identity_key = format!("{}:{}", root.kind().token(), root.authority_identity());
            if !seen.insert(identity_key) {
                denials.push(WorthUiCompositionRootMountDenial::new(
                    WorthUiCompositionRootMountDenialCode::DuplicateRootIdentity,
                    root,
                    root.authority_identity(),
                    vec![root.fact_id().clone()],
                ));
            }
        }
        if !denials.is_empty() {
            return Err(WorthUiCompositionRootMountReport::denied(denials));
        }
        Ok(WorthUiAdmittedCompositionRootSetReceipt::new(self.graphs))
    }
}

impl WorthUiAdmittedCompositionRootSetReceipt {
    fn new(graphs: Vec<WorthUiAdmittedCompositionGraphReceipt>) -> Self {
        let mut roots = graphs
            .iter()
            .map(WorthUiCompositionRootSetReceipt::from_graph)
            .collect::<Vec<_>>();
        roots.sort_by(|left, right| left.root().root_id().cmp(right.root().root_id()));
        let mut consumed_facts = graphs
            .iter()
            .flat_map(|graph| graph.consumed_facts().iter().cloned())
            .collect::<Vec<_>>();
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = super::super::digest::digest_parts(
            ["composition_root_set".to_owned()]
                .into_iter()
                .chain(roots.iter().map(|root| root.receipt_digest().to_string()))
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            roots,
            graphs,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn roots(&self) -> &[WorthUiCompositionRootSetReceipt] {
        &self.roots
    }

    pub fn graphs(&self) -> &[WorthUiAdmittedCompositionGraphReceipt] {
        &self.graphs
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionRootSetReceipt {
    fn from_graph(graph: &WorthUiAdmittedCompositionGraphReceipt) -> Self {
        let root = graph.root().clone();
        let graph_digest = graph.receipt_digest();
        let receipt_digest = super::super::digest::digest_parts([
            "composition_root_set_entry",
            root.receipt_digest().to_string().as_str(),
            graph_digest.to_string().as_str(),
        ]);
        Self {
            root,
            graph_digest,
            receipt_digest,
        }
    }

    pub fn root(&self) -> &WorthUiCompositionRootReceipt {
        &self.root
    }

    pub fn graph_digest(&self) -> u64 {
        self.graph_digest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
