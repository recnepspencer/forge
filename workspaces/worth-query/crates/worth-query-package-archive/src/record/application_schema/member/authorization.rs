use worth_query_declaration::facade::application_schema::ApplicationSchemaMember;

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::super::super::decode_budget::RecordDecodeAttempt;
use super::super::super::sequence::{decode_sequence, write_sequence};

pub(super) fn write(
    output: &mut dyn BinaryEncodingSink,
    member: &ApplicationSchemaMember,
) -> Result<(), Denial> {
    match member {
        ApplicationSchemaMember::Policy { policy } => output.text(policy),
        ApplicationSchemaMember::Ability {
            ability,
            scope_entity,
        } => {
            output.text(ability)?;
            output.text(scope_entity)
        }
        ApplicationSchemaMember::OperationAbility {
            operation,
            ability,
            scope_entity,
        } => {
            output.text(operation)?;
            output.text(ability)?;
            output.text(scope_entity)
        }
        ApplicationSchemaMember::AbilityPolicy {
            ability,
            scope_entity,
            policy,
            paths,
        } => {
            output.text(ability)?;
            output.text(scope_entity)?;
            output.text(policy)?;
            write_sequence(output, paths, |output, path| {
                super::super::authorization_path::write(output, path)
            })
        }
        _ => unreachable!("authorization member dispatch is exhaustive"),
    }
}

pub(super) fn decode(
    tag: u16,
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<ApplicationSchemaMember, Denial> {
    Ok(match tag {
        19 => ApplicationSchemaMember::Policy {
            policy: input.text()?.to_owned(),
        },
        20 => ApplicationSchemaMember::Ability {
            ability: input.text()?.to_owned(),
            scope_entity: input.text()?.to_owned(),
        },
        21 => ApplicationSchemaMember::OperationAbility {
            operation: input.text()?.to_owned(),
            ability: input.text()?.to_owned(),
            scope_entity: input.text()?.to_owned(),
        },
        22 => ApplicationSchemaMember::AbilityPolicy {
            ability: input.text()?.to_owned(),
            scope_entity: input.text()?.to_owned(),
            policy: input.text()?.to_owned(),
            paths: decode_sequence(input, budget, 8, |input, budget| {
                super::super::authorization_path::decode(input, budget)
            })?,
        },
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    })
}
