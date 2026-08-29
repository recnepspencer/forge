use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::identity::data::VersionId;
use crate::snapshots::data::SnapshotId;

use super::snapshot_handle_binding::SnapshotHandleBinding;

#[derive(Debug, Default)]
pub(crate) struct SnapshotHandles {
    active: HashMap<SnapshotId, SnapshotHandleBinding>,
    published: Arc<Mutex<PublishedSnapshotHandles>>,
    next_snapshot_id: Arc<AtomicU64>,
    active_key_lookups: AtomicU64,
    active_mutations: AtomicU64,
}

#[derive(Debug, Default)]
struct PublishedSnapshotHandles {
    by_id: HashMap<SnapshotId, SnapshotHandleBinding>,
    by_version: HashMap<VersionId, SnapshotId>,
    key_lookups: u64,
    mutations: u64,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SnapshotHandleRegistryCostCounters {
    pub(crate) active_entries: u64,
    pub(crate) active_key_lookups: u64,
    pub(crate) active_mutations: u64,
    pub(crate) published_entries: u64,
    pub(crate) published_key_lookups: u64,
    pub(crate) published_mutations: u64,
}

#[derive(Debug)]
pub(crate) struct PublishedSnapshotCapacityOwner {
    maximum_handles: usize,
    occupied_handles: AtomicUsize,
}

impl PublishedSnapshotCapacityOwner {
    pub(crate) fn new(maximum_handles: usize) -> Arc<Self> {
        Arc::new(Self {
            maximum_handles,
            occupied_handles: AtomicUsize::new(0),
        })
    }

    pub(crate) fn reserve(self: &Arc<Self>) -> Result<PublishedSnapshotSlotReservation, usize> {
        let mut occupied = self.occupied_handles.load(Ordering::Acquire);
        loop {
            if occupied >= self.maximum_handles {
                return Err(self.maximum_handles);
            }
            match self.occupied_handles.compare_exchange_weak(
                occupied,
                occupied + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    return Ok(PublishedSnapshotSlotReservation {
                        owner: Arc::clone(self),
                        releases_on_drop: true,
                    });
                }
                Err(observed) => occupied = observed,
            }
        }
    }

    pub(crate) const fn maximum_handles(&self) -> usize {
        self.maximum_handles
    }

    pub(crate) fn occupied_handles(&self) -> usize {
        self.occupied_handles.load(Ordering::Acquire)
    }

    pub(crate) fn release(&self) {
        let previous = self.occupied_handles.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "published snapshot capacity underflow");
    }
}

#[derive(Debug)]
#[must_use = "published snapshot capacity reservations must be installed or released"]
pub(crate) struct PublishedSnapshotSlotReservation {
    owner: Arc<PublishedSnapshotCapacityOwner>,
    releases_on_drop: bool,
}

impl PublishedSnapshotSlotReservation {
    pub(crate) fn install(mut self) {
        self.releases_on_drop = false;
    }
}

impl Drop for PublishedSnapshotSlotReservation {
    fn drop(&mut self) {
        if self.releases_on_drop {
            self.owner.release();
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PublishedSnapshotCloseout {
    inner: Arc<PublishedSnapshotCloseoutInner>,
}

#[derive(Debug)]
struct PublishedSnapshotCloseoutInner {
    handles: Weak<Mutex<PublishedSnapshotHandles>>,
    capacity: Arc<PublishedSnapshotCapacityOwner>,
    snapshot_id: SnapshotId,
    /// Whether dropping this closeout still owes the handle a release. It is
    /// cleared when the obligation moves to a commit-result holder, because
    /// exactly one party may release one published handle.
    armed: AtomicBool,
}

impl PublishedSnapshotCloseout {
    fn new(
        handles: &Arc<Mutex<PublishedSnapshotHandles>>,
        capacity: Arc<PublishedSnapshotCapacityOwner>,
        snapshot_id: SnapshotId,
    ) -> Self {
        Self {
            inner: Arc::new(PublishedSnapshotCloseoutInner {
                handles: Arc::downgrade(handles),
                capacity,
                snapshot_id,
                armed: AtomicBool::new(true),
            }),
        }
    }

    pub(crate) fn close(&self) {
        self.inner.close();
    }

    /// Hand the release obligation to the holder of the commit result that
    /// names this snapshot. Every clone of this closeout stops closing on
    /// drop, so the handle is released once by its new owner and not here.
    pub(crate) fn transfer_release_obligation(&self) {
        self.inner.armed.store(false, Ordering::Release);
    }
}

impl PartialEq for PublishedSnapshotCloseout {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for PublishedSnapshotCloseout {}

impl PublishedSnapshotCloseoutInner {
    fn close(&self) {
        let Some(handles) = self.handles.upgrade() else {
            return;
        };
        let removed = remove_published_handle(
            &mut handles
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
            self.snapshot_id,
        );
        if removed.is_some() {
            self.capacity.release();
        }
    }
}

impl Drop for PublishedSnapshotCloseoutInner {
    fn drop(&mut self) {
        if self.armed.load(Ordering::Acquire) {
            self.close();
        }
    }
}

impl SnapshotHandles {
    pub(crate) fn snapshot_identity_binding(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.next_snapshot_id)
    }

    pub(crate) fn new() -> Self {
        Self {
            active: HashMap::new(),
            published: Arc::new(Mutex::new(PublishedSnapshotHandles::default())),
            next_snapshot_id: Arc::new(AtomicU64::new(1)),
            active_key_lookups: AtomicU64::new(0),
            active_mutations: AtomicU64::new(0),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            active: HashMap::new(),
            published: Arc::new(Mutex::new(PublishedSnapshotHandles::default())),
            next_snapshot_id: Arc::new(AtomicU64::new(1)),
            active_key_lookups: AtomicU64::new(0),
            active_mutations: AtomicU64::new(0),
        }
    }

    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }

    pub(crate) fn published_count(&self) -> usize {
        self.lock_published().by_id.len()
    }

    pub(crate) fn next_snapshot_id(&self) -> Option<SnapshotId> {
        self.next_snapshot_id
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                (current != 0).then(|| current.checked_add(1).unwrap_or(0))
            })
            .ok()
            .map(SnapshotId)
    }

