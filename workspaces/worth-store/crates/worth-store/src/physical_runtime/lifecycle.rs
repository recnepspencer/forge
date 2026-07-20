use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

const PHASE_BITS: u32 = 2;
const PHASE_MASK: u64 = (1 << PHASE_BITS) - 1;
const ADMITTED_PHASE: u64 = 0;
const CLOSED_PHASE: u64 = 1;
const ABORTED_PHASE: u64 = 2;
const MEDIA_OWNED_PHASE: u64 = 3;
const INITIAL_GENERATION: u64 = 1;

/// Identity of one lifecycle state within a runtime incarnation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleGeneration(u64);

impl LifecycleGeneration {
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObservedLifecyclePhase {
    Admitted,
    MediaOwned,
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

    fn finish(&self, terminal_phase: ObservedLifecyclePhase) -> LifecycleStateSnapshot {
        let admitted = self.snapshot();
        assert!(
            matches!(
                admitted.phase,
                ObservedLifecyclePhase::Admitted | ObservedLifecyclePhase::MediaOwned
            ),
            "the move-only shutdown owner permits exactly one terminal transition"
        );
        let terminal = LifecycleStateSnapshot {
            generation: LifecycleGeneration(
                admitted
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
                encode_state(admitted.generation, admitted.phase),
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
}

fn encode_state(generation: LifecycleGeneration, phase: ObservedLifecyclePhase) -> u64 {
    let phase = match phase {
        ObservedLifecyclePhase::Admitted => ADMITTED_PHASE,
        ObservedLifecyclePhase::Closed => CLOSED_PHASE,
        ObservedLifecyclePhase::Aborted => ABORTED_PHASE,
        ObservedLifecyclePhase::MediaOwned => MEDIA_OWNED_PHASE,
    };
    (generation.get() << PHASE_BITS) | phase
}

fn decode_state(state: u64) -> LifecycleStateSnapshot {
    let phase = match state & PHASE_MASK {
        ADMITTED_PHASE => ObservedLifecyclePhase::Admitted,
        CLOSED_PHASE => ObservedLifecyclePhase::Closed,
        ABORTED_PHASE => ObservedLifecyclePhase::Aborted,
        MEDIA_OWNED_PHASE => ObservedLifecyclePhase::MediaOwned,
        _ => unreachable!("the private lifecycle encoding emits only known phases"),
    };
    LifecycleStateSnapshot {
        generation: LifecycleGeneration(state >> PHASE_BITS),
        phase,
    }
}
