use std::io;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use fs4::FileExt;

use super::{MediaOwnerIdentity, NamespaceConfinementDenial, NamespaceDirectoryHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationOwnershipAttempt([u8; 16]);

impl MutationOwnershipAttempt {
    pub const fn bytes(self) -> [u8; 16] {
        self.0
    }

    fn generate() -> Result<Self, MutationOwnershipDenial> {
        loop {
            let mut bytes = [0_u8; 16];
            getrandom::fill(&mut bytes)
                .map_err(|_| MutationOwnershipDenial::AttemptIdentityUnavailable)?;
            if bytes != [0_u8; 16] {
                return Ok(Self(bytes));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationOwnershipDenial {
    Confinement(NamespaceConfinementDenial),
    Contended,
    AttemptIdentityUnavailable,
    OwnershipLost,
    LockOperationFailed {
        kind: io::ErrorKind,
        raw_os_error: Option<i32>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationOwnerObservation {
    owner: MediaOwnerIdentity,
    attempt: MutationOwnershipAttempt,
    process_id: u32,
}

impl MutationOwnerObservation {
    pub const fn owner(self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn attempt(self) -> MutationOwnershipAttempt {
        self.attempt
    }

    pub const fn process_id(self) -> u32 {
        self.process_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipReleaseOutcome {
    Released,
    ReleaseUnconfirmed {
        kind: io::ErrorKind,
        raw_os_error: Option<i32>,
    },
}

/// The still-live OS file lock granting mutation authority to one process.
///
/// This type is move-only. Lock-file contents are deliberately absent from its
/// authority shape.
#[derive(Debug)]
pub struct MutationOwnershipLease {
    owner: MediaOwnerIdentity,
    attempt: MutationOwnershipAttempt,
    lock: std::fs::File,
    state: AtomicU8,
    counters: Arc<super::operation_counters::MediaCounterCells>,
}

/// Borrowed fact that one mutation was admitted while the owner's lease was
/// live. Invalidation denies later admissions without retroactively revoking
/// this already-ordered fact.
#[derive(Debug)]
pub(super) struct MutationAuthority<'owner> {
    owner: MediaOwnerIdentity,
    _lease: std::marker::PhantomData<&'owner MutationOwnershipLease>,
}

impl<'owner> MutationAuthority<'owner> {
    pub(super) const fn new(
        owner: MediaOwnerIdentity,
        _lease: &'owner MutationOwnershipLease,
    ) -> Self {
        Self {
            owner,
            _lease: std::marker::PhantomData,
        }
    }

    fn belongs_to(&self, owner: MediaOwnerIdentity) -> bool {
        self.owner == owner
    }
}

/// Mutation authority additionally serialized against every other coordinated
/// namespace rename or deletion owned by this media owner.
#[derive(Debug)]
pub(super) struct CoordinatedNamespaceMutation<'owner> {
    ownership: MutationAuthority<'owner>,
    _sequence: std::sync::MutexGuard<'owner, ()>,
}

impl<'owner> CoordinatedNamespaceMutation<'owner> {
    pub(super) const fn new(
        ownership: MutationAuthority<'owner>,
        sequence: std::sync::MutexGuard<'owner, ()>,
    ) -> Self {
        Self {
            ownership,
            _sequence: sequence,
        }
    }

    pub(super) fn belongs_to(&self, owner: MediaOwnerIdentity) -> bool {
        self.ownership.belongs_to(owner)
    }
}

const LEASE_LIVE: u8 = 0;
const LEASE_LOST: u8 = 1;
const LEASE_RELEASED: u8 = 2;

impl MutationOwnershipLease {
    pub fn observation(&self) -> MutationOwnerObservation {
        MutationOwnerObservation {
            owner: self.owner,
            attempt: self.attempt,
            process_id: std::process::id(),
        }
    }

    pub(super) fn try_acquire(
        owner: MediaOwnerIdentity,
        namespace_directory: &NamespaceDirectoryHandle,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<Self, MutationOwnershipDenial> {
        if !namespace_directory.belongs_to(owner) {
            return Err(MutationOwnershipDenial::Confinement(
                NamespaceConfinementDenial::structural(
                    super::NamespaceConfinementDenialKind::AuthorityMismatch,
                ),
            ));
        }
        let attempt = MutationOwnershipAttempt::generate()?;
        let lock = super::mutation_lock_file::open(owner, namespace_directory, boundary)?;
        acquire_os_lease(&lock, boundary)?;
        if let Err(denial) = publish_owner_observation(&lock, owner, attempt, boundary) {
            let _ = FileExt::unlock(&lock);
            boundary.shared_counters().ownership_released();
            return Err(denial);
        }
        Ok(Self {
            owner,
            attempt,
            lock,
            state: AtomicU8::new(LEASE_LIVE),
            counters: Arc::clone(boundary.shared_counters()),
        })
    }

    pub(super) fn belongs_to(&self, owner: MediaOwnerIdentity) -> bool {
        self.owner == owner && self.state.load(Ordering::Acquire) == LEASE_LIVE
    }

    pub(super) fn invalidate(&self) {
        self.state.store(LEASE_LOST, Ordering::Release);
    }

    pub(super) fn release(
        self,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> OwnershipReleaseOutcome {
        let attempt = boundary.begin_lease_release();
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return OwnershipReleaseOutcome::ReleaseUnconfirmed {
                kind: error.kind(),
                raw_os_error: error.raw_os_error(),
            };
        }
        let result = match FileExt::unlock(&self.lock) {
            Ok(()) => {
                attempt.completed(0);
                OwnershipReleaseOutcome::Released
            }
            Err(error) => {
                attempt.indeterminate(0);
                OwnershipReleaseOutcome::ReleaseUnconfirmed {
                    kind: error.kind(),
                    raw_os_error: error.raw_os_error(),
                }
            }
        };
        self.state.store(
            if matches!(result, OwnershipReleaseOutcome::Released) {
                LEASE_RELEASED
            } else {
                LEASE_LOST
            },
            Ordering::Release,
        );
        result
    }
}

impl Drop for MutationOwnershipLease {
    fn drop(&mut self) {
        self.counters.ownership_released();
    }
}

fn acquire_os_lease(
    lock: &std::fs::File,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<(), MutationOwnershipDenial> {
    let attempt = boundary.begin(super::MediaOperationRole::AcquireMutationLease, 0);
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(lock_error(error));
    }
    match lock.try_lock_exclusive() {
        Ok(()) => {
            attempt.completed(0);
            boundary.shared_counters().ownership_acquired();
            Ok(())
        }
        Err(error) => {
            attempt.denied();
            let contended = fs4::lock_contended_error();
            if error.kind() == io::ErrorKind::WouldBlock
                || error.raw_os_error() == contended.raw_os_error()
            {
                Err(MutationOwnershipDenial::Contended)
            } else {
                Err(lock_error(error))
            }
        }
    }
}

fn publish_owner_observation(
    lock: &std::fs::File,
    owner: MediaOwnerIdentity,
    ownership_attempt: MutationOwnershipAttempt,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<(), MutationOwnershipDenial> {
    use std::fmt::Write as _;
    use std::io::{Seek, Write as _};

    let mut observation = String::with_capacity(128);
    boundary
        .counters()
        .explicit_heap_allocation(observation.capacity());
    writeln!(&mut observation, "version=1").expect("writing to String cannot fail");
    writeln!(&mut observation, "process={:010}", std::process::id())
        .expect("writing to String cannot fail");
    observation.push_str("runtime=");
    append_hex(&mut observation, &owner.bytes());
    observation.push('\n');
    observation.push_str("attempt=");
    append_hex(&mut observation, &ownership_attempt.bytes());
    observation.push('\n');
    let attempted_bytes = observation.len() as u64;
    let attempt = boundary.begin(
        super::MediaOperationRole::PublishMutationLeaseObservation,
        attempted_bytes,
    );
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(lock_error(error));
    }
    if let Err(error) = lock.set_len(0) {
        attempt.indeterminate(0);
        return Err(lock_error(error));
    }
    let mut writer = lock;
    if let Err(error) = writer.seek(std::io::SeekFrom::Start(0)) {
        attempt.indeterminate(0);
        return Err(lock_error(error));
    }
    let transfer_limit = attempt.transfer_limit(attempted_bytes) as usize;
    let mut completed = 0;
    while completed < transfer_limit {
        match writer.write(&observation.as_bytes()[completed..transfer_limit]) {
            Ok(0) => {
                attempt.indeterminate(completed as u64);
                return Err(lock_error(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "lease observation write made no progress",
                )));
            }
            Ok(bytes) => completed += bytes,
            Err(error) => {
                attempt.indeterminate(completed as u64);
                return Err(lock_error(error));
            }
        }
    }
    if completed != observation.len() {
        attempt.partial(completed as u64);
        return Err(lock_error(io::Error::new(
            io::ErrorKind::WriteZero,
            "certification stopped lease observation after a known prefix",
        )));
    }
    if let Some(error) = attempt.barrier_error() {
        attempt.indeterminate(attempted_bytes);
        return Err(lock_error(error));
    }
    if let Err(error) = lock.sync_data() {
        attempt.indeterminate(attempted_bytes);
        return Err(lock_error(error));
    }
    if attempt.effect_observation_is_indeterminate() {
        attempt.indeterminate(attempted_bytes);
        return Err(lock_error(io::Error::other(
            "certification interrupted lease-publication observation",
        )));
    }
    attempt.completed(attempted_bytes);
    Ok(())
}

fn append_hex(encoded: &mut String, bytes: &[u8]) {
    use std::fmt::Write;

    for byte in bytes {
        write!(encoded, "{byte:02x}").expect("writing to String cannot fail");
    }
}

pub(super) fn lock_error(error: io::Error) -> MutationOwnershipDenial {
    MutationOwnershipDenial::LockOperationFailed {
        kind: error.kind(),
        raw_os_error: error.raw_os_error(),
    }
}
