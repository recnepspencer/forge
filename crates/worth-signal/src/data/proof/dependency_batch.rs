use serde::{Deserialize, Serialize};

use crate::data::dependency::CanonicalDependencies;
use crate::data::handle::NodeId;

use super::locality::node_sort_key;
use super::CanonicalForm;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySetEdit {
    pub node: NodeId,
    pub dependencies: CanonicalDependencies,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencyBatchEdit {
    edits: Vec<DependencySetEdit>,
}

impl DependencyBatchEdit {
    pub fn new(edits: impl IntoIterator<Item = DependencySetEdit>) -> Self {
        let mut edits = edits.into_iter().collect::<Vec<_>>();
        if edits.len() > 1 {
            edits.sort_unstable_by_key(|edit| node_sort_key(&edit.node));
            let duplicate = edits
                .windows(2)
                .find(|pair| pair[0].node == pair[1].node)
                .map(|pair| pair[0].node);
            assert!(
                duplicate.is_none(),
                "dependency batch edit cannot contain multiple edits for node {:?}",
                duplicate
            );
        }
        Self { edits }
    }

    pub fn from_pairs(
        edits: impl IntoIterator<Item = (NodeId, impl Into<CanonicalDependencies>)>,
    ) -> Self {
        Self::new(
            edits
                .into_iter()
                .map(|(node, dependencies)| DependencySetEdit {
                    node,
                    dependencies: dependencies.into(),
                }),
        )
    }

    pub fn singleton(node: NodeId, dependencies: impl Into<CanonicalDependencies>) -> Self {
        Self::new(std::iter::once(DependencySetEdit {
            node,
            dependencies: dependencies.into(),
        }))
    }

    pub fn as_slice(&self) -> &[DependencySetEdit] {
        &self.edits
    }

    pub fn into_vec(self) -> Vec<DependencySetEdit> {
        self.edits
    }

    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}

impl CanonicalForm for CanonicalDependencies {}
impl CanonicalForm for DependencyBatchEdit {}
