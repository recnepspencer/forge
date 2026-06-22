use super::super::{
    ForgeQueryAdmittedBooleanExpressionBranchKind, ForgeQueryBooleanSelectivityBranchKind,
    ForgeQueryPredicateAnchorPosture, ForgeQueryTraversalPredicateOrderingPosture,
};
use super::row::ForgeQueryBooleanPredicateSelectivityRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBooleanSelectivityBranch {
    branch_kind: ForgeQueryBooleanSelectivityBranchKind,
    expression_path: String,
    anchor_posture: ForgeQueryPredicateAnchorPosture,
    traversal_predicate_ordering_posture: ForgeQueryTraversalPredicateOrderingPosture,
    predicate_rows: Vec<ForgeQueryBooleanPredicateSelectivityRow>,
}

impl ForgeQueryBooleanSelectivityBranch {
    pub fn branch_kind(&self) -> &ForgeQueryBooleanSelectivityBranchKind {
        &self.branch_kind
    }

    pub fn expression_path(&self) -> &str {
        &self.expression_path
    }

    pub fn branch_identity(&self) -> &str {
        &self.expression_path
    }

    pub fn anchor_posture(&self) -> &ForgeQueryPredicateAnchorPosture {
        &self.anchor_posture
    }

    pub fn traversal_predicate_ordering_posture(
        &self,
    ) -> &ForgeQueryTraversalPredicateOrderingPosture {
        &self.traversal_predicate_ordering_posture
    }

    pub fn predicate_rows(&self) -> &[ForgeQueryBooleanPredicateSelectivityRow] {
        &self.predicate_rows
    }

    pub(crate) fn from_expression_branch(
        branch_kind: ForgeQueryBooleanSelectivityBranchKind,
        expression_path: impl Into<String>,
        anchor_posture: ForgeQueryPredicateAnchorPosture,
        traversal_predicate_ordering_posture: ForgeQueryTraversalPredicateOrderingPosture,
        predicate_rows: Vec<ForgeQueryBooleanPredicateSelectivityRow>,
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

impl From<&ForgeQueryAdmittedBooleanExpressionBranchKind>
    for ForgeQueryBooleanSelectivityBranchKind
{
    fn from(kind: &ForgeQueryAdmittedBooleanExpressionBranchKind) -> Self {
        match kind {
            ForgeQueryAdmittedBooleanExpressionBranchKind::EmptyRoot => Self::EmptyRoot,
            ForgeQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot => Self::ConjunctiveRoot,
        }
    }
}
