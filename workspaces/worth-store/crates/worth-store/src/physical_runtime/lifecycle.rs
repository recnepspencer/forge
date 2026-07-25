use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

const PHASE_BITS: u32 = 3;
const PHASE_MASK: u64 = (1 << PHASE_BITS) - 1;
const ADMITTED_PHASE: u64 = 0;
const CLOSED_PHASE: u64 = 1;
const ABORTED_PHASE: u64 = 2;
const MEDIA_OWNED_PHASE: u64 = 3;
const RECORD_SERVING_PHASE: u64 = 4;
const TERMINATING_PHASE: u64 = 5;
const INITIAL_GENERATION: u64 = 1;

/// Identity of one lifecycle state within a runtime incarnation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifecycleGeneration(u64);

impl LifecycleGeneration {
    pub const fn get(self) -> u64 {
        self.0
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn certification_predecessor(self) -> Self {
        Self(
            self.0
                .checked_sub(1)
                .filter(|generation| *generation != 0)
                .expect("a serving generation has a prior lifecycle generation"),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObservedLifecyclePhase {
    Admitted,
    MediaOwned,
    RecordServing,
    Terminating,
    Closed,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LifecycleStateSnapshot {
    pub(crate) generation: LifecycleGeneration,
    pub(crate) phase: ObservedLifecyclePhase,
}

pub(crate) struct LifecycleState {
    state: AtomicU64,
}

/// First field of every composite runtime that owns subordinate resources.
///
/// Rust drops fields in declaration order. This guard therefore invalidates
/// retained observers before any subordinate owner begins implicit teardown,
/// while the runtime core remains last and records the terminal phase only
/// after those owners have finished dropping.
pub(crate) struct LifecycleTerminationGuard {
    state: Arc<LifecycleState>,
}

impl LifecycleTerminationGuard {
    pub(crate) fn new(state: Arc<LifecycleState>) -> Self {
        Self { state }
    }

    fn begin(&self) {
        self.state.begin_termination();
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn begin_for_certification(&self) {
        self.begin();
    }
}

impl Drop for LifecycleTerminationGuard {
    fn drop(&mut self) {
        self.begin();
    }
}

impl LifecycleState {
    fn admitted() -> Self {
        Self {
            state: AtomicU64::new(encode_state(
                LifecycleGeneration(INITIAL_GENERATION),
                ObservedLifecyclePhase::Admitted,
            )),
        }
    }

    pub(crate) fn snapshot(&self) -> LifecycleStateSnapshot {
        decode_state(self.state.load(Ordering::Acquire))
    }

    fn begin_termination(&self) -> LifecycleStateSnapshot {
        let active = self.snapshot();
        if active.phase == ObservedLifecyclePhase::Terminating {
            return active;
        }
        assert!(
            matches!(
                active.phase,
                ObservedLifecyclePhase::Admitted
                    | ObservedLifecyclePhase::MediaOwned
                    | ObservedLifecyclePhase::RecordServing
            ),
            "only an active move-only runtime can begin termination"
        );
        let terminating = LifecycleStateSnapshot {
            generation: LifecycleGeneration(
                active
                    .generation
                    .get()
                    .checked_add(1)
                    .expect("termination cannot exhaust lifecycle generations"),
            ),
            phase: ObservedLifecyclePhase::Terminating,
        };
        self.state
            .compare_exchange(
                encode_state(active.generation, active.phase),
                encode_state(terminating.generation, terminating.phase),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .expect("the move-only shutdown owner is the sole termination writer");
        terminating
    }

    fn finish(&self, terminal_phase: ObservedLifecyclePhase) -> LifecycleStateSnapshot {
        let terminating = self.snapshot();
        assert_eq!(terminating.phase, ObservedLifecyclePhase::Terminating);
        let terminal = LifecycleStateSnapshot {
            generation: LifecycleGeneration(
                terminating
                    .generation
                    .get()
                    .checked_add(1)
                    .expect("one terminal transition cannot exhaust lifecycle generations"),
            ),
            phase: terminal_phase,
        };
        let terminal_state = encode_state(terminal.generation, terminal.phase);
        self.state
            .compare_exchange(
                encode_state(terminating.generation, terminating.phase),
                terminal_state,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .expect("no second terminal writer can race the move-only shutdown owner");
        terminal
    }

    fn progress_to_media_owned(&self) -> LifecycleStateSnapshot {
        let admitted = self.snapshot();
        assert_eq!(admitted.phase, ObservedLifecyclePhase::Admitted);
        let media_owned = LifecycleStateSnapshot {
            generation: LifecycleGeneration(
                admitted
                    .generation
                    .get()
                    .checked_add(1)
                    .expect("media progression cannot exhaust lifecycle generations"),
            ),
            phase: ObservedLifecyclePhase::MediaOwned,
        };
        self.state
            .compare_exchange(
                encode_state(admitted.generation, admitted.phase),
                encode_state(media_owned.generation, media_owned.phase),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .expect("the consuming runtime transition is the sole phase writer");
        media_owned
    }

    fn progress_to_record_serving(&self) -> LifecycleStateSnapshot {
        let media_owned = self.snapshot();
        assert_eq!(media_owned.phase, ObservedLifecyclePhase::MediaOwned);
        let record_serving = LifecycleStateSnapshot {
            generation: LifecycleGeneration(
                media_owned
                    .generation
                    .get()
                    .checked_add(1)
                    .expect("record-serving progression cannot exhaust lifecycle generations"),
            ),
            phase: ObservedLifecyclePhase::RecordServing,
        };
        self.state
            .compare_exchange(
                encode_state(media_owned.generation, media_owned.phase),
                encode_state(record_serving.generation, record_serving.phase),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .expect("the consuming record-serving transition is the sole phase writer");
        record_serving
    }
}

pub(crate) struct LifecycleCoordinator {
    state: Arc<LifecycleState>,
}

impl LifecycleCoordinator {
    pub(crate) fn admitted() -> Self {
        Self {
            state: Arc::new(LifecycleState::admitted()),
        }
    }

    pub(crate) fn observation_state(&self) -> Arc<LifecycleState> {
        Arc::clone(&self.state)
    }

    pub(crate) fn snapshot(&self) -> LifecycleStateSnapshot {
        self.state.snapshot()
    }

    pub(crate) fn finish_closed(&self) -> LifecycleStateSnapshot {
        self.state.finish(ObservedLifecyclePhase::Closed)
    }

    pub(crate) fn finish_aborted(&self) -> LifecycleStateSnapshot {
        self.state.finish(ObservedLifecyclePhase::Aborted)
    }

    pub(crate) fn progress_to_media_owned(&self) -> LifecycleStateSnapshot {
        self.state.progress_to_media_owned()
    }

    pub(crate) fn begin_termination(&self) -> LifecycleStateSnapshot {
        self.state.begin_termination()
    }

    pub(crate) fn progress_to_record_serving(&self) -> LifecycleStateSnapshot {
        self.state.progress_to_record_serving()
    }
}

fn encode_state(generation: LifecycleGeneration, phase: ObservedLifecyclePhase) -> u64 {
    let phase = match phase {
        ObservedLifecyclePhase::Admitted => ADMITTED_PHASE,
        ObservedLifecyclePhase::Closed => CLOSED_PHASE,
        ObservedLifecyclePhase::Aborted => ABORTED_PHASE,
        ObservedLifecyclePhase::MediaOwned => MEDIA_OWNED_PHASE,
        ObservedLifecyclePhase::RecordServing => RECORD_SERVING_PHASE,
        ObservedLifecyclePhase::Terminating => TERMINATING_PHASE,
    };
    (generation.get() << PHASE_BITS) | phase
}

fn decode_state(state: u64) -> LifecycleStateSnapshot {
    let phase = match state & PHASE_MASK {
        ADMITTED_PHASE => ObservedLifecyclePhase::Admitted,
        CLOSED_PHASE => ObservedLifecyclePhase::Closed,
        ABORTED_PHASE => ObservedLifecyclePhase::Aborted,
        MEDIA_OWNED_PHASE => ObservedLifecyclePhase::MediaOwned,
        RECORD_SERVING_PHASE => ObservedLifecyclePhase::RecordServing,
        TERMINATING_PHASE => ObservedLifecyclePhase::Terminating,
        _ => unreachable!("the private lifecycle encoding emits only known phases"),
    };
    LifecycleStateSnapshot {
        generation: LifecycleGeneration(state >> PHASE_BITS),
        phase,
    }
}
