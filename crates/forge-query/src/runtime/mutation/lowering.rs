use serde_json::Value;

use super::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectMutationOperationKind,
    ForgeQueryAspectValue, ForgeQuerySymbolicAspectReference,
};
use crate::identity::hash_parts;
use crate::runtime::ForgeQueryWriteCommand;

pub(crate) fn command_declared_aspect_paths(command: &ForgeQueryWriteCommand) -> Vec<String> {
    command_declared_aspect_operations(command)
        .into_iter()
        .map(|operation| operation.aspect_path().to_string())
        .collect()
}

pub(crate) fn command_declared_aspect_operations(
    command: &ForgeQueryWriteCommand,
) -> Vec<ForgeQueryAspectMutationOperation> {
    match command {
        ForgeQueryWriteCommand::InsertAspects {
            aspects,
            symbolic_aspect_references,
            ..
        } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .chain(
                symbolic_aspect_references
                    .iter()
                    .map(symbolic_aspect_reference_operation),
            )
            .collect(),
        ForgeQueryWriteCommand::UpdateAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::AssertExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::VerifyExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateSymbolicAspects { aspects, .. } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .collect(),
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            aspects,
            symbolic_aspect_references,
            ..
        } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .chain(
                symbolic_aspect_references
                    .iter()
                    .map(symbolic_aspect_reference_operation),
            )
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

pub(crate) fn command_declared_aspect_value_digest(
    command: &ForgeQueryWriteCommand,
) -> Option<String> {
    let aspects = match command {
        ForgeQueryWriteCommand::InsertAspects {
            aspects,
            symbolic_aspect_references,
            ..
        } => {
            return Some(hash_parts(
                &std::iter::once("forge_query_declared_aspect_value_digest_v2".to_string())
                    .chain(aspects.iter().map(|aspect| {
                        declared_aspect_value_digest_row(
                            "declared",
                            aspect.aspect_path(),
                            aspect.clears_existing_value(),
                            aspect.value(),
                        )
                    }))
                    .chain(symbolic_aspect_references.iter().map(|reference| {
                        format!(
                            "symbolic:{}:{}:{}:{}",
                            reference.aspect_path(),
                            reference.family(),
                            reference.reference().symbol(),
                            reference.reference().target_collection().unwrap_or("")
                        )
                    }))
                    .collect::<Vec<_>>(),
            ))
        }
        ForgeQueryWriteCommand::UpdateAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::AssertExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::VerifyExistingAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateSymbolicAspects { aspects, .. } => aspects,
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            asserted_aspects,
            aspects,
            symbolic_aspect_references,
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
                    .chain(symbolic_aspect_references.iter().map(|reference| {
                        format!(
                            "update-symbolic:{}:{}:{}:{}",
                            reference.aspect_path(),
                            reference.family(),
                            reference.reference().symbol(),
                            reference.reference().target_collection().unwrap_or("")
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
        ForgeQueryWriteCommand::UpdateAspect { .. }
        | ForgeQueryWriteCommand::DeleteAspects { .. }
        | ForgeQueryWriteCommand::DeleteExistingAspects { .. }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects { .. }
        | ForgeQueryWriteCommand::Delete { .. } => return None,
    };
    Some(hash_parts(
        &std::iter::once("forge_query_declared_aspect_value_digest_v2".to_string())
            .chain(aspects.iter().map(|aspect| {
                declared_aspect_value_digest_row(
                    "declared",
                    aspect.aspect_path(),
                    aspect.clears_existing_value(),
                    aspect.value(),
                )
            }))
            .collect::<Vec<_>>(),
    ))
}

fn symbolic_aspect_reference_operation(
    reference: &ForgeQuerySymbolicAspectReference,
) -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::new(
        reference.aspect_path().to_string(),
        ForgeQueryAspectMutationOperationKind::Set,
    )
}

fn declared_aspect_value_digest_row(
    prefix: &str,
    aspect_path: &str,
    clears_existing_value: bool,
    value: &Value,
) -> String {
    format!(
        "{prefix}:{}:{}:{}",
        aspect_path,
        if clears_existing_value {
            "clear"
        } else {
            "set"
        },
        serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
    )
}
