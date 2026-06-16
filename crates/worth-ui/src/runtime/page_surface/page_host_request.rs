#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostRequest {
    page_name: String,
}

impl WorthUiPageHostRequest {
    pub fn new(page_name: impl Into<String>) -> Self {
        Self {
            page_name: page_name.into(),
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }
}
