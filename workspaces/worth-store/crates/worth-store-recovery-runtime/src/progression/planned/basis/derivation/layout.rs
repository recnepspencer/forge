use super::super::*;
use super::actions::StagingActionBasis;
use super::materialization::ProjectedMaterializationBasis;
use super::pending::PendingProjectionBasis;

pub(super) fn assemble(
    selection: &PhysicalSourceSelection,
    pending: &PendingProjectionBasis<'_>,
    materialization: ProjectedMaterializationBasis,
    actions: StagingActionBasis,
) -> Result<RecoveryStagingLayoutPlan, ExecutionBasisDenial> {
    let commands = super::super::command::exact_commands(
        materialization.frames.values().cloned(),
        materialization
            .manifests
            .values()
            .map(|manifest| (manifest.artifact(), manifest.bytes().into())),
    )?;
    let write_bytes = commands.iter().try_fold(0_u64, |bytes, command| {
        bytes.checked_add(command.byte_count())
    });
    let write_bytes = write_bytes.ok_or(ExecutionBasisDenial::Invalid)?;
    let base = base_image(selection, pending, materialization);
    Ok(RecoveryStagingLayoutPlan {
        source_generation: pending.source_generation,
        staging_generation: pending.staging_generation,
        base,
        actions: actions.actions.into_boxed_slice(),
        commands,
        allocated_targets: actions.allocated_targets.into_boxed_slice(),
        allocated_bytes: pending.allocated_bytes,
        write_bytes,
    })
}

fn base_image(
    selection: &PhysicalSourceSelection,
    pending: &PendingProjectionBasis<'_>,
    materialization: ProjectedMaterializationBasis,
) -> RecoveryBaseImagePlan {
    let ProjectedMaterializationBasis {
        frames: _,
        placements: projected_placements,
        projected_records,
        segment_updates,
        manifests,
        root_states,
    } = materialization;
    let mut placements = selection
        .page_facts()
        .placements()
        .iter()
        .map(|placement| (placement.record(), *placement))
        .collect::<BTreeMap<_, _>>();
    placements.extend(projected_placements);
    let actions = placements
        .into_values()
        .enumerate()
        .map(|(ordinal, placement)| base_action(ordinal, placement, &projected_records))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    RecoveryBaseImagePlan {
        selected_selector: selection.root().selected().selector(),
        selected_root: selection.root().selected().manifest().clone(),
        destination_generation: pending.staging_generation,
        actions,
        segment_updates: segment_updates
            .into_values()
            .enumerate()
            .map(|(ordinal, update)| RecoverySegmentRoutingAction {
                ordinal: ordinal as u64,
                update,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        manifests: manifests
            .into_values()
            .enumerate()
            .map(|(ordinal, manifest)| RecoveryPayloadManifestAction {
                ordinal: ordinal as u64,
                artifact: manifest.artifact(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        root_states: root_states.into_boxed_slice(),
    }
}

fn base_action(
    ordinal: usize,
    placement: CurrentPhysicalRecordPlacement,
    projected_records: &BTreeSet<worth_store_physical_format::PersistedRecordIdentity>,
) -> RecoveryBaseImageAction {
    if projected_records.contains(&placement.record()) {
        RecoveryBaseImageAction::ProjectRecoveryPlacement {
            ordinal: ordinal as u64,
            placement,
        }
    } else {
        RecoveryBaseImageAction::ReuseImmutableSelectedPlacement {
            ordinal: ordinal as u64,
            placement,
        }
    }
}
