use super::{
    validation_page_catalog::dynamic_page_template, validation_page_instance::next_dynamic_handle,
    ValidationDynamicPageHandle, ValidationDynamicPageInstance, ValidationDynamicPageKind,
    ValidationDynamicPageRequest, ValidationDynamicPageRequestDenial, ValidationPageHandle,
    ValidationStaticPageId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationWorkspaceNavigation {
    active_page: ValidationPageHandle,
    open_dynamic_pages: Vec<ValidationDynamicPageInstance>,
    next_handle: u64,
}

impl Default for ValidationWorkspaceNavigation {
    fn default() -> Self {
        Self {
            active_page: ValidationPageHandle::Static(ValidationStaticPageId::Overview),
            open_dynamic_pages: Vec::new(),
            next_handle: 0,
        }
    }
}

impl ValidationWorkspaceNavigation {
    pub fn active_page(&self) -> ValidationPageHandle {
        self.active_page
    }

    pub fn open_dynamic_pages(&self) -> &[ValidationDynamicPageInstance] {
        self.open_dynamic_pages.as_slice()
    }

    pub fn select_static_page(&mut self, page_id: ValidationStaticPageId) {
        self.active_page = ValidationPageHandle::Static(page_id);
    }

    pub fn select_dynamic_page(&mut self, handle: ValidationDynamicPageHandle) -> bool {
        if self
            .open_dynamic_pages
            .iter()
            .any(|page| page.handle() == handle)
        {
            self.active_page = ValidationPageHandle::Dynamic(handle);
            true
        } else {
            false
        }
    }

    pub fn open_dynamic_page(
        &mut self,
        request: ValidationDynamicPageRequest,
    ) -> Result<ValidationDynamicPageHandle, ValidationDynamicPageRequestDenial> {
        let kind = request.kind();
        let parameter_value = request.parameter_value();
        if let Some(existing) = self
            .open_dynamic_pages
            .iter()
            .find(|page| page.same_template_instance(kind, parameter_value))
        {
            self.active_page = ValidationPageHandle::Dynamic(existing.handle());
            return Ok(existing.handle());
        }

        let handle = next_dynamic_handle(&mut self.next_handle);
        let page = ValidationDynamicPageInstance::new(handle, kind, parameter_value.to_owned());
        self.active_page = ValidationPageHandle::Dynamic(handle);
        self.open_dynamic_pages.push(page);
        Ok(handle)
    }

    pub fn close_dynamic_page(&mut self, handle: ValidationDynamicPageHandle) -> bool {
        let Some(index) = self
            .open_dynamic_pages
            .iter()
            .position(|page| page.handle() == handle)
        else {
            return false;
        };

        let removed = self.open_dynamic_pages.remove(index);
        if self.active_page == ValidationPageHandle::Dynamic(handle) {
            self.active_page = ValidationPageHandle::Static(removed.landing_page());
        }
        true
    }

    pub fn active_page_title(&self) -> String {
        self.active_page.title(&self.open_dynamic_pages)
    }

    pub fn active_parameter_badge(&self) -> Option<String> {
        match self.active_page {
            ValidationPageHandle::Static(_) => None,
            ValidationPageHandle::Dynamic(handle) => self
                .open_dynamic_pages
                .iter()
                .find(|page| page.handle() == handle)
                .map(|page| format!("{}={}", page.parameter_name(), page.parameter_value())),
        }
    }

    pub fn template_landing_page(kind: ValidationDynamicPageKind) -> ValidationStaticPageId {
        dynamic_page_template(kind).landing_page()
    }
}
