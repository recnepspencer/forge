use super::{diagnostic_code::DiagnosticCode, legal_home::LegalHome};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Clone, Debug)]
pub(crate) struct Diagnostic {
    code: DiagnosticCode,
    subject: String,
    message: String,
    legal_home: LegalHome,
}

impl Diagnostic {
    pub(crate) fn new(
        code: DiagnosticCode,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject: subject.into(),
            message: message.into(),
            legal_home: code.default_legal_home(),
        }
    }

    pub(crate) fn with_legal_home(
        code: DiagnosticCode,
        subject: impl Into<String>,
        message: impl Into<String>,
        legal_home: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject: subject.into(),
            message: message.into(),
            legal_home: LegalHome::new(legal_home).expect("diagnostic legal_home must be valid"),
        }
    }

    pub(super) fn legal_home(&self) -> &str {
        self.legal_home.as_str()
    }

    pub(crate) fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn compare_code_subject_message(&self, other: &Self) -> std::cmp::Ordering {
        (self.code.as_str(), self.subject(), self.message()).cmp(&(
            other.code.as_str(),
            other.subject(),
            other.message(),
        ))
    }

    pub(crate) fn compare_subject_message(&self, other: &Self) -> std::cmp::Ordering {
        (self.subject(), self.message()).cmp(&(other.subject(), other.message()))
    }

    pub(crate) fn has_same_code_subject_message(&self, other: &Self) -> bool {
        self.code == other.code && self.subject == other.subject && self.message == other.message
    }

    pub(crate) fn has_same_subject_message(&self, other: &Self) -> bool {
        self.subject == other.subject && self.message == other.message
    }
}

impl Serialize for Diagnostic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Diagnostic", 4)?;
        state.serialize_field("code", self.code.as_str())?;
        state.serialize_field("subject", &self.subject)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("legal_home", self.legal_home())?;
        state.end()
    }
}
