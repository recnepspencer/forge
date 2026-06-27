use forge_query::facade::{
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessStatus,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

/// Structured command readiness metadata for later runtime projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandReadinessBinding {
    source: CommandReadinessSource,
}

impl CommandReadinessBinding {
    pub fn always_admitted() -> Self {
        Self {
            source: CommandReadinessSource::StaticStatus(
                ForgeQueryDeclarationEntryReadinessStatus::Admitted,
            ),
        }
    }

    pub fn from_query_readiness_status(status: ForgeQueryDeclarationEntryReadinessStatus) -> Self {
        Self {
            source: CommandReadinessSource::StaticStatus(status),
        }
    }

    pub fn from_query_readiness_report<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        report: &ForgeQueryDeclarationEntryReadinessReport<D, I>,
    ) -> Self {
        Self {
            source: CommandReadinessSource::QueryReport {
                declaration_family_key: report.declaration_family_key(),
                readiness_digest: report.readiness_digest().to_owned(),
                strongest_status: strongest_readiness_status(report),
            },
        }
    }

    pub fn strongest_status(&self) -> ForgeQueryDeclarationEntryReadinessStatus {
        match &self.source {
            CommandReadinessSource::StaticStatus(status) => *status,
            CommandReadinessSource::QueryReport {
                strongest_status, ..
            } => *strongest_status,
        }
    }

    pub fn readiness_digest(&self) -> Option<&str> {
        match &self.source {
            CommandReadinessSource::StaticStatus(_) => None,
            CommandReadinessSource::QueryReport {
                readiness_digest, ..
            } => Some(readiness_digest),
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match &self.source {
            CommandReadinessSource::StaticStatus(status) => {
                format!("static:{}", status.as_str())
            }
            CommandReadinessSource::QueryReport {
                declaration_family_key,
                readiness_digest,
                strongest_status,
            } => format!(
                "query:{declaration_family_key}:{}:{readiness_digest}",
                strongest_status.as_str()
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CommandReadinessSource {
    StaticStatus(ForgeQueryDeclarationEntryReadinessStatus),
    QueryReport {
        declaration_family_key: &'static str,
        readiness_digest: String,
        strongest_status: ForgeQueryDeclarationEntryReadinessStatus,
    },
}

fn strongest_readiness_status<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    report: &ForgeQueryDeclarationEntryReadinessReport<D, I>,
) -> ForgeQueryDeclarationEntryReadinessStatus {
    report
        .rows()
        .iter()
        .map(|row| row.status())
        .max_by_key(|status| readiness_status_rank(*status))
        .unwrap_or(ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis)
}

fn readiness_status_rank(status: ForgeQueryDeclarationEntryReadinessStatus) -> u8 {
    match status {
        ForgeQueryDeclarationEntryReadinessStatus::Admitted => 0,
        ForgeQueryDeclarationEntryReadinessStatus::Deferred => 1,
        ForgeQueryDeclarationEntryReadinessStatus::Unsupported => 2,
        ForgeQueryDeclarationEntryReadinessStatus::InvalidBasis => 3,
    }
}
