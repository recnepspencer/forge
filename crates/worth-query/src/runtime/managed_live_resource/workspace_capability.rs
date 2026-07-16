use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{WorthQueryLiveView, WorthQueryUnrefinedLiveShape};

#[derive(Debug)]
pub(crate) struct WorthQueryAbandonedManagedLiveResource {
    view: WorthQueryLiveView<WorthQueryUnrefinedLiveShape>,
}

impl WorthQueryAbandonedManagedLiveResource {
    pub(crate) fn new(view: WorthQueryLiveView<WorthQueryUnrefinedLiveShape>) -> Self {
        Self { view }
    }

    pub(crate) fn view(&self) -> &WorthQueryLiveView<WorthQueryUnrefinedLiveShape> {
        &self.view
    }

    pub(crate) fn name(&self) -> &str {
        self.view.name()
    }
}

#[derive(Debug, Default)]
pub(crate) struct WorthQueryManagedLiveWorkspaceCapability {
    abandoned_resources: Mutex<Vec<WorthQueryAbandonedManagedLiveResource>>,
}

impl WorthQueryManagedLiveWorkspaceCapability {
    pub(crate) fn shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub(crate) fn same_instance(left: &Arc<Self>, right: &Arc<Self>) -> bool {
        Arc::ptr_eq(left, right)
    }

    pub(crate) fn abandon(&self, view: WorthQueryLiveView<WorthQueryUnrefinedLiveShape>) {
        self.abandoned_resources()
            .push(WorthQueryAbandonedManagedLiveResource::new(view));
    }

    pub(crate) fn take_abandoned(&self) -> Vec<WorthQueryAbandonedManagedLiveResource> {
        std::mem::take(&mut *self.abandoned_resources())
    }

    pub(crate) fn restore_abandoned(
        &self,
        mut resources: Vec<WorthQueryAbandonedManagedLiveResource>,
    ) {
        let mut abandoned_resources = self.abandoned_resources();
        resources.append(&mut abandoned_resources);
        *abandoned_resources = resources;
    }

    pub(crate) fn contains_name(&self, name: &str) -> bool {
        self.abandoned_resources()
            .iter()
            .any(|resource| resource.name() == name)
    }

    fn abandoned_resources(&self) -> MutexGuard<'_, Vec<WorthQueryAbandonedManagedLiveResource>> {
        self.abandoned_resources
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
