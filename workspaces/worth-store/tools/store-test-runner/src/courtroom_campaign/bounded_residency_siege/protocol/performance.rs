use std::collections::BTreeSet;

const MARKER: &str = "BOUNDED_RESIDENCY_PERFORMANCE ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyPerformanceClaim {
    GroupCommitAmplification,
    CheckpointBoundedness,
    PageBasisBoundedness,
    IdempotencyRetention,
    TerminalCloseout,
}

impl BoundedResidencyPerformanceClaim {
    const ALL: [Self; 5] = [
        Self::GroupCommitAmplification,
        Self::CheckpointBoundedness,
        Self::PageBasisBoundedness,
        Self::IdempotencyRetention,
        Self::TerminalCloseout,
    ];

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn label(
        self,
    ) -> &'static str {
        match self {
            Self::GroupCommitAmplification => "group-commit-amplification",
            Self::CheckpointBoundedness => "checkpoint-boundedness",
            Self::PageBasisBoundedness => "page-basis-boundedness",
            Self::IdempotencyRetention => "idempotency-retention",
            Self::TerminalCloseout => "terminal-closeout",
        }
    }

    fn parse(encoded: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|claim| claim.label() == encoded)
    }

    fn counter_names(self) -> &'static [&'static str] {
        match self {
            Self::GroupCommitAmplification => &[
                "store.durability.mutations",
                "store.durability.groups",
                "store.durability.acknowledgments",
                "store.durability.wal.frames",
                "store.durability.wal.bytes",
                "store.durability.data.writes",
                "store.durability.data.bytes",
                "store.durability.root.publications",
                "store.durability.group_queue.peak_members",
                "store.durability.group_queue.member_limit",
            ],
            Self::CheckpointBoundedness => &[
                "store.checkpoint.started",
                "store.checkpoint.completed",
                "store.checkpoint.terminal",
                "store.checkpoint.streams",
                "store.checkpoint.bytes",
                "store.checkpoint.dirty_records",
                "store.checkpoint.retained_wal_segments",
            ],
            Self::PageBasisBoundedness => &[
                "store.page_basis.writes",
                "store.page_basis.bytes",
                "store.page_basis.records",
            ],
            Self::IdempotencyRetention => &[
                "store.idempotency.live_bindings",
                "store.idempotency.unresolved",
                "store.idempotency.completed",
                "store.idempotency.proven_no_effect",
                "store.idempotency.indeterminate",
                "store.idempotency.completed_unobserved",
            ],
            Self::TerminalCloseout => &[
                "store.closeout.mutation_terminal",
                "store.closeout.checkpoint_terminal",
                "store.closeout.work_residual",
                "store.closeout.live_record_handles",
                "store.closeout.live_residency_bytes",
                "store.closeout.residue_classes",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyBackendProfile {
    SimulatedStrictDurable,
    PosixFileFsyncDirectorySync,
    WindowsFlushFileBuffers,
    MmapFlushNotDurabilityCertified,
    ControlledLostFlush,
    ControlledReorderedFlush,
}

impl BoundedResidencyBackendProfile {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn label(
        self,
    ) -> &'static str {
        match self {
            Self::SimulatedStrictDurable => "simulated-strict-durable",
            Self::PosixFileFsyncDirectorySync => "posix-file-fsync-directory-sync",
            Self::WindowsFlushFileBuffers => "windows-flush-file-buffers",
            Self::MmapFlushNotDurabilityCertified => "mmap-flush-not-durability-certified",
            Self::ControlledLostFlush => "controlled-lost-flush",
            Self::ControlledReorderedFlush => "controlled-reordered-flush",
        }
    }

    fn parse(encoded: &str) -> Option<Self> {
        [
            Self::SimulatedStrictDurable,
            Self::PosixFileFsyncDirectorySync,
            Self::WindowsFlushFileBuffers,
            Self::MmapFlushNotDurabilityCertified,
            Self::ControlledLostFlush,
            Self::ControlledReorderedFlush,
        ]
        .into_iter()
        .find(|profile| profile.label() == encoded)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyPerformanceCounterObservation
{
    name: Box<str>,
    observed_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyPerformanceReceiptObservation
{
    claim: BoundedResidencyPerformanceClaim,
    profile: BoundedResidencyBackendProfile,
    counters: Box<[BoundedResidencyPerformanceCounterObservation]>,
}

impl BoundedResidencyPerformanceCounterObservation {
    pub(in crate::courtroom_campaign::bounded_residency_siege) fn name(&self) -> &str {
        &self.name
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn observed_count(
        &self,
    ) -> u64 {
        self.observed_count
    }
}

impl BoundedResidencyPerformanceReceiptObservation {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn claim(
        &self,
    ) -> BoundedResidencyPerformanceClaim {
        self.claim
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn profile(
        &self,
    ) -> BoundedResidencyBackendProfile {
        self.profile
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn counters(
        &self,
    ) -> &[BoundedResidencyPerformanceCounterObservation] {
        &self.counters
    }
}

pub(super) fn parse(
    lines: &[String],
) -> Result<[BoundedResidencyPerformanceReceiptObservation; 5], String> {
    let mut receipts = lines
        .iter()
        .filter(|line| line.starts_with(MARKER))
        .map(|line| parse_receipt(line))
        .collect::<Result<Vec<_>, _>>()?;
    receipts.sort_by_key(|receipt| receipt.claim);
    if receipts.len() != BoundedResidencyPerformanceClaim::ALL.len()
        || receipts
            .iter()
            .map(|receipt| receipt.claim)
            .collect::<BTreeSet<_>>()
            != BoundedResidencyPerformanceClaim::ALL.into_iter().collect()
        || receipts
            .iter()
            .map(|receipt| receipt.profile)
            .collect::<BTreeSet<_>>()
            .len()
            != 1
    {
        return Err(
            "Courtroom C performance evidence requires five unique claims on one backend profile"
                .to_owned(),
        );
    }
    receipts
        .try_into()
        .map_err(|_| "Courtroom C performance receipt cardinality changed".to_owned())
}

fn parse_receipt(line: &str) -> Result<BoundedResidencyPerformanceReceiptObservation, String> {
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 5 {
        return Err(format!("malformed performance receipt `{line}`"));
    }
    let claim = BoundedResidencyPerformanceClaim::parse(fields[1])
        .ok_or_else(|| format!("unknown performance claim `{}`", fields[1]))?;
    let profile = BoundedResidencyBackendProfile::parse(fields[2])
        .ok_or_else(|| format!("unknown backend profile `{}`", fields[2]))?;
    let expected_count = fields[3]
        .parse::<usize>()
        .map_err(|_| "performance counter count is not a number".to_owned())?;
    let counters = fields[4]
        .split(',')
        .map(parse_counter)
        .collect::<Result<Vec<_>, _>>()?;
    let observed_names = counters
        .iter()
        .map(|counter| counter.name.as_ref())
        .collect::<BTreeSet<&str>>();
    let expected_names = claim
        .counter_names()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if counters.len() != expected_count
        || observed_names.len() != counters.len()
        || observed_names != expected_names
    {
        return Err(format!(
            "performance claim `{}` carried the wrong counter set",
            claim.label()
        ));
    }
    Ok(BoundedResidencyPerformanceReceiptObservation {
        claim,
        profile,
        counters: counters.into_boxed_slice(),
    })
}

fn parse_counter(encoded: &str) -> Result<BoundedResidencyPerformanceCounterObservation, String> {
    let (name, value) = encoded
        .split_once('=')
        .ok_or_else(|| format!("malformed performance counter `{encoded}`"))?;
    let observed_count = value
        .parse()
        .map_err(|_| format!("performance counter `{name}` is not a number"))?;
    Ok(BoundedResidencyPerformanceCounterObservation {
        name: name.into(),
        observed_count,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse, BoundedResidencyPerformanceClaim, MARKER};

    #[test]
    fn parser_requires_every_exact_claim_and_counter_family() {
        let lines = BoundedResidencyPerformanceClaim::ALL
            .into_iter()
            .map(|claim| {
                let rows = claim
                    .counter_names()
                    .iter()
                    .enumerate()
                    .map(|(index, name)| format!("{name}={index}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!(
                    "{MARKER}{} simulated-strict-durable {} {rows}",
                    claim.label(),
                    claim.counter_names().len(),
                )
            })
            .collect::<Vec<_>>();
        assert!(parse(&lines).is_ok());

        let mut missing = lines.clone();
        missing.pop();
        assert!(parse(&missing).is_err());

        let mut wrong_counter = lines;
        wrong_counter[0] = wrong_counter[0].replace("store.durability.groups", "store.other");
        assert!(parse(&wrong_counter).is_err());
    }
}
