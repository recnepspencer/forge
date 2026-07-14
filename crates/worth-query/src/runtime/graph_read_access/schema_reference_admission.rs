use super::schema_reference_evidence::{
    WorthQueryAdmittedGraphReadOrderingField, WorthQueryAdmittedGraphReadPredicateField,
    WorthQueryAdmittedGraphReadProjectionField, WorthQueryAdmittedGraphReadRelation,
    WorthQueryAdmittedGraphReadRelationDirection, WorthQueryAdmittedQuerySchemaReferences,
    WorthQueryGraphReadAdmittedSchemaFieldKind, WorthQueryGraphReadSchemaReferenceAdmissionError,
};
use crate::authoring::{AspectFieldKey, OrderingDirection};
use crate::declarative_live::DeclarativePredicateFilter;
use crate::runtime::{WorthQueryReadBuiltInOperator, WorthQueryReadGraph};

pub(crate) fn admit_query_schema_references_for_read_graph(
    read_graph: &WorthQueryReadGraph,
) -> Result<WorthQueryAdmittedQuerySchemaReferences, WorthQueryGraphReadSchemaReferenceAdmissionError>
{
    let request = read_graph.declarative_request();
    let mut relations = request
        .traversal()
        .iter()
        .map(|traversal| {
            WorthQueryAdmittedGraphReadRelation::new(
                traversal.relation_name().clone(),
                relation_direction(read_graph),
                usize::from(traversal.depth()),
            )
        })
        .collect::<Vec<_>>();
    relations.sort_by_key(|row| row.digest_part());
    let mut projections = request
        .result_fields()
        .iter()
        .map(|field| {
            let source = field.source_field_key();
            Ok(WorthQueryAdmittedGraphReadProjectionField::new(
                source.native_aspect_key(),
                source.native_field_key(),
                field.delivered_name(),
                admitted_schema_field_kind(read_graph, source)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    projections.sort_by_key(|row| row.digest_part());
    let mut predicates = request
        .predicate_filters()
        .iter()
        .map(|filter| {
            let (source, family) = predicate_parts(filter);
            Ok(WorthQueryAdmittedGraphReadPredicateField::new(
                source.native_aspect_key(),
                source.native_field_key(),
                family,
                admitted_schema_field_kind(read_graph, source)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    predicates.sort_by_key(|row| row.digest_part());
    let mut orderings = request
        .ordering()
        .iter()
        .map(|ordering| {
            let source = ordering.source_field_key();
            Ok(WorthQueryAdmittedGraphReadOrderingField::new(
                source.native_aspect_key(),
                source.native_field_key(),
                ordering_direction_label(ordering.direction()),
                admitted_schema_field_kind(read_graph, source)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    orderings.sort_by_key(|row| row.digest_part());
    Ok(WorthQueryAdmittedQuerySchemaReferences::new(
        read_graph.digest(),
        read_graph.schema_basis().as_str(),
        request.target(),
        relations,
        projections,
        predicates,
        orderings,
    ))
}

fn admitted_schema_field_kind(
    read_graph: &WorthQueryReadGraph,
    source: &AspectFieldKey,
) -> Result<
    WorthQueryGraphReadAdmittedSchemaFieldKind,
    WorthQueryGraphReadSchemaReferenceAdmissionError,
> {
    read_graph
        .schema_view()
        .field(source.aspect(), source.field())
        .map(|field| {
            WorthQueryGraphReadAdmittedSchemaFieldKind::from_schema_field_kind(field.kind())
        })
        .ok_or_else(|| {
            WorthQueryGraphReadSchemaReferenceAdmissionError::missing_field(
                read_graph,
                source.aspect().as_str(),
                source.field().as_str(),
            )
        })
}

fn ordering_direction_label(direction: OrderingDirection) -> &'static str {
    match direction {
        OrderingDirection::Ascending => "ascending",
        OrderingDirection::Descending => "descending",
    }
}

fn relation_direction(
    read_graph: &WorthQueryReadGraph,
) -> WorthQueryAdmittedGraphReadRelationDirection {
    if read_graph
        .built_in_operators()
        .contains(&WorthQueryReadBuiltInOperator::BoundedAncestor)
    {
        WorthQueryAdmittedGraphReadRelationDirection::Ancestor
    } else if read_graph
        .built_in_operators()
        .contains(&WorthQueryReadBuiltInOperator::BoundedDescendant)
    {
        WorthQueryAdmittedGraphReadRelationDirection::Descendant
    } else {
        WorthQueryAdmittedGraphReadRelationDirection::Forward
    }
}

fn predicate_parts(filter: &DeclarativePredicateFilter) -> (&AspectFieldKey, &'static str) {
    match filter {
        DeclarativePredicateFilter::Equality(filter) => (filter.source_field_key(), "equality"),
        DeclarativePredicateFilter::IntegerComparison(filter) => {
            (filter.source_field_key(), "integer-comparison")
        }
        DeclarativePredicateFilter::StringContains(filter) => {
            (filter.source_field_key(), "string-contains")
        }
        DeclarativePredicateFilter::SetMembership(filter) => {
            (filter.source_field_key(), "set-membership")
        }
        DeclarativePredicateFilter::Presence(filter) => (filter.source_field_key(), "presence"),
    }
}
