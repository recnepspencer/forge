use super::validation::{is_machine_token_only, normalize_human_text};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanReadableResponse {
    summary: String,
}

impl HumanReadableResponse {
    pub(crate) fn from_source_summary(summary: impl Into<String>) -> Self {
        let summary = normalize_human_text(summary);
        if is_machine_token_only(&summary) {
            Self {
                summary: format!(
                    "A product-facing explanation is required for internal reason token {summary}."
                ),
            }
        } else {
            Self { summary }
        }
    }

    pub fn new(summary: impl Into<String>) -> Result<Self, HumanReadableResponseError> {
        let summary = normalize_human_text(summary);
        if is_machine_token_only(&summary) {
            return Err(HumanReadableResponseError::MachineTokenOnly);
        }
        Ok(Self { summary })
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HumanReadableResponseError {
    MachineTokenOnly,
}
