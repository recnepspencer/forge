use std::collections::BTreeMap;

use super::{
    WorthQueryAdmittedBooleanPredicateExpression, WorthQueryAdmittedBooleanPredicateLeaf,
    WorthQueryBooleanExpressionAdmissionError,
};
use crate::authoring::{NativeComparisonOperator, WorthQueryPredicateOperand};
use crate::declarative_live::{
    DeclarativeNativeComparisonFilter, DeclarativePredicateFilter, DeclarativePresenceFilterKind,
};
use crate::runtime::graph_read_access::{
    WorthQueryAdmittedGraphReadPredicateField, WorthQueryAdmittedQuerySchemaReferences,
    WorthQueryPredicateOperandOperator, WorthQueryPredicateSelectivityClass,
};
use crate::runtime::WorthQueryReadGraph;
use worth_foundational::facade::{AspectKey, FieldKey};

pub(crate) fn admit_boolean_predicate_expression_for_read_graph(
    read_graph: &WorthQueryReadGraph,
    references: &WorthQueryAdmittedQuerySchemaReferences,
) -> Result<WorthQueryAdmittedBooleanPredicateExpression, WorthQueryBooleanExpressionAdmissionError>
{
    let admitted_predicates = admitted_predicate_index(references.predicates());
    let admitted_references_consulted = admitted_predicates.len();
    let mut predicate_leaves = Vec::new();
    for filter in read_graph.declarative_request().predicate_filters() {
        let family = predicate_family(filter);
        let Some(admitted_field) = admitted_predicates.get(&predicate_key(filter, family)) else {
            return Err(
                WorthQueryBooleanExpressionAdmissionError::missing_admitted_predicate_reference(
                    read_graph, filter, family,
                ),
            );
        };
        predicate_leaves.push(predicate_leaf(filter, family, admitted_field));
    }
    predicate_leaves.sort_by_key(|leaf| leaf.digest_part());
    predicate_leaves.dedup_by_key(|leaf| leaf.digest_part());
    Ok(
        WorthQueryAdmittedBooleanPredicateExpression::from_flat_conjunction(
            read_graph,
            predicate_leaves,
            admitted_references_consulted,
        ),
    )
}

fn admitted_predicate_index<'a>(
    admitted_predicates: &'a [WorthQueryAdmittedGraphReadPredicateField],
) -> BTreeMap<AdmittedPredicateKey, &'a WorthQueryAdmittedGraphReadPredicateField> {
    admitted_predicates
        .iter()
        .map(|field| (AdmittedPredicateKey::from_admitted_field(field), field))
        .collect()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AdmittedPredicateKey {
    aspect: AspectKey,
    field: FieldKey,
    family: String,
}

impl AdmittedPredicateKey {
    fn from_admitted_field(field: &WorthQueryAdmittedGraphReadPredicateField) -> Self {
        Self {
            aspect: field.native_aspect_key().clone(),
            field: field.native_field_key().clone(),
            family: field.family().to_string(),
        }
    }

    fn from_filter(filter: &DeclarativePredicateFilter, family: &str) -> Self {
        let field = filter.source_field_key();
        Self {
            aspect: field.native_aspect_key(),
            field: field.native_field_key(),
            family: family.to_string(),
        }
    }
}

fn predicate_key(filter: &DeclarativePredicateFilter, family: &str) -> AdmittedPredicateKey {
    AdmittedPredicateKey::from_filter(filter, family)
}

fn predicate_leaf(
    filter: &DeclarativePredicateFilter,
    family: &'static str,
    admitted_field: &WorthQueryAdmittedGraphReadPredicateField,
) -> WorthQueryAdmittedBooleanPredicateLeaf {
    let (operator, normalized_operand_values) = predicate_operand(filter);
    WorthQueryAdmittedBooleanPredicateLeaf::admitted(
        admitted_field.native_aspect_key().clone(),
        admitted_field.native_field_key().clone(),
        family,
        operator,
        normalized_operand_values,
        admitted_field.kind().clone(),
        predicate_selectivity_class(filter),
    )
}

fn predicate_family(filter: &DeclarativePredicateFilter) -> &'static str {
    match filter {
        DeclarativePredicateFilter::Equality(_) => "equality",
        DeclarativePredicateFilter::NativeComparison(_) => "native-comparison",
        DeclarativePredicateFilter::StringContains(_) => "string-contains",
        DeclarativePredicateFilter::SetMembership(_) => "set-membership",
        DeclarativePredicateFilter::Presence(_) => "presence",
    }
}

fn predicate_selectivity_class(
    filter: &DeclarativePredicateFilter,
) -> WorthQueryPredicateSelectivityClass {
    match filter {
        DeclarativePredicateFilter::Equality(_) => WorthQueryPredicateSelectivityClass::ExactAnchor,
        DeclarativePredicateFilter::NativeComparison(_) => {
            WorthQueryPredicateSelectivityClass::RangePredicate
        }
        DeclarativePredicateFilter::StringContains(_) => {
            WorthQueryPredicateSelectivityClass::BroadPredicate
        }
        DeclarativePredicateFilter::SetMembership(_) => {
            WorthQueryPredicateSelectivityClass::SelectivePredicate
        }
        DeclarativePredicateFilter::Presence(_) => {
            WorthQueryPredicateSelectivityClass::PostTraversalOnly
        }
    }
}

fn predicate_operand(
    filter: &DeclarativePredicateFilter,
) -> (WorthQueryPredicateOperandOperator, Vec<String>) {
    match filter {
        DeclarativePredicateFilter::Equality(filter) => (
            WorthQueryPredicateOperandOperator::Equal,
            vec![scalar_predicate_value_identity(filter.value())],
        ),
        DeclarativePredicateFilter::NativeComparison(filter) => native_comparison_operand(filter),
        DeclarativePredicateFilter::StringContains(filter) => (
            WorthQueryPredicateOperandOperator::Contains,
            vec![filter.value().to_string()],
        ),
        DeclarativePredicateFilter::SetMembership(filter) => {
            let mut values = filter
                .values()
                .iter()
                .map(scalar_predicate_value_identity)
                .collect::<Vec<_>>();
            values.sort();
            values.dedup();
            (WorthQueryPredicateOperandOperator::In, values)
        }
        DeclarativePredicateFilter::Presence(filter) => match filter.kind() {
            DeclarativePresenceFilterKind::IsPresent => (
                WorthQueryPredicateOperandOperator::Presence,
                vec!["is_present".to_string()],
            ),
        },
    }
}

fn native_comparison_operand(
    filter: &DeclarativeNativeComparisonFilter,
) -> (WorthQueryPredicateOperandOperator, Vec<String>) {
    let operator = match filter.operator() {
        NativeComparisonOperator::GreaterThan => WorthQueryPredicateOperandOperator::GreaterThan,
        NativeComparisonOperator::LessThan => WorthQueryPredicateOperandOperator::LessThan,
    };
    (
        operator,
        vec![scalar_predicate_value_identity(filter.value())],
    )
}

fn scalar_predicate_value_identity(value: &WorthQueryPredicateOperand) -> String {
    worth_foundational::facade::prepare_aspect_value_identity_basis(value.as_native())
        .as_str()
        .to_owned()
}
