mod concurrent_readers;

use worth_store::physical_runtime::{PhysicalRecordId, RecordReadSession, ServingPhysicalRuntime};

use super::super::configuration::BoundedResidencyConfiguration;
use super::super::schedule::{
    BoundedResidencySchedulePlan, EquivalentContenderIdentity, ExecutedDuplicateFaultSchedule,
};
use super::media_observation::positioned_reads;

pub(in crate::bounded_residency) struct DuplicateFaultEvidence {
    pub(in crate::bounded_residency) faults: u64,
    pub(in crate::bounded_residency) source_loads: u64,
    pub(in crate::bounded_residency) coalesced_waiters: u64,
    pub(in crate::bounded_residency) pinned_frames: u32,
    pub(in crate::bounded_residency) pin_leases: u32,
    pub(in crate::bounded_residency) positioned_reads: u64,
    pub(in crate::bounded_residency) owner_work: u64,
    pub(in crate::bounded_residency) waiter_work: u64,
    pub(in crate::bounded_residency) same_frame: bool,
    pub(in crate::bounded_residency) same_prefix: bool,
    pub(in crate::bounded_residency) waiter_created_work: bool,
}

pub(in crate::bounded_residency) struct DuplicateFaultProof {
    pub(in crate::bounded_residency) evidence: DuplicateFaultEvidence,
    pub(in crate::bounded_residency) schedule: ExecutedDuplicateFaultSchedule,
}

pub(in crate::bounded_residency) fn prove_duplicate_fault(
    serving: &ServingPhysicalRuntime,
    record: PhysicalRecordId,
    ordinal: usize,
    configuration: BoundedResidencyConfiguration,
    schedule: BoundedResidencySchedulePlan,
) -> Result<DuplicateFaultProof, String> {
    let (first, second) = open_readers(serving, record, ordinal, configuration)?;
    let contender_identity = schedule.contender_identity();
    let before = serving.residency_observation().counters();
    let media_before = positioned_reads(serving);
    let concurrent = concurrent_readers::execute(
        serving,
        first,
        second,
        before,
        contender_identity,
        schedule.gate_release_order(),
    )?;
    let after = serving.residency_observation().counters();
    if after.pinned_frames() != 0 || after.pin_leases() != 0 {
        return Err("duplicate-fault readers leaked pin leases".to_owned());
    }
    let (owner_work, waiter_work) = scheduled_work(
        contender_identity,
        concurrent.first_work,
        concurrent.second_work,
    );
    let faults = delta(concurrent.held.faults(), before.faults(), "faults")?;
    let source_loads = delta(
        concurrent.held.source_loads(),
        before.source_loads(),
        "source loads",
    )?;
    let coalesced_waiters = delta(
        concurrent.held.coalesced_waiters(),
        before.coalesced_waiters(),
        "coalesced waiters",
    )?;
    require_scheduled_contender(ScheduledContenderEvidence {
        contender: contender_identity,
        owner_work,
        waiter_work,
        waiter_created_work: concurrent.waiter_created_work,
        faults,
        source_loads,
        coalesced_waiters,
    })?;
    let evidence = DuplicateFaultEvidence {
        faults,
        source_loads,
        coalesced_waiters,
        pinned_frames: concurrent.held.pinned_frames(),
        pin_leases: concurrent.held.pin_leases(),
        positioned_reads: delta(positioned_reads(serving), media_before, "positioned reads")?,
        owner_work,
        waiter_work,
        same_frame: concurrent.same_frame,
        same_prefix: concurrent.same_prefix,
        waiter_created_work: concurrent.waiter_created_work,
    };
    Ok(DuplicateFaultProof {
        evidence,
        schedule: ExecutedDuplicateFaultSchedule::new(contender_identity, concurrent.release_order),
    })
}

