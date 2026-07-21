use std::rc::Rc;

/// Compact plan-owned route to one retained Query settlement fact.
///
/// The installed reference locates the binding-owned slot. Active application
/// generation authority is attached only when the session exposes a lane link.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQuerySettledFactLink {
    installed_reference: Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
}

impl WorthUiQuerySettledFactLink {
    pub(crate) fn seal(
        installed_reference: worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    ) -> Self {
        Self {
            installed_reference: Rc::new(installed_reference),
        }
    }

    pub fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        self.installed_reference.definition()
    }

    pub(crate) fn installed_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }

    #[cfg(test)]
    pub(crate) fn with_installed_reference_for_test(
        &self,
        installed_reference: worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    ) -> Self {
        Self {
            installed_reference: Rc::new(installed_reference),
        }
    }
}
