use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) enum DiagnosticCode {
    Bc1001IllegalCrateName,
    Bc1002UnreservedDomain,
    Bc2001BandDependencyViolation,
    Bc2002WorthToWorthyInversion,
    Bc3001QueryImportOutsideEntry,
    Bc4001OrdinaryReplayImport,
    Bc5001RootOwnsRoad1Package,
    Bc5002SubworkspaceContractViolation,
    Bc5003SeedContractViolation,
}

impl DiagnosticCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Bc1001IllegalCrateName => "BC1001_ILLEGAL_CRATE_NAME",
            Self::Bc1002UnreservedDomain => "BC1002_UNRESERVED_DOMAIN",
            Self::Bc2001BandDependencyViolation => "BC2001_BAND_DEPENDENCY_VIOLATION",
            Self::Bc2002WorthToWorthyInversion => "BC2002_WORTH_TO_WORTHY_INVERSION",
            Self::Bc3001QueryImportOutsideEntry => "BC3001_QUERY_IMPORT_OUTSIDE_ENTRY",
            Self::Bc4001OrdinaryReplayImport => "BC4001_ORDINARY_REPLAY_IMPORT",
            Self::Bc5001RootOwnsRoad1Package => "BC5001_ROOT_OWNS_ROAD1_PACKAGE",
            Self::Bc5002SubworkspaceContractViolation => "BC5002_SUBWORKSPACE_CONTRACT_VIOLATION",
            Self::Bc5003SeedContractViolation => "BC5003_SEED_CONTRACT_VIOLATION",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Diagnostic {
    pub(crate) code: DiagnosticCode,
    pub(crate) subject: String,
    pub(crate) message: String,
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
        }
    }
}

impl Serialize for Diagnostic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Diagnostic", 3)?;
        state.serialize_field("code", self.code.as_str())?;
        state.serialize_field("subject", &self.subject)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

pub(crate) fn render_human(diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{} {}: {}",
                diagnostic.code.as_str(),
                diagnostic.subject,
                diagnostic.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn render_json(diagnostics: &[Diagnostic]) -> Result<String, String> {
    serde_json::to_string_pretty(diagnostics)
        .map_err(|e| format!("serialize diagnostics to json: {e}"))
}
