use super::*;
use crate::memory_workspace::ForgeQueryEntityIdentity;

pub(super) fn admit_atomic_batch_symbolic_references(
    planned_symbolic_targets: &BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
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
    planned_symbolic_targets: &mut BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
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
        reference.symbol().to_string(),
        (planned_identity, command.declared_collection()),
    );
}

pub(super) fn symbolic_aspect_resolution_evidence_for_command(
    symbolic_targets: &BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
    command: &ForgeQueryWriteCommand,
) -> Result<Vec<ForgeQuerySymbolicAspectResolutionEvidence>, ForgeQueryRuntimeError> {
    symbolic_aspect_resolution_evidence_for_references(
        symbolic_targets,
        command.symbolic_aspect_references(),
    )
}

pub(super) fn symbolic_aspect_resolution_evidence_for_references(
    symbolic_targets: &BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Result<Vec<ForgeQuerySymbolicAspectResolutionEvidence>, ForgeQueryRuntimeError> {
    symbolic_aspect_references
        .iter()
        .map(|reference| {
            let (resolved_entity_identity, _) =
                resolve_same_batch_symbolic_target(symbolic_targets, reference.reference())?;
            Ok(ForgeQuerySymbolicAspectResolutionEvidence::from_reference(
                reference,
                &resolved_entity_identity,
            ))
        })
        .collect()
}
