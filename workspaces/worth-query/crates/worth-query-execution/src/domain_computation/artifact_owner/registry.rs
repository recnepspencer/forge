use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactDisposition,
    WorthQueryMoveOnlyArtifactHandle, WorthQueryRuntimeArtifactOwner,
};

pub struct WorthQueryWorkflowArtifactRegistry {
    run_identity: String,
    state: Mutex<WorthQueryWorkflowArtifactRegistryState>,
}

struct WorthQueryWorkflowArtifactRegistryState {
    closed: bool,
    owners: BTreeMap<String, Weak<WorthQueryRuntimeArtifactOwner>>,
}

impl WorthQueryWorkflowArtifactRegistry {
    pub(super) fn new(run_identity: String) -> Self {
        Self {
            run_identity,
            state: Mutex::new(WorthQueryWorkflowArtifactRegistryState {
                closed: false,
                owners: BTreeMap::new(),
            }),
        }
    }

    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }

    pub fn register(
        &self,
        handle: &WorthQueryMoveOnlyArtifactHandle,
    ) -> Result<(), WorthQueryArtifactDenial> {
        if handle.core.owner.binding().run_identity != self.run_identity {
            return Err(WorthQueryArtifactDenial::new(
                WorthQueryArtifactDenialKind::RunMismatch,
                Some(
                    handle
                        .core
                        .owner
                        .binding()
                        .contract
                        .contract()
                        .family()
                        .as_str(),
                ),
                "artifact registry accepts owners from its exact workflow run",
            ));
        }
        let owner = &handle.core.owner;
        let mut state = self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available");
        if state.closed {
            return Err(closed_registry_denial());
        }
        let replaced = state.owners.insert(
            owner.binding().owner_identity.clone(),
            Arc::downgrade(owner),
        );
        debug_assert!(replaced.is_none(), "artifact owner identity is unique");
        Ok(())
    }

    pub(super) fn admit_registration(&self) -> Result<(), WorthQueryArtifactDenial> {
        if self
            .state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .closed
        {
            Err(closed_registry_denial())
        } else {
            Ok(())
        }
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.state
            .lock()
            .expect("workflow artifact registry lock must remain available")
            .closed
    }

    pub fn close_released(&self) {
        self.close(WorthQueryArtifactDisposition::Released);
    }

    pub fn close_cancelled(&self) {
        self.close(WorthQueryArtifactDisposition::Cancelled);
    }

    fn close(&self, disposition: WorthQueryArtifactDisposition) {
        let owners = {
            let mut state = self
                .state
                .lock()
                .expect("workflow artifact registry lock must remain available");
            state.closed = true;
            let owners = state
                .owners
                .values()
                .filter_map(Weak::upgrade)
                .collect::<Vec<_>>();
            state.owners.clear();
            owners
        };
        for owner in owners {
            owner.request_registry_close(disposition);
        }
    }
}

fn closed_registry_denial() -> WorthQueryArtifactDenial {
    WorthQueryArtifactDenial::new(
        WorthQueryArtifactDenialKind::AlreadyDisposed,
        None,
        "workflow artifact registry is closed",
    )
}

impl Drop for WorthQueryWorkflowArtifactRegistry {
    fn drop(&mut self) {
        self.close(WorthQueryArtifactDisposition::Cancelled);
    }
}
