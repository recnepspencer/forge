//! Deterministic wrong-owner executions over a typed locality-case basis.

use sha2::{Digest, Sha256};

use super::super::case::Phase5LocalityCase;

const DEPENDENCY_ROLES: u8 = 8;
const MECHANIC_SLOTS: u8 = 2;

pub(super) struct MutantExecution {
    work: u64,
    trace_digest: [u8; 32],
}

impl MutantExecution {
    pub(super) const fn work(&self) -> u64 {
        self.work
    }

    pub(super) const fn trace_digest(&self) -> [u8; 32] {
        self.trace_digest
    }
}

pub(super) fn complete_subscriber_closure(case: Phase5LocalityCase) -> MutantExecution {
    execute_candidate_selection(case, b"complete-subscriber-closure", |_, _, _| true)
}

pub(super) fn late_aspect_filter(case: Phase5LocalityCase) -> MutantExecution {
    execute_candidate_selection(case, b"late-aspect-filter", |_, _, role| role != 7)
}

pub(super) fn late_scope_filter(case: Phase5LocalityCase) -> MutantExecution {
    execute_candidate_selection(case, b"late-scope-filter", |_, _, role| role % 2 == 0)
}

pub(super) fn global_partition_detail_range_union(case: Phase5LocalityCase) -> MutantExecution {
    execute_candidate_selection(case, b"global-scope-union", |_, _, role| role < 3)
}

pub(super) fn every_mounted_presentation(case: Phase5LocalityCase) -> MutantExecution {
    execute_rows(case, b"every-mounted-presentation", |paragraph, trace| {
        for slot in 0..MECHANIC_SLOTS {
            record(trace, paragraph, slot, 0, b"invalidate");
        }
        MECHANIC_SLOTS as u64
    })
}

pub(super) fn layout_widening(case: Phase5LocalityCase, mutant: &'static str) -> MutantExecution {
    execute_rows(case, mutant.as_bytes(), |paragraph, trace| {
        record(trace, paragraph, 0, 0, b"layout");
        1
    })
}

pub(super) fn drop_immediate_dependency(case: Phase5LocalityCase) -> MutantExecution {
    let mut trace = start(case, b"drop-immediate-dependency");
    record(
        &mut trace,
        case.target_index() as u64,
        0,
        0,
        b"content-only",
    );
    finish(trace, 1)
}

pub(super) fn hidden_retained_document_scan(case: Phase5LocalityCase) -> MutantExecution {
    execute_rows(
        case,
        b"hidden-retained-document-scan",
        |paragraph, trace| {
            for slot in 0..MECHANIC_SLOTS {
                record(trace, paragraph, slot, 0, b"retained-command-read");
            }
            MECHANIC_SLOTS as u64
        },
    )
}

pub(super) fn predicted_counter_substitution(case: Phase5LocalityCase) -> MutantExecution {
    let mut trace = start(case, b"predicted-counter-substitution");
    record(
        &mut trace,
        case.target_index() as u64,
        0,
        0,
        b"prediction-without-owner-observation",
    );
    finish(trace, 0)
}

fn execute_candidate_selection(
    case: Phase5LocalityCase,
    algorithm: &'static [u8],
    admits: impl Fn(u64, u8, u8) -> bool,
) -> MutantExecution {
    execute_rows(case, algorithm, |paragraph, trace| {
        let mut selected = 0;
        for slot in 0..MECHANIC_SLOTS {
            for role in 0..DEPENDENCY_ROLES {
                if admits(paragraph, slot, role) {
                    record(trace, paragraph, slot, role, b"enqueue");
                    selected += 1;
                }
            }
        }
        selected
    })
}

fn execute_rows(
    case: Phase5LocalityCase,
    algorithm: &'static [u8],
    mut execute: impl FnMut(u64, &mut Sha256) -> u64,
) -> MutantExecution {
    let mut trace = start(case, algorithm);
    let work = (0..case.retained_paragraphs() as u64)
        .map(|paragraph| execute(paragraph, &mut trace))
        .sum();
    finish(trace, work)
}

fn start(case: Phase5LocalityCase, algorithm: &[u8]) -> Sha256 {
    let mut trace = Sha256::new();
    trace.update(b"worth-ui-phase5-owner-mutant-v1");
    trace.update(algorithm);
    trace.update((case.retained_size() as u64).to_le_bytes());
    trace.update([case.axis() as u8]);
    trace
}

fn record(trace: &mut Sha256, paragraph: u64, slot: u8, role: u8, action: &[u8]) {
    trace.update(paragraph.to_le_bytes());
    trace.update([slot, role]);
    trace.update(action);
}

fn finish(mut trace: Sha256, work: u64) -> MutantExecution {
    trace.update(work.to_le_bytes());
    MutantExecution {
        work,
        trace_digest: trace.finalize().into(),
    }
}
