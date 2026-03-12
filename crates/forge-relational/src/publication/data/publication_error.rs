use serde::{Deserialize, Serialize};

use crate::errors::data::{ErrorContext, ErrorOperation, RelationalSubsystem, SuggestedFix};

use super::PublicationStage;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationError {
    pub stage: PublicationStage,
    pub detail: String,
    pub context: ErrorContext,
}

impl PublicationError {
    pub fn new(stage: PublicationStage, detail: impl Into<String>) -> Self {
        Self {
            stage,
            detail: detail.into(),
            context: ErrorContext::new(RelationalSubsystem::Publication, ErrorOperation::Publish)
                .with_fix(SuggestedFix::InspectDiagnostics),
        }
    }
}
