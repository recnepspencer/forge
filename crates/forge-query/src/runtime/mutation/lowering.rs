use serde_json::{Map, Value};

use super::{ForgeQueryAspectMutationOperation, ForgeQueryAspectValue};
use crate::identity::hash_parts;
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
        | ForgeQueryWriteCommand::AssertExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::VerifyExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateSymbolicAspects { aspects, .. } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .collect(),
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects { aspects, .. } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .collect(),
        ForgeQueryWriteCommand::DeleteAspects {
            touched_aspect_paths,
            ..
        }
        | ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
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

#[allow(deprecated)]
pub(crate) fn command_declared_aspect_value_digest(
    command: &ForgeQueryWriteCommand,
) -> Option<String> {
    let aspects = match command {
        ForgeQueryWriteCommand::InsertAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::AssertExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::VerifyExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateSymbolicAspects { aspects, .. } => aspects,
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            asserted_aspects,
            aspects,
            ..
        } => {
            return Some(hash_parts(
                &std::iter::once("forge_query_declared_aspect_value_digest_v2".to_string())
                    .chain(asserted_aspects.iter().map(|aspect| {
                        format!(
                            "assert:{}:{}:{}",
                            aspect.aspect_path(),
                            if aspect.clears_existing_value() {
                                "clear"
                            } else {
                                "set"
                            },
                            serde_json::to_string(aspect.value())
                                .unwrap_or_else(|_| aspect.value().to_string())
                        )
                    }))
                    .chain(aspects.iter().map(|aspect| {
                        format!(
                            "update:{}:{}:{}",
                            aspect.aspect_path(),
                            if aspect.clears_existing_value() {
                                "clear"
                            } else {
                                "set"
                            },
                            serde_json::to_string(aspect.value())
                                .unwrap_or_else(|_| aspect.value().to_string())
                        )
                    }))
                    .collect::<Vec<_>>(),
            ))
        }
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            asserted_aspects,
            touched_aspect_paths,
            ..
        } => {
            return Some(hash_parts(
                &std::iter::once("forge_query_declared_aspect_value_digest_v2".to_string())
                    .chain(asserted_aspects.iter().map(|aspect| {
                        format!(
                            "assert:{}:{}:{}",
                            aspect.aspect_path(),
                            if aspect.clears_existing_value() {
                                "clear"
                            } else {
                                "set"
                            },
                            serde_json::to_string(aspect.value())
                                .unwrap_or_else(|_| aspect.value().to_string())
                        )
                    }))
                    .chain(
                        touched_aspect_paths
                            .iter()
                            .map(|path| format!("delete:{path}")),
                    )
                    .collect::<Vec<_>>(),
            ))
        }
        ForgeQueryWriteCommand::Insert { .. }
        | ForgeQueryWriteCommand::UpdateAspect { .. }
        | ForgeQueryWriteCommand::DeleteAspects { .. }
        | ForgeQueryWriteCommand::DeleteExistingAspects { .. }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects { .. }
        | ForgeQueryWriteCommand::Delete { .. } => return None,
    };
    Some(hash_parts(
        &std::iter::once("forge_query_declared_aspect_value_digest_v2".to_string())
            .chain(aspects.iter().map(|aspect| {
                format!(
                    "declared:{}:{}:{}",
                    aspect.aspect_path(),
                    if aspect.clears_existing_value() {
                        "clear"
                    } else {
                        "set"
                    },
                    serde_json::to_string(aspect.value())
                        .unwrap_or_else(|_| aspect.value().to_string())
                )
            }))
            .collect::<Vec<_>>(),
    ))
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
