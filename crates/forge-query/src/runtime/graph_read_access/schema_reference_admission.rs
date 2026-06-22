use super::schema_reference_evidence::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryAdmittedGraphReadPredicateField,
    ForgeQueryAdmittedGraphReadProjectionField, ForgeQueryAdmittedGraphReadRelation,
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryAdmittedQuerySchemaReferences,
    ForgeQueryGraphReadAdmittedSchemaFieldKind, ForgeQueryGraphReadSchemaReferenceAdmissionError,
};
use crate::authoring::{AspectName, FieldName, OrderingDirection};
use crate::declarative_live::DeclarativePredicateFilter;
use crate::runtime::{ForgeQueryReadBuiltInOperator, ForgeQueryReadGraph};
use forge_foundational::facade::{AspectKey, FieldKey};

pub(crate) fn admit_query_schema_references_for_read_graph(
    read_graph: &ForgeQueryReadGraph,
) -> Result<ForgeQueryAdmittedQuerySchemaReferences, ForgeQueryGraphReadSchemaReferenceAdmissionError>
{
    let request = read_graph.declarative_request();
    let mut relations = request
        .traversal()
        .iter()
        .map(|traversal| {
            ForgeQueryAdmittedGraphReadRelation::new(
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
            Ok(ForgeQueryAdmittedGraphReadProjectionField::new(
                admitted_aspect_key(field.aspect()),
                admitted_field_key(read_graph, field.aspect(), field.field())?,
                field.delivered_name(),
                admitted_schema_field_kind(read_graph, field.aspect(), field.field())?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    projections.sort_by_key(|row| row.digest_part());
    let mut predicates = request
        .predicate_filters()
        .iter()
        .map(|filter| {
            let (aspect, field, family) = predicate_parts(filter);
            Ok(ForgeQueryAdmittedGraphReadPredicateField::new(
                admitted_aspect_key(aspect),
                admitted_field_key(read_graph, aspect, field)?,
                family,
                admitted_schema_field_kind(read_graph, aspect, field)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    predicates.sort_by_key(|row| row.digest_part());
    let mut orderings = request
        .ordering()
        .iter()
        .map(|ordering| {
            Ok(ForgeQueryAdmittedGraphReadOrderingField::new(
                admitted_aspect_key(ordering.aspect()),
                admitted_field_key(read_graph, ordering.aspect(), ordering.field())?,
                ordering_direction_label(ordering.direction()),
                admitted_schema_field_kind(read_graph, ordering.aspect(), ordering.field())?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    orderings.sort_by_key(|row| row.digest_part());
    Ok(ForgeQueryAdmittedQuerySchemaReferences::new(
        read_graph.digest(),
        read_graph.schema_basis().as_str(),
        request.target(),
        relations,
        projections,
        predicates,
        orderings,
    ))
}

fn admitted_aspect_key(aspect: &str) -> AspectKey {
    AspectKey::new(aspect).expect("schema-admitted graph read aspect should be a valid AspectKey")
}

fn admitted_field_key(
    read_graph: &ForgeQueryReadGraph,
    aspect: &str,
    field: &str,
) -> Result<FieldKey, ForgeQueryGraphReadSchemaReferenceAdmissionError> {
    FieldKey::new(field.to_string()).ok_or_else(|| {
        ForgeQueryGraphReadSchemaReferenceAdmissionError::missing_field(read_graph, aspect, field)
    })
}

fn admitted_schema_field_kind(
    read_graph: &ForgeQueryReadGraph,
    aspect: &str,
    field: &str,
) -> Result<
    ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ForgeQueryGraphReadSchemaReferenceAdmissionError,
> {
    let aspect_name = AspectName::new(aspect.to_string()).map_err(|_| {
        ForgeQueryGraphReadSchemaReferenceAdmissionError::missing_field(read_graph, aspect, field)
    })?;
    let field_name = FieldName::new(field.to_string()).map_err(|_| {
        ForgeQueryGraphReadSchemaReferenceAdmissionError::missing_field(read_graph, aspect, field)
    })?;
    read_graph
        .schema_view()
        .field(&aspect_name, &field_name)
        .map(|field| {
            ForgeQueryGraphReadAdmittedSchemaFieldKind::from_schema_field_kind(field.kind())
        })
        .ok_or_else(|| {
            ForgeQueryGraphReadSchemaReferenceAdmissionError::missing_field(
                read_graph, aspect, field,
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
    read_graph: &ForgeQueryReadGraph,
) -> ForgeQueryAdmittedGraphReadRelationDirection {
    if read_graph
        .built_in_operators()
        .contains(&ForgeQueryReadBuiltInOperator::BoundedAncestor)
    {
        ForgeQueryAdmittedGraphReadRelationDirection::Ancestor
    } else if read_graph
        .built_in_operators()
        .contains(&ForgeQueryReadBuiltInOperator::BoundedDescendant)
    {
        ForgeQueryAdmittedGraphReadRelationDirection::Descendant
    } else {
        ForgeQueryAdmittedGraphReadRelationDirection::Forward
    }
}

fn predicate_parts(filter: &DeclarativePredicateFilter) -> (&str, &str, &'static str) {
    match filter {
        DeclarativePredicateFilter::Equality(filter) => {
            (filter.aspect(), filter.field(), "equality")
        }
        DeclarativePredicateFilter::IntegerComparison(filter) => {
            (filter.aspect(), filter.field(), "integer-comparison")
        }
        DeclarativePredicateFilter::StringContains(filter) => {
            (filter.aspect(), filter.field(), "string-contains")
        }
        DeclarativePredicateFilter::SetMembership(filter) => {
            (filter.aspect(), filter.field(), "set-membership")
        }
        DeclarativePredicateFilter::Presence(filter) => {
            (filter.aspect(), filter.field(), "presence")
        }
    }
}
