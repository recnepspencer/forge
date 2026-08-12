use std::collections::BTreeSet;

use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    VerifiedCheckpointStream, PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN,
};

use crate::{VerifiedWalArtifact, PHYSICAL_MUTATION_ATTEMPT_BINDING_DOMAIN};

mod group;
#[cfg(test)]
mod tests;

const TERMINAL_STATE: u8 = 3;
const PROVEN_NO_EFFECT_FATE: u8 = 1;
const COMPLETED_FATE: u8 = 2;
const IDEMPOTENCY_KEY_DOMAIN: &[u8] = b"store.physical.mutation.idempotency-key.v1";
const MEMBER_IDENTITY_DOMAIN: &[u8] = b"store.physical.wal-member-identity.v1";
const MEMBERSHIP_DOMAIN: &[u8] = b"store.physical.durability-group-membership.v1";

#[derive(Clone, Copy)]
struct BindingBasis {
    idempotency: [u8; 32],
    store: [u8; 16],
    runtime: u64,
    operation: u64,
}

#[derive(Clone, Copy)]
struct WalMemberBinding {
    basis: BindingBasis,
    group_identity: [u8; 32],
    member_identity: [u8; 32],
    ordinal: u32,
    count: u32,
    membership: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointCoveredWalCleanupDenial {
    EmptyWal,
    OutsideCheckpointCoverage,
    MissingCheckpointSecurityBinding,
    InvalidCheckpointRecord,
    InvalidWalMember,
    NonTerminalWalMember,
}

/// Proof that every member of one complete verified WAL artifact is covered
/// by terminal evidence in the exact verified checkpoint.
///
/// The recovery media owner consumes this proof immediately while it
/// revalidates the same artifact bytes. Callers cannot make malformed or
/// merely checkpoint-adjacent WAL satisfy this admission.
pub struct CheckpointCoveredWalCleanupAdmission {
    operation_count: u64,
}

pub fn admit_checkpoint_covered_wal_cleanup(
    checkpoint: &VerifiedCheckpointStream,
    wal: &VerifiedWalArtifact,
) -> Result<CheckpointCoveredWalCleanupAdmission, CheckpointCoveredWalCleanupDenial> {
    let source = checkpoint.source();
    let inspection = wal.inspection();
    if wal.frames().is_empty() {
        return Err(CheckpointCoveredWalCleanupDenial::EmptyWal);
    }
    if inspection.lsn_range().end_exclusive().get() > source.wal().covered_end_lsn_exclusive() {
        return Err(CheckpointCoveredWalCleanupDenial::OutsideCheckpointCoverage);
    }
    let security = source
        .security_binding()
        .ok_or(CheckpointCoveredWalCleanupDenial::MissingCheckpointSecurityBinding)?;
    let store = source.identity().store_identity().bytes();
    let policy = security.policy_identity();
    let retention = security.idempotency_retention_generations();
    let terminal = terminal_operation_identities(checkpoint, store, policy, retention)?;
    let mut identities = BTreeSet::new();
    let mut members = Vec::new();
    for frame in wal.frames() {
        let member = wal_member(
            frame.payload(),
            store,
            policy,
            retention,
            frame.lsn_range().start().get(),
            frame.lsn_range().end_exclusive().get(),
        )?;
        if !terminal.contains(&member.basis.idempotency) {
            return Err(CheckpointCoveredWalCleanupDenial::NonTerminalWalMember);
        }
        if !identities.insert(member.basis.idempotency) {
            return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
        }
        members.push(member);
    }
    group::validate(&members)?;
    Ok(CheckpointCoveredWalCleanupAdmission {
        operation_count: identities.len() as u64,
    })
}

impl CheckpointCoveredWalCleanupAdmission {
    pub const fn operation_count(&self) -> u64 {
        self.operation_count
    }
}

fn terminal_operation_identities(
    checkpoint: &VerifiedCheckpointStream,
    store: [u8; 16],
    policy: [u8; 32],
    retention: u64,
) -> Result<BTreeSet<[u8; 32]>, CheckpointCoveredWalCleanupDenial> {
    let mut terminal = BTreeSet::new();
    for record in checkpoint.binding_records() {
        let mut cursor = Cursor::new(record);
        cursor
            .require_field(PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN)
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord)?;
        if cursor
            .byte()
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord)?
            != TERMINAL_STATE
        {
            continue;
        }
        let basis = binding_basis(&mut cursor, store, policy, retention)
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord)?;
        terminal_fate(&mut cursor, basis.idempotency, store, policy, retention)
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord)?;
        terminal.insert(basis.idempotency);
    }
    Ok(terminal)
}

fn terminal_fate(
    cursor: &mut Cursor<'_>,
    identity: [u8; 32],
    store: [u8; 16],
    policy: [u8; 32],
    retention: u64,
) -> Result<(), CheckpointCoveredWalCleanupDenial> {
    match cursor.byte()? {
        PROVEN_NO_EFFECT_FATE if matches!(cursor.byte()?, 1..=4) => cursor.require_end(),
        COMPLETED_FATE => {
            let binding = cursor.field()?;
            if persisted_binding_identity(binding, store, policy, retention)? != identity {
                return Err(CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord);
            }
            cursor.u32()?;
            cursor.u64()?;
            let record_count = cursor.u32()?;
            for _ in 0..record_count {
                cursor.skip_field(16)?;
                cursor.u64()?;
            }
            for _ in 0..13 {
                cursor.u64()?;
            }
            cursor.require_end()
        }
        _ => Err(CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord),
    }
}

