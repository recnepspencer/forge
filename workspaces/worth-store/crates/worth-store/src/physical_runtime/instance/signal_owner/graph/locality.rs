use std::collections::{HashMap, HashSet};

use worth_signal::facade::{RawCompletionEnvelope, ResourceRequestHandle};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, PhysicalSignalAspectBindingDigest, PhysicalWorkIdentity,
    PhysicalWorkScope,
};

pub(super) struct PhysicalSignalLocalityIndex {
    entries: HashMap<PhysicalWorkIdentity, PhysicalSignalLocalityEntry>,
    by_artifact: HashMap<RecordArtifactFile, HashSet<PhysicalWorkIdentity>>,
    capacity: usize,
}

struct PhysicalSignalLocalityEntry {
    scope: PhysicalWorkScope,
    binding: PhysicalSignalAspectBindingDigest,
    signal: Option<ResourceRequestHandle>,
    invalidated: bool,
}

impl PhysicalSignalLocalityIndex {
    pub(super) fn bounded(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
            by_artifact: HashMap::new(),
            capacity,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(super) fn register(&mut self, admitted: &AdmittedPhysicalWork) -> bool {
        let identity = admitted.intent().identity();
        if let Some(entry) = self.entries.get(&identity) {
            return entry.scope == *admitted.intent().scope()
                && entry.binding == admitted.authority().binding();
        }
        if self.entries.len() == self.capacity {
            return false;
        }
        let scope = admitted.intent().scope().clone();
        for coordinate in scope.coordinates() {
            self.by_artifact
                .entry(coordinate.artifact())
                .or_default()
                .insert(identity);
        }
        self.entries.insert(
            identity,
            PhysicalSignalLocalityEntry {
                scope,
                binding: admitted.authority().binding(),
                signal: None,
                invalidated: false,
            },
        );
        true
    }

    pub(super) fn bind(
        &mut self,
        identity: PhysicalWorkIdentity,
        signal: ResourceRequestHandle,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&identity) else {
            return false;
        };
        entry.signal = Some(signal);
        entry.invalidated = false;
        true
    }

    pub(super) fn invalidated(&self, identity: PhysicalWorkIdentity) -> bool {
        self.entries
            .get(&identity)
            .is_some_and(|entry| entry.invalidated)
    }

    pub(super) fn invalidate_scope(
        &mut self,
        binding: PhysicalSignalAspectBindingDigest,
        changed: &PhysicalWorkScope,
    ) {
        let mut candidates = HashSet::new();
        for coordinate in changed.coordinates() {
            if let Some(identities) = self.by_artifact.get(&coordinate.artifact()) {
                candidates.extend(identities.iter().copied());
            }
        }
        for identity in candidates {
            let Some(entry) = self.entries.get_mut(&identity) else {
                continue;
            };
            if entry.binding == binding && scopes_overlap(&entry.scope, changed) {
                entry.invalidated = true;
            }
        }
    }

    pub(super) fn signal(&self, identity: PhysicalWorkIdentity) -> Option<ResourceRequestHandle> {
        self.entries.get(&identity).and_then(|entry| entry.signal)
    }

    pub(super) fn release_identity(&mut self, identity: PhysicalWorkIdentity) {
        let Some(entry) = self.entries.remove(&identity) else {
            return;
        };
        for coordinate in entry.scope.coordinates() {
            let artifact = coordinate.artifact();
            let remove_artifact = self
                .by_artifact
                .get_mut(&artifact)
                .is_some_and(|identities| {
                    identities.remove(&identity);
                    identities.is_empty()
                });
            if remove_artifact {
                self.by_artifact.remove(&artifact);
            }
        }
    }

    pub(super) fn release_signal(&mut self, signal: ResourceRequestHandle) {
        let identity = self
            .entries
            .iter()
            .find_map(|(identity, entry)| (entry.signal == Some(signal)).then_some(*identity));
        if let Some(identity) = identity {
            self.release_identity(identity);
        }
    }

    pub(super) fn release_envelope(&mut self, envelope: &RawCompletionEnvelope) {
        let identity = self.entries.iter().find_map(|(identity, entry)| {
            entry
                .signal
                .is_some_and(|signal| {
                    signal.request_id() == envelope.request_id()
                        && signal.generation() == envelope.generation()
                        && signal.branch_epoch() == envelope.branch_epoch()
                })
                .then_some(*identity)
        });
        if let Some(identity) = identity {
            self.release_identity(identity);
        }
    }
}

fn scopes_overlap(left: &PhysicalWorkScope, right: &PhysicalWorkScope) -> bool {
    let mut left_index = 0;
    let mut right_index = 0;
    let left = left.coordinates();
    let right = right.coordinates();
    while left_index < left.len() && right_index < right.len() {
        let l = left[left_index];
        let r = right[right_index];
        if ranges_overlap(l, r) {
            return true;
        }
        if coordinate_end(l) <= r.offset() || l.artifact() < r.artifact() {
            left_index += 1;
        } else {
            right_index += 1;
        }
    }
    false
}

fn ranges_overlap(left: RecordFrameCoordinate, right: RecordFrameCoordinate) -> bool {
    left.artifact() == right.artifact()
        && left.offset() < coordinate_end(right)
        && right.offset() < coordinate_end(left)
}

fn coordinate_end(coordinate: RecordFrameCoordinate) -> u64 {
    coordinate
        .offset()
        .saturating_add(u64::from(coordinate.length()))
}
