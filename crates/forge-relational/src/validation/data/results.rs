use serde::{Deserialize, Serialize};

use super::execution::InvariantClass;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantViolation {
    pub class: InvariantClass,
    pub code: crate::diagnostics::data::DiagnosticCode,
    pub detail: String,
}
