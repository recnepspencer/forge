use super::super::artifacts::{
    S0ArtifactValidationCostSurface, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::physical_matrix::MilestonePhysicalStatusMatrix;
use super::physical_status::{
    MilestonePhysicalStatusRow, S0PhysicalStatus, SemanticPhysicalClaimFamily,
};
use super::sequence_status::{
    MilestoneCloseoutStatus, MilestonePrerequisiteEdge, MilestoneSpecStatus,
    MilestoneStatusDeclaration, PrerequisiteWaiverRationale, RoadmapSequenceStatusMatrix,
};
use super::validation::{
    S0MilestoneMatrixParseRejection, S0ValidatedMilestonePhysicalStatusMatrixArtifact,
};
use serde::Deserialize;

impl MilestonePhysicalStatusMatrix {
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
        roadmap_gate: Option<&super::sequence_status::RoadmapGateReadinessWitness>,
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
