use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryDeleteMutationBuilder, ForgeQueryWriteCommand,
};

use crate::{
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode, ForgeServerQueryOperation,
};

use super::request::{
    ForgeServerCompatibilityMutationCommand, ForgeServerCompatibilityMutationRequest,
};

pub(crate) fn lower_query_operation(
    operation_name: &str,
    request: &ForgeServerCompatibilityMutationRequest,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<ForgeServerQueryOperation, ForgeServerQueryHandoffDenial> {
    let mut commands = Vec::with_capacity(request.commands().len());
    for command in request.commands() {
        commands.push(lower_command(command, diagnostics_profile)?);
    }
    Ok(if request.is_batch() {
        ForgeServerQueryOperation::batch_mutation(operation_name, commands)
    } else {
        ForgeServerQueryOperation::single_mutation(
            operation_name,
            commands
                .into_iter()
                .next()
                .expect("single mutation request should produce one command"),
        )
    })
}

fn lower_command(
    command: &ForgeServerCompatibilityMutationCommand,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<ForgeQueryWriteCommand, ForgeServerQueryHandoffDenial> {
    match command {
        ForgeServerCompatibilityMutationCommand::Insert {
            collection,
            aspects,
            metadata,
        } => {
            let mut builder = ForgeQueryAspectMutationBuilder::new();
            for (name, value) in aspects {
                builder = builder.aspect(name, value.clone());
            }
            for (name, value) in metadata {
                builder = builder.metadata(name, value.clone());
            }
            builder.build_insert(collection).map_err(|error| {
                ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                    diagnostics_profile,
                    error.to_string(),
                )
            })
        }
        ForgeServerCompatibilityMutationCommand::Update {
            entity_identity,
            aspects,
            metadata,
        } => {
            let mut builder = ForgeQueryAspectMutationBuilder::new();
            for (name, value) in aspects {
                builder = builder.aspect(name, value.clone());
            }
            for (name, value) in metadata {
                builder = builder.metadata(name, value.clone());
            }
            builder.build_update(entity_identity).map_err(|error| {
                ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                    diagnostics_profile,
                    error.to_string(),
                )
            })
        }
        ForgeServerCompatibilityMutationCommand::Delete {
            entity_identity,
            declared_collection,
            touched_aspect_paths,
            metadata,
        } => {
            let mut builder = ForgeQueryDeleteMutationBuilder::new();
            if let Some(collection) = declared_collection {
                builder = builder.target_collection(collection);
            }
            builder = builder.touches(touched_aspect_paths.iter().cloned());
            for (name, value) in metadata {
                builder = builder.metadata(name, value.clone());
            }
            builder.build_delete(entity_identity).map_err(|error| {
                ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                    diagnostics_profile,
                    error.to_string(),
                )
            })
        }
        ForgeServerCompatibilityMutationCommand::VerifyExisting { .. } => {
            Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityMutationFamilyForbidden,
                diagnostics_profile,
                "compatibility mutation family `verify_existing` is forbidden at the external server boundary",
            ))
        }
    }
}
