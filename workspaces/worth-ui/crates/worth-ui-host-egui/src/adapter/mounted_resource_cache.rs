use std::collections::BTreeMap;

const NATIVE_RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEguiMountedResourceHandle(egui::Id);

#[derive(Debug, Default)]
pub struct WorthUiEguiMountedResourceCache {
    binding: Option<worth_ui_host_contract::UiSurfaceBindingGeneration>,
    by_content: BTreeMap<u64, UiEguiMountedResourceHandle>,
}

impl WorthUiEguiMountedResourceCache {
    pub fn reconcile(
        &mut self,
        view: &worth_ui_host_contract::UiMountedProjectionView,
    ) -> Result<(), worth_ui_host_contract::WorthUiMountedResourceCacheDenial> {
        self.require_binding(view.binding());
        for resource in view.resources().entries() {
            if self.by_content.contains_key(&resource.content_identity()) {
                continue;
            }
            if self.by_content.len() >= NATIVE_RESOURCE_LIMIT {
                return Err(
                    worth_ui_host_contract::WorthUiMountedResourceCacheDenial::CapacityExceeded,
                );
            }
            let handle = UiEguiMountedResourceHandle(egui::Id::new((
                "worth-ui-mounted-resource",
                view.binding(),
                resource.content_identity(),
            )));
            self.by_content.insert(resource.content_identity(), handle);
        }
        Ok(())
    }

    pub fn handle_for(&self, content_identity: u64) -> Option<UiEguiMountedResourceHandle> {
        self.by_content.get(&content_identity).copied()
    }

    pub fn binding(&self) -> Option<worth_ui_host_contract::UiSurfaceBindingGeneration> {
        self.binding
    }

    fn require_binding(&mut self, binding: worth_ui_host_contract::UiSurfaceBindingGeneration) {
        if self.binding == Some(binding) {
            return;
        }
        self.binding = Some(binding);
        self.by_content.clear();
    }
}
