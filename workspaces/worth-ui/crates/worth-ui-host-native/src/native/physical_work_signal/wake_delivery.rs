use worth_signal::facade::ResourceRequestHandle;

use super::routing::UiNativePhysicalSignalWork;

#[derive(Clone, Copy)]
struct UiNativePhysicalSignalReadyWork {
    work: UiNativePhysicalSignalWork,
    predecessor: Option<ResourceRequestHandle>,
}

pub(crate) struct UiNativePhysicalSignalWakeDelivery {
    ready: Vec<UiNativePhysicalSignalReadyWork>,
}

impl UiNativePhysicalSignalWakeDelivery {
    pub(crate) fn new() -> Self {
        Self { ready: Vec::new() }
    }

    pub(crate) fn request(&mut self, work: UiNativePhysicalSignalWork) {
        if !self.ready.iter().any(|ready| ready.work == work) {
            self.ready.push(UiNativePhysicalSignalReadyWork {
                work,
                predecessor: None,
            });
        }
    }

    pub(crate) fn request_successor(
        &mut self,
        work: UiNativePhysicalSignalWork,
        predecessor: ResourceRequestHandle,
    ) {
        if let Some(ready) = self.ready.iter_mut().find(|ready| ready.work == work) {
            ready.predecessor = Some(predecessor);
            return;
        }
        self.ready.push(UiNativePhysicalSignalReadyWork {
            work,
            predecessor: Some(predecessor),
        });
    }

    pub(crate) fn predecessor(
        &self,
        work: UiNativePhysicalSignalWork,
    ) -> Option<ResourceRequestHandle> {
        self.ready
            .iter()
            .find(|ready| ready.work == work)
            .and_then(|ready| ready.predecessor)
    }

    pub(crate) fn take(&mut self, work: UiNativePhysicalSignalWork) -> bool {
        let Some(index) = self
            .ready
            .iter()
            .position(|candidate| candidate.work == work)
        else {
            return false;
        };
        self.ready.remove(index);
        true
    }

    pub(crate) fn remove(&mut self, work: UiNativePhysicalSignalWork) {
        if let Some(index) = self
            .ready
            .iter()
            .position(|candidate| candidate.work == work)
        {
            self.ready.remove(index);
        }
    }

    pub(crate) fn pending(&self) -> usize {
        self.ready.len()
    }

    pub(crate) fn next(&self) -> Option<UiNativePhysicalSignalWork> {
        self.ready.first().map(|ready| ready.work)
    }

    pub(crate) fn clear(&mut self) {
        self.ready.clear();
    }
}
