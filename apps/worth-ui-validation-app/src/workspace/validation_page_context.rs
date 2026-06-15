use super::{
    ValidationDynamicPageHandle, ValidationDynamicPageInstance, ValidationDynamicPageKind,
    ValidationPageHandle, ValidationStaticPageId, ValidationWorkspaceState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ValidationResolvedPage {
    Static {
        page_id: ValidationStaticPageId,
    },
    Dynamic {
        handle: ValidationDynamicPageHandle,
        kind: ValidationDynamicPageKind,
        title: String,
        parameter_name: &'static str,
        parameter_value: String,
        authoring_page_name: &'static str,
        landing_page: ValidationStaticPageId,
    },
}

impl ValidationResolvedPage {
    pub(crate) fn from_state(state: &ValidationWorkspaceState) -> Self {
        let active_page = state.navigation().active_page();
        match active_page {
            ValidationPageHandle::Static(page_id) => Self::Static { page_id },
            ValidationPageHandle::Dynamic(handle) => {
                let page = state
                    .navigation()
                    .open_dynamic_pages()
                    .iter()
                    .find(|page| page.handle() == handle)
                    .expect("active dynamic page handle should resolve to an open page");
                Self::from_dynamic_page(page)
            }
        }
    }

    pub(crate) fn authoring_page_name(&self, state: &ValidationWorkspaceState) -> &'static str {
        match self {
            Self::Static { page_id } => ValidationPageHandle::Static(*page_id)
                .authoring_page_name(state.navigation().open_dynamic_pages()),
            Self::Dynamic {
                authoring_page_name,
                ..
            } => authoring_page_name,
        }
    }

    pub(crate) fn parameter_badge(&self) -> Option<String> {
        match self {
            Self::Static { .. } => None,
            Self::Dynamic {
                parameter_name,
                parameter_value,
                ..
            } => Some(format!("{parameter_name}={parameter_value}")),
        }
    }

    pub(crate) fn parameter_value(&self) -> Option<&str> {
        match self {
            Self::Static { .. } => None,
            Self::Dynamic {
                parameter_value, ..
            } => Some(parameter_value.as_str()),
        }
    }

    pub(crate) fn landing_page(&self) -> Option<ValidationStaticPageId> {
        match self {
            Self::Static { .. } => None,
            Self::Dynamic { landing_page, .. } => Some(*landing_page),
        }
    }

    pub(crate) fn handle(&self) -> Option<ValidationDynamicPageHandle> {
        match self {
            Self::Static { .. } => None,
            Self::Dynamic { handle, .. } => Some(*handle),
        }
    }

    fn from_dynamic_page(page: &ValidationDynamicPageInstance) -> Self {
        Self::Dynamic {
            handle: page.handle(),
            kind: page.kind(),
            title: page.title().to_owned(),
            parameter_name: page.parameter_name(),
            parameter_value: page.parameter_value().to_owned(),
            authoring_page_name: page.authoring_page_name(),
            landing_page: page.landing_page(),
        }
    }
}
