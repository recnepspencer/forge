#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiHostObservationWorkReport {
    batches_handled: u64,
    raw_entries_handled: u64,
    validated_entries: u64,
    retained_entries: u64,
    coalesced_entries: u64,
    duplicate_entries: u64,
    quarantined_entries: u64,
    denied_entries: u64,
    overflowed_entries: u64,
}

pub(super) struct UiHostObservationDeclaredWork {
    raw_entries: usize,
    coalesced_entries: u64,
    overflowed_entries: u64,
}

impl UiHostObservationWorkReport {
    pub const fn batches_handled(self) -> u64 {
        self.batches_handled
    }

    pub const fn raw_entries_handled(self) -> u64 {
        self.raw_entries_handled
    }

    pub const fn validated_entries(self) -> u64 {
        self.validated_entries
    }

    pub const fn retained_entries(self) -> u64 {
        self.retained_entries
    }

    pub const fn coalesced_entries(self) -> u64 {
        self.coalesced_entries
    }

    pub const fn duplicate_entries(self) -> u64 {
        self.duplicate_entries
    }

    pub const fn quarantined_entries(self) -> u64 {
        self.quarantined_entries
    }

    pub const fn denied_entries(self) -> u64 {
        self.denied_entries
    }

    pub const fn overflowed_entries(self) -> u64 {
        self.overflowed_entries
    }

    pub(super) fn record(
        &mut self,
        declared: UiHostObservationDeclaredWork,
        outcome: &super::UiHostObservationReportOutcome,
    ) {
        add(&mut self.batches_handled, 1);
        add(&mut self.raw_entries_handled, declared.raw_entries);
        add_u64(&mut self.coalesced_entries, declared.coalesced_entries);
        add_u64(&mut self.overflowed_entries, declared.overflowed_entries);
        match outcome {
            super::UiHostObservationReportOutcome::Validated(validated) => {
                add(&mut self.validated_entries, validated.reports().len());
                for report in validated.reports() {
                    match report.disposition() {
                        super::UiHostObservationDisposition::Retained => {
                            add(&mut self.retained_entries, 1)
                        }
                        super::UiHostObservationDisposition::Coalesced { .. } => {
                            add(&mut self.coalesced_entries, 1)
                        }
                    }
                }
            }
            super::UiHostObservationReportOutcome::Duplicate(_) => {
                add(&mut self.duplicate_entries, declared.raw_entries)
            }
            super::UiHostObservationReportOutcome::Quarantined(_) => {
                add(&mut self.quarantined_entries, declared.raw_entries)
            }
            super::UiHostObservationReportOutcome::Denied(_) => {
                add(&mut self.denied_entries, declared.raw_entries);
            }
        }
    }
}

impl UiHostObservationDeclaredWork {
    pub(super) fn from_batch(batch: &worth_ui_host_contract::UiHostObservationBatch) -> Self {
        let (coalesced_entries, overflowed_entries) = match batch.canonical_core().loss() {
            worth_ui_host_contract::UiHostObservationLoss::Complete => (0, 0),
            worth_ui_host_contract::UiHostObservationLoss::Coalesced { replaced, .. } => {
                (range_cardinality(replaced), 0)
            }
            worth_ui_host_contract::UiHostObservationLoss::Overflow { affected, .. } => {
                (0, range_cardinality(affected))
            }
        };
        Self {
            raw_entries: batch.reports().len(),
            coalesced_entries,
            overflowed_entries,
        }
    }
}

fn add(target: &mut u64, count: usize) {
    let count = u64::try_from(count).expect("one in-memory report count fits u64");
    add_u64(target, count);
}

fn add_u64(target: &mut u64, count: u64) {
    *target = target
        .checked_add(count)
        .expect("host observation work report exhausted u64");
}

fn range_cardinality(range: worth_ui_host_contract::UiHostObservationSequenceRange) -> u64 {
    range
        .last()
        .value()
        .checked_sub(range.first().value())
        .and_then(|distance| distance.checked_add(1))
        .unwrap_or(0)
}
