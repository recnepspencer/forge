use std::collections::BTreeMap;

use super::{
    ForgeQueryAdmittedBooleanPredicateExpression, ForgeQueryAdmittedBooleanPredicateLeaf,
    ForgeQueryBooleanExpressionAdmissionError,
};
use crate::authoring::{IntegerComparisonOperator, ScalarPredicateValue};
use crate::declarative_live::{
    DeclarativeIntegerComparisonFilter, DeclarativePredicateFilter, DeclarativePresenceFilterKind,
};
use crate::runtime::graph_read_access::{
    ForgeQueryAdmittedGraphReadPredicateField, ForgeQueryAdmittedQuerySchemaReferences,
    ForgeQueryPredicateOperandOperator, ForgeQueryPredicateSelectivityClass,
};
use crate::runtime::ForgeQueryReadGraph;

pub(crate) fn admit_boolean_predicate_expression_for_read_graph(
    read_graph: &ForgeQueryReadGraph,
    references: &ForgeQueryAdmittedQuerySchemaReferences,
) -> Result<ForgeQueryAdmittedBooleanPredicateExpression, ForgeQueryBooleanExpressionAdmissionError>
{
    let admitted_predicates = admitted_predicate_index(references.predicates());
    let admitted_references_consulted = admitted_predicates.len();
    let mut predicate_leaves = Vec::new();
    for filter in read_graph.declarative_request().predicate_filters() {
        let family = predicate_family(filter);
        let Some(admitted_field) = admitted_predicates.get(&predicate_key(filter, family)) else {
            return Err(
                ForgeQueryBooleanExpressionAdmissionError::missing_admitted_predicate_reference(
                    read_graph, filter, family,
                ),
            );
        };
        predicate_leaves.push(predicate_leaf(filter, family, admitted_field));
    }
    predicate_leaves.sort_by_key(|leaf| leaf.digest_part());
    predicate_leaves.dedup_by_key(|leaf| leaf.digest_part());
    Ok(
        ForgeQueryAdmittedBooleanPredicateExpression::from_flat_conjunction(
            read_graph,
            predicate_leaves,
            admitted_references_consulted,
        ),
    )
}

fn admitted_predicate_index<'a>(
    admitted_predicates: &'a [ForgeQueryAdmittedGraphReadPredicateField],
) -> BTreeMap<(String, String, String), &'a ForgeQueryAdmittedGraphReadPredicateField> {
    admitted_predicates
        .iter()
        .map(|field| {
            (
                (
                    field.aspect().to_string(),
                    field.field().to_string(),
                    field.family().to_string(),
                ),
                field,
            )
        })
        .collect()
}

fn predicate_key(filter: &DeclarativePredicateFilter, family: &str) -> (String, String, String) {
    (
        filter.aspect().to_string(),
        filter.field().to_string(),
        family.to_string(),
    )
}

fn predicate_leaf(
    filter: &DeclarativePredicateFilter,
    family: &'static str,
    admitted_field: &ForgeQueryAdmittedGraphReadPredicateField,
) -> ForgeQueryAdmittedBooleanPredicateLeaf {
    let (operator, normalized_operand_values) = predicate_operand(filter);
    ForgeQueryAdmittedBooleanPredicateLeaf::admitted(
        filter.aspect(),
        filter.field(),
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
        DeclarativePredicateFilter::IntegerComparison(_) => "integer-comparison",
        DeclarativePredicateFilter::StringContains(_) => "string-contains",
        DeclarativePredicateFilter::SetMembership(_) => "set-membership",
        DeclarativePredicateFilter::Presence(_) => "presence",
    }
}

fn predicate_selectivity_class(
    filter: &DeclarativePredicateFilter,
) -> ForgeQueryPredicateSelectivityClass {
    match filter {
        DeclarativePredicateFilter::Equality(_) => ForgeQueryPredicateSelectivityClass::ExactAnchor,
        DeclarativePredicateFilter::IntegerComparison(_) => {
            ForgeQueryPredicateSelectivityClass::RangePredicate
        }
        DeclarativePredicateFilter::StringContains(_) => {
            ForgeQueryPredicateSelectivityClass::BroadPredicate
        }
        DeclarativePredicateFilter::SetMembership(_) => {
            ForgeQueryPredicateSelectivityClass::SelectivePredicate
        }
        DeclarativePredicateFilter::Presence(_) => {
            ForgeQueryPredicateSelectivityClass::PostTraversalOnly
        }
    }
}

fn predicate_operand(
    filter: &DeclarativePredicateFilter,
) -> (ForgeQueryPredicateOperandOperator, Vec<String>) {
    match filter {
        DeclarativePredicateFilter::Equality(filter) => (
            ForgeQueryPredicateOperandOperator::Equal,
            vec![scalar_predicate_value_identity(filter.value())],
        ),
        DeclarativePredicateFilter::IntegerComparison(filter) => integer_comparison_operand(filter),
        DeclarativePredicateFilter::StringContains(filter) => (
            ForgeQueryPredicateOperandOperator::Contains,
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
            (ForgeQueryPredicateOperandOperator::In, values)
        }
        DeclarativePredicateFilter::Presence(filter) => match filter.kind() {
            DeclarativePresenceFilterKind::IsPresent => (
                ForgeQueryPredicateOperandOperator::Presence,
                vec!["is_present".to_string()],
            ),
        },
    }
}

fn integer_comparison_operand(
    filter: &DeclarativeIntegerComparisonFilter,
) -> (ForgeQueryPredicateOperandOperator, Vec<String>) {
    let operator = match filter.operator() {
        IntegerComparisonOperator::GreaterThan => ForgeQueryPredicateOperandOperator::GreaterThan,
        IntegerComparisonOperator::LessThan => ForgeQueryPredicateOperandOperator::LessThan,
    };
    (operator, vec![filter.value().to_string()])
}

fn scalar_predicate_value_identity(value: &ScalarPredicateValue) -> String {
    match value {
        ScalarPredicateValue::String(value) => format!("string:{value}"),
        ScalarPredicateValue::Integer(value) => format!("integer:{value}"),
        ScalarPredicateValue::Boolean(value) => format!("boolean:{value}"),
    }
}
