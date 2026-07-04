#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyInvalidationRouteInputAdmissionError {
    detail: String,
}

impl TopologyInvalidationRouteInputAdmissionError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
