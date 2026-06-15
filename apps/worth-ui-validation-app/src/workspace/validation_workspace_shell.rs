use egui::{Context, Ui};

use crate::runtime::{PreparedValidationWorkbenchLaunch, ValidationWorkbenchSnapshot};

use super::{
    validation_workspace_shell_renderer, ValidationDynamicPageHandle,
    ValidationDynamicPageInstance, ValidationDynamicPageRequest,
    ValidationDynamicPageRequestDenial, ValidationPageHandle, ValidationStaticPageId,
    ValidationWorkspaceNavigation, ValidationWorkspaceRestoreSnapshot, ValidationWorkspaceState,
};

pub struct ValidationWorkspaceShell {
    launch: PreparedValidationWorkbenchLaunch,
    state: ValidationWorkspaceState,
}

impl ValidationWorkspaceShell {
    pub fn from_launch(launch: PreparedValidationWorkbenchLaunch) -> Self {
        Self {
            launch,
            state: ValidationWorkspaceState::default(),
        }
    }

    pub fn from_launch_with_restore_snapshot(
        launch: PreparedValidationWorkbenchLaunch,
        restore_snapshot: ValidationWorkspaceRestoreSnapshot,
    ) -> Self {
        Self {
            launch,
            state: restore_snapshot.into_state(),
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        validation_workspace_shell_renderer::render(self, ctx);
    }

    pub fn render_page_host(&mut self, ui: &mut Ui, snapshot: ValidationWorkbenchSnapshot) {
        super::validation_page_host::render_page_host(ui, &self.launch, snapshot, &mut self.state);
    }

    pub fn snapshot(&self) -> ValidationWorkbenchSnapshot {
        self.launch.snapshot()
    }

    pub fn launch(&self) -> &PreparedValidationWorkbenchLaunch {
        &self.launch
    }

    pub fn capture_restore_snapshot(&self) -> ValidationWorkspaceRestoreSnapshot {
        ValidationWorkspaceRestoreSnapshot::capture(&self.state)
    }

    pub fn restore_from_snapshot(&mut self, restore_snapshot: ValidationWorkspaceRestoreSnapshot) {
        self.state = restore_snapshot.into_state();
    }

    pub fn navigation(&self) -> &ValidationWorkspaceNavigation {
        self.state.navigation()
    }

    pub fn active_page(&self) -> ValidationPageHandle {
        self.navigation().active_page()
    }

    pub fn open_dynamic_pages(&self) -> &[ValidationDynamicPageInstance] {
        self.navigation().open_dynamic_pages()
    }

    pub fn rail_width(&self) -> f32 {
        self.state.rail_width()
    }

    pub fn inspector_width(&self) -> f32 {
        self.state.inspector_width()
    }

    pub fn status_height(&self) -> f32 {
        self.state.status_height()
    }

    pub fn select_static_page(&mut self, page_id: ValidationStaticPageId) {
        self.state.navigation_mut().select_static_page(page_id);
    }

    pub fn open_dynamic_page(
        &mut self,
        request: ValidationDynamicPageRequest,
    ) -> Result<ValidationDynamicPageHandle, ValidationDynamicPageRequestDenial> {
        self.state.navigation_mut().open_dynamic_page(request)
    }

    pub fn select_dynamic_page(&mut self, handle: ValidationDynamicPageHandle) -> bool {
        self.state.navigation_mut().select_dynamic_page(handle)
    }

    pub fn close_dynamic_page(&mut self, handle: ValidationDynamicPageHandle) -> bool {
        self.state.navigation_mut().close_dynamic_page(handle)
    }

    pub fn set_rail_width(&mut self, width: f32) {
        self.state.set_rail_width(width);
    }

    pub fn set_inspector_width(&mut self, width: f32) {
        self.state.set_inspector_width(width);
    }

    pub fn set_status_height(&mut self, height: f32) {
        self.state.set_status_height(height);
    }

    pub(crate) fn state(&self) -> &ValidationWorkspaceState {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut ValidationWorkspaceState {
        &mut self.state
    }
}
