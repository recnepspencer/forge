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

pub(crate) fn command_declared_aspect_touches(
    command: &ForgeQueryWriteCommand,
) -> Vec<crate::runtime::ForgeQueryAspectTouch> {
    command_declared_aspect_operations(command)
        .into_iter()
        .map(|operation| operation.aspect_touch().clone())
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
            touched_aspects, ..
        }
        | ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            touched_aspects, ..
        }
        | ForgeQueryWriteCommand::DeleteExistingAspects {
            touched_aspects, ..
        }
        | ForgeQueryWriteCommand::DeleteSymbolicAspects {
            touched_aspects, ..
        } => touched_aspects
            .iter()
            .map(|touch| {
                ForgeQueryAspectMutationOperation::from_touch(
                    touch.clone(),
                    crate::runtime::ForgeQueryAspectMutationOperationKind::Clear,
                )
            })
            .collect(),
        ForgeQueryWriteCommand::UpdateAspect { aspect, .. } => vec![aspect.declared_operation()],
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
                            aspect.aspect_touch().admitted_touch_digest_part(),
                            aspect.clears_existing_value(),
                            aspect.native_digest_material(),
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
                        aspect.aspect_touch().admitted_touch_digest_part(),
                        aspect.clears_existing_value(),
                        aspect.native_digest_material(),
                    )
                })
                .chain(aspects.iter().map(|aspect| {
                    declared_aspect_value_digest_row(
                        "update",
                        aspect.aspect_touch().admitted_touch_digest_part(),
                        aspect.clears_existing_value(),
                        aspect.native_digest_material(),
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
            touched_aspects,
            ..
        } => {
            let rows = asserted_aspects
                .iter()
                .map(|aspect| {
                    declared_aspect_value_digest_row(
                        "assert",
                        aspect.aspect_touch().admitted_touch_digest_part(),
                        aspect.clears_existing_value(),
                        aspect.native_digest_material(),
                    )
                })
                .chain(touched_aspects.iter().map(|touch| {
                    touched_aspect_digest_row("delete", touch.admitted_touch_digest_part())
                }))
                .collect::<Vec<_>>();
            return Some(declared_aspect_value_identity(rows));
        }
        ForgeQueryWriteCommand::UpdateAspect { aspect, .. } => {
            return Some(declared_aspect_value_identity(vec![
                declared_aspect_value_digest_row(
                    "declared",
                    aspect.aspect_touch().admitted_touch_digest_part(),
                    aspect.clears_existing_value(),
                    aspect.native_digest_material(),
                ),
            ]));
        }
        ForgeQueryWriteCommand::DeleteAspects { .. }
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
                    aspect.aspect_touch().admitted_touch_digest_part(),
                    aspect.clears_existing_value(),
                    aspect.native_digest_material(),
                )
            })
            .collect::<Vec<_>>(),
    ))
}

fn symbolic_aspect_reference_operation(
    reference: &ForgeQuerySymbolicAspectReference,
) -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::from_touch(
        reference.aspect_touch().clone(),
        ForgeQueryAspectMutationOperationKind::Set,
    )
}

fn declared_aspect_value_digest_row(
    prefix: &str,
    aspect_touch_digest: String,
    clears_existing_value: bool,
    native_value_digest: String,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "declared-aspect-value-row",
        )
        .field_shape(ForgeQueryEvidenceTag::new("prefix"), prefix)
        .field_value(
            ForgeQueryEvidenceTag::new("aspect_touch"),
            aspect_touch_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operation"),
            if clears_existing_value {
                "clear"
            } else {
                "set"
            },
        )
        .field_value(
            ForgeQueryEvidenceTag::new("native_value"),
            native_value_digest,
        )
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
                ForgeQueryEvidenceTag::new("aspect_touch"),
                reference.aspect_touch().admitted_touch_digest_part(),
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
    aspect_touch_digest: String,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "touched-aspect-row")
        .field_shape(ForgeQueryEvidenceTag::new("prefix"), prefix)
        .field_value(
            ForgeQueryEvidenceTag::new("aspect_touch"),
            aspect_touch_digest,
        )
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
