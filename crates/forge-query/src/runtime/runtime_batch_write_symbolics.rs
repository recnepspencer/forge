use super::*;
pub(super) fn admit_atomic_batch_symbolic_references(
    planned_symbolic_targets: &BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryRuntimeError> {
    if let Some(reference) = command.symbolic_target_reference() {
        if !matches!(command.mutation_family(), ForgeQueryMutationFamily::Insert) {
            resolve_same_batch_symbolic_target(planned_symbolic_targets, reference)?;
        }
    }
    for reference in command.symbolic_aspect_references() {
        resolve_same_batch_symbolic_target(planned_symbolic_targets, reference.reference())?;
    }
    Ok(())
}

pub(super) fn record_planned_same_batch_symbolic_target(
    planned_symbolic_targets: &mut BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    command: &ForgeQueryWriteCommand,
) {
    if !matches!(command.mutation_family(), ForgeQueryMutationFamily::Insert) {
        return;
    }
    let Some(reference) = command.symbolic_target_reference() else {
        return;
    };
    let planned_identity = crate::memory_workspace::admit_authored_entity_label(format!(
        "planned-symbolic:{}",
        reference.symbol()
    ));
    planned_symbolic_targets.insert(
        ForgeQuerySameBatchSymbolicTargetKey::from_reference(reference),
        ForgeQuerySameBatchSymbolicTarget::new(
            planned_identity,
            command.declared_collection_identity(),
        ),
    );
}

pub(super) fn symbolic_aspect_resolution_evidence_for_command(
    symbolic_targets: &BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    command: &ForgeQueryWriteCommand,
) -> Result<Vec<ForgeQuerySymbolicAspectResolutionEvidence>, ForgeQueryRuntimeError> {
    symbolic_aspect_resolution_evidence_for_references(
        symbolic_targets,
        command.symbolic_aspect_references(),
    )
}

pub(super) fn symbolic_aspect_resolution_evidence_for_references(
    symbolic_targets: &BTreeMap<
        ForgeQuerySameBatchSymbolicTargetKey,
        ForgeQuerySameBatchSymbolicTarget,
    >,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Result<Vec<ForgeQuerySymbolicAspectResolutionEvidence>, ForgeQueryRuntimeError> {
    symbolic_aspect_references
        .iter()
        .map(|reference| {
            let resolved_target =
                resolve_same_batch_symbolic_target(symbolic_targets, reference.reference())?;
            Ok(ForgeQuerySymbolicAspectResolutionEvidence::from_reference(
                reference,
                resolved_target.entity_identity(),
            ))
        })
        .collect()
}
