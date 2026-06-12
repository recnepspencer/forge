use serde_json::Value;

use super::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectMutationOperationKind,
    ForgeQueryAspectValue, ForgeQuerySymbolicAspectReference,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryMutationSymbolIdentity, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryWriteCommand,
};

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
    command_declared_aspect_value_identity(command).map(|identity| identity.as_str().to_string())
}

pub(crate) fn command_declared_aspect_value_identity(
    command: &ForgeQueryWriteCommand,
) -> Option<ForgeQueryEvidenceIdentity> {
    let aspects = match command {
        ForgeQueryWriteCommand::InsertAspects {
            aspects,
            symbolic_aspect_references,
            ..
        } => {
            let rows =
                aspects
                    .iter()
                    .map(|aspect| {
                        declared_aspect_value_digest_row(
                            "declared",
                            aspect.aspect_path(),
                            aspect.clears_existing_value(),
                            aspect.value(),
                        )
                    })
                    .chain(symbolic_aspect_references.iter().map(|reference| {
                        symbolic_aspect_reference_digest_row("symbolic", reference)
                    }))
                    .collect::<Vec<_>>();
            return Some(declared_aspect_value_identity(rows));
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
            let rows = asserted_aspects
                .iter()
                .map(|aspect| {
                    declared_aspect_value_digest_row(
                        "assert",
                        aspect.aspect_path(),
                        aspect.clears_existing_value(),
                        aspect.value(),
                    )
                })
                .chain(aspects.iter().map(|aspect| {
                    declared_aspect_value_digest_row(
                        "update",
                        aspect.aspect_path(),
                        aspect.clears_existing_value(),
                        aspect.value(),
                    )
                }))
                .chain(symbolic_aspect_references.iter().map(|reference| {
                    symbolic_aspect_reference_digest_row("update-symbolic", reference)
                }))
                .collect::<Vec<_>>();
            return Some(declared_aspect_value_identity(rows));
        }
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            asserted_aspects,
            touched_aspect_paths,
            ..
        } => {
            let rows = asserted_aspects
                .iter()
                .map(|aspect| {
                    declared_aspect_value_digest_row(
                        "assert",
                        aspect.aspect_path(),
                        aspect.clears_existing_value(),
                        aspect.value(),
                    )
                })
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| touched_aspect_digest_row("delete", path)),
                )
                .collect::<Vec<_>>();
            return Some(declared_aspect_value_identity(rows));
        }
        ForgeQueryWriteCommand::UpdateAspect { .. }
        | ForgeQueryWriteCommand::DeleteAspects { .. }
        | ForgeQueryWriteCommand::DeleteExistingAspects { .. }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects { .. }
        | ForgeQueryWriteCommand::Delete { .. } => return None,
    };
    Some(declared_aspect_value_identity(
        aspects
            .iter()
            .map(|aspect| {
                declared_aspect_value_digest_row(
                    "declared",
                    aspect.aspect_path(),
                    aspect.clears_existing_value(),
                    aspect.value(),
                )
            })
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
) -> ForgeQueryEvidenceIdentity {
    let value_json = serde_json::to_string(value).unwrap_or_else(|_| value.to_string());
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "declared-aspect-value-row",
        )
        .field_shape(ForgeQueryEvidenceTag::new("prefix"), prefix)
        .field_value(ForgeQueryEvidenceTag::new("aspect_path"), aspect_path)
        .field_shape(
            ForgeQueryEvidenceTag::new("operation"),
            if clears_existing_value {
                "clear"
            } else {
                "set"
            },
        )
        .field_value(ForgeQueryEvidenceTag::new("value"), value_json)
        .seal()
}

fn symbolic_aspect_reference_digest_row(
    prefix: &'static str,
    reference: &ForgeQuerySymbolicAspectReference,
) -> ForgeQueryEvidenceIdentity {
    let symbol_identity = ForgeQueryMutationSymbolIdentity::new(
        "symbolic-aspect-reference",
        reference.reference().symbol(),
    );
    let collection_identity = reference.reference().target_collection().map(|collection| {
        ForgeQueryMutationTargetCollectionIdentity::new("symbolic-aspect-reference", collection)
    });
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("role"),
                "symbolic-aspect-reference-row",
            )
            .field_shape(ForgeQueryEvidenceTag::new("prefix"), prefix)
            .field_value(
                ForgeQueryEvidenceTag::new("aspect_path"),
                reference.aspect_path(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                reference.family().as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("symbol"),
                symbol_identity.evidence_identity(),
            );
    if let Some(collection) = collection_identity.as_ref() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("collection"),
            collection.evidence_identity(),
        );
    }
    identity.seal()
}

fn touched_aspect_digest_row(
    prefix: &'static str,
    aspect_path: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "touched-aspect-row")
        .field_shape(ForgeQueryEvidenceTag::new("prefix"), prefix)
        .field_value(ForgeQueryEvidenceTag::new("aspect_path"), aspect_path)
        .seal()
}

fn declared_aspect_value_identity(
    rows: Vec<ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "declared-aspect-value-digest",
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("row"), rows.iter())
        .seal()
}
