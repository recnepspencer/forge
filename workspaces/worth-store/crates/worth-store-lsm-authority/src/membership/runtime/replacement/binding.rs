use super::super::state::KeyState;
use crate::membership::{durable_artifact, LsmCompactionMembership};
use crate::{BlobWalRecordIdentity, CheckpointPublicationScope, WalFramePublicationScope};
use std::path::Path;

pub(in crate::membership::runtime) fn selected_state_matches(
    state: &KeyState,
    identities: [BlobWalRecordIdentity; 3],
    base: Option<BlobWalRecordIdentity>,
    version: u64,
) -> bool {
    state.version == version
        && active_identities(state) == Some(identities)
        && state
            .published_replacement
            .as_ref()
            .map(|published| published.output())
            == base
}

pub(in crate::membership::runtime) fn manifest_matches_membership(
    selected: &LsmCompactionMembership,
    output: BlobWalRecordIdentity,
    output_scope: &WalFramePublicationScope,
    scope: &CheckpointPublicationScope,
    path: &Path,
    bytes: u64,
) -> bool {
    let expected = durable_artifact::lsm_membership_activation_digest_prefix(
        selected.key(),
        selected.identities(),
        selected.base().map(|base| base.output()),
        output,
        selected.store_binding(),
        output_scope,
    );
    activation_scope_matches(selected, output, scope)
        && scope
            .manifest_digest()
            .strip_prefix(&expected)
            .is_some_and(|physical| !physical.is_empty())
        && path.is_file()
        && std::fs::metadata(path).is_ok_and(|metadata| metadata.len() == bytes)
}

pub(in crate::membership::runtime) fn replacement_output_matches(
    selected: &LsmCompactionMembership,
    identity: BlobWalRecordIdentity,
    scope: &WalFramePublicationScope,
    path: &Path,
    offset: u64,
    bytes: u64,
) -> bool {
    selected.expected_output_identity() == Some(identity)
        && selected.record_set().iter().all(|record| {
            record.durable_scope().segment_id() == scope.segment_id()
                && record.durable_scope().generation() == scope.generation()
        })
        && selected
            .record_set()
            .iter()
            .map(|record| record.durable_scope().lsn_end())
            .max()
            .is_some_and(|expected| scope.lsn_start() == expected)
        && scope.expected_bytes() == bytes
        && durable_artifact::persisted_artifact_range_matches(
            path,
            offset,
            bytes,
            &durable_artifact::lsm_membership_output_bytes(scope),
        )
}

fn active_identities(state: &KeyState) -> Option<[BlobWalRecordIdentity; 3]> {
    let [value, generation, tombstone] = state.records.each_ref().map(|entry| {
        entry
            .as_ref()
            .filter(|record| !record.retired)
            .map(|record| record.record.identity())
    });
    Some([value?, generation?, tombstone?])
}

fn activation_scope_matches(
    selected: &LsmCompactionMembership,
    output: BlobWalRecordIdentity,
    scope: &CheckpointPublicationScope,
) -> bool {
    let expected_checkpoint = selected
        .base()
        .map(|base| base.activation_scope().checkpoint().checkpoint_epoch())
        .unwrap_or(0)
        .checked_add(1);
    let expected_start = selected
        .base()
        .map_or(selected.identities()[0].sequence(), |base| {
            base.activation_scope().covered_lsn_start()
        });
    expected_checkpoint == Some(scope.checkpoint().checkpoint_epoch())
        && scope.covered_lsn_start() == expected_start
        && output
            .sequence()
            .checked_add(1)
            .is_some_and(|end| scope.covered_lsn_end() == end)
}
