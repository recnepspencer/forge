use std::num::NonZeroU32;

use crate::physical_runtime::{
    PhysicalDurabilityGroupIdentity, PhysicalDurabilityGroupMemberBinding,
    PhysicalWalMemberIdentity,
};

use super::super::fate::PersistedPhysicalMutationFate;
use super::super::persisted_binding::{
    decode_binding_basis, CanonicalBindingCursor, PersistedPhysicalMutationAttemptBinding,
    PhysicalBindingDecodingContext, PhysicalPersistedBindingDecodeDenial,
};
use super::super::registry::PhysicalMutationBindingBasis;
use super::encoding::{
    encode_group_sealed, encode_terminal, encode_unsealed, encode_wal_bound,
    COMPACTION_RECORD_DOMAIN, STATE_GROUP_SEALED, STATE_TERMINAL, STATE_UNSEALED, STATE_WAL_BOUND,
};

pub(in crate::physical_runtime) enum DecodedPhysicalMutationBindingRecord {
    RebuiltUnsealed(PhysicalMutationBindingBasis),
    RebuiltGroupSealed {
        basis: PhysicalMutationBindingBasis,
        group: PhysicalDurabilityGroupMemberBinding,
    },
    Terminal {
        basis: PhysicalMutationBindingBasis,
        fate: PersistedPhysicalMutationFate,
    },
    WalBound {
        basis: PhysicalMutationBindingBasis,
        persisted: PersistedPhysicalMutationAttemptBinding,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalBindingCompactionRecordDecodeDenial {
    Persisted(PhysicalPersistedBindingDecodeDenial),
    UnknownState,
    InvalidGroupBinding,
    MemberIdentityMismatch,
    UnknownTerminalCause,
    NonCanonicalEncoding,
}

impl DecodedPhysicalMutationBindingRecord {
    pub(in crate::physical_runtime) fn decode(
        bytes: &[u8],
        context: PhysicalBindingDecodingContext,
    ) -> Result<Self, PhysicalBindingCompactionRecordDecodeDenial> {
        let mut cursor = CanonicalBindingCursor::new(bytes);
        cursor
            .require_field(COMPACTION_RECORD_DOMAIN)
            .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
        let state = cursor
            .byte()
            .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
        let decoded = match state {
            STATE_UNSEALED => {
                let basis = decode_basis(&mut cursor, context)?;
                cursor
                    .require_end()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                require_canonical(bytes, encode_unsealed(&basis))?;
                Self::RebuiltUnsealed(basis)
            }
            STATE_GROUP_SEALED => {
                let basis = decode_basis(&mut cursor, context)?;
                let group_identity = PhysicalDurabilityGroupIdentity::from_reopened(
                    cursor
                        .array_field()
                        .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?,
                );
                let encoded_member: [u8; 32] = cursor
                    .array_field()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                let member = PhysicalWalMemberIdentity::for_mutation(basis.mutation());
                if encoded_member != member.bytes() {
                    return Err(
                        PhysicalBindingCompactionRecordDecodeDenial::MemberIdentityMismatch,
                    );
                }
                let ordinal = NonZeroU32::new(
                    cursor
                        .u32()
                        .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?,
                )
                .ok_or(PhysicalBindingCompactionRecordDecodeDenial::InvalidGroupBinding)?;
                let member_count = NonZeroU32::new(
                    cursor
                        .u32()
                        .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?,
                )
                .ok_or(PhysicalBindingCompactionRecordDecodeDenial::InvalidGroupBinding)?;
                let membership = cursor
                    .array_field()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                cursor
                    .require_end()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                let group = PhysicalDurabilityGroupMemberBinding::from_reopened(
                    group_identity,
                    member,
                    ordinal,
                    member_count,
                    membership,
                )
                .ok_or(PhysicalBindingCompactionRecordDecodeDenial::InvalidGroupBinding)?;
                require_canonical(bytes, encode_group_sealed(&basis, group))?;
                Self::RebuiltGroupSealed { basis, group }
            }
            STATE_TERMINAL => {
                let basis = decode_basis(&mut cursor, context)?;
                let fate = PersistedPhysicalMutationFate::decode(&mut cursor, &basis, context)
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?
                    .ok_or(PhysicalBindingCompactionRecordDecodeDenial::UnknownTerminalCause)?;
                cursor
                    .require_end()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                require_canonical(bytes, encode_terminal(&basis, &fate))?;
                Self::Terminal { basis, fate }
            }
            STATE_WAL_BOUND => {
                let persisted_bytes = cursor
                    .field()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                cursor
                    .require_end()
                    .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                let persisted = PersistedPhysicalMutationAttemptBinding::decode_from_compaction(
                    persisted_bytes,
                    context,
                )
                .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
                let basis = PhysicalMutationBindingBasis::new(
                    persisted.key().clone(),
                    persisted.fingerprint(),
                    persisted.mutation(),
                );
                require_canonical(bytes, encode_wal_bound(&persisted))?;
                Self::WalBound { basis, persisted }
            }
            _ => return Err(PhysicalBindingCompactionRecordDecodeDenial::UnknownState),
        };
        Ok(decoded)
    }
}

fn decode_basis(
    cursor: &mut CanonicalBindingCursor<'_>,
    context: PhysicalBindingDecodingContext,
) -> Result<PhysicalMutationBindingBasis, PhysicalBindingCompactionRecordDecodeDenial> {
    let decoded = decode_binding_basis(cursor, context)
        .map_err(PhysicalBindingCompactionRecordDecodeDenial::Persisted)?;
    Ok(PhysicalMutationBindingBasis::new(
        decoded.key,
        decoded.fingerprint,
        decoded.mutation,
    ))
}

fn require_canonical(
    actual: &[u8],
    canonical: Vec<u8>,
) -> Result<(), PhysicalBindingCompactionRecordDecodeDenial> {
    if actual == canonical {
        Ok(())
    } else {
        Err(PhysicalBindingCompactionRecordDecodeDenial::NonCanonicalEncoding)
    }
}
