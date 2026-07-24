use std::sync::Arc;

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDisposition, WorthQueryArtifactOwnerSnapshot,
    WorthQueryArtifactSemanticProjection, WorthQueryRuntimeArtifactOwner,
};

pub struct WorthQueryRetainedArtifactLease {
    pub(super) owner: Arc<WorthQueryRuntimeArtifactOwner>,
    pub(super) lifecycle_generation: u64,
    pub(super) lease_role: String,
    pub(super) active: bool,
}

impl WorthQueryRetainedArtifactLease {
    pub(super) fn admit(
        owner: &Arc<WorthQueryRuntimeArtifactOwner>,
        generation: u64,
        lease_role: impl Into<String>,
    ) -> Result<Self, WorthQueryArtifactDenial> {
        let lifecycle_generation = owner.admit_lease(generation)?;
        Ok(Self {
            owner: Arc::clone(owner),
            lifecycle_generation,
            lease_role: lease_role.into(),
            active: true,
        })
    }

    pub fn lease_role(&self) -> &str {
        &self.lease_role
    }

    pub fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        self.owner.semantic_projection()
    }

    pub fn owner_snapshot(&self) -> WorthQueryArtifactOwnerSnapshot {
        self.owner.snapshot()
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.owner.binding().occurrence_identity
    }

    pub fn release(mut self) -> bool {
        self.active = false;
        self.owner
            .release_lease(WorthQueryArtifactDisposition::Released)
    }
}

impl Drop for WorthQueryRetainedArtifactLease {
    fn drop(&mut self) {
        if self.active {
            self.owner
                .release_lease(WorthQueryArtifactDisposition::Released);
        }
    }
}
