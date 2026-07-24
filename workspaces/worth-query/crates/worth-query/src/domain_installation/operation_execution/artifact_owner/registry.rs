use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

use super::{
    WorthQueryArtifactDisposition, WorthQueryMoveOnlyArtifactHandle, WorthQueryRuntimeArtifactOwner,
};

pub(crate) struct WorthQueryWorkflowArtifactRegistry {
    run_identity: String,
    owners: Mutex<BTreeMap<String, Weak<WorthQueryRuntimeArtifactOwner>>>,
}

impl WorthQueryWorkflowArtifactRegistry {
    pub(crate) fn new(run_identity: String) -> Self {
        Self {
            run_identity,
            owners: Mutex::new(BTreeMap::new()),
        }
    }

    pub(crate) fn register(&self, handle: &WorthQueryMoveOnlyArtifactHandle) {
        debug_assert_eq!(
            handle.core.owner.binding().run_identity,
            self.run_identity,
            "artifact registry accepts owners from its exact workflow run"
        );
        let owner = &handle.core.owner;
        let replaced = self
            .owners
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .insert(
                owner.binding().owner_identity.clone(),
                Arc::downgrade(owner),
            );
        debug_assert!(replaced.is_none(), "artifact owner identity is unique");
    }

    pub(crate) fn close_released(&self) {
        self.close(WorthQueryArtifactDisposition::Released);
    }

    fn close(&self, disposition: WorthQueryArtifactDisposition) {
        let owners = {
            let mut registered = self
                .owners
                .lock()
                .expect("workflow artifact registry lock must remain available");
            let owners = registered
                .values()
                .filter_map(Weak::upgrade)
                .collect::<Vec<_>>();
            registered.clear();
            owners
        };
        for owner in owners {
            owner.request_registry_close(disposition);
        }
    }
}

impl Drop for WorthQueryWorkflowArtifactRegistry {
    fn drop(&mut self) {
        self.close(WorthQueryArtifactDisposition::Cancelled);
    }
}
