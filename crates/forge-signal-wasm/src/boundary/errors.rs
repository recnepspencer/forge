use serde::Serialize;
use wasm_bindgen::JsValue;

use forge_signal::facade::SignalError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgeSignalJsError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl ForgeSignalJsError {
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self {
            code: "invalidInput".to_owned(),
            message: message.into(),
            context: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "internal".to_owned(),
            message: message.into(),
            context: None,
        }
    }
}

impl From<SignalError> for ForgeSignalJsError {
    fn from(value: SignalError) -> Self {
        match value {
            SignalError::InvalidInput { message, context } => Self {
                code: "invalidInput".to_owned(),
                message,
                context,
            },
            SignalError::CycleDetected { .. } => Self {
                code: "cycleDetected".to_owned(),
                message: value.to_string(),
                context: None,
            },
            SignalError::IncompatibleSnapshot { reason } => Self {
                code: "incompatibleSnapshot".to_owned(),
                message: reason,
                context: None,
            },
            SignalError::UnknownBranch { .. } => Self {
                code: "unknownBranch".to_owned(),
                message: value.to_string(),
                context: None,
            },
            SignalError::BranchMergeFailed { message, .. } => Self {
                code: "branchMergeFailed".to_owned(),
                message,
                context: None,
            },
            SignalError::Internal { message, context } => Self {
                code: "internal".to_owned(),
                message,
                context,
            },
            other => Self {
                code: "signalError".to_owned(),
                message: other.to_string(),
                context: None,
            },
        }
    }
}

impl From<ForgeSignalJsError> for JsValue {
    fn from(value: ForgeSignalJsError) -> Self {
        serde_wasm_bindgen::to_value(&value)
            .unwrap_or_else(|_| JsValue::from_str("forge-signal wasm error"))
    }
}
