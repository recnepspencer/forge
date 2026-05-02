use super::evidence::S0StableDigest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StoreBackendCapabilityTier {
    Bootstrap,
    SemanticCertification,
    Compatibility,
    PhysicalFoundation,
    PlatformGrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BackendForbiddenClaimKind {
    PlatformGradeDurability,
    PlatformGradeRecovery,
    PlatformGradeConcurrency,
    PlatformGradeMultiTenantIsolation,
    PhysicalPersistence,
    PhysicalQueryPerformance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Roadmap2SequenceId(String);

impl Roadmap2SequenceId {
    pub fn new(value: impl Into<String>) -> Result<Self, S0ClaimPromotionRejection> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(S0ClaimPromotionRejection::MissingSequenceMapping);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: Roadmap2SequenceId,
}

impl BackendForbiddenClaim {
    pub fn new(
        claim_kind: BackendForbiddenClaimKind,
        deferred_sequence: impl Into<String>,
    ) -> Result<Self, S0ClaimPromotionRejection> {
        Ok(Self {
            claim_kind,
            deferred_sequence: Roadmap2SequenceId::new(deferred_sequence)?,
        })
    }

    pub fn claim_kind(&self) -> BackendForbiddenClaimKind {
        self.claim_kind
    }

    pub fn deferred_sequence(&self) -> &Roadmap2SequenceId {
        &self.deferred_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackendCapabilityDeclaration {
    subject: String,
    declared_tier: StoreBackendCapabilityTier,
    forbidden_claims: Vec<BackendForbiddenClaim>,
}

impl BackendCapabilityDeclaration {
    pub fn new(
        subject: impl Into<String>,
        declared_tier: StoreBackendCapabilityTier,
    ) -> Result<Self, S0ClaimPromotionRejection> {
        let subject = subject.into();
        if subject.trim().is_empty() {
            return Err(S0ClaimPromotionRejection::EmptySubject);
        }
        Ok(Self {
            subject,
            declared_tier,
            forbidden_claims: Vec::new(),
        })
    }

    pub fn with_forbidden_claim(mut self, claim: BackendForbiddenClaim) -> Self {
        self.forbidden_claims.push(claim);
        self
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn declared_tier(&self) -> StoreBackendCapabilityTier {
        self.declared_tier
    }

    pub fn forbidden_claims(&self) -> &[BackendForbiddenClaim] {
        &self.forbidden_claims
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnclassifiedBackendClaim {
    declaration: BackendCapabilityDeclaration,
    requested_tier: StoreBackendCapabilityTier,
}

impl UnclassifiedBackendClaim {
    pub fn new(
        declaration: BackendCapabilityDeclaration,
        requested_tier: StoreBackendCapabilityTier,
    ) -> Self {
        Self {
            declaration,
            requested_tier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassifiedBackendClaim {
    declaration: BackendCapabilityDeclaration,
    requested_tier: StoreBackendCapabilityTier,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForbiddenClaimAudited {
    claim: ClassifiedBackendClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Roadmap2EvidenceBound {
    SemanticOnly(SemanticOnlyClaimWitness),
    PhysicalDebt(PhysicalDebtWitness),
    Foundation(FoundationEvidenceWitness),
    PlatformGrade(PlatformGradeEvidenceWitness),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Roadmap2EvidenceBoundClaim {
    audited: ForbiddenClaimAudited,
    evidence: Roadmap2EvidenceBound,
}

impl Roadmap2EvidenceBoundClaim {
    pub fn evidence_kind(&self) -> StoreBackendCapabilityTier {
        match self.evidence {
            Roadmap2EvidenceBound::SemanticOnly(_) => {
                StoreBackendCapabilityTier::SemanticCertification
            }
            Roadmap2EvidenceBound::PhysicalDebt(_) => StoreBackendCapabilityTier::Compatibility,
            Roadmap2EvidenceBound::Foundation(_) => StoreBackendCapabilityTier::PhysicalFoundation,
            Roadmap2EvidenceBound::PlatformGrade(_) => StoreBackendCapabilityTier::PlatformGrade,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlatformGradeEvidenceBoundClaim {
    audited: ForbiddenClaimAudited,
    evidence: PlatformGradeEvidenceWitness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticOnlyClaimWitness {
    reason: String,
}

impl SemanticOnlyClaimWitness {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PhysicalDebtWitness {
    deferred_sequence: Roadmap2SequenceId,
    reason: String,
}

impl PhysicalDebtWitness {
    pub fn new(deferred_sequence: Roadmap2SequenceId, reason: impl Into<String>) -> Self {
        Self {
            deferred_sequence,
            reason: reason.into(),
        }
    }

    pub fn deferred_sequence(&self) -> &Roadmap2SequenceId {
        &self.deferred_sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FoundationEvidenceWitness {
    sequence_id: Roadmap2SequenceId,
    evidence_digest: S0StableDigest,
}

impl FoundationEvidenceWitness {
    #[allow(dead_code)]
    pub(crate) fn new(sequence_id: Roadmap2SequenceId, evidence_digest: S0StableDigest) -> Self {
        Self {
            sequence_id,
            evidence_digest,
        }
    }

    pub fn sequence_id(&self) -> &Roadmap2SequenceId {
        &self.sequence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlatformGradeEvidenceWitness {
    declaration: BackendCapabilityDeclaration,
    foundation_witnesses: Vec<FoundationEvidenceWitness>,
    accepted_sequence_count: u64,
}

impl PlatformGradeEvidenceWitness {
    #[allow(dead_code)]
    pub(crate) fn from_foundation_witnesses(
        declaration: BackendCapabilityDeclaration,
        foundation_witnesses: Vec<FoundationEvidenceWitness>,
        required_sequences: impl IntoIterator<Item = Roadmap2SequenceId>,
    ) -> Result<Self, S0ClaimPromotionRejection> {
        if declaration.declared_tier() != StoreBackendCapabilityTier::PlatformGrade {
            return Err(S0ClaimPromotionRejection::PlatformGradeEvidenceMissing);
        }
        let present = foundation_witnesses
            .iter()
            .map(|witness| witness.sequence_id().clone())
            .collect::<BTreeSet<_>>();
        let required = required_sequences.into_iter().collect::<BTreeSet<_>>();
        if required.is_empty() || !required.iter().all(|sequence| present.contains(sequence)) {
            return Err(S0ClaimPromotionRejection::MissingSequenceMapping);
        }
        Ok(Self {
            declaration,
            foundation_witnesses,
            accepted_sequence_count: required.len() as u64,
        })
    }

    pub fn declaration(&self) -> &BackendCapabilityDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlatformGradeClaimAdmitted {
    subject: String,
    admitted_tier: StoreBackendCapabilityTier,
    accepted_sequence_count: u64,
}

impl PlatformGradeClaimAdmitted {
    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn admitted_tier(&self) -> StoreBackendCapabilityTier {
        self.admitted_tier
    }

    pub fn accepted_sequence_count(&self) -> u64 {
        self.accepted_sequence_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ClaimPromotionRejection {
    EmptySubject,
    RequestedTierExceedsDeclaration,
    ForbiddenClaimsMissing,
    MissingSequenceMapping,
    PlatformGradeEvidenceMissing,
    EvidenceSubjectMismatch,
}

pub fn classify_backend_claim(
    claim: UnclassifiedBackendClaim,
) -> Result<ClassifiedBackendClaim, S0ClaimPromotionRejection> {
    if claim.requested_tier > claim.declaration.declared_tier() {
        return Err(S0ClaimPromotionRejection::RequestedTierExceedsDeclaration);
    }
    Ok(ClassifiedBackendClaim {
        declaration: claim.declaration,
        requested_tier: claim.requested_tier,
    })
}

pub fn audit_forbidden_claims(
    claim: ClassifiedBackendClaim,
) -> Result<ForbiddenClaimAudited, S0ClaimPromotionRejection> {
    let requires_deferred_claims = claim.requested_tier < StoreBackendCapabilityTier::PlatformGrade;
    if requires_deferred_claims && claim.declaration.forbidden_claims().is_empty() {
        return Err(S0ClaimPromotionRejection::ForbiddenClaimsMissing);
    }
    if claim
        .declaration
        .forbidden_claims()
        .iter()
        .any(|forbidden| forbidden.deferred_sequence().as_str().trim().is_empty())
    {
        return Err(S0ClaimPromotionRejection::MissingSequenceMapping);
    }
    Ok(ForbiddenClaimAudited { claim })
}

pub fn bind_roadmap2_evidence(
    audited: ForbiddenClaimAudited,
    evidence: Roadmap2EvidenceBound,
) -> Result<Roadmap2EvidenceBoundClaim, S0ClaimPromotionRejection> {
    if let Roadmap2EvidenceBound::PlatformGrade(platform) = &evidence {
        if platform.declaration().subject() != audited.claim.declaration.subject() {
            return Err(S0ClaimPromotionRejection::EvidenceSubjectMismatch);
        }
    }
    Ok(Roadmap2EvidenceBoundClaim { audited, evidence })
}

pub fn bind_platform_grade_evidence(
    audited: ForbiddenClaimAudited,
    evidence: PlatformGradeEvidenceWitness,
) -> Result<PlatformGradeEvidenceBoundClaim, S0ClaimPromotionRejection> {
    if evidence.declaration().subject() != audited.claim.declaration.subject() {
        return Err(S0ClaimPromotionRejection::EvidenceSubjectMismatch);
    }
    Ok(PlatformGradeEvidenceBoundClaim { audited, evidence })
}

pub fn admit_platform_grade_claim(
    claim: PlatformGradeEvidenceBoundClaim,
) -> Result<PlatformGradeClaimAdmitted, S0ClaimPromotionRejection> {
    if claim.audited.claim.requested_tier != StoreBackendCapabilityTier::PlatformGrade {
        return Err(S0ClaimPromotionRejection::PlatformGradeEvidenceMissing);
    }
    Ok(PlatformGradeClaimAdmitted {
        subject: claim.evidence.declaration.subject,
        admitted_tier: StoreBackendCapabilityTier::PlatformGrade,
        accepted_sequence_count: claim.evidence.accepted_sequence_count,
    })
}
