use worth_foundational::facade::{
    FoundationalMergeScopeFamily, FoundationalMergeScopeLocator,
    FoundationalSelectedAspectScopeLocator, FoundationalSelectedNodeScopeLocator,
    FoundationalTransitionLocator,
};

use crate::data::handle::NodeId;
use crate::logic::transaction::runtime::{
    BranchMergeExecutionSummary, BranchMergeRequestScopeFamily, BranchMergeResult,
    BranchMergeScopedDenialFailureEvidence, BranchMergeScopedDeniedLocus,
    BranchMergeScopedUnavailableFailureEvidence, LoweredMergePlan, ScopedMergeProofPacket,
    SignalSelectedAspectRequestEntry,
};
use crate::state::SignalBranchId;

use super::foundational_scope::{foundational_denied_aspect_locus, foundational_denied_node_locus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalScopedMergeLocatorBundle {
    scope: FoundationalTransitionLocator,
    requested: Vec<FoundationalTransitionLocator>,
    admitted: Vec<FoundationalTransitionLocator>,
    skipped: Vec<FoundationalTransitionLocator>,
    no_op: Vec<FoundationalTransitionLocator>,
    support_closure: Vec<FoundationalTransitionLocator>,
}

impl SignalScopedMergeLocatorBundle {
    pub fn scope(&self) -> &FoundationalTransitionLocator {
        &self.scope
    }

    pub fn requested(&self) -> &[FoundationalTransitionLocator] {
        &self.requested
    }

    pub fn admitted(&self) -> &[FoundationalTransitionLocator] {
        &self.admitted
    }

    pub fn skipped(&self) -> &[FoundationalTransitionLocator] {
        &self.skipped
    }

    pub fn no_op(&self) -> &[FoundationalTransitionLocator] {
        &self.no_op
    }

    pub fn support_closure(&self) -> &[FoundationalTransitionLocator] {
        &self.support_closure
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalScopedMergeDiagnosticRow {
    code: &'static str,
    locator: FoundationalTransitionLocator,
    labels: Vec<&'static str>,
    digest: String,
}

impl SignalScopedMergeDiagnosticRow {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn locator(&self) -> &FoundationalTransitionLocator {
        &self.locator
    }

    pub fn labels(&self) -> &[&'static str] {
        &self.labels
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

impl ScopedMergeProofPacket {
    pub fn locator_bundle(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> SignalScopedMergeLocatorBundle {
        SignalScopedMergeLocatorBundle {
            scope: scope_locator(source_branch_id, target_branch_id, self.scope_family()),
            requested: node_locators(source_branch_id, target_branch_id, self.requested_nodes())
                .into_iter()
                .chain(aspect_locators(
                    source_branch_id,
                    target_branch_id,
                    self.requested_aspects(),
                ))
                .collect(),
            admitted: node_locators(source_branch_id, target_branch_id, self.admitted_nodes())
                .into_iter()
                .chain(aspect_locators(
                    source_branch_id,
                    target_branch_id,
                    self.admitted_aspects(),
                ))
                .collect(),
            skipped: node_locators(source_branch_id, target_branch_id, self.skipped_nodes())
                .into_iter()
                .chain(aspect_locators(
                    source_branch_id,
                    target_branch_id,
                    self.skipped_aspects(),
                ))
                .collect(),
            no_op: node_locators(source_branch_id, target_branch_id, self.no_op_nodes())
                .into_iter()
                .chain(aspect_locators(
                    source_branch_id,
                    target_branch_id,
                    self.no_op_aspects(),
                ))
                .collect(),
            support_closure: node_locators(
                source_branch_id,
                target_branch_id,
                self.support_closure_nodes(),
            ),
        }
    }

    pub fn compact_diagnostic_rows(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> Vec<SignalScopedMergeDiagnosticRow> {
        let locators = self.locator_bundle(source_branch_id, target_branch_id);
        let mut rows = vec![SignalScopedMergeDiagnosticRow {
            code: "merge-scope.requested",
            locator: locators.scope().clone(),
            labels: vec![
                "merge-scope",
                "requested",
                family_label(self.scope_family()),
            ],
            digest: self.declaration_digest().to_owned(),
        }];
        if !locators.admitted().is_empty()
            || self.scope_family() == BranchMergeRequestScopeFamily::FullBranch
        {
            rows.push(SignalScopedMergeDiagnosticRow {
                code: "merge-scope.admitted",
                locator: locators.scope().clone(),
                labels: vec!["merge-scope", "admitted", family_label(self.scope_family())],
                digest: self.admitted_scope_digest().to_owned(),
            });
        }
        if let Some(digest) = self.skipped_scope_digest() {
            rows.push(SignalScopedMergeDiagnosticRow {
                code: "merge-scope.skipped",
                locator: locators.scope().clone(),
                labels: vec!["merge-scope", "skipped"],
                digest: digest.to_owned(),
            });
        }
        if let Some(digest) = self.no_op_scope_digest() {
            rows.push(SignalScopedMergeDiagnosticRow {
                code: "merge-scope.no-op",
                locator: locators.scope().clone(),
                labels: vec!["merge-scope", "no-op"],
                digest: digest.to_owned(),
            });
        }
        rows
    }
}

impl LoweredMergePlan {
    pub fn scoped_merge_locator_bundle(&self) -> SignalScopedMergeLocatorBundle {
        self.scoped_merge_proof()
            .locator_bundle(self.source_branch_id(), self.target_branch_id())
    }

    pub fn scoped_merge_compact_diagnostic_rows(&self) -> Vec<SignalScopedMergeDiagnosticRow> {
        self.scoped_merge_proof()
            .compact_diagnostic_rows(self.source_branch_id(), self.target_branch_id())
    }
}

impl BranchMergeExecutionSummary {
    pub fn scoped_merge_locator_bundle(&self) -> SignalScopedMergeLocatorBundle {
        self.scoped_merge_proof
            .locator_bundle(self.source_branch_id, self.target_branch_id)
    }

    pub fn scoped_merge_compact_diagnostic_rows(&self) -> Vec<SignalScopedMergeDiagnosticRow> {
        self.scoped_merge_proof
            .compact_diagnostic_rows(self.source_branch_id, self.target_branch_id)
    }
}

impl BranchMergeResult {
    pub fn scoped_merge_locator_bundle(&self) -> SignalScopedMergeLocatorBundle {
        self.scoped_merge_proof
            .locator_bundle(self.source_branch, self.target_branch)
    }

    pub fn scoped_merge_compact_diagnostic_rows(&self) -> Vec<SignalScopedMergeDiagnosticRow> {
        self.scoped_merge_proof
            .compact_diagnostic_rows(self.source_branch, self.target_branch)
    }
}

impl BranchMergeScopedDenialFailureEvidence {
    pub fn denied_locator(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> FoundationalTransitionLocator {
        match &self.denied_locus {
            BranchMergeScopedDeniedLocus::Node(node) => {
                node_locator(source_branch_id, target_branch_id, *node)
            }
            BranchMergeScopedDeniedLocus::Aspect(entry) => {
                aspect_locator(source_branch_id, target_branch_id, entry.clone())
            }
            BranchMergeScopedDeniedLocus::ScopeFamily(family) => {
                scope_locator(source_branch_id, target_branch_id, *family)
            }
        }
    }

    pub fn compact_diagnostic_row(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> SignalScopedMergeDiagnosticRow {
        SignalScopedMergeDiagnosticRow {
            code: "merge-scope.denied",
            locator: self.denied_locator(source_branch_id, target_branch_id),
            labels: vec!["merge-scope", "denied"],
            digest: self.scope_digest.clone(),
        }
    }
}

impl BranchMergeScopedUnavailableFailureEvidence {
    pub fn unavailable_locator(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> FoundationalTransitionLocator {
        scope_locator(source_branch_id, target_branch_id, self.scope_family)
    }

    pub fn compact_diagnostic_row(
        &self,
        source_branch_id: SignalBranchId,
        target_branch_id: SignalBranchId,
    ) -> SignalScopedMergeDiagnosticRow {
        SignalScopedMergeDiagnosticRow {
            code: "merge-scope.unavailable",
            locator: self.unavailable_locator(source_branch_id, target_branch_id),
            labels: vec!["merge-scope", "unavailable"],
            digest: self.scope_digest.clone(),
        }
    }
}

pub(crate) fn foundational_branch_id_from_runtime_id(
    branch_id: SignalBranchId,
) -> worth_foundational::facade::FoundationalBranchId {
    worth_foundational::facade::FoundationalBranchId::new(format!("signal-branch:{}", branch_id.0))
        .expect("signal branch ids must lower to non-empty foundational branch ids")
}

pub(crate) fn scope_locator(
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    family: BranchMergeRequestScopeFamily,
) -> FoundationalTransitionLocator {
    FoundationalTransitionLocator::MergeScope(FoundationalMergeScopeLocator::new(
        foundational_branch_id_from_runtime_id(source_branch_id),
        foundational_branch_id_from_runtime_id(target_branch_id),
        match family {
            BranchMergeRequestScopeFamily::FullBranch => FoundationalMergeScopeFamily::FullBranch,
            BranchMergeRequestScopeFamily::SelectedNodes => {
                FoundationalMergeScopeFamily::SelectedNodes
            }
            BranchMergeRequestScopeFamily::SelectedAspects => {
                FoundationalMergeScopeFamily::SelectedAspects
            }
        },
    ))
}

pub(crate) fn node_locators(
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    nodes: &[NodeId],
) -> Vec<FoundationalTransitionLocator> {
    nodes
        .iter()
        .copied()
        .map(|node| node_locator(source_branch_id, target_branch_id, node))
        .collect()
}

fn node_locator(
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    node: NodeId,
) -> FoundationalTransitionLocator {
    FoundationalTransitionLocator::SelectedNodeScope(FoundationalSelectedNodeScopeLocator::new(
        foundational_branch_id_from_runtime_id(source_branch_id),
        foundational_branch_id_from_runtime_id(target_branch_id),
        foundational_denied_node_locus(node),
    ))
}

pub(crate) fn aspect_locators(
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    aspects: &[SignalSelectedAspectRequestEntry],
) -> Vec<FoundationalTransitionLocator> {
    aspects
        .iter()
        .cloned()
        .map(|aspect| aspect_locator(source_branch_id, target_branch_id, aspect))
        .collect()
}

fn aspect_locator(
    source_branch_id: SignalBranchId,
    target_branch_id: SignalBranchId,
    aspect: SignalSelectedAspectRequestEntry,
) -> FoundationalTransitionLocator {
    FoundationalTransitionLocator::SelectedAspectScope(FoundationalSelectedAspectScopeLocator::new(
        foundational_branch_id_from_runtime_id(source_branch_id),
        foundational_branch_id_from_runtime_id(target_branch_id),
        foundational_denied_aspect_locus(&aspect),
    ))
}

fn family_label(family: BranchMergeRequestScopeFamily) -> &'static str {
    match family {
        BranchMergeRequestScopeFamily::FullBranch => "full-branch",
        BranchMergeRequestScopeFamily::SelectedNodes => "selected-nodes",
        BranchMergeRequestScopeFamily::SelectedAspects => "selected-aspects",
    }
}
