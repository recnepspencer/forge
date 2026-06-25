#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewReadinessProjectionDeclaration {
    readiness_id: String,
    required_bindings: Vec<String>,
}

impl WorthUiLiveViewReadinessProjectionDeclaration {
    pub fn new(readiness_id: impl Into<String>, required_bindings: Vec<String>) -> Self {
        Self {
            readiness_id: readiness_id.into(),
            required_bindings,
        }
    }

    pub fn readiness_id(&self) -> &str {
        &self.readiness_id
    }

    pub fn required_bindings(&self) -> &[String] {
        &self.required_bindings
    }
}
