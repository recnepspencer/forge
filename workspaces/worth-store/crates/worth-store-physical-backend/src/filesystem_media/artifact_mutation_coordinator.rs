use std::collections::HashSet;
use std::sync::{Arc, Condvar, Mutex};

use super::mutation_ownership::{CoordinatedNamespaceMutation, MutationAuthority};

#[derive(Debug, Default)]
pub(super) struct ArtifactMutationCoordinator {
    active_paths: Mutex<HashSet<String>>,
    changed: Condvar,
    #[cfg(test)]
    waiting_reservations: Mutex<usize>,
}

#[derive(Debug)]
struct ArtifactPathReservation {
    coordinator: Arc<ArtifactMutationCoordinator>,
    paths: Vec<String>,
}

#[derive(Debug)]
pub(super) struct CoordinatedArtifactMutation<'owner> {
    _ownership: MutationAuthority<'owner>,
    _paths: ArtifactPathReservation,
}

#[derive(Debug)]
pub(super) struct CoordinatedArtifactNamespaceMutation<'owner> {
    namespace: CoordinatedNamespaceMutation<'owner>,
    paths: ArtifactPathReservation,
}

impl ArtifactMutationCoordinator {
    fn reserve(self: &Arc<Self>, mut paths: Vec<String>) -> ArtifactPathReservation {
        paths.sort_unstable();
        paths.dedup();
        let mut active = self
            .active_paths
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        #[cfg(test)]
        if paths.iter().any(|path| active.contains(path)) {
            *self
                .waiting_reservations
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) += 1;
            self.changed.notify_all();
        }
        while paths.iter().any(|path| active.contains(path)) {
            active = self
                .changed
                .wait(active)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
        #[cfg(test)]
        {
            let mut waiting = self
                .waiting_reservations
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if *waiting != 0 {
                *waiting -= 1;
            }
        }
        active.extend(paths.iter().cloned());
        ArtifactPathReservation {
            coordinator: Arc::clone(self),
            paths,
        }
    }

    #[cfg(test)]
    pub(super) fn wait_until_contended(&self) {
        let mut active = self
            .active_paths
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            let waiting = *self
                .waiting_reservations
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if waiting != 0 {
                return;
            }
            active = self
                .changed
                .wait(active)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    pub(super) fn coordinate<'owner>(
        self: &Arc<Self>,
        ownership: MutationAuthority<'owner>,
        paths: Vec<String>,
    ) -> CoordinatedArtifactMutation<'owner> {
        CoordinatedArtifactMutation {
            _ownership: ownership,
            _paths: self.reserve(paths),
        }
    }

    pub(super) fn coordinate_namespace<'owner>(
        self: &Arc<Self>,
        namespace: CoordinatedNamespaceMutation<'owner>,
        paths: Vec<String>,
    ) -> CoordinatedArtifactNamespaceMutation<'owner> {
        CoordinatedArtifactNamespaceMutation {
            namespace,
            paths: self.reserve(paths),
        }
    }
}

impl<'owner> CoordinatedArtifactNamespaceMutation<'owner> {
    pub(super) fn release_namespace(self) -> CoordinatedArtifactMutation<'owner> {
        let Self { namespace, paths } = self;
        CoordinatedArtifactMutation {
            _ownership: namespace.into_ownership(),
            _paths: paths,
        }
    }
}

impl Drop for ArtifactPathReservation {
    fn drop(&mut self) {
        let mut active = self
            .coordinator
            .active_paths
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for path in &self.paths {
            active.remove(path);
        }
        self.coordinator.changed.notify_all();
    }
}
