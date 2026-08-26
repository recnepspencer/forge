use worth_foundational::facade::{AspectContractRevision, AspectIdentity};
use worth_query_declaration::facade::application_schema::{
    ApplicationFieldPresence, ApplicationSchemaMember,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::super::super::{foundational_aspect, foundational_value};

pub(super) fn write(
    output: &mut dyn BinaryEncodingSink,
    member: &ApplicationSchemaMember,
) -> Result<(), Denial> {
    match member {
        ApplicationSchemaMember::Entity { entity } => output.text(entity),
        ApplicationSchemaMember::Aspect {
            entity,
            aspect,
            identity,
            revision,
        } => {
            output.text(entity)?;
            output.text(aspect)?;
            output.u64(identity.0)?;
            output.u64(revision.0)
        }
        ApplicationSchemaMember::Field {
            entity,
            aspect,
            field,
            presence,
            scalar_family,
            value_type,
            unit,
            writable,
            equality_queryable,
        } => {
            output.text(entity)?;
            output.text(aspect)?;
            output.text(field)?;
            write_presence(output, *presence)?;
            foundational_aspect::write_scalar_type(output, *scalar_family)?;
            output.text(value_type)?;
            super::super::wire_vocabulary::write_optional(
                output,
                unit.as_ref(),
                |output, unit| output.text(unit),
            )?;
            foundational_value::write_bool(output, *writable)?;
            foundational_value::write_bool(output, *equality_queryable)
        }
        ApplicationSchemaMember::Relation { relation, from, to } => {
            output.text(relation)?;
            output.text(from)?;
            output.text(to)
        }
        ApplicationSchemaMember::PrincipalBinding {
            binding,
            mapping_entity,
            identity_aspect,
            identity_field,
            status_aspect,
            status_field,
            target_relation,
            principal_entity,
            principal_identity_aspect,
            principal_identity_field,
            principal_identity_scalar_family,
            principal_identity_value_type,
        } => {
            for value in [
                binding,
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
            ] {
                output.text(value)?;
            }
            foundational_aspect::write_scalar_type(output, *principal_identity_scalar_family)?;
            output.text(principal_identity_value_type)
        }
        _ => unreachable!("schema member dispatch is exhaustive"),
    }
}

pub(super) fn decode(
    tag: u16,
    input: &mut BinaryInput<'_>,
) -> Result<ApplicationSchemaMember, Denial> {
    Ok(match tag {
        1 => ApplicationSchemaMember::Entity {
            entity: input.text()?.to_owned(),
        },
        2 => ApplicationSchemaMember::Aspect {
            entity: input.text()?.to_owned(),
            aspect: input.text()?.to_owned(),
            identity: AspectIdentity(input.u64()?),
            revision: AspectContractRevision(input.u64()?),
        },
        3 => ApplicationSchemaMember::Field {
            entity: input.text()?.to_owned(),
            aspect: input.text()?.to_owned(),
            field: input.text()?.to_owned(),
            presence: decode_presence(input)?,
            scalar_family: foundational_aspect::decode_scalar_type(input)?,
            value_type: input.text()?.to_owned(),
            unit: super::super::wire_vocabulary::decode_optional(input, |input| {
                Ok(input.text()?.to_owned())
            })?,
            writable: foundational_value::decode_bool(input)?,
            equality_queryable: foundational_value::decode_bool(input)?,
        },
        4 => ApplicationSchemaMember::Relation {
            relation: input.text()?.to_owned(),
            from: input.text()?.to_owned(),
            to: input.text()?.to_owned(),
        },
        5 => ApplicationSchemaMember::PrincipalBinding {
            binding: input.text()?.to_owned(),
            mapping_entity: input.text()?.to_owned(),
            identity_aspect: input.text()?.to_owned(),
            identity_field: input.text()?.to_owned(),
            status_aspect: input.text()?.to_owned(),
            status_field: input.text()?.to_owned(),
            target_relation: input.text()?.to_owned(),
            principal_entity: input.text()?.to_owned(),
            principal_identity_aspect: input.text()?.to_owned(),
            principal_identity_field: input.text()?.to_owned(),
            principal_identity_scalar_family: foundational_aspect::decode_scalar_type(input)?,
            principal_identity_value_type: input.text()?.to_owned(),
        },
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    })
}

pub(super) fn write_presence(
    output: &mut dyn BinaryEncodingSink,
    value: ApplicationFieldPresence,
) -> Result<(), Denial> {
    output.u16(match value {
        ApplicationFieldPresence::Required => 1,
        ApplicationFieldPresence::Optional => 2,
    })
}

pub(super) fn decode_presence(
    input: &mut BinaryInput<'_>,
) -> Result<ApplicationFieldPresence, Denial> {
    match input.u16()? {
        1 => Ok(ApplicationFieldPresence::Required),
        2 => Ok(ApplicationFieldPresence::Optional),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
