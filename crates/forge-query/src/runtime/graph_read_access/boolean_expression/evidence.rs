use super::super::{
    ForgeQueryGraphReadAdmittedSchemaFieldKind, ForgeQueryPredicateOperandOperator,
    ForgeQueryPredicateSelectivityClass,
};
use crate::declarative_live::DeclarativePredicateFilter;
use crate::runtime::ForgeQueryReadGraph;
use forge_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryBooleanExpressionAdmissionErrorKind {
    MissingAdmittedPredicateReference,
}

impl ForgeQueryBooleanExpressionAdmissionErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingAdmittedPredicateReference => "missing_admitted_predicate_reference",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBooleanExpressionAdmissionError {
    kind: ForgeQueryBooleanExpressionAdmissionErrorKind,
    read_graph_digest: String,
    aspect: AspectKey,
    field: FieldKey,
    family: String,
}

impl ForgeQueryBooleanExpressionAdmissionError {
    pub fn kind(&self) -> &ForgeQueryBooleanExpressionAdmissionErrorKind {
        &self.kind
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub(super) fn missing_admitted_predicate_reference(
        read_graph: &ForgeQueryReadGraph,
        filter: &DeclarativePredicateFilter,
        family: &str,
    ) -> Self {
        let field = filter.source_field_key();
        Self {
            kind: ForgeQueryBooleanExpressionAdmissionErrorKind::MissingAdmittedPredicateReference,
            read_graph_digest: read_graph.digest().to_string(),
            aspect: field.native_aspect_key(),
            field: field.native_field_key(),
            family: family.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmittedBooleanExpressionTopology {
    Empty,
    ConjunctiveFlat,
}

impl ForgeQueryAdmittedBooleanExpressionTopology {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::ConjunctiveFlat => "conjunctive_flat",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmittedBooleanExpressionBranchKind {
    EmptyRoot,
    ConjunctiveRoot,
}

impl ForgeQueryAdmittedBooleanExpressionBranchKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRoot => "empty_root",
            Self::ConjunctiveRoot => "conjunctive_root",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedBooleanPredicateLeaf {
    aspect: AspectKey,
    field: FieldKey,
    family: String,
    operator: ForgeQueryPredicateOperandOperator,
    normalized_operand_values: Vec<String>,
    field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    selectivity_class: ForgeQueryPredicateSelectivityClass,
}

impl ForgeQueryAdmittedBooleanPredicateLeaf {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn operator(&self) -> &ForgeQueryPredicateOperandOperator {
        &self.operator
    }

    pub fn normalized_operand_values(&self) -> &[String] {
        &self.normalized_operand_values
    }

    pub fn field_kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.field_kind
    }

    pub fn selectivity_class(&self) -> &ForgeQueryPredicateSelectivityClass {
        &self.selectivity_class
    }

    pub(super) fn admitted(
        aspect: AspectKey,
        field: FieldKey,
        family: impl Into<String>,
        operator: ForgeQueryPredicateOperandOperator,
        normalized_operand_values: Vec<String>,
        field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
        selectivity_class: ForgeQueryPredicateSelectivityClass,
    ) -> Self {
        Self {
            aspect,
            field,
            family: family.into(),
            operator,
            normalized_operand_values,
            field_kind,
            selectivity_class,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "leaf:{}:{}:{}:{}:{}:{}:{}",
            self.aspect.as_str(),
            self.field.as_str(),
            self.family,
            self.operator.as_str(),
            self.normalized_operand_values.join("|"),
            self.field_kind.as_str(),
            self.selectivity_class.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedBooleanExpressionBranch {
    branch_kind: ForgeQueryAdmittedBooleanExpressionBranchKind,
    expression_path: String,
    predicate_leaves: Vec<ForgeQueryAdmittedBooleanPredicateLeaf>,
}

impl ForgeQueryAdmittedBooleanExpressionBranch {
    pub fn branch_kind(&self) -> &ForgeQueryAdmittedBooleanExpressionBranchKind {
        &self.branch_kind
    }

    pub fn expression_path(&self) -> &str {
        &self.expression_path
    }

    pub fn predicate_leaves(&self) -> &[ForgeQueryAdmittedBooleanPredicateLeaf] {
        &self.predicate_leaves
    }

    fn root(predicate_leaves: Vec<ForgeQueryAdmittedBooleanPredicateLeaf>) -> Self {
        let branch_kind = if predicate_leaves.is_empty() {
            ForgeQueryAdmittedBooleanExpressionBranchKind::EmptyRoot
        } else {
            ForgeQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot
        };
        Self {
            branch_kind,
            expression_path: "root".to_string(),
            predicate_leaves,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        let leaves = self
            .predicate_leaves
            .iter()
            .map(|leaf| leaf.digest_part())
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "branch:{}:{}:{}",
            self.branch_kind.as_str(),
            self.expression_path,
            leaves
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedBooleanExpressionCounters {
    expression_nodes_visited: usize,
    predicate_leaves_visited: usize,
    admitted_references_consulted: usize,
}

impl ForgeQueryAdmittedBooleanExpressionCounters {
    pub fn expression_nodes_visited(&self) -> usize {
        self.expression_nodes_visited
    }

    pub fn predicate_leaves_visited(&self) -> usize {
        self.predicate_leaves_visited
    }

    pub fn admitted_references_consulted(&self) -> usize {
        self.admitted_references_consulted
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedBooleanPredicateExpression {
    read_graph_digest: String,
    topology: ForgeQueryAdmittedBooleanExpressionTopology,
    branches: Vec<ForgeQueryAdmittedBooleanExpressionBranch>,
    counters: ForgeQueryAdmittedBooleanExpressionCounters,
}

impl ForgeQueryAdmittedBooleanPredicateExpression {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn topology(&self) -> &ForgeQueryAdmittedBooleanExpressionTopology {
        &self.topology
    }

    pub fn branches(&self) -> &[ForgeQueryAdmittedBooleanExpressionBranch] {
        &self.branches
    }

    pub fn counters(&self) -> &ForgeQueryAdmittedBooleanExpressionCounters {
        &self.counters
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("read_graph:{}", self.read_graph_digest),
            format!("topology:{}", self.topology.as_str()),
            format!(
                "expression_nodes:{}",
                self.counters.expression_nodes_visited()
            ),
            format!(
                "predicate_leaves:{}",
                self.counters.predicate_leaves_visited()
            ),
        ];
        parts.extend(self.branches.iter().map(|branch| branch.digest_part()));
        parts
    }

    pub(super) fn from_flat_conjunction(
        read_graph: &ForgeQueryReadGraph,
        predicate_leaves: Vec<ForgeQueryAdmittedBooleanPredicateLeaf>,
        admitted_references_consulted: usize,
    ) -> Self {
        let topology = if predicate_leaves.is_empty() {
            ForgeQueryAdmittedBooleanExpressionTopology::Empty
        } else {
            ForgeQueryAdmittedBooleanExpressionTopology::ConjunctiveFlat
        };
        let counters = ForgeQueryAdmittedBooleanExpressionCounters {
            expression_nodes_visited: usize::from(!predicate_leaves.is_empty()),
            predicate_leaves_visited: predicate_leaves.len(),
            admitted_references_consulted,
        };
        Self {
            read_graph_digest: read_graph.digest().to_string(),
            topology,
            branches: vec![ForgeQueryAdmittedBooleanExpressionBranch::root(
                predicate_leaves,
            )],
            counters,
        }
    }
}