fn scheduled_work(
    contender: EquivalentContenderIdentity,
    first_work: u64,
    second_work: u64,
) -> (u64, u64) {
    match contender {
        EquivalentContenderIdentity::FirstOwner => (first_work, second_work),
        EquivalentContenderIdentity::SecondOwner => (second_work, first_work),
    }
}

struct ScheduledContenderEvidence {
    contender: EquivalentContenderIdentity,
    owner_work: u64,
    waiter_work: u64,
    waiter_created_work: bool,
    faults: u64,
    source_loads: u64,
    coalesced_waiters: u64,
}

fn require_scheduled_contender(evidence: ScheduledContenderEvidence) -> Result<(), String> {
    if evidence.owner_work == 0
        || evidence.waiter_work != 0
        || evidence.waiter_created_work
        || evidence.faults != 1
        || evidence.source_loads != 1
        || evidence.coalesced_waiters != 1
    {
        return Err(format!(
            "duplicate-fault contender roles did not execute as scheduled: \
             contender={:?}, owner_work={}, waiter_work={}, waiter_created_work={}, faults={}, \
             source_loads={}, coalesced_waiters={}",
            evidence.contender,
            evidence.owner_work,
            evidence.waiter_work,
            evidence.waiter_created_work,
            evidence.faults,
            evidence.source_loads,
            evidence.coalesced_waiters,
        ));
    }
    Ok(())
}

fn open_readers(
    serving: &ServingPhysicalRuntime,
    record: PhysicalRecordId,
    ordinal: usize,
    configuration: BoundedResidencyConfiguration,
) -> Result<(RecordReadSession, RecordReadSession), String> {
    let limits = super::read_limits(configuration, ordinal)?;
    let reader = serving.records();
    let owner = reader
        .open(record, limits)
        .map_err(|failure| format!("duplicate-fault owner open failed: {failure:?}"))?;
    let waiter = reader
        .open(record, limits)
        .map_err(|failure| format!("duplicate-fault waiter open failed: {failure:?}"))?;
    Ok((owner, waiter))
}

fn delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("duplicate-fault {label} counter regressed"))
}

#[cfg(test)]
mod tests {
    use super::{
        require_scheduled_contender, scheduled_work, EquivalentContenderIdentity,
        ScheduledContenderEvidence,
    };

    #[test]
    fn second_equivalent_contender_can_own_the_one_causal_fault() {
        let contender = EquivalentContenderIdentity::SecondOwner;
        let (owner, waiter) = scheduled_work(contender, 0, 2);
        if require_scheduled_contender(ScheduledContenderEvidence {
            contender,
            owner_work: owner,
            waiter_work: waiter,
            waiter_created_work: false,
            faults: 1,
            source_loads: 1,
            coalesced_waiters: 1,
        })
        .is_err()
        {
            panic!("MUTANT_PREDICATE:scheduled-contender-identity-ignored");
        }
    }

    #[test]
    fn owner_work_cardinality_can_grow_without_creating_a_second_fault() {
        let contender = EquivalentContenderIdentity::FirstOwner;
        let (owner, waiter) = scheduled_work(contender, 2, 0);
        if require_scheduled_contender(ScheduledContenderEvidence {
            contender,
            owner_work: owner,
            waiter_work: waiter,
            waiter_created_work: false,
            faults: 1,
            source_loads: 1,
            coalesced_waiters: 1,
        })
        .is_err()
        {
            panic!("MUTANT_PREDICATE:owner-work-cardinality-assumed");
        }
    }

    #[test]
    fn waiter_work_cannot_be_relabelled_as_scheduled_ownership() {
        let contender = EquivalentContenderIdentity::SecondOwner;
        let (owner, waiter) = scheduled_work(contender, 1, 2);
        assert!(require_scheduled_contender(ScheduledContenderEvidence {
            contender,
            owner_work: owner,
            waiter_work: waiter,
            waiter_created_work: true,
            faults: 2,
            source_loads: 2,
            coalesced_waiters: 0,
        })
        .is_err());
    }
}
