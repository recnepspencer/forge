use worth_store_layout_indexes::{
    layout_lsm_maintenance, lsm_compaction_runtime, lsm_physical_compaction_runtime,
    lsm_publication_runtime, lsm_replay_runtime, LsmExecutionOwnerCaseObservation,
    LsmReplayAdmissionRequest,
};
use worth_store_wal::StoreWalRecordIdentity;

use super::super::{PreExecutionBudgetEnvelope, WalRecordFamily};
use super::execution_world::{self, LsmExecutionWorld};

pub(super) fn observe() -> Vec<LsmExecutionOwnerCaseObservation> {
    let mut observations = Vec::new();
    observe_preparation(&mut observations);
    observe_physical_binding(&mut observations);
    observe_activation(&mut observations);
    observe_publication(&mut observations);
    observe_replay(&mut observations);
    observations
}

fn observe_preparation(observations: &mut Vec<LsmExecutionOwnerCaseObservation>) {
    for (request, records) in [(43, [41, 42, 43]), (43, [42, 41, 43]), (43, [41, 43, 42])] {
        let world = execution_world::world(request, records);
        observations.push(
            lsm_compaction_runtime()
                .execute(world.demand())
                .owner_case_observation(),
        );
    }
}

fn observe_physical_binding(observations: &mut Vec<LsmExecutionOwnerCaseObservation>) {
    let admitted_world = execution_world::world(43, [41, 42, 43]);
    observations.push(
        lsm_physical_compaction_runtime()
            .admit(
                prepare(&admitted_world),
                admitted_world.physical_publication.clone(),
            )
            .owner_case_observation(),
    );

    let denied_world = execution_world::world(43, [41, 42, 43]);
    let wrong =
        crate::harness::physical_isolation::compaction::execute_compaction_cutover_for_manifest(
            denied_world.physical_intent.plan(),
            denied_world
                .physical_intent
                .manifest_epoch()
                .saturating_add(1),
        )
        .into_parts()
        .0;
    observations.push(
        lsm_physical_compaction_runtime()
            .admit(prepare(&denied_world), wrong)
            .owner_case_observation(),
    );
}

fn observe_activation(observations: &mut Vec<LsmExecutionOwnerCaseObservation>) {
    let world = execution_world::world(43, [41, 42, 43]);
    observations.push(
        interlocked(&world)
            .prepare_membership_activation()
            .owner_case_observation(),
    );
}

fn observe_publication(observations: &mut Vec<LsmExecutionOwnerCaseObservation>) {
    observations.extend([
        publication_admitted(),
        publication_wrong_key(),
        publication_noncovering_manifest(),
        publication_manifest_mismatch(),
        publication_output_mismatch(),
        publication_stale(),
    ]);
}

fn publication_admitted() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let interlocked = interlocked(&world);
    let activation = activation(&interlocked);
    let manifest = world.manifest(&activation);
    lsm_publication_runtime()
        .publish(
            &mut world.session,
            world.publication.clone(),
            interlocked,
            activation,
            manifest,
        )
        .owner_case_observation()
}

fn publication_wrong_key() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let interlocked = interlocked(&world);
    let activation = activation(&interlocked);
    let manifest = world.manifest(&activation);
    let wrong = world.publication_for(99);
    lsm_publication_runtime()
        .publish(&mut world.session, wrong, interlocked, activation, manifest)
        .owner_case_observation()
}

fn publication_noncovering_manifest() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let interlocked = interlocked(&world);
    let activation = activation(&interlocked);
    let manifest = world.noncovering_manifest(&activation);
    publish(&mut world, interlocked, activation, manifest)
}

fn publication_manifest_mismatch() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let interlocked = interlocked(&world);
    let activation = activation(&interlocked);
    let manifest = world.manifest_with_scope(activation.scope().clone(), b"hostile");
    publish(&mut world, interlocked, activation, manifest)
}

fn publication_output_mismatch() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let interlocked = interlocked(&world);
    let activation = activation(&interlocked);
    let manifest = world.manifest(&activation);
    corrupt_byte(&world.output_path, world.output_offset);
    publish(&mut world, interlocked, activation, manifest)
}

fn corrupt_byte(path: &std::path::Path, offset: u64) {
    use std::io::{Seek, SeekFrom, Write};
    let mut artifact = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    artifact.seek(SeekFrom::Start(offset)).unwrap();
    artifact.write_all(b"hostile").unwrap();
}

fn publication_stale() -> LsmExecutionOwnerCaseObservation {
    let mut world = execution_world::world(43, [41, 42, 43]);
    let first = interlocked(&world);
    let second = interlocked(&world);
    let first_activation = activation(&first);
    let second_activation = activation(&second);
    let first_manifest = world.manifest(&first_activation);
    let second_manifest = world.manifest(&second_activation);
    lsm_publication_runtime()
        .publish(
            &mut world.session,
            world.publication.clone(),
            first,
            first_activation,
            first_manifest,
        )
        .into_result()
        .unwrap();
    publish(&mut world, second, second_activation, second_manifest)
}

fn observe_replay(observations: &mut Vec<LsmExecutionOwnerCaseObservation>) {
    let world = execution_world::world(43, [41, 42, 43]);
    let source = worth_store_layout_indexes::lsm_strategy()
        .admit_replay_source(&world.plan, None, None)
        .unwrap();
    let catalog = super::super::super::layout::admitted_layout_bootstrap_catalog();
    let security =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let admission = layout_lsm_maintenance()
        .admit_replay(LsmReplayAdmissionRequest::new(
            &catalog,
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            &source,
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    observations.push(
        lsm_replay_runtime()
            .execute(admission)
            .owner_case_observation(),
    );
}

fn prepare(world: &LsmExecutionWorld) -> worth_store_layout_indexes::PreparedLsmCompaction {
    lsm_compaction_runtime()
        .execute(world.demand())
        .into_result()
        .unwrap()
}

fn interlocked(world: &LsmExecutionWorld) -> worth_store_layout_indexes::InterlockedLsmCompaction {
    lsm_physical_compaction_runtime()
        .admit(prepare(world), world.physical_publication.clone())
        .into_result()
        .unwrap()
}

fn activation(
    interlocked: &worth_store_layout_indexes::InterlockedLsmCompaction,
) -> worth_store_lsm_authority::LsmMembershipActivationDeclaration {
    interlocked
        .prepare_membership_activation()
        .into_result()
        .unwrap()
}

fn publish(
    world: &mut LsmExecutionWorld,
    interlocked: worth_store_layout_indexes::InterlockedLsmCompaction,
    activation: worth_store_lsm_authority::LsmMembershipActivationDeclaration,
    manifest: worth_store_wal::AdmittedCheckpointPublicationReceipt,
) -> LsmExecutionOwnerCaseObservation {
    lsm_publication_runtime()
        .publish(
            &mut world.session,
            world.publication.clone(),
            interlocked,
            activation,
            manifest,
        )
        .owner_case_observation()
}
