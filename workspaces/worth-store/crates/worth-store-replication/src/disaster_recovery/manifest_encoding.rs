use super::{
    manifest_vocabulary, DisasterRecoveryBundleDenial, DisasterRecoveryComponent,
    DisasterRecoveryComponentSemantics, DisasterRecoverySecurityBinding,
};
use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity};

pub(super) const MANIFEST_MAGIC: &[u8; 8] = b"WORTHDR2";
pub(super) const MAXIMUM_MANIFEST_BYTES: usize = 16 * 1024 * 1024;

pub(super) fn encode_manifest(
    lineage: &ReplicationLineageIdentity,
    frontier: ReplicaRecoveryFrontier,
    security: DisasterRecoverySecurityBinding,
    expected_rpo_lsn: u64,
    components: &[DisasterRecoveryComponent],
) -> Result<Vec<u8>, DisasterRecoveryBundleDenial> {
    let encoded_length = encoded_length(lineage, components)?;
    if encoded_length > MAXIMUM_MANIFEST_BYTES {
        return Err(DisasterRecoveryBundleDenial::ManifestTooLarge);
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(encoded_length)
        .map_err(|_| DisasterRecoveryBundleDenial::AllocationFailed)?;
    output.extend_from_slice(MANIFEST_MAGIC);
    write_text(&mut output, lineage.as_str())?;
    for value in [
        frontier.observed_lsn(),
        frontier.durable_lsn(),
        frontier.client_acknowledged_lsn(),
        frontier.replication_acknowledged_lsn(),
        frontier.authority_epoch(),
    ] {
        write_u64(&mut output, value);
    }
    output.extend_from_slice(&security.scope_fingerprint());
    output.push(manifest_vocabulary::key_scope_tag(security.key_scope()));
    output.push(manifest_vocabulary::key_version_tag(
        security.key_version_posture(),
    ));
    output.push(manifest_vocabulary::tenant_scope_tag(
        security.tenant_scope(),
    ));
    output.push(manifest_vocabulary::authenticity_tag(
        security.authenticity_requirement(),
    ));
    output.push(manifest_vocabulary::custody_tag(security.custody_posture()));
    write_u64(&mut output, expected_rpo_lsn);
    write_u32(
        &mut output,
        u32::try_from(components.len())
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestTooLarge)?,
    );
    for component in components {
        encode_component(&mut output, component)?;
    }
    debug_assert_eq!(output.len(), encoded_length);
    Ok(output)
}

fn encode_component(
    output: &mut Vec<u8>,
    component: &DisasterRecoveryComponent,
) -> Result<(), DisasterRecoveryBundleDenial> {
    write_text(
        output,
        component
            .relative_path()
            .to_str()
            .ok_or(DisasterRecoveryBundleDenial::InvalidComponent)?,
    )?;
    let evidence = component.evidence();
    output.extend_from_slice(&evidence.expected_digest());
    write_u64(output, evidence.byte_length());
    output.extend_from_slice(&evidence.format_identity());
    output.extend_from_slice(&evidence.backend_assumption_identity());
    encode_semantics(output, component.semantics());
    Ok(())
}

fn encode_semantics(output: &mut Vec<u8>, semantics: DisasterRecoveryComponentSemantics) {
    match semantics {
        DisasterRecoveryComponentSemantics::Authority {
            lineage_identity,
            authority_epoch,
        } => {
            output.push(1);
            output.extend_from_slice(&lineage_identity);
            write_u64(output, authority_epoch);
        }
        DisasterRecoveryComponentSemantics::Checkpoint {
            lineage_identity,
            authority_epoch,
            checkpoint_identity,
            checkpoint_lsn,
            blob_closure_identity,
        } => {
            output.push(2);
            output.extend_from_slice(&lineage_identity);
            write_u64(output, authority_epoch);
            output.extend_from_slice(&checkpoint_identity);
            write_u64(output, checkpoint_lsn);
            output.extend_from_slice(&blob_closure_identity);
        }
        DisasterRecoveryComponentSemantics::Wal {
            lineage_identity,
            authority_epoch,
            start_lsn,
            end_lsn_exclusive,
        } => {
            output.push(3);
            output.extend_from_slice(&lineage_identity);
            write_u64(output, authority_epoch);
            write_u64(output, start_lsn);
            write_u64(output, end_lsn_exclusive);
        }
        DisasterRecoveryComponentSemantics::Page {
            checkpoint_identity,
        } => encode_reference(output, 4, checkpoint_identity),
        DisasterRecoveryComponentSemantics::Blob {
            blob_closure_identity,
        } => encode_reference(output, 5, blob_closure_identity),
        DisasterRecoveryComponentSemantics::Layout {
            checkpoint_identity,
        } => encode_reference(output, 6, checkpoint_identity),
    }
}

fn encode_reference(output: &mut Vec<u8>, tag: u8, identity: [u8; 32]) {
    output.push(tag);
    output.extend_from_slice(&identity);
}

fn encoded_length(
    lineage: &ReplicationLineageIdentity,
    components: &[DisasterRecoveryComponent],
) -> Result<usize, DisasterRecoveryBundleDenial> {
    let mut length = 8_usize
        .checked_add(4)
        .and_then(|value| value.checked_add(lineage.as_str().len()))
        .and_then(|value| value.checked_add(40 + 32 + 5 + 8 + 4))
        .ok_or(DisasterRecoveryBundleDenial::ManifestTooLarge)?;
    for component in components {
        let path_length = component
            .relative_path()
            .to_str()
            .ok_or(DisasterRecoveryBundleDenial::InvalidComponent)?
            .len();
        length = length
            .checked_add(4 + path_length + 32 + 8 + 32 + 32)
            .and_then(|value| value.checked_add(semantic_length(component.semantics())))
            .ok_or(DisasterRecoveryBundleDenial::ManifestTooLarge)?;
    }
    Ok(length)
}

const fn semantic_length(semantics: DisasterRecoveryComponentSemantics) -> usize {
    match semantics {
        DisasterRecoveryComponentSemantics::Authority { .. } => 1 + 32 + 8,
        DisasterRecoveryComponentSemantics::Checkpoint { .. } => 1 + 32 + 8 + 32 + 8 + 32,
        DisasterRecoveryComponentSemantics::Wal { .. } => 1 + 32 + 8 + 8 + 8,
        DisasterRecoveryComponentSemantics::Page { .. }
        | DisasterRecoveryComponentSemantics::Blob { .. }
        | DisasterRecoveryComponentSemantics::Layout { .. } => 1 + 32,
    }
}

fn write_text(output: &mut Vec<u8>, value: &str) -> Result<(), DisasterRecoveryBundleDenial> {
    write_u32(
        output,
        u32::try_from(value.len()).map_err(|_| DisasterRecoveryBundleDenial::ManifestTooLarge)?,
    );
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

fn write_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn write_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_be_bytes());
}
