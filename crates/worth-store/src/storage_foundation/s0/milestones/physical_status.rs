use super::super::capability::{BackendForbiddenClaim, Roadmap2SequenceId};
use super::sequence_status::RoadmapGateReadinessWitness;
use super::validation::{
    require_non_empty, validate_physical_debt_status, validate_platform_grade_status,
    S0MilestoneAuditRejection,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

struct ValidatedMilestonePhysicalStatusText {
    milestone_id: String,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
}

struct MilestonePhysicalStatusValidation<'a> {
    milestone_id: &'a str,
    statuses: [S0PhysicalStatus; 5],
    native_blob_chunk_status: Option<S0PhysicalStatus>,
    operator_security_status: Option<S0PhysicalStatus>,
    deferred_s_sequences: &'a [Roadmap2SequenceId],
    roadmap_gate: Option<&'a RoadmapGateReadinessWitness>,
}

struct MilestonePhysicalStatusRowConstruction {
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
        let required_text = validate_required_text_fields(
            milestone_id,
            semantic_capability_proven,
            closeout_or_planned_source,
            named_suite,
        )?;
        validate_required_collections(&evidence_lanes, &claim_families)?;
        let physical_statuses = [
            physical_substrate_status,
            bounded_memory_status,
            physical_integrity_status,
            recovery_physics_status,
            io_qos_status,
        ];
        validate_physical_statuses(MilestonePhysicalStatusValidation {
            milestone_id: &required_text.milestone_id,
            statuses: physical_statuses,
            native_blob_chunk_status,
            operator_security_status,
            deferred_s_sequences: &deferred_s_sequences,
            roadmap_gate,
        })?;
        Ok(construct_row(MilestonePhysicalStatusRowConstruction {
            milestone_id: required_text.milestone_id,
            semantic_capability_proven: required_text.semantic_capability_proven,
            closeout_or_planned_source: required_text.closeout_or_planned_source,
            named_suite: required_text.named_suite,
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
        }))
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

fn validate_required_text_fields(
    milestone_id: impl Into<String>,
    semantic_capability_proven: impl Into<String>,
    closeout_or_planned_source: impl Into<String>,
    named_suite: impl Into<String>,
) -> Result<ValidatedMilestonePhysicalStatusText, S0MilestoneAuditRejection> {
    let milestone_id = require_non_empty(milestone_id)?;
    let semantic_capability_proven = require_non_empty(semantic_capability_proven)?;
    let closeout_or_planned_source = require_non_empty(closeout_or_planned_source)?;
    let named_suite = require_non_empty(named_suite)?;
    Ok(ValidatedMilestonePhysicalStatusText {
        milestone_id,
        semantic_capability_proven,
        closeout_or_planned_source,
        named_suite,
    })
}

fn validate_required_collections(
    evidence_lanes: &[String],
    claim_families: &[SemanticPhysicalClaimFamily],
) -> Result<(), S0MilestoneAuditRejection> {
    if evidence_lanes.is_empty() {
        return Err(S0MilestoneAuditRejection::MissingEvidenceLane);
    }
    if claim_families.is_empty() {
        return Err(S0MilestoneAuditRejection::MissingClaimFamily);
    }
    Ok(())
}

fn validate_physical_statuses(
    validation: MilestonePhysicalStatusValidation<'_>,
) -> Result<(), S0MilestoneAuditRejection> {
    validate_platform_grade_status(
        validation.milestone_id,
        validation.statuses,
        validation.native_blob_chunk_status,
        validation.operator_security_status,
        validation.roadmap_gate,
    )?;
    validate_physical_debt_status(
        validation.statuses,
        validation.native_blob_chunk_status,
        validation.operator_security_status,
        validation.deferred_s_sequences.is_empty(),
    )
}

fn construct_row(fields: MilestonePhysicalStatusRowConstruction) -> MilestonePhysicalStatusRow {
    MilestonePhysicalStatusRow {
        milestone_id: fields.milestone_id,
        semantic_capability_proven: fields.semantic_capability_proven,
        closeout_or_planned_source: fields.closeout_or_planned_source,
        named_suite: fields.named_suite,
        evidence_lanes: fields.evidence_lanes,
        claim_families: fields.claim_families,
        physical_substrate_status: fields.physical_substrate_status,
        bounded_memory_status: fields.bounded_memory_status,
        physical_integrity_status: fields.physical_integrity_status,
        recovery_physics_status: fields.recovery_physics_status,
        io_qos_status: fields.io_qos_status,
        native_blob_chunk_status: fields.native_blob_chunk_status,
        operator_security_status: fields.operator_security_status,
        forbidden_claims: fields.forbidden_claims,
        deferred_s_sequences: fields.deferred_s_sequences,
        required_wording_cleanup: fields.required_wording_cleanup,
    }
}
