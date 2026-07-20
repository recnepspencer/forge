use super::super::{
    WorthQueryAdmittedBooleanExpressionBranchKind, WorthQueryBooleanSelectivityBranchKind,
    WorthQueryPredicateAnchorPosture, WorthQueryTraversalPredicateOrderingPosture,
};
use super::row::WorthQueryBooleanPredicateSelectivityRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBooleanSelectivityBranch {
    branch_kind: WorthQueryBooleanSelectivityBranchKind,
    expression_path: String,
    anchor_posture: WorthQueryPredicateAnchorPosture,
    traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture,
    predicate_rows: Vec<WorthQueryBooleanPredicateSelectivityRow>,
}

impl WorthQueryBooleanSelectivityBranch {
    pub fn branch_kind(&self) -> &WorthQueryBooleanSelectivityBranchKind {
        &self.branch_kind
    }

    pub fn expression_path(&self) -> &str {
        &self.expression_path
    }

    pub fn branch_identity(&self) -> &str {
        &self.expression_path
    }

    pub fn anchor_posture(&self) -> &WorthQueryPredicateAnchorPosture {
        &self.anchor_posture
    }

    pub fn traversal_predicate_ordering_posture(
        &self,
    ) -> &WorthQueryTraversalPredicateOrderingPosture {
        &self.traversal_predicate_ordering_posture
    }

    pub fn predicate_rows(&self) -> &[WorthQueryBooleanPredicateSelectivityRow] {
        &self.predicate_rows
    }

    pub(crate) fn from_expression_branch(
        branch_kind: WorthQueryBooleanSelectivityBranchKind,
        expression_path: impl Into<String>,
        anchor_posture: WorthQueryPredicateAnchorPosture,
        traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture,
        predicate_rows: Vec<WorthQueryBooleanPredicateSelectivityRow>,
    ) -> Self {
        Self {
            branch_kind,
            expression_path: expression_path.into(),
            anchor_posture,
            traversal_predicate_ordering_posture,
            predicate_rows,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        let row_parts = self
            .predicate_rows
            .iter()
            .map(|row| row.digest_part())
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "branch:{}:{}:{}:{}:{}",
            self.branch_kind.as_str(),
            self.expression_path,
            self.anchor_posture.as_str(),
            self.traversal_predicate_ordering_posture.as_str(),
            row_parts
        )
    }
}

impl From<&WorthQueryAdmittedBooleanExpressionBranchKind>
    for WorthQueryBooleanSelectivityBranchKind
{
    fn from(kind: &WorthQueryAdmittedBooleanExpressionBranchKind) -> Self {
        match kind {
            WorthQueryAdmittedBooleanExpressionBranchKind::EmptyRoot => Self::EmptyRoot,
            WorthQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot => Self::ConjunctiveRoot,
        }
    }
}
