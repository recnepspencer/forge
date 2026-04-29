use super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0NondeterministicMetadata,
    S0_ARTIFACT_SCHEMA_VERSION,
};
use super::capability::{BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId};
use super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MilestoneSpecStatus {
    Planned,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MilestoneCloseoutStatus {
    Missing,
    Planned,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MilestoneSequenceInconsistency {
    SpecCloseoutStatusMismatch,
    ClosedWithUnclosedPrerequisite,
    MissingGatePredecessorEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PrerequisiteWaiverRationale {
    SemanticDocumentationDrift,
    IntentionalOutOfOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SemanticPhysicalClaimFamily {
    SemanticAuthority,
    RecoverySemantics,
    RetentionSemantics,
    SubscriptionSupport,
    CompatibilitySemantics,
    TieringPlacement,
    ReplicationSemantics,
    PhysicalSubstrate,
    PhysicalBoundedness,
    PhysicalIntegrity,
    PhysicalRecoveryPhysics,
    PhysicalIsolation,
    PhysicalIo,
    PhysicalOperationalSafety,
    PhysicalSecurity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S0PhysicalStatus {
    NotApplicable,
    NotStarted,
    SemanticOnly,
    BootstrapPhysical,
    PhysicalDebt,
    PartiallyFoundationBacked,
    FoundationBacked,
    PlatformGrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MilestonePrerequisiteEdge {
    milestone_id: String,
    prerequisite_milestone_id: String,
    waiver_rationale: Option<PrerequisiteWaiverRationale>,
}

impl MilestonePrerequisiteEdge {
    pub fn new(
        milestone_id: impl Into<String>,
        prerequisite_milestone_id: impl Into<String>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        let milestone_id = require_non_empty(milestone_id)?;
        let prerequisite_milestone_id = require_non_empty(prerequisite_milestone_id)?;
        Ok(Self {
            milestone_id,
            prerequisite_milestone_id,
            waiver_rationale: None,
        })
    }

    pub fn waived(mut self, rationale: PrerequisiteWaiverRationale) -> Self {
        self.waiver_rationale = Some(rationale);
        self
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn prerequisite_milestone_id(&self) -> &str {
        &self.prerequisite_milestone_id
    }

    pub fn waiver_rationale(&self) -> Option<PrerequisiteWaiverRationale> {
        self.waiver_rationale
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MilestoneStatusDeclaration {
    milestone_id: String,
    spec_status: MilestoneSpecStatus,
    closeout_status: MilestoneCloseoutStatus,
    predecessor_evidence: Vec<S0EvidenceRef>,
}

impl MilestoneStatusDeclaration {
    pub fn new(
        milestone_id: impl Into<String>,
        spec_status: MilestoneSpecStatus,
        closeout_status: MilestoneCloseoutStatus,
        predecessor_evidence: Vec<S0EvidenceRef>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        let milestone_id = require_non_empty(milestone_id)?;
        Ok(Self {
            milestone_id,
            spec_status,
            closeout_status,
            predecessor_evidence,
        })
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn spec_status(&self) -> MilestoneSpecStatus {
        self.spec_status
    }

    pub fn closeout_status(&self) -> MilestoneCloseoutStatus {
        self.closeout_status
    }

    pub fn predecessor_evidence(&self) -> &[S0EvidenceRef] {
        &self.predecessor_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RoadmapSequenceStatusMatrix {
    declarations: Vec<MilestoneStatusDeclaration>,
    prerequisite_edges: Vec<MilestonePrerequisiteEdge>,
    inconsistencies: Vec<(String, MilestoneSequenceInconsistency)>,
}

impl RoadmapSequenceStatusMatrix {
    pub fn new(
        declarations: Vec<MilestoneStatusDeclaration>,
        prerequisite_edges: Vec<MilestonePrerequisiteEdge>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        if declarations.is_empty() {
            return Err(S0MilestoneAuditRejection::MissingMilestoneDeclaration);
        }
        reject_duplicate_declarations(&declarations)?;
        let declaration_map = declarations
            .iter()
            .map(|declaration| (declaration.milestone_id(), declaration))
            .collect::<BTreeMap<_, _>>();
        let mut inconsistencies = Vec::new();

        for declaration in &declarations {
            if declaration.spec_status() == MilestoneSpecStatus::Planned
                && declaration.closeout_status() == MilestoneCloseoutStatus::Closed
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::SpecCloseoutStatusMismatch,
                ));
            }
            if declaration.closeout_status() == MilestoneCloseoutStatus::Closed
                && declaration.predecessor_evidence().is_empty()
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::MissingGatePredecessorEvidence,
                ));
            }
        }

        for edge in &prerequisite_edges {
            let declaration = declaration_map
                .get(edge.milestone_id())
                .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
            let prerequisite = declaration_map
                .get(edge.prerequisite_milestone_id())
                .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
            if declaration.closeout_status() == MilestoneCloseoutStatus::Closed
                && prerequisite.closeout_status() != MilestoneCloseoutStatus::Closed
                && edge.waiver_rationale().is_none()
            {
                inconsistencies.push((
                    declaration.milestone_id().to_string(),
                    MilestoneSequenceInconsistency::ClosedWithUnclosedPrerequisite,
                ));
            }
        }

        Ok(Self {
            declarations,
            prerequisite_edges,
            inconsistencies,
        })
    }

    pub fn declarations(&self) -> &[MilestoneStatusDeclaration] {
        &self.declarations
    }

    pub fn prerequisite_edges(&self) -> &[MilestonePrerequisiteEdge] {
        &self.prerequisite_edges
    }

    pub fn inconsistencies(&self) -> &[(String, MilestoneSequenceInconsistency)] {
        &self.inconsistencies
    }

    pub fn unwaived_inconsistency_count(&self) -> u64 {
        self.inconsistencies.len() as u64
    }

    pub fn gate_readiness_witness(
        &self,
        milestone_id: &str,
    ) -> Result<RoadmapGateReadinessWitness, S0MilestoneAuditRejection> {
        let declaration = self
            .declarations
            .iter()
            .find(|declaration| declaration.milestone_id() == milestone_id)
            .ok_or(S0MilestoneAuditRejection::UnknownMilestoneReference)?;
        if self
            .inconsistencies
            .iter()
            .any(|(id, _)| id == milestone_id)
        {
            return Err(S0MilestoneAuditRejection::GateReadinessBlockedBySequenceInconsistency);
        }
        if declaration.predecessor_evidence().is_empty() {
            return Err(S0MilestoneAuditRejection::MissingGatePredecessorEvidence);
        }
        Ok(RoadmapGateReadinessWitness {
            milestone_id: declaration.milestone_id().to_string(),
            predecessor_evidence_count: declaration.predecessor_evidence().len() as u64,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RoadmapGateReadinessWitness {
    milestone_id: String,
    predecessor_evidence_count: u64,
}

impl RoadmapGateReadinessWitness {
    pub(crate) fn new(milestone_id: String, predecessor_evidence_count: u64) -> Self {
        Self {
            milestone_id,
            predecessor_evidence_count,
        }
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn predecessor_evidence_count(&self) -> u64 {
        self.predecessor_evidence_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MilestonePhysicalStatusRow {
    milestone_id: String,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
    evidence_lanes: Vec<String>,
    claim_families: Vec<SemanticPhysicalClaimFamily>,
    physical_substrate_status: S0PhysicalStatus,
    bounded_memory_status: S0PhysicalStatus,
    physical_integrity_status: S0PhysicalStatus,
    recovery_physics_status: S0PhysicalStatus,
    io_qos_status: S0PhysicalStatus,
    native_blob_chunk_status: Option<S0PhysicalStatus>,
    operator_security_status: Option<S0PhysicalStatus>,
    forbidden_claims: Vec<BackendForbiddenClaim>,
    deferred_s_sequences: Vec<Roadmap2SequenceId>,
    required_wording_cleanup: Vec<String>,
}

impl MilestonePhysicalStatusRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        milestone_id: impl Into<String>,
        semantic_capability_proven: impl Into<String>,
        closeout_or_planned_source: impl Into<String>,
        named_suite: impl Into<String>,
        evidence_lanes: Vec<String>,
        claim_families: Vec<SemanticPhysicalClaimFamily>,
        physical_substrate_status: S0PhysicalStatus,
        bounded_memory_status: S0PhysicalStatus,
        physical_integrity_status: S0PhysicalStatus,
        recovery_physics_status: S0PhysicalStatus,
        io_qos_status: S0PhysicalStatus,
        native_blob_chunk_status: Option<S0PhysicalStatus>,
        operator_security_status: Option<S0PhysicalStatus>,
        forbidden_claims: Vec<BackendForbiddenClaim>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        required_wording_cleanup: Vec<String>,
        roadmap_gate: Option<&RoadmapGateReadinessWitness>,
    ) -> Result<Self, S0MilestoneAuditRejection> {
        let milestone_id = require_non_empty(milestone_id)?;
        let semantic_capability_proven = require_non_empty(semantic_capability_proven)?;
        let closeout_or_planned_source = require_non_empty(closeout_or_planned_source)?;
        let named_suite = require_non_empty(named_suite)?;
        if evidence_lanes.is_empty() {
            return Err(S0MilestoneAuditRejection::MissingEvidenceLane);
        }
        if claim_families.is_empty() {
            return Err(S0MilestoneAuditRejection::MissingClaimFamily);
        }
        if any_status_is_platform_grade([
            physical_substrate_status,
            bounded_memory_status,
            physical_integrity_status,
            recovery_physics_status,
            io_qos_status,
        ]) || native_blob_chunk_status == Some(S0PhysicalStatus::PlatformGrade)
            || operator_security_status == Some(S0PhysicalStatus::PlatformGrade)
        {
            match roadmap_gate {
                Some(witness) if witness.milestone_id() == milestone_id => {}
                _ => {
                    return Err(S0MilestoneAuditRejection::PlatformGradeStatusRequiresGateReadiness)
                }
            }
        }
        if any_status_is_physical_debt([
            physical_substrate_status,
            bounded_memory_status,
            physical_integrity_status,
            recovery_physics_status,
            io_qos_status,
        ]) || native_blob_chunk_status == Some(S0PhysicalStatus::PhysicalDebt)
            || operator_security_status == Some(S0PhysicalStatus::PhysicalDebt)
        {
            if deferred_s_sequences.is_empty() {
                return Err(S0MilestoneAuditRejection::PhysicalDebtRequiresDeferredSequence);
            }
        }
        Ok(Self {
            milestone_id,
            semantic_capability_proven,
            closeout_or_planned_source,
            named_suite,
            evidence_lanes,
            claim_families,
            physical_substrate_status,
            bounded_memory_status,
            physical_integrity_status,
            recovery_physics_status,
            io_qos_status,
            native_blob_chunk_status,
            operator_security_status,
            forbidden_claims,
            deferred_s_sequences,
            required_wording_cleanup,
        })
    }

    pub fn milestone_id(&self) -> &str {
        &self.milestone_id
    }

    pub fn semantic_capability_proven(&self) -> &str {
        &self.semantic_capability_proven
    }

    pub fn closeout_or_planned_source(&self) -> &str {
        &self.closeout_or_planned_source
    }

    pub fn named_suite(&self) -> &str {
        &self.named_suite
    }

    pub fn evidence_lanes(&self) -> &[String] {
        &self.evidence_lanes
    }

    pub fn claim_families(&self) -> &[SemanticPhysicalClaimFamily] {
        &self.claim_families
    }

    pub fn forbidden_claims(&self) -> &[BackendForbiddenClaim] {
        &self.forbidden_claims
    }

    pub fn deferred_s_sequences(&self) -> &[Roadmap2SequenceId] {
        &self.deferred_s_sequences
    }

    pub fn required_wording_cleanup(&self) -> &[String] {
        &self.required_wording_cleanup
    }

    pub fn native_blob_chunk_status(&self) -> Option<S0PhysicalStatus> {
        self.native_blob_chunk_status
    }

    pub fn operator_security_status(&self) -> Option<S0PhysicalStatus> {
        self.operator_security_status
    }

    pub fn physical_status_for_claim_family(
        &self,
        family: SemanticPhysicalClaimFamily,
    ) -> S0PhysicalStatus {
        match family {
            SemanticPhysicalClaimFamily::SemanticAuthority
            | SemanticPhysicalClaimFamily::RecoverySemantics
            | SemanticPhysicalClaimFamily::RetentionSemantics
            | SemanticPhysicalClaimFamily::SubscriptionSupport
            | SemanticPhysicalClaimFamily::CompatibilitySemantics
            | SemanticPhysicalClaimFamily::TieringPlacement
            | SemanticPhysicalClaimFamily::ReplicationSemantics => S0PhysicalStatus::SemanticOnly,
            SemanticPhysicalClaimFamily::PhysicalSubstrate => self.physical_substrate_status,
            SemanticPhysicalClaimFamily::PhysicalBoundedness => self.bounded_memory_status,
            SemanticPhysicalClaimFamily::PhysicalIntegrity => self.physical_integrity_status,
            SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics => self.recovery_physics_status,
            SemanticPhysicalClaimFamily::PhysicalIsolation => self.io_qos_status,
            SemanticPhysicalClaimFamily::PhysicalIo => self.io_qos_status,
            SemanticPhysicalClaimFamily::PhysicalOperationalSafety
            | SemanticPhysicalClaimFamily::PhysicalSecurity => self
                .operator_security_status
                .unwrap_or(S0PhysicalStatus::NotApplicable),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MilestonePhysicalStatusMatrix {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    roadmap_sequence_status: RoadmapSequenceStatusMatrix,
    rows: Vec<MilestonePhysicalStatusRow>,
}

impl MilestonePhysicalStatusMatrix {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        roadmap_sequence_status: RoadmapSequenceStatusMatrix,
        required_milestone_ids: Vec<String>,
        mut rows: Vec<MilestonePhysicalStatusRow>,
    ) -> Result<Self, S0MilestoneMatrixBuildRejection> {
        let source_revision = require_non_empty(source_revision)
            .map_err(|_| S0MilestoneMatrixBuildRejection::EmptyRequiredField)?;
        let generated_by = require_non_empty(generated_by)
            .map_err(|_| S0MilestoneMatrixBuildRejection::EmptyRequiredField)?;
        if rows.is_empty() {
            return Err(S0MilestoneMatrixBuildRejection::MissingMilestoneRow);
        }
        rows.sort_by(|left, right| left.milestone_id.cmp(&right.milestone_id));
        reject_duplicate_milestone_rows(&rows)?;
        reject_unknown_matrix_rows(&roadmap_sequence_status, &rows)?;
        reject_missing_required_milestone_rows(&required_milestone_ids, &rows)?;
        let deterministic_digest = stable_digest(&MilestonePhysicalStatusMatrixDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::MilestonePhysicalStatusMatrix,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            roadmap_sequence_status: &roadmap_sequence_status,
            rows: &rows,
        })
        .map_err(|_| S0MilestoneMatrixBuildRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            roadmap_sequence_status,
            rows,
        })
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn roadmap_sequence_status(&self) -> &RoadmapSequenceStatusMatrix {
        &self.roadmap_sequence_status
    }

    pub fn rows(&self) -> &[MilestonePhysicalStatusRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0MilestoneMatrixParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0MilestoneMatrixParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedMilestonePhysicalStatusMatrixArtifact, S0MilestoneMatrixParseRejection>
    {
        let raw = serde_json::from_slice::<RawMilestonePhysicalStatusMatrix>(bytes)
            .map_err(|_| S0MilestoneMatrixParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0MilestoneMatrixParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::MilestonePhysicalStatusMatrix {
            return Err(S0MilestoneMatrixParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0MilestoneMatrixParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0MilestoneMatrixParseRejection::InvalidDigest)?;
        let roadmap_sequence_status = raw.roadmap_sequence_status.into_validated()?;
        let required_milestone_ids = roadmap_sequence_status
            .declarations()
            .iter()
            .map(|declaration| declaration.milestone_id().to_string())
            .collect::<Vec<_>>();
        let rows = raw
            .rows
            .into_iter()
            .map(|row| {
                let gate = roadmap_sequence_status
                    .gate_readiness_witness(&row.milestone_id)
                    .ok();
                row.into_validated(gate.as_ref())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let matrix = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            roadmap_sequence_status,
            required_milestone_ids,
            rows,
        )?;
        let row_count = matrix.rows().len() as u64;
        if matrix.envelope().deterministic_digest() != &expected_digest {
            return Err(S0MilestoneMatrixParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(matrix.rows())
            .map_err(|_| S0MilestoneMatrixParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedMilestonePhysicalStatusMatrixArtifact {
            matrix,
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
pub struct S0ValidatedMilestonePhysicalStatusMatrixArtifact {
    matrix: MilestonePhysicalStatusMatrix,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedMilestonePhysicalStatusMatrixArtifact {
    pub fn matrix(&self) -> &MilestonePhysicalStatusMatrix {
        &self.matrix
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0MilestoneAuditRejection {
    EmptyRequiredField,
    MissingMilestoneDeclaration,
    DuplicateMilestoneDeclaration,
    UnknownMilestoneReference,
    MissingGatePredecessorEvidence,
    GateReadinessBlockedBySequenceInconsistency,
    MissingEvidenceLane,
    MissingClaimFamily,
    PlatformGradeStatusRequiresGateReadiness,
    PhysicalDebtRequiresDeferredSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0MilestoneMatrixBuildRejection {
    EmptyRequiredField,
    MissingMilestoneRow,
    DuplicateMilestoneRow,
    UnknownMilestoneReference,
    MissingRequiredMilestoneRow,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    InvalidForbiddenClaim,
    SequenceRejected(S0MilestoneAuditRejection),
    RowRejected(S0MilestoneAuditRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0MilestoneMatrixParseRejection {
    NonParseable,
    SerializationFailed,
    SchemaVersionMismatch,
    ArtifactKindMismatch,
    InvalidDigest,
    InvalidEvidenceRef,
    InvalidDeferredSequence,
    InvalidForbiddenClaim,
    InvalidGeneratedMetadata,
    SequenceRejected(S0MilestoneAuditRejection),
    RowRejected(S0MilestoneAuditRejection),
    MatrixRejected(S0MilestoneMatrixBuildRejection),
    DeterministicDigestMismatch,
}

impl From<S0MilestoneAuditRejection> for S0MilestoneMatrixParseRejection {
    fn from(value: S0MilestoneAuditRejection) -> Self {
        Self::SequenceRejected(value)
    }
}

impl From<S0MilestoneMatrixBuildRejection> for S0MilestoneMatrixParseRejection {
    fn from(value: S0MilestoneMatrixBuildRejection) -> Self {
        Self::MatrixRejected(value)
    }
}

fn reject_duplicate_declarations(
    declarations: &[MilestoneStatusDeclaration],
) -> Result<(), S0MilestoneAuditRejection> {
    let mut seen = BTreeSet::new();
    if declarations
        .iter()
        .any(|declaration| !seen.insert(declaration.milestone_id()))
    {
        return Err(S0MilestoneAuditRejection::DuplicateMilestoneDeclaration);
    }
    Ok(())
}

fn reject_duplicate_milestone_rows(
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let mut seen = BTreeSet::new();
    if rows
        .iter()
        .any(|row| !seen.insert(row.milestone_id().to_string()))
    {
        return Err(S0MilestoneMatrixBuildRejection::DuplicateMilestoneRow);
    }
    Ok(())
}

fn reject_unknown_matrix_rows(
    sequence: &RoadmapSequenceStatusMatrix,
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let declared = sequence
        .declarations()
        .iter()
        .map(|declaration| declaration.milestone_id())
        .collect::<BTreeSet<_>>();
    if rows
        .iter()
        .any(|row| !declared.contains(row.milestone_id()))
    {
        return Err(S0MilestoneMatrixBuildRejection::UnknownMilestoneReference);
    }
    Ok(())
}

fn reject_missing_required_milestone_rows(
    required_milestone_ids: &[String],
    rows: &[MilestonePhysicalStatusRow],
) -> Result<(), S0MilestoneMatrixBuildRejection> {
    let present = rows
        .iter()
        .map(|row| row.milestone_id())
        .collect::<BTreeSet<_>>();
    if required_milestone_ids
        .iter()
        .any(|milestone_id| !present.contains(milestone_id.as_str()))
    {
        return Err(S0MilestoneMatrixBuildRejection::MissingRequiredMilestoneRow);
    }
    Ok(())
}

fn any_status_is_platform_grade(statuses: [S0PhysicalStatus; 5]) -> bool {
    statuses
        .into_iter()
        .any(|status| status == S0PhysicalStatus::PlatformGrade)
}

fn any_status_is_physical_debt(statuses: [S0PhysicalStatus; 5]) -> bool {
    statuses
        .into_iter()
        .any(|status| status == S0PhysicalStatus::PhysicalDebt)
}

fn require_non_empty(value: impl Into<String>) -> Result<String, S0MilestoneAuditRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0MilestoneAuditRejection::EmptyRequiredField);
    }
    Ok(value)
}

fn stable_digest(value: &impl Serialize) -> Result<S0StableDigest, serde_json::Error> {
    let canonical = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(&canonical);
    let digest = format!("sha256:{:x}", hasher.finalize());
    S0StableDigest::new(digest).map_err(|_| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid digest",
        ))
    })
}

#[derive(Serialize)]
struct MilestonePhysicalStatusMatrixDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    roadmap_sequence_status: &'a RoadmapSequenceStatusMatrix,
    rows: &'a [MilestonePhysicalStatusRow],
}

#[derive(Deserialize)]
struct RawMilestonePhysicalStatusMatrix {
    #[serde(flatten)]
    envelope: RawS0ArtifactEnvelope,
    roadmap_sequence_status: RawRoadmapSequenceStatusMatrix,
    rows: Vec<RawMilestonePhysicalStatusRow>,
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
    fn into_validated(self) -> Result<S0NondeterministicMetadata, S0MilestoneMatrixParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| S0MilestoneMatrixParseRejection::InvalidGeneratedMetadata)
    }
}

#[derive(Deserialize)]
struct RawRoadmapSequenceStatusMatrix {
    declarations: Vec<RawMilestoneStatusDeclaration>,
    prerequisite_edges: Vec<RawMilestonePrerequisiteEdge>,
}

impl RawRoadmapSequenceStatusMatrix {
    fn into_validated(
        self,
    ) -> Result<RoadmapSequenceStatusMatrix, S0MilestoneMatrixParseRejection> {
        let declarations = self
            .declarations
            .into_iter()
            .map(RawMilestoneStatusDeclaration::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let prerequisite_edges = self
            .prerequisite_edges
            .into_iter()
            .map(RawMilestonePrerequisiteEdge::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        RoadmapSequenceStatusMatrix::new(declarations, prerequisite_edges)
            .map_err(S0MilestoneMatrixParseRejection::SequenceRejected)
    }
}

#[derive(Deserialize)]
struct RawMilestoneStatusDeclaration {
    milestone_id: String,
    spec_status: MilestoneSpecStatus,
    closeout_status: MilestoneCloseoutStatus,
    predecessor_evidence: Vec<RawS0EvidenceRef>,
}

impl RawMilestoneStatusDeclaration {
    fn into_validated(self) -> Result<MilestoneStatusDeclaration, S0MilestoneMatrixParseRejection> {
        let predecessor_evidence = self
            .predecessor_evidence
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        MilestoneStatusDeclaration::new(
            self.milestone_id,
            self.spec_status,
            self.closeout_status,
            predecessor_evidence,
        )
        .map_err(S0MilestoneMatrixParseRejection::SequenceRejected)
    }
}

#[derive(Deserialize)]
struct RawMilestonePrerequisiteEdge {
    milestone_id: String,
    prerequisite_milestone_id: String,
    waiver_rationale: Option<PrerequisiteWaiverRationale>,
}

impl RawMilestonePrerequisiteEdge {
    fn into_validated(self) -> Result<MilestonePrerequisiteEdge, S0MilestoneMatrixParseRejection> {
        let edge =
            MilestonePrerequisiteEdge::new(self.milestone_id, self.prerequisite_milestone_id)
                .map_err(S0MilestoneMatrixParseRejection::SequenceRejected)?;
        Ok(match self.waiver_rationale {
            Some(rationale) => edge.waived(rationale),
            None => edge,
        })
    }
}

#[derive(Deserialize)]
struct RawMilestonePhysicalStatusRow {
    milestone_id: String,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
    evidence_lanes: Vec<String>,
    claim_families: Vec<SemanticPhysicalClaimFamily>,
    physical_substrate_status: S0PhysicalStatus,
    bounded_memory_status: S0PhysicalStatus,
    physical_integrity_status: S0PhysicalStatus,
    recovery_physics_status: S0PhysicalStatus,
    io_qos_status: S0PhysicalStatus,
    native_blob_chunk_status: Option<S0PhysicalStatus>,
    operator_security_status: Option<S0PhysicalStatus>,
    forbidden_claims: Vec<RawBackendForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    required_wording_cleanup: Vec<String>,
}

impl RawMilestonePhysicalStatusRow {
    fn into_validated(
        self,
        roadmap_gate: Option<&RoadmapGateReadinessWitness>,
    ) -> Result<MilestonePhysicalStatusRow, S0MilestoneMatrixParseRejection> {
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawBackendForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(Roadmap2SequenceId::new)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| S0MilestoneMatrixParseRejection::InvalidDeferredSequence)?;
        MilestonePhysicalStatusRow::new(
            self.milestone_id,
            self.semantic_capability_proven,
            self.closeout_or_planned_source,
            self.named_suite,
            self.evidence_lanes,
            self.claim_families,
            self.physical_substrate_status,
            self.bounded_memory_status,
            self.physical_integrity_status,
            self.recovery_physics_status,
            self.io_qos_status,
            self.native_blob_chunk_status,
            self.operator_security_status,
            forbidden_claims,
            deferred_s_sequences,
            self.required_wording_cleanup,
            roadmap_gate,
        )
        .map_err(S0MilestoneMatrixParseRejection::RowRejected)
    }
}

#[derive(Deserialize)]
struct RawBackendForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawBackendForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0MilestoneMatrixParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0MilestoneMatrixParseRejection::InvalidForbiddenClaim)
    }
}

#[derive(Deserialize)]
struct RawS0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawS0EvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0MilestoneMatrixParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0MilestoneMatrixParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}
