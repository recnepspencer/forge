use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPublicBridgeForbiddenAccessPattern {
    PublishedBinding,
    MaterializationByName,
    MaterializationRows,
}

impl ForgeQueryPublicBridgeForbiddenAccessPattern {
    pub fn needle(&self) -> &'static str {
        match self {
            Self::PublishedBinding => "published_binding",
            Self::MaterializationByName => "materialization_by_name",
            Self::MaterializationRows => ".rows(",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PublishedBinding => "published_binding",
            Self::MaterializationByName => "materialization_by_name",
            Self::MaterializationRows => "materialization_rows",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicBridgeForbiddenAccessFinding {
    path: String,
    line: usize,
    pattern: ForgeQueryPublicBridgeForbiddenAccessPattern,
    matched_text: String,
}

impl ForgeQueryPublicBridgeForbiddenAccessFinding {
    fn new(
        path: impl Into<String>,
        line: usize,
        pattern: ForgeQueryPublicBridgeForbiddenAccessPattern,
        matched_text: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            line,
            pattern,
            matched_text: matched_text.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn pattern(&self) -> ForgeQueryPublicBridgeForbiddenAccessPattern {
        self.pattern
    }

    pub fn matched_text(&self) -> &str {
        &self.matched_text
    }

    pub fn localized_pattern(&self) -> String {
        format!("{}:{}:{}", self.path, self.line, self.pattern.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicBridgeReaderLaneInventory {
    paths: Vec<String>,
    forbidden_findings: Vec<ForgeQueryPublicBridgeForbiddenAccessFinding>,
    digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPublicBridgeReaderLaneInventory {
    pub fn scan<'a>(sources: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        let mut paths = Vec::new();
        let mut forbidden_findings = Vec::new();
        for (path, source) in sources {
            paths.push(path.to_string());
            forbidden_findings.extend(scan_source(path, source));
        }
        let digest = inventory_digest(&paths, &forbidden_findings);
        Self {
            paths,
            forbidden_findings,
            digest,
        }
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    pub fn forbidden_findings(&self) -> &[ForgeQueryPublicBridgeForbiddenAccessFinding] {
        &self.forbidden_findings
    }

    pub fn direct_materialization_read_count(&self) -> usize {
        self.forbidden_findings.len()
    }

    pub fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}

fn scan_source(path: &str, source: &str) -> Vec<ForgeQueryPublicBridgeForbiddenAccessFinding> {
    source
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            forbidden_patterns()
                .iter()
                .copied()
                .filter(move |pattern| line.contains(pattern.needle()))
                .map(move |pattern| {
                    ForgeQueryPublicBridgeForbiddenAccessFinding::new(
                        path,
                        line_index + 1,
                        pattern,
                        line.trim(),
                    )
                })
        })
        .collect()
}

fn forbidden_patterns() -> &'static [ForgeQueryPublicBridgeForbiddenAccessPattern] {
    &[
        ForgeQueryPublicBridgeForbiddenAccessPattern::PublishedBinding,
        ForgeQueryPublicBridgeForbiddenAccessPattern::MaterializationByName,
        ForgeQueryPublicBridgeForbiddenAccessPattern::MaterializationRows,
    ]
}

fn inventory_digest(
    paths: &[String],
    findings: &[ForgeQueryPublicBridgeForbiddenAccessFinding],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("inventory_path"),
            paths.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("forbidden_access"),
            findings
                .iter()
                .map(ForgeQueryPublicBridgeForbiddenAccessFinding::localized_pattern),
        )
        .seal()
}
