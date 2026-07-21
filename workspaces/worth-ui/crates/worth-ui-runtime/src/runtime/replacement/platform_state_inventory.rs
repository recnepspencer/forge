use crate::runtime::{WorthUiDurableStateFamily, WorthUiRuntime};

impl WorthUiRuntime {
    pub(super) fn platform_durable_state_inventory(
        &self,
    ) -> crate::runtime::WorthUiDurableStateInventoryBuilder {
        self.durable_state_inventory()
            .register_platform_family(WorthUiDurableStateFamily::focus_chain())
            .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
            .register_platform_family(WorthUiDurableStateFamily::selection_range())
            .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
            .register_platform_family(WorthUiDurableStateFamily::splitter_position())
            .register_platform_family(WorthUiDurableStateFamily::tab_state())
            .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
    }
}
