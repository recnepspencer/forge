use worth_store_lsm_authority::{
    lookup_published_lsm_membership, persist_lsm_membership_record, replace_lsm_membership,
    select_lsm_compaction_membership, LsmMembershipOwnerCaseObservation,
};
use worth_store_wal::BlobWalRecordKind;

use super::super::begin_durability_fixture;
use super::world;

pub(super) fn observe() -> Vec<LsmMembershipOwnerCaseObservation> {
    let mut observations = Vec::new();
    observe_persistence(&mut observations);
    observe_selection(&mut observations);
    observe_replacement(&mut observations);
    observe_lookup(&mut observations);
    observations
}

fn observe_persistence(observations: &mut Vec<LsmMembershipOwnerCaseObservation>) {
    begin_durability_fixture();
    let (_, admitted_key) = world::admission_and_key(51);
    let (record, admitted_anchor) =
        world::admitted_record(admitted_key, 51, BlobWalRecordKind::LsmValue);
    let mut admitted_session = world::empty_session(&admitted_anchor);
    let admitted = persist_lsm_membership_record(&mut admitted_session, record);
    observations.push(admitted.owner_case_observation());

    begin_durability_fixture();
    let (_, key) = world::admission_and_key(61);
    let (anchor_record, anchor) = world::admitted_record(key, 60, BlobWalRecordKind::LsmValue);
    let mut session = world::empty_session(&anchor);
    persist_lsm_membership_record(&mut session, anchor_record)
        .into_result()
        .unwrap();
    let unsupported = world::admitted_record(key, 61, BlobWalRecordKind::RootCandidate).0;
    let unsupported = persist_lsm_membership_record(&mut session, unsupported);
    observations.push(unsupported.owner_case_observation());

    let foreign = world::foreign_store_record(key, 62, BlobWalRecordKind::LsmTombstone);
    let foreign = persist_lsm_membership_record(&mut session, foreign);
    observations.push(foreign.owner_case_observation());

    let (corrupt, durable) =
        world::admitted_record(key, 62, BlobWalRecordKind::GenerationPublication);
    let ambiguous = world::admitted_record(key, 63, BlobWalRecordKind::LsmValue).0;
    corrupt_byte(durable.persisted_path(), durable.persisted_offset());
    let corrupt = persist_lsm_membership_record(&mut session, corrupt);
    observations.push(corrupt.owner_case_observation());

    let ambiguous = persist_lsm_membership_record(&mut session, ambiguous);
    observations.push(ambiguous.owner_case_observation());
}

fn observe_selection(observations: &mut Vec<LsmMembershipOwnerCaseObservation>) {
    let complete = world::complete_membership();
    let admitted = select_lsm_compaction_membership(&complete.session, complete.key);
    observations.push(admitted.owner_case_observation());

    for missing in [
        BlobWalRecordKind::LsmValue,
        BlobWalRecordKind::GenerationPublication,
        BlobWalRecordKind::LsmTombstone,
    ] {
        let (session, key) = world::membership_missing(missing);
        let denied = select_lsm_compaction_membership(&session, key);
        observations.push(denied.owner_case_observation());
    }

    let mut replaced = world::replacement_world();
    replace_lsm_membership(
        &mut replaced.session,
        &replaced.selected,
        &replaced.replacement,
    )
    .into_result()
    .unwrap();
    corrupt_byte(&replaced.output_path, replaced.output_offset);
    let invalid = select_lsm_compaction_membership(&replaced.session, replaced.key);
    observations.push(invalid.owner_case_observation());
}

fn observe_replacement(observations: &mut Vec<LsmMembershipOwnerCaseObservation>) {
    let mut admitted_world = world::replacement_world();
    let admitted = replace_lsm_membership(
        &mut admitted_world.session,
        &admitted_world.selected,
        &admitted_world.replacement,
    );
    observations.push(admitted.owner_case_observation());

    let stale = replace_lsm_membership(
        &mut admitted_world.session,
        &admitted_world.selected,
        &admitted_world.replacement,
    );
    observations.push(stale.owner_case_observation());

    let mut manifest_world = world::replacement_world();
    std::fs::write(&manifest_world.activation_path, b"short").unwrap();
    let manifest = replace_lsm_membership(
        &mut manifest_world.session,
        &manifest_world.selected,
        &manifest_world.replacement,
    );
    observations.push(manifest.owner_case_observation());

    let mut output_world = world::replacement_world();
    corrupt_byte(&output_world.output_path, output_world.output_offset);
    let output = replace_lsm_membership(
        &mut output_world.session,
        &output_world.selected,
        &output_world.replacement,
    );
    observations.push(output.owner_case_observation());
}

fn corrupt_byte(path: &std::path::Path, offset: u64) {
    use std::io::{Seek, SeekFrom, Write};
    let mut artifact = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    artifact.seek(SeekFrom::Start(offset)).unwrap();
    artifact.write_all(&[0xff]).unwrap();
}

fn observe_lookup(observations: &mut Vec<LsmMembershipOwnerCaseObservation>) {
    let mut admitted_world = world::replacement_world();
    replace_lsm_membership(
        &mut admitted_world.session,
        &admitted_world.selected,
        &admitted_world.replacement,
    )
    .into_result()
    .unwrap();
    let admitted = lookup_published_lsm_membership(&admitted_world.session, admitted_world.key);
    observations.push(admitted.owner_case_observation());

    let complete = world::complete_membership();
    let incomplete = lookup_published_lsm_membership(&complete.session, complete.key);
    observations.push(incomplete.owner_case_observation());
}