fn wal_member(
    payload: &[u8],
    store: [u8; 16],
    policy: [u8; 32],
    retention: u64,
    expected_start: u64,
    expected_end: u64,
) -> Result<WalMemberBinding, CheckpointCoveredWalCleanupDenial> {
    let mut payload = Cursor::new(payload);
    let binding = payload.field()?;
    let redo = payload.field()?;
    payload.require_end()?;
    if binding.is_empty() || redo.is_empty() {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    let mut binding = Cursor::new(binding);
    binding.require_field(PHYSICAL_MUTATION_ATTEMPT_BINDING_DOMAIN)?;
    let basis = binding_basis(&mut binding, store, policy, retention)?;
    let group_identity = binding.array_field::<32>()?;
    let ordinal = binding.u32()?;
    let count = binding.u32()?;
    let membership = binding.array_field::<32>()?;
    let member_identity = binding.array_field::<32>()?;
    let start = binding.u64()?;
    let end = binding.u64()?;
    let redo_digest = binding.array_field::<32>()?;
    binding.require_end()?;
    if ordinal == 0
        || count == 0
        || ordinal > count
        || start != expected_start
        || end != expected_end
        || redo_digest != <[u8; 32]>::from(Sha256::digest(redo))
        || member_identity != member_identity_for(basis)
    {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    Ok(WalMemberBinding {
        basis,
        group_identity,
        member_identity,
        ordinal,
        count,
        membership,
    })
}

fn persisted_binding_identity(
    binding: &[u8],
    store: [u8; 16],
    policy: [u8; 32],
    retention: u64,
) -> Result<[u8; 32], CheckpointCoveredWalCleanupDenial> {
    let mut binding = Cursor::new(binding);
    binding.require_field(PHYSICAL_MUTATION_ATTEMPT_BINDING_DOMAIN)?;
    let identity = binding_basis(&mut binding, store, policy, retention)?.idempotency;
    binding.skip_field(32)?;
    let ordinal = binding.u32()?;
    let count = binding.u32()?;
    binding.skip_field(32)?;
    binding.skip_field(32)?;
    let start = binding.u64()?;
    let end = binding.u64()?;
    binding.skip_field(32)?;
    binding.require_end()?;
    if ordinal == 0 || count == 0 || ordinal > count || start >= end {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidCheckpointRecord);
    }
    Ok(identity)
}

fn binding_basis(
    cursor: &mut Cursor<'_>,
    store: [u8; 16],
    policy: [u8; 32],
    retention: u64,
) -> Result<BindingBasis, CheckpointCoveredWalCleanupDenial> {
    let identity = cursor.array_field::<32>()?;
    if cursor.array_field::<16>()? != store || cursor.array_field::<32>()? != policy {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    let issuance = cursor.u64()?;
    let expiry = cursor.u64()?;
    let material = cursor.array_field::<32>()?;
    cursor.skip_field(32)?;
    if cursor.array_field::<16>()? != store || issuance.checked_add(retention) != Some(expiry) {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    let runtime = cursor.u64()?;
    let _lifecycle = cursor.u64()?;
    let operation = cursor.u64()?;
    if runtime == 0 || operation == 0 {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    let mut digest = Sha256::new();
    digest.update((IDEMPOTENCY_KEY_DOMAIN.len() as u64).to_le_bytes());
    digest.update(IDEMPOTENCY_KEY_DOMAIN);
    digest.update(store);
    digest.update(policy);
    digest.update(issuance.to_le_bytes());
    digest.update(expiry.to_le_bytes());
    digest.update(material);
    if <[u8; 32]>::from(digest.finalize()) != identity {
        return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
    }
    Ok(BindingBasis {
        idempotency: identity,
        store,
        runtime,
        operation,
    })
}

fn member_identity_for(basis: BindingBasis) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(MEMBER_IDENTITY_DOMAIN);
    digest.update(basis.store);
    digest.update(basis.runtime.to_le_bytes());
    digest.update(basis.operation.to_le_bytes());
    digest.finalize().into()
}

struct Cursor<'a> {
    bytes: &'a [u8],
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    fn require_field(&mut self, expected: &[u8]) -> Result<(), CheckpointCoveredWalCleanupDenial> {
        (self.field()? == expected)
            .then_some(())
            .ok_or(CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    }

    fn field(&mut self) -> Result<&'a [u8], CheckpointCoveredWalCleanupDenial> {
        let length = usize::try_from(self.u64()?)
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidWalMember)?;
        self.take(length)
    }

    fn skip_field(&mut self, expected: usize) -> Result<(), CheckpointCoveredWalCleanupDenial> {
        (self.field()?.len() == expected)
            .then_some(())
            .ok_or(CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    }

    fn array_field<const N: usize>(
        &mut self,
    ) -> Result<[u8; N], CheckpointCoveredWalCleanupDenial> {
        self.field()?
            .try_into()
            .map_err(|_| CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    }

    fn byte(&mut self) -> Result<u8, CheckpointCoveredWalCleanupDenial> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, CheckpointCoveredWalCleanupDenial> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().map_err(
            |_| CheckpointCoveredWalCleanupDenial::InvalidWalMember,
        )?))
    }

    fn u64(&mut self) -> Result<u64, CheckpointCoveredWalCleanupDenial> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(
            |_| CheckpointCoveredWalCleanupDenial::InvalidWalMember,
        )?))
    }

    fn require_end(&self) -> Result<(), CheckpointCoveredWalCleanupDenial> {
        self.bytes
            .is_empty()
            .then_some(())
            .ok_or(CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], CheckpointCoveredWalCleanupDenial> {
        let (value, remaining) = self
            .bytes
            .split_at_checked(length)
            .ok_or(CheckpointCoveredWalCleanupDenial::InvalidWalMember)?;
        self.bytes = remaining;
        Ok(value)
    }
}
