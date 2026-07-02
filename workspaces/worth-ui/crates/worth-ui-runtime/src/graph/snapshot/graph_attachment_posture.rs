#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphAttachmentPosture {
    query_binding_attached: bool,
    service_usage_attached: bool,
}

impl UiGraphAttachmentPosture {
    pub(crate) const fn new(query_binding_attached: bool, service_usage_attached: bool) -> Self {
        Self {
            query_binding_attached,
            service_usage_attached,
        }
    }

    pub fn query_binding_attached(self) -> bool {
        self.query_binding_attached
    }

    pub fn service_usage_attached(self) -> bool {
        self.service_usage_attached
    }
}
