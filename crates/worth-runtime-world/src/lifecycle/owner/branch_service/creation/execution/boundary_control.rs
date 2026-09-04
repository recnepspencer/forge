//! Test-only rendezvous at the between-owners boundary of one creation.
//!
//! The creation path rechecks the product head between its two owner forks.
//! Reaching that recheck with a moved head requires another operation to
//! settle after the Relational fork and before the recheck, which no budget or
//! intent can arrange on its own. This control stops exactly one armed
//! creation there and hands its owner identity to the arming test, which moves
//! the head and releases it.
//!
//! The rehearsal is keyed by owner identity, armed for one creation, never
//! process-global, and its wait is bounded so a released-nowhere test fails by
//! name instead of hanging.

use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::identity::RuntimeWorldOwnerIdentity;
use crate::lifecycle::RuntimeWorldOwnerRoot;

const CREATION_BOUNDARY_PAUSE_TIMEOUT: Duration = Duration::from_secs(5);

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// Arm one creation on this owner to stop after its Relational fork and
    /// before the head recheck that guards the Signal fork. The owner identity
    /// of the paused creation arrives on `reached`; dropping the returned guard
    /// releases the creation and disarms the rehearsal.
    pub(crate) fn rehearse_creation_fork_boundary(
        &self,
        reached: SyncSender<RuntimeWorldOwnerIdentity>,
    ) -> CreationBoundaryRehearsalGuard {
        let owner = self.owner_identity();
        let rehearsal = Arc::new(CreationBoundaryRehearsal {
            armed: Mutex::new(true),
            reached,
            release: (Mutex::new(false), Condvar::new()),
        });
        let mut armed = rehearsals()
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        assert!(
            !armed.contains_key(&owner),
            "only one creation boundary rehearsal may be armed for an owner"
        );
        armed.insert(owner, Arc::clone(&rehearsal));
        CreationBoundaryRehearsalGuard { owner, rehearsal }
    }
}

/// Hold the single armed creation between its two owner forks. Every other
/// creation passes straight through.
pub(super) fn pause_between_creation_forks(owner: RuntimeWorldOwnerIdentity) {
    let Some(rehearsal) = armed_rehearsal(owner) else {
        return;
    };
    if rehearsal.claim() {
        rehearsal.wait_for_release(owner);
    }
}

pub(crate) struct CreationBoundaryRehearsalGuard {
    owner: RuntimeWorldOwnerIdentity,
    rehearsal: Arc<CreationBoundaryRehearsal>,
}

impl std::fmt::Debug for CreationBoundaryRehearsalGuard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CreationBoundaryRehearsalGuard")
            .field("owner", &self.owner)
            .finish_non_exhaustive()
    }
}

impl Drop for CreationBoundaryRehearsalGuard {
    fn drop(&mut self) {
        self.rehearsal.release();
        let mut armed = rehearsals()
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let owned_registration = armed
            .get(&self.owner)
            .is_some_and(|current| Arc::ptr_eq(current, &self.rehearsal));
        if owned_registration {
            armed.remove(&self.owner);
        }
    }
}

#[derive(Debug)]
struct CreationBoundaryRehearsal {
    armed: Mutex<bool>,
    reached: SyncSender<RuntimeWorldOwnerIdentity>,
    release: (Mutex<bool>, Condvar),
}

impl CreationBoundaryRehearsal {
    fn claim(&self) -> bool {
        let mut armed = self.armed.lock().unwrap_or_else(|error| error.into_inner());
        let claimed = *armed;
        *armed = false;
        claimed
    }

    fn wait_for_release(&self, owner: RuntimeWorldOwnerIdentity) {
        if self.reached.send(owner).is_err() {
            return;
        }
        let deadline = Instant::now() + CREATION_BOUNDARY_PAUSE_TIMEOUT;
        let (opened, signal) = &self.release;
        let mut opened = opened.lock().unwrap_or_else(|error| error.into_inner());
        while !*opened {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return;
            };
            let (next, result) = signal
                .wait_timeout(opened, remaining)
                .unwrap_or_else(|error| error.into_inner());
            opened = next;
            if result.timed_out() {
                return;
            }
        }
    }

    fn release(&self) {
        let (opened, signal) = &self.release;
        let mut opened = opened.lock().unwrap_or_else(|error| error.into_inner());
        *opened = true;
        signal.notify_all();
    }
}

static ARMED_REHEARSALS: OnceLock<
    Mutex<HashMap<RuntimeWorldOwnerIdentity, Arc<CreationBoundaryRehearsal>>>,
> = OnceLock::new();

fn rehearsals() -> &'static Mutex<HashMap<RuntimeWorldOwnerIdentity, Arc<CreationBoundaryRehearsal>>>
{
    ARMED_REHEARSALS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn armed_rehearsal(owner: RuntimeWorldOwnerIdentity) -> Option<Arc<CreationBoundaryRehearsal>> {
    rehearsals()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&owner)
        .map(Arc::clone)
}
