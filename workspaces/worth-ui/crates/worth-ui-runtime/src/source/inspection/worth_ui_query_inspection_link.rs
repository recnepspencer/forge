use crate::capability::{AdmittedCapability, QueryDenialPresentation, ViewBindingId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthUiQueryInspectionLinkRole {
    BindingViewBindingQuery,
    SurfaceViewBindingQuery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiQueryInspectionLink {
    role: WorthUiQueryInspectionLinkRole,
    view_binding: AdmittedCapability<ViewBindingId>,
    definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
    denial_presentation: QueryDenialPresentation,
}

impl WorthUiQueryInspectionLink {
    pub(crate) fn new(
        role: WorthUiQueryInspectionLinkRole,
        view_binding: AdmittedCapability<ViewBindingId>,
        definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        Self {
            role,
            view_binding,
            definition,
            denial_presentation,
        }
    }

    pub(crate) fn role(&self) -> WorthUiQueryInspectionLinkRole {
        self.role
    }
    pub(crate) fn view_binding(&self) -> &AdmittedCapability<ViewBindingId> {
        &self.view_binding
    }
    pub(crate) fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        &self.definition
    }
    pub(crate) fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }
}
