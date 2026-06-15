#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HarnessScenarioId {
    text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessScenarioIdError {
    Empty,
}

impl HarnessScenarioId {
    pub fn new(text: impl Into<String>) -> Result<Self, HarnessScenarioIdError> {
        let text = text.into();
        if text.trim().is_empty() {
            Err(HarnessScenarioIdError::Empty)
        } else {
            Ok(Self { text })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}
