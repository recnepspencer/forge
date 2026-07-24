use std::sync::{Arc, Mutex};

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactSemanticProjection,
    WorthQueryErasedArtifactProviderResource, WorthQueryPreparedArtifactResource,
    WorthQueryRuntimeArtifactLifecycle,
};

pub(crate) struct WorthQueryRuntimeArtifactOwner {
    binding: WorthQueryRuntimeArtifactBinding,
    semantic_projection: WorthQueryArtifactSemanticProjection,
    retained_bytes: usize,
    created_thread: std::thread::ThreadId,
    resource: Mutex<Option<Box<dyn WorthQueryErasedArtifactProviderResource>>>,
    pub(super) lifecycle: Mutex<WorthQueryRuntimeArtifactLifecycle>,
}

pub(crate) struct WorthQueryRuntimeArtifactBinding {
    pub(super) contract:
        Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>,
    pub(super) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(super) operation_identity: String,
    pub(super) binding_identity: String,
    pub(super) run_identity: String,
    pub(super) producing_stage: String,
    pub(super) basis_identity: String,
    pub(super) provenance_identity: String,
    pub(super) dependency_identity: String,
    pub(super) owner_identity: String,
    pub(super) occurrence_identity: String,
}

impl WorthQueryRuntimeArtifactOwner {
    pub(super) fn register(
        binding: WorthQueryRuntimeArtifactBinding,
        prepared: WorthQueryPreparedArtifactResource,
    ) -> Arc<Self> {
        let (semantic_projection, retained_bytes, resource) = prepared.into_owner_parts();
        Arc::new(Self {
            binding,
            semantic_projection,
            retained_bytes,
            created_thread: std::thread::current().id(),
            resource: Mutex::new(Some(resource)),
            lifecycle: Mutex::new(WorthQueryRuntimeArtifactLifecycle::new(retained_bytes)),
        })
    }

    pub(super) fn binding(&self) -> &WorthQueryRuntimeArtifactBinding {
        &self.binding
    }

    pub(super) fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        &self.semantic_projection
    }

    pub(super) const fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    pub(super) fn created_thread(&self) -> std::thread::ThreadId {
        self.created_thread
    }

    pub(super) fn dispose_provider_if_required(&self, should_dispose: bool) {
        if !should_dispose {
            return;
        }
        let resource = self
            .resource
            .lock()
            .expect("artifact provider resource lock must remain available")
            .take()
            .expect("live artifact owner retains exactly one provider resource");
        resource.dispose();
    }

    pub(super) fn denial(
        &self,
        kind: WorthQueryArtifactDenialKind,
        detail: &'static str,
    ) -> WorthQueryArtifactDenial {
        WorthQueryArtifactDenial::new(
            kind,
            Some(self.binding.contract.contract().family().as_str()),
            detail,
        )
    }
}

impl Drop for WorthQueryRuntimeArtifactOwner {
    fn drop(&mut self) {
        let Some(resource) = self
            .resource
            .get_mut()
            .expect("artifact provider resource lock must remain available")
            .take()
        else {
            return;
        };
        resource.dispose();
    }
}
