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

    pub(super) fn into_ownership(self) -> MutationAuthority<'owner> {
        let Self {
            ownership,
            _sequence,
        } = self;
        drop(_sequence);
        ownership
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

    pub(super) fn try_acquire_or_create(
        owner: MediaOwnerIdentity,
        namespace_directory: &NamespaceDirectoryHandle,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<Self, super::owner_admission_effect::MutationOwnershipAcquisitionFailure> {
        Self::try_acquire_with(
            owner,
            namespace_directory,
            boundary,
            |owner, namespace, boundary| {
                super::mutation_lock_file::open_or_create(owner, namespace, boundary)
            },
        )
    }

    #[cfg(feature = "recovery-runtime-owner")]
    pub(super) fn try_acquire_existing(
        owner: MediaOwnerIdentity,
        namespace_directory: &NamespaceDirectoryHandle,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<Self, super::owner_admission_effect::MutationOwnershipAcquisitionFailure> {
        Self::try_acquire_with(
            owner,
            namespace_directory,
            boundary,
            |owner, namespace, boundary| {
                super::mutation_lock_file::open_existing(owner, namespace, boundary)
            },
        )
    }

    fn try_acquire_with(
        owner: MediaOwnerIdentity,
        namespace_directory: &NamespaceDirectoryHandle,
        boundary: &super::fault_interposition::MediaFaultInterposer,
        open_lock: impl FnOnce(
            MediaOwnerIdentity,
            &NamespaceDirectoryHandle,
            &super::fault_interposition::MediaFaultInterposer,
        ) -> Result<
            super::mutation_lock_file::OpenedMutationLock,
            super::owner_admission_effect::MutationOwnershipAcquisitionFailure,
        >,
    ) -> Result<Self, super::owner_admission_effect::MutationOwnershipAcquisitionFailure> {
        use super::owner_admission_effect::MutationOwnershipAcquisitionFailure;

        if !namespace_directory.belongs_to(owner) {
            return Err(MutationOwnershipAcquisitionFailure::before_effect(
                MutationOwnershipDenial::Confinement(NamespaceConfinementDenial::structural(
                    super::NamespaceConfinementDenialKind::AuthorityMismatch,
                )),
            ));
        }
        let attempt = MutationOwnershipAttempt::generate()
            .map_err(MutationOwnershipAcquisitionFailure::before_effect)?;
        let opened = open_lock(owner, namespace_directory, boundary)?;
        if let Err(denial) = acquire_os_lease(&opened.file, boundary) {
            return Err(MutationOwnershipAcquisitionFailure::new(
                denial,
                opened.effect_fate,
                None,
            ));
        }
        let lease = Self {
            owner,
            attempt,
            lock: opened.file,
            state: AtomicU8::new(LEASE_LIVE),
            counters: Arc::clone(boundary.shared_counters()),
        };
        if let Err(publication) = super::mutation_owner_publication::publish_owner_observation(
            &lease.lock,
            owner,
            attempt,
            boundary,
        ) {
            let effect_fate = opened.effect_fate.combine(publication.effect_fate);
            let release = lease.release(boundary);
            return Err(MutationOwnershipAcquisitionFailure::new(
                publication.denial,
                effect_fate,
                Some(release),
            ));
        }
        Ok(lease)
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

pub(super) fn lock_error(error: io::Error) -> MutationOwnershipDenial {
    MutationOwnershipDenial::LockOperationFailed {
        kind: error.kind(),
        raw_os_error: error.raw_os_error(),
    }
}