    pub(crate) fn insert_active(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        assert!(
            !self.active.contains_key(&snapshot_id),
            "snapshot identity allocator collided with a live active handle"
        );
        self.active_key_lookups.fetch_add(1, Ordering::Relaxed);
        self.active_mutations.fetch_add(1, Ordering::Relaxed);
        self.active.insert(snapshot_id, binding);
    }

    pub(crate) fn remove_active(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.active_key_lookups.fetch_add(1, Ordering::Relaxed);
        let removed = self.active.remove(&snapshot_id);
        if removed.is_some() {
            self.active_mutations.fetch_add(1, Ordering::Relaxed);
        }
        removed
    }

    pub(crate) fn active_binding(&self, snapshot_id: SnapshotId) -> Option<&SnapshotHandleBinding> {
        self.active_key_lookups.fetch_add(1, Ordering::Relaxed);
        self.active.get(&snapshot_id)
    }

    pub(crate) fn is_known_snapshot(&self, snapshot_id: SnapshotId) -> bool {
        self.active.contains_key(&snapshot_id)
            || self.lock_published().by_id.contains_key(&snapshot_id)
    }

    pub(crate) fn active_versions(&self) -> impl Iterator<Item = VersionId> + '_ {
        self.active.values().map(|binding| binding.version_id)
    }

    pub(crate) fn insert_published(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        let mut published = self.lock_published();
        let version_id = binding.version_id;
        assert!(
            !published.by_id.contains_key(&snapshot_id),
            "snapshot identity allocator collided with a live published handle"
        );
        assert!(
            !published.by_version.contains_key(&version_id),
            "one published snapshot already owns this exact version"
        );
        published.key_lookups = published.key_lookups.saturating_add(2);
        published.mutations = published.mutations.saturating_add(2);
        let previous_binding = published.by_id.insert(snapshot_id, binding);
        let previous_snapshot = published.by_version.insert(version_id, snapshot_id);
        debug_assert!(previous_binding.is_none());
        debug_assert!(previous_snapshot.is_none());
    }

    pub(crate) fn remove_published(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        remove_published_handle(&mut self.lock_published(), snapshot_id)
    }

    pub(crate) fn published_versions(&self) -> Vec<VersionId> {
        self.lock_published()
            .by_id
            .values()
            .map(|binding| binding.version_id)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn retains_published_version(&self, version_id: VersionId) -> bool {
        self.lock_published()
            .by_id
            .values()
            .any(|binding| binding.version_id == version_id)
    }

    pub(crate) fn published_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.lock_published().by_id.get(&snapshot_id).cloned()
    }

    pub(crate) fn published_binding_for_version(
        &self,
        version_id: VersionId,
    ) -> Option<(SnapshotId, SnapshotHandleBinding)> {
        let published = self.lock_published();
        let snapshot_id = *published.by_version.get(&version_id)?;
        published
            .by_id
            .get(&snapshot_id)
            .cloned()
            .map(|binding| (snapshot_id, binding))
    }

    pub(crate) fn published_closeout(
        &self,
        capacity: Arc<PublishedSnapshotCapacityOwner>,
        snapshot_id: SnapshotId,
    ) -> Option<PublishedSnapshotCloseout> {
        self.lock_published()
            .by_id
            .contains_key(&snapshot_id)
            .then(|| PublishedSnapshotCloseout::new(&self.published, capacity, snapshot_id))
    }

    fn lock_published(&self) -> std::sync::MutexGuard<'_, PublishedSnapshotHandles> {
        self.published
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[cfg(test)]
    pub(crate) fn registry_cost_counters(&self) -> SnapshotHandleRegistryCostCounters {
        let published = self.lock_published();
        SnapshotHandleRegistryCostCounters {
            active_entries: self.active.len() as u64,
            active_key_lookups: self.active_key_lookups.load(Ordering::Relaxed),
            active_mutations: self.active_mutations.load(Ordering::Relaxed),
            published_entries: published.by_id.len() as u64,
            published_key_lookups: published.key_lookups,
            published_mutations: published.mutations,
        }
    }
}

fn remove_published_handle(
    published: &mut PublishedSnapshotHandles,
    snapshot_id: SnapshotId,
) -> Option<SnapshotHandleBinding> {
    published.key_lookups = published.key_lookups.saturating_add(1);
    let binding = published.by_id.remove(&snapshot_id)?;
    published.mutations = published.mutations.saturating_add(1);
    published.key_lookups = published.key_lookups.saturating_add(1);
    if published.by_version.get(&binding.version_id) == Some(&snapshot_id) {
        published.key_lookups = published.key_lookups.saturating_add(1);
        published.mutations = published.mutations.saturating_add(1);
        published.by_version.remove(&binding.version_id);
    }
    Some(binding)
}
