use super::validation_page_catalog::{
    dynamic_page_template, static_page, ValidationDynamicPageKind, ValidationStaticPageId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationDynamicPageHandle(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationDynamicPageInstance {
    handle: ValidationDynamicPageHandle,
    kind: ValidationDynamicPageKind,
    parameter_name: &'static str,
    parameter_value: String,
    title: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationPageHandle {
    Static(ValidationStaticPageId),
    Dynamic(ValidationDynamicPageHandle),
}

impl ValidationDynamicPageHandle {
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl ValidationDynamicPageInstance {
    pub fn new(
        handle: ValidationDynamicPageHandle,
        kind: ValidationDynamicPageKind,
        parameter_value: impl Into<String>,
    ) -> Self {
        let template = dynamic_page_template(kind);
        let parameter_value = parameter_value.into();
        Self {
            handle,
            kind,
            parameter_name: template.parameter_name(),
            title: format!("{} {}", template.title(), parameter_value),
            parameter_value,
        }
    }

    pub fn handle(&self) -> ValidationDynamicPageHandle {
        self.handle
    }

    pub fn kind(&self) -> ValidationDynamicPageKind {
        self.kind
    }

    pub fn parameter_name(&self) -> &'static str {
        self.parameter_name
    }

    pub fn parameter_value(&self) -> &str {
        self.parameter_value.as_str()
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn authoring_page_name(&self) -> &'static str {
        dynamic_page_template(self.kind).authoring_page_name()
    }

    pub fn same_template_instance(
        &self,
        kind: ValidationDynamicPageKind,
        parameter_value: &str,
    ) -> bool {
        self.kind == kind && self.parameter_value() == parameter_value
    }

    pub fn landing_page(&self) -> ValidationStaticPageId {
        dynamic_page_template(self.kind).landing_page()
    }
}

impl ValidationPageHandle {
    pub fn title(self, dynamic_pages: &[ValidationDynamicPageInstance]) -> String {
        match self {
            Self::Static(page_id) => static_page(page_id).title().to_owned(),
            Self::Dynamic(handle) => dynamic_pages
                .iter()
                .find(|page| page.handle() == handle)
                .map(|page| page.title().to_owned())
                .expect("active dynamic page handle should resolve to an open page"),
        }
    }

    pub fn authoring_page_name(
        self,
        dynamic_pages: &[ValidationDynamicPageInstance],
    ) -> &'static str {
        match self {
            Self::Static(page_id) => static_page(page_id).authoring_page_name(),
            Self::Dynamic(handle) => dynamic_pages
                .iter()
                .find(|page| page.handle() == handle)
                .map(ValidationDynamicPageInstance::authoring_page_name)
                .expect("active dynamic page handle should resolve to an open page"),
        }
    }
}

pub(crate) fn next_dynamic_handle(counter: &mut u64) -> ValidationDynamicPageHandle {
    *counter += 1;
    ValidationDynamicPageHandle(*counter)
}
