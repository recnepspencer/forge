use super::super::protocol::{
    BoundedResidencyPerformanceClaim, BoundedResidencyPerformanceReceiptObservation,
    BoundedResidencySiegeObservation,
};

pub(super) fn verify(child: &BoundedResidencySiegeObservation) -> Result<(), String> {
    let group = verify_group_commit(child)?;
    let checkpoint_terminal = verify_checkpoint(child)?;
    verify_page_basis(child, group.data_writes, group.data_bytes)?;
    verify_idempotency(child)?;
    verify_terminal_closeout(child, group.mutations, checkpoint_terminal)
}

struct GroupCommitCost {
    mutations: u64,
    data_writes: u64,
    data_bytes: u64,
}

fn verify_group_commit(
    child: &BoundedResidencySiegeObservation,
) -> Result<GroupCommitCost, String> {
    let group = receipt(
        child,
        BoundedResidencyPerformanceClaim::GroupCommitAmplification,
    )?;
    let mutations = counter(group, "store.durability.mutations")?;
    let groups = counter(group, "store.durability.groups")?;
    let acknowledgments = counter(group, "store.durability.acknowledgments")?;
    let wal_frames = counter(group, "store.durability.wal.frames")?;
    let wal_bytes = counter(group, "store.durability.wal.bytes")?;
    let data_writes = counter(group, "store.durability.data.writes")?;
    let data_bytes = counter(group, "store.durability.data.bytes")?;
    let peak_group_members = counter(group, "store.durability.group_queue.peak_members")?;
    let group_member_limit = counter(group, "store.durability.group_queue.member_limit")?;
    if groups > mutations
        || acknowledgments > mutations
        || counter(group, "store.durability.root.publications")? != groups
        || (wal_frames == 0) != (wal_bytes == 0)
        || (data_writes == 0) != (data_bytes == 0)
        || group_member_limit == 0
        || peak_group_members > group_member_limit
        || (mutations != 0 && peak_group_members == 0)
    {
        return Err("Courtroom C group-cost receipt violated its causal bounds".to_owned());
    }
    Ok(GroupCommitCost {
        mutations,
        data_writes,
        data_bytes,
    })
}

fn verify_checkpoint(child: &BoundedResidencySiegeObservation) -> Result<u64, String> {
    let checkpoint = receipt(
        child,
        BoundedResidencyPerformanceClaim::CheckpointBoundedness,
    )?;
    let checkpoint_started = counter(checkpoint, "store.checkpoint.started")?;
    let checkpoint_completed = counter(checkpoint, "store.checkpoint.completed")?;
    let checkpoint_terminal = counter(checkpoint, "store.checkpoint.terminal")?;
    let checkpoint_streams = counter(checkpoint, "store.checkpoint.streams")?;
    verify_checkpoint_counts(
        checkpoint_started,
        checkpoint_completed,
        checkpoint_terminal,
        checkpoint_streams,
    )
}

fn verify_checkpoint_counts(
    checkpoint_started: u64,
    checkpoint_completed: u64,
    checkpoint_terminal: u64,
    checkpoint_streams: u64,
) -> Result<u64, String> {
    if checkpoint_started != 1 {
        return Err(format!(
            "Courtroom C omitted the required seed checkpoint: started={checkpoint_started}"
        ));
    }
    if checkpoint_completed > checkpoint_started
        || checkpoint_terminal != checkpoint_started
        || checkpoint_streams != checkpoint_completed
    {
        return Err("Courtroom C checkpoint-cost receipt violated terminal accounting".to_owned());
    }
    Ok(checkpoint_terminal)
}

fn verify_page_basis(
    child: &BoundedResidencySiegeObservation,
    data_writes: u64,
    data_bytes: u64,
) -> Result<(), String> {
    let page = receipt(
        child,
        BoundedResidencyPerformanceClaim::PageBasisBoundedness,
    )?;
    if counter(page, "store.page_basis.writes")? != data_writes
        || counter(page, "store.page_basis.bytes")? != data_bytes
    {
        return Err("Courtroom C page-basis receipt diverged from physical data cost".to_owned());
    }
    Ok(())
}

