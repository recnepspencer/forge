#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ReadinessCertificationCounterFamily {
    ForegroundReservation,
    BackgroundPacing,
    QueueExecution,
    FlushDurability,
    LatencyInterference,
    LaterReadinessHandoff,
    SecureIoPreservation,
    AccessPolicy,
    PostAdmissionViolation,
    QualificationMatrix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ReadinessCertificationCounterStrength {
    Exact,
    Bounded,
    Sampled,
    Derived,
    CertificationOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ReadinessCertificationCounterEvidence {
    family: S6ReadinessCertificationCounterFamily,
    strength: S6ReadinessCertificationCounterStrength,
    observed_rows: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ReadinessCertificationProofSummary {
    checked_execution: bool,
    readmission_boundaries: usize,
    access_policy_rows: usize,
    post_admission_violation_rows: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ReadinessResidualDebtEvidenceKind {
    UnsupportedBackendProfile,
    UnavailableEvidence,
    DegradedBackendPosture,
    DeniedClaim,
    StaleEvidence,
    RebindRequired,
    ResidualQualificationDebt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ReadinessResidualDebtEvidenceRow {
    kind: S6ReadinessResidualDebtEvidenceKind,
    observed_claims: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ReadinessCertificationProofTopology {
    resolution_success: bool,
    lowering_success: bool,
    readiness_success: bool,
    execution_success: bool,
    resolved_current: bool,
    lowered_current: bool,
    readmitted_current: bool,
    ready_stage_execution_ready: bool,
    executed_stage_executed: bool,
    executed_current: bool,
    identity_bound: bool,
    lane_binding_bound: bool,
    readiness_readmission_boundaries: usize,
    executed_readmission_boundaries: usize,
    freshness_readmitted_boundaries: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6MaterializedCertificationAdoptionReceipt {
    canonical_execution_identity_tag: u64,
    proof_execution_identity_tag: u64,
    canonical_lane_binding_mask: u16,
    proof_lane_binding_mask: u16,
    profile_count: usize,
    profile_boundary_certification_only: bool,
    performance_receipt_count: usize,
    counter_strengths: Vec<S6ReadinessCertificationCounterEvidence>,
    canonical_access_policy_rows: usize,
    canonical_post_admission_violation_rows: usize,
    proof: S6ReadinessCertificationProofSummary,
    proof_topology: S6ReadinessCertificationProofTopology,
    residual_debt_rows: Vec<S6ReadinessResidualDebtEvidenceRow>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6MaterializedCertificationAdoptionDenial {
    CertificationEvidenceCannotStrengthenRuntimeAuthority,
    CertificationEvidenceCannotSatisfyCloseout,
}

pub const fn reject_materialized_s6_certification_as_runtime_authority(
) -> S6MaterializedCertificationAdoptionDenial {
    S6MaterializedCertificationAdoptionDenial::CertificationEvidenceCannotStrengthenRuntimeAuthority
}

impl S6ReadinessCertificationCounterEvidence {
    pub const fn new(
        family: S6ReadinessCertificationCounterFamily,
        strength: S6ReadinessCertificationCounterStrength,
        observed_rows: usize,
    ) -> Self {
        Self {
            family,
            strength,
            observed_rows,
        }
    }

    pub const fn family(&self) -> S6ReadinessCertificationCounterFamily {
        self.family
    }

    pub const fn strength(&self) -> S6ReadinessCertificationCounterStrength {
        self.strength
    }

    pub const fn observed_rows(&self) -> usize {
        self.observed_rows
    }
}

impl S6ReadinessCertificationProofSummary {
    pub const fn new(
        checked_execution: bool,
        readmission_boundaries: usize,
        access_policy_rows: usize,
        post_admission_violation_rows: usize,
    ) -> Self {
        Self {
            checked_execution,
            readmission_boundaries,
            access_policy_rows,
            post_admission_violation_rows,
        }
    }

    pub const fn checked_execution(&self) -> bool {
        self.checked_execution
    }

    pub const fn readmission_boundaries(&self) -> usize {
        self.readmission_boundaries
    }

    pub const fn access_policy_rows(&self) -> usize {
        self.access_policy_rows
    }

    pub const fn post_admission_violation_rows(&self) -> usize {
        self.post_admission_violation_rows
    }
}

impl S6ReadinessResidualDebtEvidenceRow {
    pub const fn new(kind: S6ReadinessResidualDebtEvidenceKind, observed_claims: usize) -> Self {
        Self {
            kind,
            observed_claims,
        }
    }

    pub const fn kind(&self) -> S6ReadinessResidualDebtEvidenceKind {
        self.kind
    }

    pub const fn observed_claims(&self) -> usize {
        self.observed_claims
    }
}

impl S6ReadinessCertificationProofTopology {
    pub const fn new(
        resolution_success: bool,
        lowering_success: bool,
        readiness_success: bool,
        execution_success: bool,
        resolved_current: bool,
        lowered_current: bool,
        readmitted_current: bool,
        ready_stage_execution_ready: bool,
        executed_stage_executed: bool,
        executed_current: bool,
        identity_bound: bool,
        lane_binding_bound: bool,
        readiness_readmission_boundaries: usize,
        executed_readmission_boundaries: usize,
        freshness_readmitted_boundaries: usize,
    ) -> Self {
        Self {
            resolution_success,
            lowering_success,
            readiness_success,
            execution_success,
            resolved_current,
            lowered_current,
            readmitted_current,
            ready_stage_execution_ready,
            executed_stage_executed,
            executed_current,
            identity_bound,
            lane_binding_bound,
            readiness_readmission_boundaries,
            executed_readmission_boundaries,
            freshness_readmitted_boundaries,
        }
    }

    pub const fn readiness_readmission_boundaries(&self) -> usize {
        self.readiness_readmission_boundaries
    }

    pub const fn executed_readmission_boundaries(&self) -> usize {
        self.executed_readmission_boundaries
    }

    pub const fn freshness_readmitted_boundaries(&self) -> usize {
        self.freshness_readmitted_boundaries
    }

    pub const fn is_checked_for_closeout(
        &self,
        proof: S6ReadinessCertificationProofSummary,
    ) -> bool {
        self.resolution_success
            && self.lowering_success
            && self.readiness_success
            && self.execution_success
            && self.resolved_current
            && self.lowered_current
            && self.readmitted_current
            && self.ready_stage_execution_ready
            && self.executed_stage_executed
            && self.executed_current
            && self.identity_bound
            && self.lane_binding_bound
            && self.readiness_readmission_boundaries == proof.readmission_boundaries()
            && self.executed_readmission_boundaries == proof.readmission_boundaries()
            && self.freshness_readmitted_boundaries == proof.readmission_boundaries()
    }
}

impl S6MaterializedCertificationAdoptionReceipt {
    pub const fn canonical_execution_identity_tag(&self) -> u64 {
        self.canonical_execution_identity_tag
    }

    pub const fn proof_execution_identity_tag(&self) -> u64 {
        self.proof_execution_identity_tag
    }

    pub const fn canonical_lane_binding_mask(&self) -> u16 {
        self.canonical_lane_binding_mask
    }

    pub const fn proof_lane_binding_mask(&self) -> u16 {
        self.proof_lane_binding_mask
    }

    pub const fn profile_count(&self) -> usize {
        self.profile_count
    }

    pub const fn profile_boundary_certification_only(&self) -> bool {
        self.profile_boundary_certification_only
    }

    pub const fn performance_receipt_count(&self) -> usize {
        self.performance_receipt_count
    }

    pub fn counter_strengths(&self) -> &[S6ReadinessCertificationCounterEvidence] {
        &self.counter_strengths
    }

    pub const fn canonical_access_policy_rows(&self) -> usize {
        self.canonical_access_policy_rows
    }

    pub const fn canonical_post_admission_violation_rows(&self) -> usize {
        self.canonical_post_admission_violation_rows
    }

    pub const fn proof(&self) -> S6ReadinessCertificationProofSummary {
        self.proof
    }

    pub const fn proof_topology(&self) -> S6ReadinessCertificationProofTopology {
        self.proof_topology
    }

    pub fn residual_debt_rows(&self) -> &[S6ReadinessResidualDebtEvidenceRow] {
        &self.residual_debt_rows
    }
}
