use worth_query_declaration::facade::application_query::{
    ApplicationQueryResultField, ApplicationQueryResultRelation, ApplicationQueryResultShape,
    ApplicationQueryResultTraversalDirection, WorthQueryPortableApplicationQueryResultFieldParts,
    WorthQueryPortableApplicationQueryResultRelationParts,
    WorthQueryPortableApplicationQueryResultShapeParts,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::super::super::super::super::decode_budget::RecordDecodeAttempt;
use super::super::super::super::super::foundational_aspect;
use super::super::super::super::super::sequence::{decode_sequence, write_sequence};
use super::super::super::super::wire_vocabulary::{decode_type_identity, write_type_identity};

pub(super) fn write(
    output: &mut dyn BinaryEncodingSink,
    shape: &ApplicationQueryResultShape,
) -> Result<(), Denial> {
    write_type_identity(output, &shape.query_identity())?;
    output.text(shape.root_entity())?;
    write_type_identity(output, &shape.result_identity())?;
    write_sequence(output, shape.fields(), write_field)?;
    write_sequence(output, shape.relations(), write_relation)
}

pub(super) fn decode(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<ApplicationQueryResultShape, Denial> {
    decode_at_depth(input, budget, 1)
}

fn decode_at_depth(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
    depth: u32,
) -> Result<ApplicationQueryResultShape, Denial> {
    budget.require_nesting_depth(depth)?;
    let query_type = decode_type_identity(input)?;
    let root_entity = input.text()?.to_owned();
    let result_type = decode_type_identity(input)?;
    let fields = decode_sequence(input, budget, 24, |input, _| decode_field(input))?;
    let relations = decode_sequence(input, budget, 24, |input, budget| {
        decode_relation(input, budget, depth)
    })?;
    Ok(ApplicationQueryResultShape::from_untrusted_parts(
        WorthQueryPortableApplicationQueryResultShapeParts {
            query_type,
            root_entity,
            result_type,
            fields,
            relations,
        },
    ))
}

fn write_field(
    output: &mut dyn BinaryEncodingSink,
    field: &ApplicationQueryResultField,
) -> Result<(), Denial> {
    write_type_identity(output, &field.query_identity())?;
    write_type_identity(output, &field.slot_identity())?;
    output.text(field.entity())?;
    output.text(field.aspect())?;
    output.text(field.field())?;
    output.text(field.output_name())?;
    foundational_aspect::write_scalar_type(output, field.scalar_family())?;
    output.text(field.value_type())?;
    super::super::super::schema::write_presence(output, field.presence())
}

fn decode_field(input: &mut BinaryInput<'_>) -> Result<ApplicationQueryResultField, Denial> {
    Ok(ApplicationQueryResultField::from_untrusted_parts(
        WorthQueryPortableApplicationQueryResultFieldParts {
            query_type: decode_type_identity(input)?,
            slot_type: decode_type_identity(input)?,
            entity: input.text()?.to_owned(),
            aspect: input.text()?.to_owned(),
            field: input.text()?.to_owned(),
            output_name: input.text()?.to_owned(),
            scalar_family: foundational_aspect::decode_scalar_type(input)?,
            value_type: decode_type_identity(input)?,
            presence: super::super::super::schema::decode_presence(input)?,
        },
    ))
}

fn write_relation(
    output: &mut dyn BinaryEncodingSink,
    relation: &ApplicationQueryResultRelation,
) -> Result<(), Denial> {
    write_type_identity(output, &relation.query_identity())?;
    write_type_identity(output, &relation.slot_identity())?;
    output.text(relation.relation())?;
    output.text(relation.from())?;
    output.text(relation.to())?;
    write_traversal_direction(output, relation.direction())?;
    output.text(relation.output_name())?;
    super::controls::write_cardinality(output, relation.cardinality())?;
    write(output, relation.nested_shape())
}

fn decode_relation(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
    parent_depth: u32,
) -> Result<ApplicationQueryResultRelation, Denial> {
    let query_type = decode_type_identity(input)?;
    let slot_type = decode_type_identity(input)?;
    let relation = input.text()?.to_owned();
    let from = input.text()?.to_owned();
    let to = input.text()?.to_owned();
    let direction = decode_traversal_direction(input)?;
    let output_name = input.text()?.to_owned();
    let cardinality = super::controls::decode_cardinality(input)?;
    let nested_depth = parent_depth
        .checked_add(1)
        .ok_or_else(|| Denial::new(Kind::NestingDepthBudgetExceeded))?;
    let nested_shape = decode_at_depth(input, budget, nested_depth)?;
    Ok(ApplicationQueryResultRelation::from_untrusted_parts(
        WorthQueryPortableApplicationQueryResultRelationParts {
            query_type,
            slot_type,
            relation,
            from,
            to,
            direction,
            output_name,
            cardinality,
            nested_shape,
        },
    ))
}

pub(super) fn require_nesting_depth(
    shape: &ApplicationQueryResultShape,
    maximum_depth: u32,
) -> Result<(), Denial> {
    require_nesting_depth_at(shape, maximum_depth, 1)
}

fn require_nesting_depth_at(
    shape: &ApplicationQueryResultShape,
    maximum_depth: u32,
    depth: u32,
) -> Result<(), Denial> {
    if depth > maximum_depth {
        return Err(Denial::new(Kind::NestingDepthBudgetExceeded));
    }
    let nested_depth = depth
        .checked_add(1)
        .ok_or_else(|| Denial::new(Kind::NestingDepthBudgetExceeded))?;
    for relation in shape.relations() {
        require_nesting_depth_at(relation.nested_shape(), maximum_depth, nested_depth)?;
    }
    Ok(())
}

pub(super) fn write_traversal_direction(
    output: &mut dyn BinaryEncodingSink,
    value: ApplicationQueryResultTraversalDirection,
) -> Result<(), Denial> {
    output.u16(match value {
        ApplicationQueryResultTraversalDirection::Forward => 1,
        ApplicationQueryResultTraversalDirection::Reverse => 2,
    })
}
pub(super) fn decode_traversal_direction(
    input: &mut BinaryInput<'_>,
) -> Result<ApplicationQueryResultTraversalDirection, Denial> {
    match input.u16()? {
        1 => Ok(ApplicationQueryResultTraversalDirection::Forward),
        2 => Ok(ApplicationQueryResultTraversalDirection::Reverse),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
