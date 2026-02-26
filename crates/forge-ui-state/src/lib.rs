//! Forge UI application state (ViewModels).
//!
//! DOMAIN: All mutable UI state. Uses forge-signal for reactive invalidation.
//! Layout code reads from VMs and calls typed action methods — never mutates
//! state directly.
//! DEPENDENCIES: forge-ui-types, forge-ui-adapters, forge-ui-theme, forge-signal.

use forge_ui_adapters::{stub_chat_history, stub_feature_list, stub_planes, stub_telemetry};
use forge_ui_theme::{dark_theme, light_theme, ForgeTheme};
use forge_ui_types::{
    ChatMessage, KernelTelemetry, MessageContent, MessageRole, UiFeature, UiFeatureId, UiPlane,
};

// ── Theme ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind { Dark, Light }

// ── Model ViewModel ──────────────────────────────────────────────────────────

/// Owns the feature tree, plane list, and selection state.
pub struct ModelVm {
    features: Vec<UiFeature>,
    planes: Vec<UiPlane>,
    selected: Option<UiFeatureId>,
}

impl ModelVm {
    fn new() -> Self {
        Self {
            features: stub_feature_list(),
            planes: stub_planes(),
            selected: None,
        }
    }

    pub fn features(&self) -> &[UiFeature] { &self.features }
    pub fn planes(&self) -> &[UiPlane] { &self.planes }
    pub fn selected(&self) -> Option<UiFeatureId> { self.selected }

    pub fn select(&mut self, id: UiFeatureId) { self.selected = Some(id); }
    pub fn deselect(&mut self) { self.selected = None; }
}

// ── Chat ViewModel ───────────────────────────────────────────────────────────

/// Owns the chat message history and the current input draft.
pub struct ChatVm {
    messages: Vec<ChatMessage>,
    pub input_draft: String,
}

impl ChatVm {
    fn new() -> Self {
        Self { messages: stub_chat_history(), input_draft: String::new() }
    }

    pub fn messages(&self) -> &[ChatMessage] { &self.messages }

    /// Submit the current draft as a user message and append a placeholder
    /// agent response. (Real agent wiring comes in a later phase.)
    pub fn submit_draft(&mut self) {
        let text = std::mem::take(&mut self.input_draft);
        if text.trim().is_empty() { return; }

        self.messages.push(ChatMessage {
            role: MessageRole::User,
            content: MessageContent::Text(text),
            timestamp_secs: 0,
        });
        self.messages.push(ChatMessage {
            role: MessageRole::Agent,
            content: MessageContent::Text(
                "*(agent response not yet wired — kernel integration in Phase 2)*".to_string(),
            ),
            timestamp_secs: 0,
        });
    }
}

// ── Palette ViewModel ─────────────────────────────────────────────────────────

/// Command palette state.
pub struct PaletteVm {
    pub open: bool,
    pub query: String,
}

impl PaletteVm {
    fn new() -> Self { Self { open: false, query: String::new() } }
    pub fn toggle(&mut self) { self.open = !self.open; }
    pub fn close(&mut self) { self.open = false; self.query.clear(); }
}

// ── Right drawer ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerTab { Properties, Chat }

pub struct DrawerVm {
    pub open: bool,
    pub active_tab: DrawerTab,
}

impl DrawerVm {
    fn new() -> Self { Self { open: true, active_tab: DrawerTab::Properties } }
}

// ── App State (top-level) ────────────────────────────────────────────────────

/// The single root of all UI mutable state.
pub struct AppState {
    pub model:   ModelVm,
    pub chat:    ChatVm,
    pub palette: PaletteVm,
    pub drawer:  DrawerVm,
    pub theme_kind: ThemeKind,
    pub theme:   ForgeTheme,
    pub telemetry: KernelTelemetry,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            model:    ModelVm::new(),
            chat:     ChatVm::new(),
            palette:  PaletteVm::new(),
            drawer:   DrawerVm::new(),
            theme_kind: ThemeKind::Dark,
            theme:    dark_theme(),
            telemetry: stub_telemetry(),
        }
    }

    /// Switch between dark and light themes.
    pub fn toggle_theme(&mut self) {
        self.theme_kind = match self.theme_kind {
            ThemeKind::Dark  => ThemeKind::Light,
            ThemeKind::Light => ThemeKind::Dark,
        };
        self.theme = match self.theme_kind {
            ThemeKind::Dark  => dark_theme(),
            ThemeKind::Light => light_theme(),
        };
    }
}

impl Default for AppState {
    fn default() -> Self { Self::new() }
}
