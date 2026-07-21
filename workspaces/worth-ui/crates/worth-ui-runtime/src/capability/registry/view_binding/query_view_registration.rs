use worth_ui_query_binding::compatibility::managed_live::WorthUiInstalledLiveQueryView;
use worth_ui_query_binding::{WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView};

use super::{QueryDenialPresentation, VisibleStateBindingDeclaration};

/// One installed Query view plus the UI-owned presentation attached to it.
/// Query semantics and runtime authority come only from `view`; presentation
/// methods cannot redefine result shape, basis, lifecycle, or projection law.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryViewRegistration {
    view: WorthUiInstalledQueryView,
    visible_state_bindings: Vec<VisibleStateBindingDeclaration>,
    denial_presentation: QueryDenialPresentation,
}

impl WorthUiQueryViewRegistration {
    pub fn new(view: impl Into<WorthUiInstalledQueryView>) -> Self {
        Self {
            view: view.into(),
            visible_state_bindings: Vec::new(),
            denial_presentation: QueryDenialPresentation::StructuredStatus,
        }
    }

    pub fn with_visible_state_binding(mut self, binding: VisibleStateBindingDeclaration) -> Self {
        self.visible_state_bindings.push(binding);
        self
    }

    pub fn with_denial_presentation(mut self, presentation: QueryDenialPresentation) -> Self {
        self.denial_presentation = presentation;
        self
    }

    pub fn view(&self) -> &WorthUiInstalledQueryView {
        &self.view
    }

    pub fn visible_state_bindings(&self) -> &[VisibleStateBindingDeclaration] {
        &self.visible_state_bindings
    }

    pub fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiInstalledQueryView,
        Vec<VisibleStateBindingDeclaration>,
        QueryDenialPresentation,
    ) {
        (
            self.view,
            self.visible_state_bindings,
            self.denial_presentation,
        )
    }
}

impl From<WorthUiInstalledQueryView> for WorthUiQueryViewRegistration {
    fn from(view: WorthUiInstalledQueryView) -> Self {
        Self::new(view)
    }
}

impl From<WorthUiInstalledSnapshotQueryView> for WorthUiQueryViewRegistration {
    fn from(view: WorthUiInstalledSnapshotQueryView) -> Self {
        Self::new(view)
    }
}

impl From<WorthUiInstalledLiveQueryView> for WorthUiQueryViewRegistration {
    fn from(view: WorthUiInstalledLiveQueryView) -> Self {
        Self::new(view)
    }
}
