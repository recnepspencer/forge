use worth_store_replication::{
    DisasterRecoveryComponentSemantics, MaterializedDisasterRecoveryBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterRecoveryClosureDenial {
    DuplicateAuthorityComponent,
    DuplicateCheckpointComponent,
    AuthorityLineageMismatch,
    AuthorityEpochMismatch,
    CheckpointLineageMismatch,
    CheckpointAuthorityEpochMismatch,
    RecoveryPointFrontierMismatch,
    CheckpointAfterRecoveryPoint,
    WalLineageMismatch,
    WalAuthorityEpochMismatch,
    WalCoverageGapOrOverlap,
    CheckpointReferenceMismatch,
    BlobClosureMismatch,
    CounterOverflow,
}

pub(super) fn verify_cross_component_closure(
    bundle: &MaterializedDisasterRecoveryBundle,
) -> Result<u64, DisasterRecoveryClosureDenial> {
    let expected_lineage = bundle.source_lineage().stable_fingerprint();
    let frontier = bundle.frontier();
    if bundle.expected_rpo_lsn() != frontier.replication_acknowledged_lsn() {
        return Err(DisasterRecoveryClosureDenial::RecoveryPointFrontierMismatch);
    }
    let mut checks = 1_u64;
    let mut authority = None;
    let mut checkpoint = None;
    let mut wal_ranges = Vec::new();
    let mut referenced_checkpoints = Vec::new();
    let mut blob_closures = Vec::new();
    for component in bundle.components() {
        match component.semantics() {
            DisasterRecoveryComponentSemantics::Authority {
                lineage_identity,
                authority_epoch,
            } => {
                if authority
                    .replace((lineage_identity, authority_epoch))
                    .is_some()
                {
                    return Err(DisasterRecoveryClosureDenial::DuplicateAuthorityComponent);
                }
            }
            DisasterRecoveryComponentSemantics::Checkpoint {
                lineage_identity,
                authority_epoch,
                checkpoint_identity,
                checkpoint_lsn,
                blob_closure_identity,
            } => {
                if checkpoint
                    .replace((
                        lineage_identity,
                        authority_epoch,
                        checkpoint_identity,
                        checkpoint_lsn,
                        blob_closure_identity,
                    ))
                    .is_some()
                {
                    return Err(DisasterRecoveryClosureDenial::DuplicateCheckpointComponent);
                }
            }
            DisasterRecoveryComponentSemantics::Wal {
                lineage_identity,
                authority_epoch,
                start_lsn,
                end_lsn_exclusive,
            } => wal_ranges.push((
                start_lsn,
                end_lsn_exclusive,
                lineage_identity,
                authority_epoch,
            )),
            DisasterRecoveryComponentSemantics::Page {
                checkpoint_identity,
            }
            | DisasterRecoveryComponentSemantics::Layout {
                checkpoint_identity,
            } => referenced_checkpoints.push(checkpoint_identity),
            DisasterRecoveryComponentSemantics::Blob {
                blob_closure_identity,
            } => blob_closures.push(blob_closure_identity),
        }
    }
    let (authority_lineage, authority_epoch) =
        authority.expect("required Authority family is validated before closure verification");
    if authority_lineage != expected_lineage {
        return Err(DisasterRecoveryClosureDenial::AuthorityLineageMismatch);
    }
    if authority_epoch != frontier.authority_epoch() {
        return Err(DisasterRecoveryClosureDenial::AuthorityEpochMismatch);
    }
    checks += 2;
    let (
        checkpoint_lineage,
        checkpoint_epoch,
        checkpoint_identity,
        checkpoint_lsn,
        blob_closure_identity,
    ) = checkpoint.expect("required Checkpoint family is validated before closure verification");
    if checkpoint_lineage != expected_lineage {
        return Err(DisasterRecoveryClosureDenial::CheckpointLineageMismatch);
    }
    if checkpoint_epoch != frontier.authority_epoch() {
        return Err(DisasterRecoveryClosureDenial::CheckpointAuthorityEpochMismatch);
    }
    if checkpoint_lsn > bundle.expected_rpo_lsn() {
        return Err(DisasterRecoveryClosureDenial::CheckpointAfterRecoveryPoint);
    }
    checks += 3;
    verify_wal_closure(
        &mut wal_ranges,
        checkpoint_lsn,
        bundle.expected_rpo_lsn(),
        expected_lineage,
        frontier.authority_epoch(),
        &mut checks,
    )?;
    for reference in referenced_checkpoints {
        if reference != checkpoint_identity {
            return Err(DisasterRecoveryClosureDenial::CheckpointReferenceMismatch);
        }
        checks = checks
            .checked_add(1)
            .ok_or(DisasterRecoveryClosureDenial::CounterOverflow)?;
    }
    for closure in blob_closures {
        if closure != blob_closure_identity {
            return Err(DisasterRecoveryClosureDenial::BlobClosureMismatch);
        }
        checks = checks
            .checked_add(1)
            .ok_or(DisasterRecoveryClosureDenial::CounterOverflow)?;
    }
    Ok(checks)
}

fn verify_wal_closure(
    ranges: &mut [(u64, u64, [u8; 32], u64)],
    checkpoint_lsn: u64,
    recovery_lsn: u64,
    expected_lineage: [u8; 32],
    expected_epoch: u64,
    checks: &mut u64,
) -> Result<(), DisasterRecoveryClosureDenial> {
    ranges.sort_by_key(|range| range.0);
    let mut expected_start = checkpoint_lsn
        .checked_add(1)
        .ok_or(DisasterRecoveryClosureDenial::WalCoverageGapOrOverlap)?;
    let expected_end = recovery_lsn
        .checked_add(1)
        .ok_or(DisasterRecoveryClosureDenial::WalCoverageGapOrOverlap)?;
    for &(start, end, lineage, epoch) in ranges.iter() {
        if lineage != expected_lineage {
            return Err(DisasterRecoveryClosureDenial::WalLineageMismatch);
        }
        if epoch != expected_epoch {
            return Err(DisasterRecoveryClosureDenial::WalAuthorityEpochMismatch);
        }
        if start != expected_start || end > expected_end {
            return Err(DisasterRecoveryClosureDenial::WalCoverageGapOrOverlap);
        }
        expected_start = end;
        *checks = checks
            .checked_add(3)
            .ok_or(DisasterRecoveryClosureDenial::CounterOverflow)?;
    }
    if expected_start != expected_end {
        return Err(DisasterRecoveryClosureDenial::WalCoverageGapOrOverlap);
    }
    Ok(())
}
