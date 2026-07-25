use std::collections::BTreeMap;

const NATIVE_RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedResourceCacheDenial {
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHeadlessMountedResourceHandle(u64);

#[derive(Debug, Default)]
pub struct WorthUiHeadlessMountedResourceCache {
    binding: Option<crate::UiSurfaceBindingGeneration>,
    next_handle: u64,
    by_content: BTreeMap<u64, UiHeadlessMountedResourceHandle>,
}

impl WorthUiHeadlessMountedResourceCache {
    pub fn reconcile(
        &mut self,
        view: &super::UiMountedProjectionView,
    ) -> Result<(), WorthUiMountedResourceCacheDenial> {
        self.require_binding(view.binding());
        for resource in view.resources().entries() {
            if self.by_content.contains_key(&resource.content_identity()) {
                continue;
            }
            if self.by_content.len() >= NATIVE_RESOURCE_LIMIT {
                return Err(WorthUiMountedResourceCacheDenial::CapacityExceeded);
            }
            self.next_handle = self
                .next_handle
                .checked_add(1)
                .ok_or(WorthUiMountedResourceCacheDenial::CapacityExceeded)?;
            self.by_content.insert(
                resource.content_identity(),
                UiHeadlessMountedResourceHandle(self.next_handle),
            );
        }
        Ok(())
    }

    pub fn handle_for(&self, content_identity: u64) -> Option<UiHeadlessMountedResourceHandle> {
        self.by_content.get(&content_identity).copied()
    }

    pub fn binding(&self) -> Option<crate::UiSurfaceBindingGeneration> {
        self.binding
    }

    fn require_binding(&mut self, binding: crate::UiSurfaceBindingGeneration) {
        if self.binding == Some(binding) {
            return;
        }
        self.binding = Some(binding);
        self.by_content.clear();
    }
}
