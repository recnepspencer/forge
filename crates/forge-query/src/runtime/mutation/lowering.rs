use serde_json::{Map, Value};

use super::{ForgeQueryAspectMutationOperation, ForgeQueryAspectValue};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::ForgeQueryWriteCommand;

pub(crate) fn aspect_values_to_payload(
    aspects: &[ForgeQueryAspectValue],
) -> Result<Value, ForgeQueryWorkspaceError> {
    let mut payload = Value::Object(Map::new());
    for aspect in aspects {
        set_json_path(&mut payload, aspect.aspect_path(), aspect.value().clone())?;
    }
    Ok(payload)
}

#[allow(deprecated)]
pub(crate) fn command_declared_aspect_paths(command: &ForgeQueryWriteCommand) -> Vec<String> {
    command_declared_aspect_operations(command)
        .into_iter()
        .map(|operation| operation.aspect_path().to_string())
        .collect()
}

#[allow(deprecated)]
pub(crate) fn command_declared_aspect_operations(
    command: &ForgeQueryWriteCommand,
) -> Vec<ForgeQueryAspectMutationOperation> {
    match command {
        ForgeQueryWriteCommand::Insert { .. } => Vec::new(),
        ForgeQueryWriteCommand::InsertAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateSymbolicAspects { aspects, .. } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .collect(),
        ForgeQueryWriteCommand::DeleteAspects {
            touched_aspect_paths,
            ..
        }
        | ForgeQueryWriteCommand::DeleteExistingAspects {
            touched_aspect_paths,
            ..
        }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects {
            touched_aspect_paths,
            ..
        } => touched_aspect_paths
            .iter()
            .map(|path| {
                ForgeQueryAspectMutationOperation::new(
                    path.clone(),
                    crate::runtime::ForgeQueryAspectMutationOperationKind::Clear,
                )
            })
            .collect(),
        ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
            vec![ForgeQueryAspectMutationOperation::new(
                aspect_path.clone(),
                crate::runtime::ForgeQueryAspectMutationOperationKind::Set,
            )]
        }
        ForgeQueryWriteCommand::Delete { .. } => Vec::new(),
    }
}

fn set_json_path(
    root: &mut Value,
    path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    if path.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(
            "aspect path may not be empty",
        ));
    }

    let mut current = root;
    let mut segments = path.split('.').peekable();
    while let Some(segment) = segments.next() {
        if segment.is_empty() {
            return Err(ForgeQueryWorkspaceError::new(format!(
                "aspect path `{path}` contains an empty segment"
            )));
        }
        let is_leaf = segments.peek().is_none();
        if is_leaf {
            let object = current.as_object_mut().ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "aspect path `{path}` traverses through a non-object payload segment"
                ))
            })?;
            object.insert(segment.to_string(), value);
            return Ok(());
        }

        let object = current.as_object_mut().ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!(
                "aspect path `{path}` traverses through a non-object payload segment"
            ))
        })?;
        current = object
            .entry(segment.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }

    Ok(())
}
