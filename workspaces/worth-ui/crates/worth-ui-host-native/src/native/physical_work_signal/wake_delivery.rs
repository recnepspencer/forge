use super::routing::UiNativePhysicalSignalWork;

pub(crate) struct UiNativePhysicalSignalWakeDelivery {
    ready: Vec<UiNativePhysicalSignalWork>,
}

impl UiNativePhysicalSignalWakeDelivery {
    pub(crate) fn new() -> Self {
        Self { ready: Vec::new() }
    }

    pub(crate) fn request(&mut self, work: UiNativePhysicalSignalWork) {
        if !self.ready.contains(&work) {
            self.ready.push(work);
        }
    }

    pub(crate) fn take(&mut self, work: UiNativePhysicalSignalWork) -> bool {
        let Some(index) = self.ready.iter().position(|candidate| *candidate == work) else {
            return false;
        };
        self.ready.remove(index);
        true
    }

    pub(crate) fn remove(&mut self, work: UiNativePhysicalSignalWork) {
        if let Some(index) = self.ready.iter().position(|candidate| *candidate == work) {
            self.ready.remove(index);
        }
    }

    pub(crate) fn pending(&self) -> usize {
        self.ready.len()
    }

    pub(crate) fn next(&self) -> Option<UiNativePhysicalSignalWork> {
        self.ready.first().copied()
    }

    pub(crate) fn clear(&mut self) {
        self.ready.clear();
    }
}
