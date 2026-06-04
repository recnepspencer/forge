use std::collections::BTreeSet;

use super::vocabulary::FoundationalMergeConstructionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalMergeScopeFamily {
    FullBranch,
    SelectedNodes,
    SelectedAspects,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSelectedNodeLocus(String);

impl FoundationalSelectedNodeLocus {
    pub fn new(value: impl Into<String>) -> Result<Self, FoundationalMergeConstructionDenial> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(FoundationalMergeConstructionDenial::EmptySelectedNodeLocus);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSelectedAspectLocus(String);

impl FoundationalSelectedAspectLocus {
    pub fn new(value: impl Into<String>) -> Result<Self, FoundationalMergeConstructionDenial> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(FoundationalMergeConstructionDenial::EmptySelectedAspectLocus);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSelectedAspectRequestEntry {
    node: FoundationalSelectedNodeLocus,
    aspect: FoundationalSelectedAspectLocus,
}

impl FoundationalSelectedAspectRequestEntry {
    pub fn new(
        node: FoundationalSelectedNodeLocus,
        aspect: FoundationalSelectedAspectLocus,
    ) -> Self {
        Self { node, aspect }
    }

    pub fn node(&self) -> &FoundationalSelectedNodeLocus {
        &self.node
    }

    pub fn aspect(&self) -> &FoundationalSelectedAspectLocus {
        &self.aspect
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeScope {
    family: FoundationalMergeScopeFamily,
    selected_nodes: Vec<FoundationalSelectedNodeLocus>,
    selected_aspects: Vec<FoundationalSelectedAspectRequestEntry>,
}

impl FoundationalMergeScope {
    pub fn full_branch() -> Self {
        Self {
            family: FoundationalMergeScopeFamily::FullBranch,
            selected_nodes: Vec::new(),
            selected_aspects: Vec::new(),
        }
    }

    pub fn selected_nodes(
        nodes: impl IntoIterator<Item = FoundationalSelectedNodeLocus>,
    ) -> Result<Self, FoundationalMergeConstructionDenial> {
        let selected_nodes = sorted_unique_nodes(nodes)?;
        if selected_nodes.is_empty() {
            return Err(FoundationalMergeConstructionDenial::EmptySelectedNodeScope);
        }
        Ok(Self {
            family: FoundationalMergeScopeFamily::SelectedNodes,
            selected_nodes,
            selected_aspects: Vec::new(),
        })
    }

    pub fn selected_aspects(
        aspects: impl IntoIterator<Item = FoundationalSelectedAspectRequestEntry>,
    ) -> Result<Self, FoundationalMergeConstructionDenial> {
        let selected_aspects = sorted_unique_aspects(aspects)?;
        if selected_aspects.is_empty() {
            return Err(FoundationalMergeConstructionDenial::EmptySelectedAspectScope);
        }
        Ok(Self {
            family: FoundationalMergeScopeFamily::SelectedAspects,
            selected_nodes: Vec::new(),
            selected_aspects,
        })
    }

    pub const fn family(&self) -> FoundationalMergeScopeFamily {
        self.family
    }

    pub fn selected_nodes_loci(&self) -> &[FoundationalSelectedNodeLocus] {
        &self.selected_nodes
    }

    pub fn selected_aspect_loci(&self) -> &[FoundationalSelectedAspectRequestEntry] {
        &self.selected_aspects
    }

    pub(crate) fn requested_locus_count(&self) -> u64 {
        match self.family {
            FoundationalMergeScopeFamily::FullBranch => 1,
            FoundationalMergeScopeFamily::SelectedNodes => self.selected_nodes.len() as u64,
            FoundationalMergeScopeFamily::SelectedAspects => self.selected_aspects.len() as u64,
        }
    }
}

pub(crate) fn sorted_unique_nodes(
    nodes: impl IntoIterator<Item = FoundationalSelectedNodeLocus>,
) -> Result<Vec<FoundationalSelectedNodeLocus>, FoundationalMergeConstructionDenial> {
    let mut seen = BTreeSet::new();
    let mut selected_nodes = Vec::new();
    for node in nodes {
        if !seen.insert(node.clone()) {
            return Err(FoundationalMergeConstructionDenial::DuplicateSelectedNodeLocus);
        }
        selected_nodes.push(node);
    }
    selected_nodes.sort();
    Ok(selected_nodes)
}

pub(crate) fn sorted_unique_aspects(
    aspects: impl IntoIterator<Item = FoundationalSelectedAspectRequestEntry>,
) -> Result<Vec<FoundationalSelectedAspectRequestEntry>, FoundationalMergeConstructionDenial> {
    let mut seen = BTreeSet::new();
    let mut selected_aspects = Vec::new();
    for aspect in aspects {
        if !seen.insert(aspect.clone()) {
            return Err(FoundationalMergeConstructionDenial::DuplicateSelectedAspectLocus);
        }
        selected_aspects.push(aspect);
    }
    selected_aspects.sort();
    Ok(selected_aspects)
}
