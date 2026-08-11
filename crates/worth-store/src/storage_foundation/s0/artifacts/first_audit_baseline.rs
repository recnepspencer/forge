use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
    StoreBackendCapabilityTier,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::capability_matrix::BackendCapabilityMatrixRow;
use super::row_identity::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum S0FirstAuditBaselineRowId {
    AbsentMode,
    InMemoryHarness,
    EmbeddedMode,
    DurableMode,
    LocalFileBackend,
    SqliteBackend,
    SemanticCertificationHarness,
    SubscriptionSupportTrustEvidence,
    Roadmap2PhysicalBackendCandidate,
    FuturePlatformGradeBackend,
}

impl S0FirstAuditBaselineRowId {
    pub fn required() -> [Self; 10] {
        [
            Self::AbsentMode,
            Self::InMemoryHarness,
            Self::EmbeddedMode,
            Self::DurableMode,
            Self::LocalFileBackend,
            Self::SqliteBackend,
            Self::SemanticCertificationHarness,
            Self::SubscriptionSupportTrustEvidence,
            Self::Roadmap2PhysicalBackendCandidate,
            Self::FuturePlatformGradeBackend,
        ]
    }

    pub fn row_id(self) -> S0ArtifactRowId {
        S0ArtifactRowId::new(match self {
            Self::AbsentMode => "AbsentMode",
            Self::InMemoryHarness => "InMemoryHarness",
            Self::EmbeddedMode => "EmbeddedMode",
            Self::DurableMode => "DurableMode",
            Self::LocalFileBackend => "LocalFileBackend",
            Self::SqliteBackend => "SqliteBackend",
            Self::SemanticCertificationHarness => "SemanticCertificationHarness",
            Self::SubscriptionSupportTrustEvidence => "SubscriptionSupportTrustEvidence",
            Self::Roadmap2PhysicalBackendCandidate => "Roadmap2PhysicalBackendCandidate",
            Self::FuturePlatformGradeBackend => "FuturePlatformGradeBackend",
        })
        .expect("required S.0 first-audit row ids are stable constants")
    }
}

pub(super) fn first_audit_baseline_rows() -> Vec<BackendCapabilityMatrixRow> {
    S0FirstAuditBaselineRowId::required()
        .into_iter()
        .map(first_audit_baseline_row)
        .collect()
}

fn first_audit_baseline_row(id: S0FirstAuditBaselineRowId) -> BackendCapabilityMatrixRow {
    build_first_audit_baseline_row(id, baseline_row_facts(id))
}

struct BaselineRowFacts {
    subject: &'static str,
    subject_kind: S0ArtifactSubjectKind,
    tier: StoreBackendCapabilityTier,
    classification: &'static str,
    valid_use: &'static str,
    semantic_guarantees: &'static [&'static str],
    physical_gaps: &'static [&'static str],
    deferred_sequences: &'static [&'static str],
    status: S0ArtifactRowStatus,
}

fn build_first_audit_baseline_row(
    id: S0FirstAuditBaselineRowId,
    facts: BaselineRowFacts,
) -> BackendCapabilityMatrixRow {
    let deferred_sequences = facts
        .deferred_sequences
        .iter()
        .map(|sequence| Roadmap2SequenceId::new(*sequence).unwrap())
        .collect::<Vec<_>>();
    BackendCapabilityMatrixRow::new(
        id.row_id(),
        facts.subject_kind,
        facts.subject,
        facts.classification,
        vec![baseline_evidence_ref()],
        baseline_forbidden_claims(&deferred_sequences),
        deferred_sequences,
        facts.status,
        "S.0 first-audit baseline row.",
        facts.tier,
        facts.valid_use,
        vec!["Closed Roadmap 2 evidence witness".to_string()],
        facts
            .semantic_guarantees
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        facts
            .physical_gaps
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    )
    .expect("first-audit baseline constants must satisfy row invariants")
}

fn baseline_row_facts(id: S0FirstAuditBaselineRowId) -> BaselineRowFacts {
    match id {
        S0FirstAuditBaselineRowId::AbsentMode => absent_mode_baseline_facts(),
        S0FirstAuditBaselineRowId::InMemoryHarness => in_memory_harness_baseline_facts(),
        S0FirstAuditBaselineRowId::EmbeddedMode => embedded_mode_baseline_facts(),
        S0FirstAuditBaselineRowId::DurableMode => durable_mode_baseline_facts(),
        S0FirstAuditBaselineRowId::LocalFileBackend => local_file_backend_baseline_facts(),
        S0FirstAuditBaselineRowId::SqliteBackend => sqlite_backend_baseline_facts(),
        S0FirstAuditBaselineRowId::SemanticCertificationHarness => {
            semantic_certification_harness_baseline_facts()
        }
        S0FirstAuditBaselineRowId::SubscriptionSupportTrustEvidence => {
            subscription_support_trust_baseline_facts()
        }
        S0FirstAuditBaselineRowId::Roadmap2PhysicalBackendCandidate => {
            roadmap2_physical_backend_baseline_facts()
        }
        S0FirstAuditBaselineRowId::FuturePlatformGradeBackend => {
            future_platform_grade_baseline_facts()
        }
    }
}