fn verify_idempotency(child: &BoundedResidencySiegeObservation) -> Result<(), String> {
    let idempotency = receipt(
        child,
        BoundedResidencyPerformanceClaim::IdempotencyRetention,
    )?;
    let terminal_bindings = [
        "store.idempotency.unresolved",
        "store.idempotency.completed",
        "store.idempotency.proven_no_effect",
        "store.idempotency.indeterminate",
    ]
    .into_iter()
    .try_fold(0_u64, |total, name| {
        total
            .checked_add(counter(idempotency, name)?)
            .ok_or_else(|| "Courtroom C idempotency counter total overflowed".to_owned())
    })?;
    if terminal_bindings != counter(idempotency, "store.idempotency.live_bindings")?
        || counter(idempotency, "store.idempotency.completed_unobserved")?
            > counter(idempotency, "store.idempotency.completed")?
    {
        return Err("Courtroom C idempotency receipt violated retained-fate accounting".to_owned());
    }
    Ok(())
}

fn verify_terminal_closeout(
    child: &BoundedResidencySiegeObservation,
    mutations: u64,
    checkpoint_terminal: u64,
) -> Result<(), String> {
    let closeout = receipt(child, BoundedResidencyPerformanceClaim::TerminalCloseout)?;
    let mutation_terminal = counter(closeout, "store.closeout.mutation_terminal")?;
    let closeout_checkpoint_terminal = counter(closeout, "store.closeout.checkpoint_terminal")?;
    if mutation_terminal != mutations || closeout_checkpoint_terminal != checkpoint_terminal {
        return Err(format!(
            "Courtroom C terminal-cost receipt diverged from completed work: mutation_terminal={mutation_terminal}, mutations={mutations}, checkpoint_terminal={closeout_checkpoint_terminal}, checkpoints_started={checkpoint_terminal}"
        ));
    }
    for name in [
        "store.closeout.work_residual",
        "store.closeout.live_record_handles",
        "store.closeout.residue_classes",
    ] {
        let observed = counter(closeout, name)?;
        if observed != 0 {
            return Err(format!(
                "Courtroom C terminal-cost receipt retained `{name}`={observed}"
            ));
        }
    }
    let live_residency_bytes = counter(closeout, "store.closeout.live_residency_bytes")?;
    if live_residency_bytes > child.close.peak_admitted_bytes {
        return Err(format!(
            "Courtroom C terminal residency cost exceeded its observed admission bound: live={live_residency_bytes}, peak_admitted={}",
            child.close.peak_admitted_bytes
        ));
    }
    Ok(())
}

fn receipt(
    child: &BoundedResidencySiegeObservation,
    claim: BoundedResidencyPerformanceClaim,
) -> Result<&BoundedResidencyPerformanceReceiptObservation, String> {
    child
        .performance
        .iter()
        .find(|receipt| receipt.claim() == claim)
        .ok_or_else(|| format!("Courtroom C omitted `{}` receipt", claim.label()))
}

fn counter(
    receipt: &BoundedResidencyPerformanceReceiptObservation,
    name: &str,
) -> Result<u64, String> {
    receipt
        .counters()
        .iter()
        .find(|counter| counter.name() == name)
        .map(|counter| counter.observed_count())
        .ok_or_else(|| format!("Courtroom C performance receipt omitted `{name}`"))
}

#[cfg(test)]
mod tests {
    use super::verify_checkpoint_counts;

    #[test]
    fn zero_started_checkpoints_are_rejected() {
        if verify_checkpoint_counts(0, 0, 0, 0).is_ok() {
            panic!("MUTANT_PREDICATE:c7-zero-checkpoint-accepted");
        }
        assert_eq!(verify_checkpoint_counts(1, 1, 1, 1), Ok(1));
    }
}
