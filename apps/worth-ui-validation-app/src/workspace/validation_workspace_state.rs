use super::ValidationWorkspaceNavigation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidationWorkspaceToast {
    message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidationWorkspaceState {
    navigation: ValidationWorkspaceNavigation,
    rail_width: f32,
    inspector_width: f32,
    status_height: f32,
    command_palette_open: bool,
    toasts: Vec<ValidationWorkspaceToast>,
}

impl Default for ValidationWorkspaceState {
    fn default() -> Self {
        Self {
            navigation: ValidationWorkspaceNavigation::default(),
            rail_width: 240.0,
            inspector_width: 320.0,
            status_height: 132.0,
            command_palette_open: false,
            toasts: Vec::new(),
        }
    }
}

impl ValidationWorkspaceState {
    pub(crate) fn navigation(&self) -> &ValidationWorkspaceNavigation {
        &self.navigation
    }

    pub(crate) fn navigation_mut(&mut self) -> &mut ValidationWorkspaceNavigation {
        &mut self.navigation
    }

    pub(crate) fn rail_width(&self) -> f32 {
        self.rail_width
    }

    pub(crate) fn inspector_width(&self) -> f32 {
        self.inspector_width
    }

    pub(crate) fn status_height(&self) -> f32 {
        self.status_height
    }

    pub(crate) fn command_palette_open(&self) -> bool {
        self.command_palette_open
    }

    pub(crate) fn toasts(&self) -> &[ValidationWorkspaceToast] {
        self.toasts.as_slice()
    }

    pub(crate) fn set_rail_width(&mut self, width: f32) {
        self.rail_width = width.clamp(180.0, 420.0);
    }

    pub(crate) fn set_inspector_width(&mut self, width: f32) {
        self.inspector_width = width.clamp(220.0, 480.0);
    }

    pub(crate) fn set_status_height(&mut self, height: f32) {
        self.status_height = height.clamp(88.0, 240.0);
    }

    pub(crate) fn toggle_command_palette(&mut self) {
        self.command_palette_open = !self.command_palette_open;
    }

    pub(crate) fn close_command_palette(&mut self) {
        self.command_palette_open = false;
    }

    pub(crate) fn push_toast(&mut self, message: impl Into<String>) {
        self.toasts.push(ValidationWorkspaceToast {
            message: message.into(),
        });
        if self.toasts.len() > 3 {
            self.toasts.remove(0);
        }
    }
}

impl ValidationWorkspaceToast {
    pub(crate) fn message(&self) -> &str {
        self.message.as_str()
    }
}
