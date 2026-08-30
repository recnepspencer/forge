use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::super::wire_vocabulary::{decode_type_identity, write_type_identity};

pub(super) fn write(
    output: &mut dyn BinaryEncodingSink,
    member: &ApplicationSchemaMember,
) -> Result<(), Denial> {
    match member {
        ApplicationSchemaMember::Unit { unit } => output.text(unit),
        ApplicationSchemaMember::Effect {
            effect,
            payload_type,
        } => {
            output.text(effect)?;
            write_type_identity(output, payload_type)
        }
        _ => unreachable!("vocabulary member dispatch is exhaustive"),
    }
}

pub(super) fn decode(
    tag: u16,
    input: &mut BinaryInput<'_>,
) -> Result<ApplicationSchemaMember, Denial> {
    match tag {
        23 => Ok(ApplicationSchemaMember::Unit {
            unit: input.text()?.to_owned(),
        }),
        24 => Ok(ApplicationSchemaMember::Effect {
            effect: input.text()?.to_owned(),
            payload_type: decode_type_identity(input)?,
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