fn absent_mode_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::modes::AbsentMode",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::Bootstrap,
        classification: "optional-store-boundary",
        valid_use: "Proves optional Store boundaries without persistence claims.",
        semantic_guarantees: &["optional Store semantics"],
        physical_gaps: &["physical persistence"],
        deferred_sequences: &["S1", "S4"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn in_memory_harness_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::tests::harness",
        subject_kind: S0ArtifactSubjectKind::Harness,
        tier: StoreBackendCapabilityTier::SemanticCertification,
        classification: "semantic-harness",
        valid_use: "Exercises semantic behavior without durable survival evidence.",
        semantic_guarantees: &["semantic replay"],
        physical_gaps: &["durable media survival"],
        deferred_sequences: &["S1", "S2"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn embedded_mode_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::modes::EmbeddedMode",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::SemanticCertification,
        classification: "embedded-semantic-mode",
        valid_use: "Proves lifecycle and artifact reception semantics.",
        semantic_guarantees: &["embedded lifecycle"],
        physical_gaps: &["platform-grade physical database posture"],
        deferred_sequences: &["S1", "S5"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn durable_mode_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::modes::DurableMode",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::SemanticCertification,
        classification: "durable-mode-orchestration",
        valid_use: "Proves durable-mode orchestration semantics.",
        semantic_guarantees: &["semantic durable-mode orchestration"],
        physical_gaps: &["S.4 recovery physics"],
        deferred_sequences: &["S4"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn local_file_backend_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::backend::local_file",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::Compatibility,
        classification: "local-file-compatibility",
        valid_use: "Compatibility path until Roadmap 2 physical gates are proven.",
        semantic_guarantees: &["bootstrap file persistence"],
        physical_gaps: &["bounded page substrate"],
        deferred_sequences: &["S1", "S3", "S6"],
        status: S0ArtifactRowStatus::Deferred,
    }
}

fn sqlite_backend_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::backend::sqlite",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::Compatibility,
        classification: "sqlite-compatibility",
        valid_use: "Compatibility path until Store-owned physical gates are proven.",
        semantic_guarantees: &["bootstrap SQLite interoperability"],
        physical_gaps: &["Store-native physical authority"],
        deferred_sequences: &["S1", "S6"],
        status: S0ArtifactRowStatus::Deferred,
    }
}

fn semantic_certification_harness_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::evidence",
        subject_kind: S0ArtifactSubjectKind::EvidenceLane,
        tier: StoreBackendCapabilityTier::SemanticCertification,
        classification: "semantic-certification-evidence",
        valid_use: "Certifies semantic Store behavior without physical substrate claims.",
        semantic_guarantees: &["semantic certification"],
        physical_gaps: &["physical boundedness"],
        deferred_sequences: &["S2", "S12"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn subscription_support_trust_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::subscription_support::trust",
        subject_kind: S0ArtifactSubjectKind::EvidenceLane,
        tier: StoreBackendCapabilityTier::SemanticCertification,
        classification: "closed-semantic-trust-evidence",
        valid_use: "Milestone 13.3 trust evidence; not physical database readiness.",
        semantic_guarantees: &["role-scoped subscription-support trust"],
        physical_gaps: &["physical database readiness"],
        deferred_sequences: &["S12"],
        status: S0ArtifactRowStatus::Present,
    }
}

fn roadmap2_physical_backend_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::storage_foundation::s1",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::PhysicalFoundation,
        classification: "physical-backend-candidate",
        valid_use: "Candidate row for S.1 physical substrate evidence.",
        semantic_guarantees: &["none admitted yet"],
        physical_gaps: &["closed physical foundation gates"],
        deferred_sequences: &["S1"],
        status: S0ArtifactRowStatus::Deferred,
    }
}

fn future_platform_grade_baseline_facts() -> BaselineRowFacts {
    BaselineRowFacts {
        subject: "worth_store::storage_foundation::platform_grade",
        subject_kind: S0ArtifactSubjectKind::Backend,
        tier: StoreBackendCapabilityTier::PlatformGrade,
        classification: "future-platform-grade-target",
        valid_use: "Target posture only; requires closed Roadmap 2 platform evidence.",
        semantic_guarantees: &["none admitted yet"],
        physical_gaps: &["all required Roadmap 2 platform gates"],
        deferred_sequences: &[
            "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10", "S11", "S12",
        ],
        status: S0ArtifactRowStatus::Deferred,
    }
}

fn baseline_forbidden_claims(sequences: &[Roadmap2SequenceId]) -> Vec<BackendForbiddenClaim> {
    let sequence = sequences
        .first()
        .cloned()
        .unwrap_or_else(|| Roadmap2SequenceId::new("S1").unwrap());
    [
        BackendForbiddenClaimKind::PlatformGradeDurability,
        BackendForbiddenClaimKind::PhysicalPersistence,
    ]
    .into_iter()
    .map(|kind| {
        BackendForbiddenClaim::new(kind, sequence.as_str())
            .expect("baseline deferred sequence is known non-empty")
    })
    .collect()
}

fn baseline_evidence_ref() -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::S0EvidenceBundle,
        S0StableDigest::new("s0:first-audit-baseline").unwrap(),
    )
}
