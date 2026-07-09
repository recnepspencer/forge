use super::super::{
    WorthQueryGraphReadAdmittedSchemaFieldKind, WorthQueryPredicateOperandOperator,
    WorthQueryPredicateSelectivityClass,
};
use crate::declarative_live::DeclarativePredicateFilter;
use crate::runtime::WorthQueryReadGraph;
use worth_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryBooleanExpressionAdmissionErrorKind {
    MissingAdmittedPredicateReference,
}

impl WorthQueryBooleanExpressionAdmissionErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingAdmittedPredicateReference => "missing_admitted_predicate_reference",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBooleanExpressionAdmissionError {
    kind: WorthQueryBooleanExpressionAdmissionErrorKind,
    read_graph_digest: String,
    aspect: AspectKey,
    field: FieldKey,
    family: String,
}

impl WorthQueryBooleanExpressionAdmissionError {
    pub fn kind(&self) -> &WorthQueryBooleanExpressionAdmissionErrorKind {
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
        read_graph: &WorthQueryReadGraph,
        filter: &DeclarativePredicateFilter,
        family: &str,
    ) -> Self {
        let field = filter.source_field_key();
        Self {
            kind: WorthQueryBooleanExpressionAdmissionErrorKind::MissingAdmittedPredicateReference,
            read_graph_digest: read_graph.digest().to_string(),
            aspect: field.native_aspect_key(),
            field: field.native_field_key(),
            family: family.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAdmittedBooleanExpressionTopology {
    Empty,
    ConjunctiveFlat,
}

impl WorthQueryAdmittedBooleanExpressionTopology {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::ConjunctiveFlat => "conjunctive_flat",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAdmittedBooleanExpressionBranchKind {
    EmptyRoot,
    ConjunctiveRoot,
}

impl WorthQueryAdmittedBooleanExpressionBranchKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRoot => "empty_root",
            Self::ConjunctiveRoot => "conjunctive_root",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedBooleanPredicateLeaf {
    aspect: AspectKey,
    field: FieldKey,
    family: String,
    operator: WorthQueryPredicateOperandOperator,
    normalized_operand_values: Vec<String>,
    field_kind: WorthQueryGraphReadAdmittedSchemaFieldKind,
    selectivity_class: WorthQueryPredicateSelectivityClass,
}

impl WorthQueryAdmittedBooleanPredicateLeaf {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn operator(&self) -> &WorthQueryPredicateOperandOperator {
        &self.operator
    }

    pub fn normalized_operand_values(&self) -> &[String] {
        &self.normalized_operand_values
    }

    pub fn field_kind(&self) -> &WorthQueryGraphReadAdmittedSchemaFieldKind {
        &self.field_kind
    }

    pub fn selectivity_class(&self) -> &WorthQueryPredicateSelectivityClass {
        &self.selectivity_class
    }

    pub(super) fn admitted(
        aspect: AspectKey,
        field: FieldKey,
        family: impl Into<String>,
        operator: WorthQueryPredicateOperandOperator,
        normalized_operand_values: Vec<String>,
        field_kind: WorthQueryGraphReadAdmittedSchemaFieldKind,
        selectivity_class: WorthQueryPredicateSelectivityClass,
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
pub struct WorthQueryAdmittedBooleanExpressionBranch {
    branch_kind: WorthQueryAdmittedBooleanExpressionBranchKind,
    expression_path: String,
    predicate_leaves: Vec<WorthQueryAdmittedBooleanPredicateLeaf>,
}

impl WorthQueryAdmittedBooleanExpressionBranch {
    pub fn branch_kind(&self) -> &WorthQueryAdmittedBooleanExpressionBranchKind {
        &self.branch_kind
    }

    pub fn expression_path(&self) -> &str {
        &self.expression_path
    }

    pub fn predicate_leaves(&self) -> &[WorthQueryAdmittedBooleanPredicateLeaf] {
        &self.predicate_leaves
    }

    fn root(predicate_leaves: Vec<WorthQueryAdmittedBooleanPredicateLeaf>) -> Self {
        let branch_kind = if predicate_leaves.is_empty() {
            WorthQueryAdmittedBooleanExpressionBranchKind::EmptyRoot
        } else {
            WorthQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot
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
pub struct WorthQueryAdmittedBooleanExpressionCounters {
    expression_nodes_visited: usize,
    predicate_leaves_visited: usize,
    admitted_references_consulted: usize,
}

impl WorthQueryAdmittedBooleanExpressionCounters {
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
pub struct WorthQueryAdmittedBooleanPredicateExpression {
    read_graph_digest: String,
    topology: WorthQueryAdmittedBooleanExpressionTopology,
    branches: Vec<WorthQueryAdmittedBooleanExpressionBranch>,
    counters: WorthQueryAdmittedBooleanExpressionCounters,
}

impl WorthQueryAdmittedBooleanPredicateExpression {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn topology(&self) -> &WorthQueryAdmittedBooleanExpressionTopology {
        &self.topology
    }

    pub fn branches(&self) -> &[WorthQueryAdmittedBooleanExpressionBranch] {
        &self.branches
    }

    pub fn counters(&self) -> &WorthQueryAdmittedBooleanExpressionCounters {
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
        read_graph: &WorthQueryReadGraph,
        predicate_leaves: Vec<WorthQueryAdmittedBooleanPredicateLeaf>,
        admitted_references_consulted: usize,
    ) -> Self {
        let topology = if predicate_leaves.is_empty() {
            WorthQueryAdmittedBooleanExpressionTopology::Empty
        } else {
            WorthQueryAdmittedBooleanExpressionTopology::ConjunctiveFlat
        };
        let counters = WorthQueryAdmittedBooleanExpressionCounters {
            expression_nodes_visited: usize::from(!predicate_leaves.is_empty()),
            predicate_leaves_visited: predicate_leaves.len(),
            admitted_references_consulted,
        };
        Self {
            read_graph_digest: read_graph.digest().to_string(),
            topology,
            branches: vec![WorthQueryAdmittedBooleanExpressionBranch::root(
                predicate_leaves,
            )],
            counters,
        }
    }
}
