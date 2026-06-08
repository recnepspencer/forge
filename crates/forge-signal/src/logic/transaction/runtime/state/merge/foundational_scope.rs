use forge_foundational::facade::{
    FoundationalMergeConstructionDenial, FoundationalMergeScope, FoundationalSelectedAspectLocus,
    FoundationalSelectedAspectRequestEntry, FoundationalSelectedNodeLocus,
};

use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeRequestScopeFamily, NormalizedBranchMergeRequest, NormalizedBranchMergeRequestScope,
    SignalSelectedAspectRequestEntry,
};
use crate::state::SignalBranchHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredFoundationalMergeRequest {
    normalized_request: NormalizedBranchMergeRequest,
    foundational_scope: FoundationalMergeScope,
}

impl LoweredFoundationalMergeRequest {
    pub fn normalized_request(&self) -> &NormalizedBranchMergeRequest {
        &self.normalized_request
    }

    pub fn foundational_scope(&self) -> &FoundationalMergeScope {
        &self.foundational_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalScopeLoweringDenial {
    FoundationalConstruction(FoundationalMergeConstructionDenial),
}

impl FoundationalScopeLoweringDenial {
    pub fn into_signal_error(self) -> SignalError {
        match self {
            Self::FoundationalConstruction(denial) => SignalError::invalid_input(format!(
                "foundational merge scope lowering denied the normalized signal request: {denial:?}"
            )),
        }
    }
}

impl NormalizedBranchMergeRequest {
    pub fn lower_to_foundational_scope(
        &self,
    ) -> Result<LoweredFoundationalMergeRequest, FoundationalScopeLoweringDenial> {
        Ok(LoweredFoundationalMergeRequest {
            normalized_request: self.clone(),
            foundational_scope: lower_foundational_merge_scope(self.normalized_scope())?,
        })
    }
}

fn lower_foundational_merge_scope(
    scope: &NormalizedBranchMergeRequestScope,
) -> Result<FoundationalMergeScope, FoundationalScopeLoweringDenial> {
    match scope {
        NormalizedBranchMergeRequestScope::FullBranch { .. } => {
            Ok(FoundationalMergeScope::full_branch())
        }
        NormalizedBranchMergeRequestScope::SelectedNodes { selected_nodes, .. } => {
            let selected_nodes = selected_nodes.iter().map(|node| {
                FoundationalSelectedNodeLocus::new(foundational_selected_node_locus(node))
                    .map_err(FoundationalScopeLoweringDenial::FoundationalConstruction)
            });
            FoundationalMergeScope::selected_nodes(collect_results(selected_nodes)?)
                .map_err(FoundationalScopeLoweringDenial::FoundationalConstruction)
        }
        NormalizedBranchMergeRequestScope::SelectedAspects {
            selected_aspects, ..
        } => {
            let selected_aspects = selected_aspects.iter().map(|aspect| {
                let node = FoundationalSelectedNodeLocus::new(foundational_selected_node_locus(
                    &aspect.node(),
                ))
                .map_err(FoundationalScopeLoweringDenial::FoundationalConstruction)?;
                let aspect = FoundationalSelectedAspectLocus::new(
                    foundational_selected_aspect_locus(aspect.aspect().id()),
                )
                .map_err(FoundationalScopeLoweringDenial::FoundationalConstruction)?;
                Ok::<_, FoundationalScopeLoweringDenial>(
                    FoundationalSelectedAspectRequestEntry::new(node, aspect),
                )
            });
            FoundationalMergeScope::selected_aspects(collect_results(selected_aspects)?)
                .map_err(FoundationalScopeLoweringDenial::FoundationalConstruction)
        }
    }
}

fn collect_results<T>(
    values: impl IntoIterator<Item = Result<T, FoundationalScopeLoweringDenial>>,
) -> Result<Vec<T>, FoundationalScopeLoweringDenial> {
    values.into_iter().collect()
}

pub(crate) fn foundational_branch_id(
    branch: &SignalBranchHandle,
) -> forge_foundational::facade::FoundationalBranchId {
    forge_foundational::facade::FoundationalBranchId::new(format!(
        "signal-branch:{}:{}",
        branch.id.0, branch.name
    ))
    .expect("signal branch handles must always lower to a non-empty foundational branch id")
}

pub(crate) fn foundational_selected_node_locus(node: &crate::data::handle::NodeId) -> String {
    format!("signal-node:{}:{}", node.index(), node.generation())
}

pub(crate) fn foundational_selected_aspect_locus(aspect_id: u8) -> String {
    format!("signal-aspect:{aspect_id}")
}

pub(crate) fn foundational_denied_node_locus(
    node: crate::data::handle::NodeId,
) -> forge_foundational::facade::FoundationalSelectedNodeLocus {
    forge_foundational::facade::FoundationalSelectedNodeLocus::new(
        foundational_selected_node_locus(&node),
    )
    .expect("signal node ids must lower to a non-empty foundational selected-node locus")
}

pub(crate) fn foundational_denied_aspect_locus(
    aspect: &SignalSelectedAspectRequestEntry,
) -> forge_foundational::facade::FoundationalSelectedAspectRequestEntry {
    forge_foundational::facade::FoundationalSelectedAspectRequestEntry::new(
        foundational_denied_node_locus(aspect.node()),
        forge_foundational::facade::FoundationalSelectedAspectLocus::new(
            foundational_selected_aspect_locus(aspect.aspect().id()),
        )
        .expect("signal aspect ids must lower to a non-empty foundational selected-aspect locus"),
    )
}

#[allow(dead_code)]
pub fn foundational_scope_family_label(
    family: forge_foundational::facade::FoundationalMergeScopeFamily,
) -> &'static str {
    match family {
        forge_foundational::facade::FoundationalMergeScopeFamily::FullBranch => "full-branch",
        forge_foundational::facade::FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        forge_foundational::facade::FoundationalMergeScopeFamily::SelectedAspects => {
            "selected-aspects"
        }
    }
}

pub fn signal_scope_family_matches_foundational_family(
    signal_family: BranchMergeRequestScopeFamily,
    foundational_family: forge_foundational::facade::FoundationalMergeScopeFamily,
) -> bool {
    matches!(
        (signal_family, foundational_family),
        (
            BranchMergeRequestScopeFamily::FullBranch,
            forge_foundational::facade::FoundationalMergeScopeFamily::FullBranch,
        ) | (
            BranchMergeRequestScopeFamily::SelectedNodes,
            forge_foundational::facade::FoundationalMergeScopeFamily::SelectedNodes,
        ) | (
            BranchMergeRequestScopeFamily::SelectedAspects,
            forge_foundational::facade::FoundationalMergeScopeFamily::SelectedAspects,
        )
    )
}
