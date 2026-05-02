use super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    S0ArtifactValidationCostSurface, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::capability::Roadmap2SequenceId;
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::manifest::S0AuditInputManifest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const TERMINOLOGY_RISK_PHRASES: [&str; 14] = [
    "production-grade",
    "platform-grade",
    "database",
    "embedded backend",
    "wal",
    "crash recovery",
    "durability",
    "physical",
    "blob",
    "replication",
    "certification",
    "bounded",
    "integrity",
    "repair",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyScanScope {
    path: String,
}

impl TerminologyScanScope {
    pub fn new(path: impl Into<String>) -> Result<Self, TerminologyCleanupRejection> {
        let path = normalize_relative_path(path)?;
        if path == "." || path == "./" || path.contains("..") {
            return Err(TerminologyCleanupRejection::RejectedWorkspaceGlobalScope);
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyScanPlan {
    scopes: Vec<TerminologyScanScope>,
}

impl TerminologyScanPlan {
    pub fn new(scopes: Vec<TerminologyScanScope>) -> Result<Self, TerminologyCleanupRejection> {
        if scopes.is_empty() {
            return Err(TerminologyCleanupRejection::MissingScanScope);
        }
        let mut seen = BTreeSet::new();
        if scopes.iter().any(|scope| !seen.insert(scope.path())) {
            return Err(TerminologyCleanupRejection::DuplicateScanScope);
        }
        Ok(Self { scopes })
    }

    pub fn scopes(&self) -> &[TerminologyScanScope] {
        &self.scopes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyScanInputFile {
    path: String,
    contents: String,
}

impl TerminologyScanInputFile {
    pub fn new(
        path: impl Into<String>,
        contents: impl Into<String>,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let path = normalize_relative_path(path)?;
        Ok(Self {
            path,
            contents: contents.into(),
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TerminologyAllowedUse {
    AllowedSemanticUse,
    QualifiedPhysicalDebt {
        deferred_sequence: Roadmap2SequenceId,
    },
    ClosedFoundationEvidence {
        evidence_ref: S0EvidenceRef,
    },
    OverclaimedPhysicalPosture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyAllowlistEntry {
    path: String,
    line_number: u64,
    phrase: String,
    allowed_use: TerminologyAllowedUse,
}

impl TerminologyAllowlistEntry {
    pub fn new(
        path: impl Into<String>,
        line_number: u64,
        phrase: impl Into<String>,
        allowed_use: TerminologyAllowedUse,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let path = normalize_relative_path(path)?;
        let phrase = phrase.into();
        if phrase.trim().is_empty() {
            return Err(TerminologyCleanupRejection::EmptyRequiredField);
        }
        if line_number == 0 {
            return Err(TerminologyCleanupRejection::InvalidLineNumber);
        }
        if matches!(
            allowed_use,
            TerminologyAllowedUse::QualifiedPhysicalDebt { .. }
        ) && !phrase_requires_qualification(&phrase)
        {
            return Err(TerminologyCleanupRejection::QualifierAppliedToNonRiskPhrase);
        }
        Ok(Self {
            path,
            line_number,
            phrase: phrase.to_ascii_lowercase(),
            allowed_use,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminologyRequiredQualifier {
    SemanticOnly,
    NamesDeferredSequence,
    ReferencesClosedFoundationEvidence,
    RejectAsOverclaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyPhraseFinding {
    row_id: S0ArtifactRowId,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<S0EvidenceRef>,
    forbidden_claims: Vec<super::capability::BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    status: S0ArtifactRowStatus,
    notes: String,
    phrase: String,
    line_number: u64,
    line_excerpt: String,
    allowed_use: TerminologyAllowedUse,
    required_qualifier: TerminologyRequiredQualifier,
}

impl TerminologyPhraseFinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_path_or_symbol: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        phrase: impl Into<String>,
        line_number: u64,
        line_excerpt: impl Into<String>,
        allowed_use: TerminologyAllowedUse,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let subject_path_or_symbol = subject_path_or_symbol.into();
        let notes = notes.into();
        let phrase = phrase.into().to_ascii_lowercase();
        let line_excerpt = line_excerpt.into();
        if subject_path_or_symbol.trim().is_empty()
            || notes.trim().is_empty()
            || phrase.trim().is_empty()
            || line_excerpt.trim().is_empty()
        {
            return Err(TerminologyCleanupRejection::EmptyRequiredField);
        }
        if line_number == 0 {
            return Err(TerminologyCleanupRejection::InvalidLineNumber);
        }
        if evidence_refs.is_empty() {
            return Err(TerminologyCleanupRejection::MissingEvidenceRef);
        }
        let required_qualifier = required_qualifier(&allowed_use);
        if matches!(
            allowed_use,
            TerminologyAllowedUse::QualifiedPhysicalDebt { .. }
        ) && deferred_s_sequences.is_empty()
        {
            return Err(TerminologyCleanupRejection::QualifiedPhysicalDebtMissingSequence);
        }
        Ok(Self {
            row_id,
            subject_kind: S0ArtifactSubjectKind::ClaimSurface,
            subject_path_or_symbol,
            classification: "terminology-risk".to_string(),
            evidence_refs,
            forbidden_claims: Vec::new(),
            deferred_s_sequences,
            status,
            notes,
            phrase,
            line_number,
            line_excerpt,
            allowed_use,
            required_qualifier,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn allowed_use(&self) -> &TerminologyAllowedUse {
        &self.allowed_use
    }

    pub fn evidence_refs(&self) -> &[S0EvidenceRef] {
        &self.evidence_refs
    }

    pub fn subject_path_or_symbol(&self) -> &str {
        &self.subject_path_or_symbol
    }

    pub fn line_number(&self) -> u64 {
        self.line_number
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminologyRiskReport {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    rows: Vec<TerminologyPhraseFinding>,
    scan_digest: S0StableDigest,
}

impl TerminologyRiskReport {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<TerminologyPhraseFinding>,
        scan_digest: S0StableDigest,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = stable_digest(&TerminologyRiskReportDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::TerminologyRiskReport,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            scan_digest: &scan_digest,
            rows: &rows,
        })?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::TerminologyRiskReport,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
            scan_digest,
        })
    }

    pub fn scan(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        plan: &TerminologyScanPlan,
        manifest: &S0AuditInputManifest,
        inputs: &[TerminologyScanInputFile],
        allowlist: &[TerminologyAllowlistEntry],
    ) -> Result<Self, TerminologyCleanupRejection> {
        let manifest_paths = manifest
            .matched_files()
            .iter()
            .map(|file| file.path())
            .collect::<BTreeSet<_>>();
        let scope_paths = plan
            .scopes()
            .iter()
            .map(TerminologyScanScope::path)
            .collect::<BTreeSet<_>>();
        let mut seen_inputs = BTreeSet::new();
        let mut rows = Vec::new();

        let allowlist_index = allowlist
            .iter()
            .try_fold(BTreeMap::new(), |mut index, entry| {
                let key = (
                    entry.path.as_str(),
                    entry.line_number,
                    entry.phrase.as_str(),
                );
                if index.insert(key, entry.allowed_use.clone()).is_some() {
                    return Err(TerminologyCleanupRejection::DuplicateAllowlistEntry);
                }
                Ok(index)
            })?;

        for input in inputs {
            if !seen_inputs.insert(input.path()) {
                return Err(TerminologyCleanupRejection::DuplicateScanInput);
            }
            if !scope_paths
                .iter()
                .any(|scope| path_is_under_scope(input.path(), scope))
            {
                return Err(TerminologyCleanupRejection::InputOutsideDeclaredScanScope);
            }
            if !manifest_paths.contains(input.path()) {
                return Err(TerminologyCleanupRejection::InputOutsideManifest);
            }

            for (line_idx, line) in input.contents().lines().enumerate() {
                let normalized_line = line.to_ascii_lowercase();
                for phrase in TERMINOLOGY_RISK_PHRASES {
                    if !normalized_line.contains(phrase) {
                        continue;
                    }
                    let line_number = (line_idx + 1) as u64;
                    let allowed_use = allowlist_index
                        .get(&(input.path(), line_number, phrase))
                        .cloned()
                        .ok_or(TerminologyCleanupRejection::UnclassifiedPhraseFinding)?;
                    let deferred_s_sequences = match &allowed_use {
                        TerminologyAllowedUse::QualifiedPhysicalDebt { deferred_sequence } => {
                            vec![deferred_sequence.clone()]
                        }
                        _ => Vec::new(),
                    };
                    let status = match allowed_use {
                        TerminologyAllowedUse::OverclaimedPhysicalPosture => {
                            S0ArtifactRowStatus::Deferred
                        }
                        _ => S0ArtifactRowStatus::Admitted,
                    };
                    rows.push(TerminologyPhraseFinding::new(
                        finding_row_id(input.path(), line_number, phrase)?,
                        input.path(),
                        vec![terminology_evidence_ref(input.path(), line_number, phrase)],
                        deferred_s_sequences,
                        status,
                        "S.0 terminology risk finding.",
                        phrase,
                        line_number,
                        line.trim(),
                        allowed_use,
                    )?);
                }
            }
        }

        let mut scope_paths = plan
            .scopes()
            .iter()
            .map(|scope| scope.path().to_string())
            .collect::<Vec<_>>();
        scope_paths.sort();
        let mut allowlist_basis = allowlist
            .iter()
            .map(|entry| {
                (
                    entry.path.clone(),
                    entry.line_number,
                    entry.phrase.clone(),
                    allowed_use_basis(&entry.allowed_use),
                )
            })
            .collect::<Vec<_>>();
        allowlist_basis.sort();
        let mut input_basis = inputs
            .iter()
            .map(|input| (input.path().to_string(), input.contents().to_string()))
            .collect::<Vec<_>>();
        input_basis.sort();
        let mut row_basis = rows
            .iter()
            .map(|row| {
                (
                    row.row_id().as_str().to_string(),
                    row.subject_path_or_symbol().to_string(),
                    row.line_number(),
                    allowed_use_basis(row.allowed_use()),
                )
            })
            .collect::<Vec<_>>();
        row_basis.sort();
        let scan_digest = stable_digest(&TerminologyScanDigestBasis {
            scopes: scope_paths,
            allowlist: allowlist_basis,
            inputs: input_basis,
            rows: row_basis,
        })?;
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
            scan_digest,
        )
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[TerminologyPhraseFinding] {
        &self.rows
    }

    pub fn scan_digest(&self) -> &S0StableDigest {
        &self.scan_digest
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, TerminologyCleanupRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| TerminologyCleanupRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedTerminologyRiskReportArtifact, TerminologyCleanupRejection> {
        let raw = serde_json::from_slice::<RawTerminologyRiskReport>(bytes)
            .map_err(|_| TerminologyCleanupRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(TerminologyCleanupRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::TerminologyRiskReport {
            return Err(TerminologyCleanupRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(RawTerminologyPhraseFinding::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let scan_digest = S0StableDigest::new(raw.scan_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
            scan_digest,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(TerminologyCleanupRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| TerminologyCleanupRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedTerminologyRiskReportArtifact {
            report,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedTerminologyRiskReportArtifact {
    report: TerminologyRiskReport,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedTerminologyRiskReportArtifact {
    pub fn report(&self) -> &TerminologyRiskReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReleaseClaimScanPlan {
    release_surface_paths: Vec<String>,
}

impl ReleaseClaimScanPlan {
    pub fn new(release_surface_paths: Vec<String>) -> Result<Self, TerminologyCleanupRejection> {
        if release_surface_paths.is_empty() {
            return Err(TerminologyCleanupRejection::MissingReleaseSurface);
        }
        let mut seen = BTreeSet::new();
        if release_surface_paths
            .iter()
            .any(|path| path.trim().is_empty() || !seen.insert(path.as_str()))
        {
            return Err(TerminologyCleanupRejection::DuplicateReleaseSurface);
        }
        Ok(Self {
            release_surface_paths,
        })
    }

    pub fn release_surface_paths(&self) -> &[String] {
        &self.release_surface_paths
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PublicClaimRejection {
    OverclaimedPhysicalPosture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReleaseClaimReport {
    scanned_surface_count: u64,
    rejection_count: u64,
    unqualified_release_claim_count: u64,
    rejected: Vec<(String, u64, PublicClaimRejection)>,
}

impl ReleaseClaimReport {
    pub fn from_terminology_report(
        plan: &ReleaseClaimScanPlan,
        report: &TerminologyRiskReport,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let release_paths = plan
            .release_surface_paths()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let scanned_release_paths = report
            .rows()
            .iter()
            .map(TerminologyPhraseFinding::subject_path_or_symbol)
            .filter(|path| release_paths.contains(path))
            .collect::<BTreeSet<_>>();
        if scanned_release_paths.len() != release_paths.len() {
            return Err(TerminologyCleanupRejection::UnscannedReleaseSurface);
        }
        let rejected = report
            .rows()
            .iter()
            .filter(|row| release_paths.contains(row.subject_path_or_symbol()))
            .filter_map(|row| match row.allowed_use() {
                TerminologyAllowedUse::OverclaimedPhysicalPosture => Some((
                    row.subject_path_or_symbol().to_string(),
                    row.line_number(),
                    PublicClaimRejection::OverclaimedPhysicalPosture,
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(Self {
            scanned_surface_count: release_paths.len() as u64,
            rejection_count: rejected.len() as u64,
            unqualified_release_claim_count: rejected.len() as u64,
            rejected,
        })
    }

    pub fn rejection_count(&self) -> u64 {
        self.rejection_count
    }

    pub fn unqualified_release_claim_count(&self) -> u64 {
        self.unqualified_release_claim_count
    }

    pub fn scanned_surface_count(&self) -> u64 {
        self.scanned_surface_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TerminologyCleanupRejection {
    EmptyRequiredField,
    AbsolutePath,
    ParentTraversal,
    MissingScanScope,
    DuplicateScanScope,
    DuplicateAllowlistEntry,
    RejectedWorkspaceGlobalScope,
    MissingEvidenceRef,
    InvalidLineNumber,
    QualifiedPhysicalDebtMissingSequence,
    QualifierAppliedToNonRiskPhrase,
    DuplicateScanInput,
    InputOutsideDeclaredScanScope,
    InputOutsideManifest,
    UnclassifiedPhraseFinding,
    SerializationFailed,
    NonParseable,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    DuplicateRowId,
    MissingReleaseSurface,
    DuplicateReleaseSurface,
    UnscannedReleaseSurface,
    DeterministicDigestMismatch,
}

#[derive(Serialize)]
struct TerminologyRiskReportDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    scan_digest: &'a S0StableDigest,
    rows: &'a [TerminologyPhraseFinding],
}

#[derive(Serialize)]
struct TerminologyScanDigestBasis {
    scopes: Vec<String>,
    allowlist: Vec<(String, u64, String, String)>,
    inputs: Vec<(String, String)>,
    rows: Vec<(String, String, u64, String)>,
}

#[derive(Deserialize)]
struct RawTerminologyRiskReport {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    rows: Vec<RawTerminologyPhraseFinding>,
    scan_digest: String,
}

#[derive(Deserialize)]
struct RawS0ArtifactEnvelope {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: String,
    generated_by: String,
    deterministic_digest: String,
    nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
struct RawS0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    fn into_validated(self) -> Result<S0NondeterministicMetadata, TerminologyCleanupRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)
    }
}

#[derive(Deserialize)]
struct RawTerminologyPhraseFinding {
    row_id: String,
    subject_path_or_symbol: String,
    evidence_refs: Vec<RawS0EvidenceRef>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    phrase: String,
    line_number: u64,
    line_excerpt: String,
    allowed_use: RawTerminologyAllowedUse,
}

impl RawTerminologyPhraseFinding {
    fn into_validated(self) -> Result<TerminologyPhraseFinding, TerminologyCleanupRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id)
            .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| TerminologyCleanupRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        TerminologyPhraseFinding::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.phrase,
            self.line_number,
            self.line_excerpt,
            self.allowed_use.into_validated()?,
        )
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum RawTerminologyAllowedUse {
    AllowedSemanticUse,
    QualifiedPhysicalDebt {
        deferred_sequence: String,
    },
    ClosedFoundationEvidence {
        artifact_kind: S0ArtifactKind,
        digest: String,
    },
    OverclaimedPhysicalPosture,
}

impl RawTerminologyAllowedUse {
    fn into_validated(self) -> Result<TerminologyAllowedUse, TerminologyCleanupRejection> {
        match self {
            Self::AllowedSemanticUse => Ok(TerminologyAllowedUse::AllowedSemanticUse),
            Self::QualifiedPhysicalDebt { deferred_sequence } => {
                Ok(TerminologyAllowedUse::QualifiedPhysicalDebt {
                    deferred_sequence: Roadmap2SequenceId::new(deferred_sequence)
                        .map_err(|_| TerminologyCleanupRejection::InvalidDeferredSequence)?,
                })
            }
            Self::ClosedFoundationEvidence {
                artifact_kind,
                digest,
            } => Ok(TerminologyAllowedUse::ClosedFoundationEvidence {
                evidence_ref: S0EvidenceRef::new(
                    artifact_kind,
                    S0StableDigest::new(digest)
                        .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?,
                ),
            }),
            Self::OverclaimedPhysicalPosture => {
                Ok(TerminologyAllowedUse::OverclaimedPhysicalPosture)
            }
        }
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, TerminologyCleanupRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

fn required_qualifier(allowed_use: &TerminologyAllowedUse) -> TerminologyRequiredQualifier {
    match allowed_use {
        TerminologyAllowedUse::AllowedSemanticUse => TerminologyRequiredQualifier::SemanticOnly,
        TerminologyAllowedUse::QualifiedPhysicalDebt { .. } => {
            TerminologyRequiredQualifier::NamesDeferredSequence
        }
        TerminologyAllowedUse::ClosedFoundationEvidence { .. } => {
            TerminologyRequiredQualifier::ReferencesClosedFoundationEvidence
        }
        TerminologyAllowedUse::OverclaimedPhysicalPosture => {
            TerminologyRequiredQualifier::RejectAsOverclaim
        }
    }
}

fn allowed_use_basis(allowed_use: &TerminologyAllowedUse) -> String {
    match allowed_use {
        TerminologyAllowedUse::AllowedSemanticUse => "allowed_semantic_use".to_string(),
        TerminologyAllowedUse::QualifiedPhysicalDebt { deferred_sequence } => {
            format!("qualified_physical_debt:{}", deferred_sequence.as_str())
        }
        TerminologyAllowedUse::ClosedFoundationEvidence { evidence_ref } => format!(
            "closed_foundation_evidence:{:?}:{}",
            evidence_ref.artifact_kind(),
            evidence_ref.digest().as_str()
        ),
        TerminologyAllowedUse::OverclaimedPhysicalPosture => {
            "overclaimed_physical_posture".to_string()
        }
    }
}

fn phrase_requires_qualification(phrase: &str) -> bool {
    TERMINOLOGY_RISK_PHRASES.contains(&phrase.to_ascii_lowercase().as_str())
}

fn path_is_under_scope(path: &str, scope: &str) -> bool {
    path == scope
        || path
            .strip_prefix(scope)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn finding_row_id(
    path: &str,
    line_number: u64,
    phrase: &str,
) -> Result<S0ArtifactRowId, TerminologyCleanupRejection> {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update(line_number.to_le_bytes());
    hasher.update(phrase.as_bytes());
    let digest = format!("{:x}", hasher.finalize());
    S0ArtifactRowId::new(format!("TerminologyFinding{}", &digest[..16]))
        .map_err(|_| TerminologyCleanupRejection::EmptyRequiredField)
}

fn terminology_evidence_ref(path: &str, line_number: u64, phrase: &str) -> S0EvidenceRef {
    let digest = stable_digest(&(path, line_number, phrase))
        .expect("terminology evidence digest basis must serialize");
    S0EvidenceRef::new(S0ArtifactKind::TerminologyRiskReport, digest)
}

fn reject_duplicate_rows(
    rows: &[TerminologyPhraseFinding],
) -> Result<(), TerminologyCleanupRejection> {
    let mut seen = BTreeSet::new();
    if rows.iter().any(|row| !seen.insert(row.row_id().clone())) {
        return Err(TerminologyCleanupRejection::DuplicateRowId);
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, TerminologyCleanupRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| TerminologyCleanupRejection::SerializationFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| TerminologyCleanupRejection::InvalidDigest)
}

fn require_non_empty(value: impl Into<String>) -> Result<String, TerminologyCleanupRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(TerminologyCleanupRejection::EmptyRequiredField);
    }
    Ok(value)
}

fn normalize_relative_path(
    value: impl Into<String>,
) -> Result<String, TerminologyCleanupRejection> {
    let normalized = require_non_empty(value)?.replace('\\', "/");
    if normalized.starts_with('/') || normalized.contains(":/") {
        return Err(TerminologyCleanupRejection::AbsolutePath);
    }
    if normalized.split('/').any(|part| part == "..") {
        return Err(TerminologyCleanupRejection::ParentTraversal);
    }
    Ok(normalized.trim_matches('/').to_string())
}
