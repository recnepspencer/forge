use worth_proof::NonEmpty;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::admission::PhysicalDurabilityGroupAdmissionDenial;
use super::unique_membership::{
    prove_prepared_group_membership, PreparedGroupMembershipDenial, PreparedGroupMembershipProof,
};
use crate::physical_runtime::{
    PhysicalDurabilityObservation, PhysicalDurabilityPolicyIdentity,
    PhysicalMutationAdmissionDisposition, PreparedPhysicalMutation, RuntimeIdentity,
};

pub(super) struct PhysicalGroupAdmissionContext {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    policy: PhysicalDurabilityPolicyIdentity,
    durability: PhysicalDurabilityObservation,
    current_tick: u64,
    aggregate_byte_limit: u64,
}

pub(super) struct GroupAdmissionProof {
    pub(super) membership: PreparedGroupMembershipProof,
    pub(super) aggregate_bytes: u64,
    pub(super) oldest_queue_age: u64,
}

pub(super) fn validate_members(
    members: &NonEmpty<PreparedPhysicalMutation>,
    context: PhysicalGroupAdmissionContext,
) -> Result<GroupAdmissionProof, PhysicalDurabilityGroupAdmissionDenial> {
    validate_group_width(members, context.durability)?;
    let (aggregate_bytes, oldest_tick) = validate_member_bases(members, &context)?;
    validate_aggregate_bytes(aggregate_bytes, context.aggregate_byte_limit)?;
    let oldest_queue_age = validate_queue_age(members.len(), oldest_tick, context)?;
    let membership = prove_prepared_group_membership(members).map_err(membership_denial)?;
    debug_assert_eq!(membership.mutation_identities().len(), members.len());
    debug_assert_eq!(membership.idempotency_identities().len(), members.len());
    Ok(GroupAdmissionProof {
        membership,
        aggregate_bytes,
        oldest_queue_age,
    })
}

fn validate_group_width(
    members: &NonEmpty<PreparedPhysicalMutation>,
    durability: PhysicalDurabilityObservation,
) -> Result<(), PhysicalDurabilityGroupAdmissionDenial> {
    let requested = u32::try_from(members.len()).unwrap_or(u32::MAX);
    let admitted = durability.group_commit_limit().get().get();
    if requested > admitted {
        return Err(PhysicalDurabilityGroupAdmissionDenial::WidthExceeded {
            admitted,
            requested,
        });
    }
    Ok(())
}

fn validate_member_bases(
    members: &NonEmpty<PreparedPhysicalMutation>,
    context: &PhysicalGroupAdmissionContext,
) -> Result<(u64, u64), PhysicalDurabilityGroupAdmissionDenial> {
    let first_signal = members.first().signal_profile();
    let first_basis = members.first().durability_policy_basis();
    let mut aggregate_bytes = 0_u64;
    let mut oldest_tick = context.current_tick;
    for member in members.as_slice() {
        validate_member_authority(member, context)?;
        if member.signal_profile() != first_signal {
            return Err(PhysicalDurabilityGroupAdmissionDenial::SignalProfileMismatch);
        }
        if member.durability_policy_basis() != first_basis {
            return Err(PhysicalDurabilityGroupAdmissionDenial::DurabilityBasisMismatch);
        }
        aggregate_bytes = aggregate_bytes
            .checked_add(member.resources().prepared_payload_bytes())
            .ok_or(
                PhysicalDurabilityGroupAdmissionDenial::AggregateBytesExceeded {
                    admitted: context.aggregate_byte_limit,
                    requested: u64::MAX,
                },
            )?;
        oldest_tick = oldest_tick.min(member.group_queue_admission_tick().get());
    }
    Ok((aggregate_bytes, oldest_tick))
}

fn validate_member_authority(
    member: &PreparedPhysicalMutation,
    context: &PhysicalGroupAdmissionContext,
) -> Result<(), PhysicalDurabilityGroupAdmissionDenial> {
    if member.disposition() != PhysicalMutationAdmissionDisposition::Fresh {
        return Err(PhysicalDurabilityGroupAdmissionDenial::DuplicateUnresolvedMutation);
    }
    let mutation = member.mutation_identity();
    if mutation.store_identity() != context.store {
        return Err(PhysicalDurabilityGroupAdmissionDenial::ForeignStore);
    }
    if mutation.runtime_identity() != context.runtime {
        return Err(PhysicalDurabilityGroupAdmissionDenial::ForeignRuntime);
    }
    if member.idempotency_lease().policy_identity() != context.policy
        || context.durability.policy_identity() != context.policy
    {
        return Err(PhysicalDurabilityGroupAdmissionDenial::DurabilityPolicyMismatch);
    }
    Ok(())
}

fn validate_aggregate_bytes(
    aggregate_bytes: u64,
    admitted: u64,
) -> Result<(), PhysicalDurabilityGroupAdmissionDenial> {
    if aggregate_bytes > admitted {
        return Err(
            PhysicalDurabilityGroupAdmissionDenial::AggregateBytesExceeded {
                admitted,
                requested: aggregate_bytes,
            },
        );
    }
    Ok(())
}

fn validate_queue_age(
    member_count: usize,
    oldest_tick: u64,
    context: PhysicalGroupAdmissionContext,
) -> Result<u64, PhysicalDurabilityGroupAdmissionDenial> {
    let observed = context
        .current_tick
        .checked_sub(oldest_tick)
        .ok_or(PhysicalDurabilityGroupAdmissionDenial::QueueClockRegressed)?;
    let admitted = context
        .durability
        .group_commit_delay()
        .signal_duration()
        .get();
    if member_count > 1 && observed > admitted {
        return Err(PhysicalDurabilityGroupAdmissionDenial::QueueAgeExceeded {
            admitted,
            observed,
        });
    }
    Ok(observed)
}

fn membership_denial(
    denial: PreparedGroupMembershipDenial,
) -> PhysicalDurabilityGroupAdmissionDenial {
    match denial {
        PreparedGroupMembershipDenial::Mutation => {
            PhysicalDurabilityGroupAdmissionDenial::DuplicateMutationIdentity
        }
        PreparedGroupMembershipDenial::Member => {
            PhysicalDurabilityGroupAdmissionDenial::DuplicateMemberIdentity
        }
        PreparedGroupMembershipDenial::Idempotency => {
            PhysicalDurabilityGroupAdmissionDenial::DuplicateIdempotencyIdentity
        }
    }
}

impl PhysicalGroupAdmissionContext {
    pub(super) const fn new(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        durability: PhysicalDurabilityObservation,
        current_tick: u64,
        aggregate_byte_limit: u64,
    ) -> Self {
        Self {
            store,
            runtime,
            policy,
            durability,
            current_tick,
            aggregate_byte_limit,
        }
    }
}
