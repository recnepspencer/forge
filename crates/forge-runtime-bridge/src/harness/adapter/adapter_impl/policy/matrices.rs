use crate::identity::{BridgeIdentity, LoweredExecutionPolicyIdentityTag};

pub(in crate::harness::adapter::adapter_impl) struct PolicyCertificationMatrix {
    rows: Vec<PolicyCertificationRow>,
}

impl PolicyCertificationMatrix {
    pub(super) fn from_admitted_rows(rows: Vec<AdmittedPolicyMatrixRow>) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(PolicyCertificationRow::Admitted)
                .collect(),
        }
    }

    pub(super) fn from_rejection_rows(rows: Vec<PolicyRejectionMatrixRow>) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(PolicyCertificationRow::Rejection)
                .collect(),
        }
    }

    pub(super) fn rows(&self) -> &[PolicyCertificationRow] {
        &self.rows
    }
}

pub(super) enum PolicyCertificationRow {
    Admitted(AdmittedPolicyMatrixRow),
    Rejection(PolicyRejectionMatrixRow),
}

pub(super) struct AdmittedPolicyMatrixRow {
    label: String,
    declaration_identity: crate::facade::BridgePolicyDeclarationIdentity,
    request_kind: crate::facade::BridgeRequestKind,
    execution_class: crate::facade::BridgeExecutionPolicyClass,
    diagnostics_tier: crate::facade::BridgeDiagnosticsTier,
    route_artifacts: bool,
    replay_artifacts: bool,
    policy_digest: String,
    lowered_policy_digest: String,
    provenance_digest: String,
    replay_digest: String,
}

impl AdmittedPolicyMatrixRow {
    pub(super) fn from_evidence(evidence: AdmittedPolicyMatrixRowEvidence) -> Self {
        Self {
            label: evidence.label,
            declaration_identity: evidence.declaration_identity,
            request_kind: evidence.request_kind,
            execution_class: evidence.execution_class,
            diagnostics_tier: evidence.diagnostics_tier,
            route_artifacts: evidence.route_artifacts,
            replay_artifacts: evidence.replay_artifacts,
            policy_digest: evidence.policy_digest,
            lowered_policy_digest: evidence.lowered_policy_digest,
            provenance_digest: evidence.provenance_digest,
            replay_digest: evidence.replay_digest,
        }
    }

    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn declaration_identity(&self) -> &crate::facade::BridgePolicyDeclarationIdentity {
        &self.declaration_identity
    }

    pub(super) fn request_kind(&self) -> crate::facade::BridgeRequestKind {
        self.request_kind
    }

    pub(super) fn execution_class(&self) -> crate::facade::BridgeExecutionPolicyClass {
        self.execution_class
    }

    pub(super) fn diagnostics_tier(&self) -> crate::facade::BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub(super) fn route_artifacts(&self) -> bool {
        self.route_artifacts
    }

    pub(super) fn replay_artifacts(&self) -> bool {
        self.replay_artifacts
    }

    pub(super) fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub(super) fn lowered_policy_digest(&self) -> &str {
        &self.lowered_policy_digest
    }

    pub(super) fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }

    pub(super) fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
}

pub(super) struct AdmittedPolicyMatrixRowEvidence {
    pub(super) label: String,
    pub(super) declaration_identity: crate::facade::BridgePolicyDeclarationIdentity,
    pub(super) request_kind: crate::facade::BridgeRequestKind,
    pub(super) execution_class: crate::facade::BridgeExecutionPolicyClass,
    pub(super) diagnostics_tier: crate::facade::BridgeDiagnosticsTier,
    pub(super) route_artifacts: bool,
    pub(super) replay_artifacts: bool,
    pub(super) policy_digest: String,
    pub(super) lowered_policy_digest: String,
    pub(super) provenance_digest: String,
    pub(super) replay_digest: String,
}

pub(super) struct PolicyRejectionMatrixRow {
    label: String,
    declaration_identity: crate::facade::BridgePolicyDeclarationIdentity,
    failure_kind: crate::facade::BridgePolicyRejectionKind,
    stage: crate::facade::BridgePolicyRejectionStage,
    field_kind: crate::facade::BridgePolicyFieldKind,
    primary_source: crate::facade::BridgePolicySourceClass,
    secondary_source: crate::facade::BridgePolicySourceClass,
    digest: String,
}

impl PolicyRejectionMatrixRow {
    pub(super) fn from_evidence(evidence: PolicyRejectionMatrixRowEvidence) -> Self {
        Self {
            label: evidence.label,
            declaration_identity: evidence.declaration_identity,
            failure_kind: evidence.failure_kind,
            stage: evidence.stage,
            field_kind: evidence.field_kind,
            primary_source: evidence.primary_source,
            secondary_source: evidence.secondary_source,
            digest: evidence.digest,
        }
    }

    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn declaration_identity(&self) -> &crate::facade::BridgePolicyDeclarationIdentity {
        &self.declaration_identity
    }

    pub(super) fn failure_kind(&self) -> crate::facade::BridgePolicyRejectionKind {
        self.failure_kind
    }

    pub(super) fn stage(&self) -> crate::facade::BridgePolicyRejectionStage {
        self.stage
    }

    pub(super) fn field_kind(&self) -> crate::facade::BridgePolicyFieldKind {
        self.field_kind
    }

    pub(super) fn primary_source(&self) -> crate::facade::BridgePolicySourceClass {
        self.primary_source
    }

    pub(super) fn secondary_source(&self) -> crate::facade::BridgePolicySourceClass {
        self.secondary_source
    }

    pub(super) fn digest(&self) -> &str {
        &self.digest
    }
}

pub(super) struct PolicyRejectionMatrixRowEvidence {
    pub(super) label: String,
    pub(super) declaration_identity: crate::facade::BridgePolicyDeclarationIdentity,
    pub(super) failure_kind: crate::facade::BridgePolicyRejectionKind,
    pub(super) stage: crate::facade::BridgePolicyRejectionStage,
    pub(super) field_kind: crate::facade::BridgePolicyFieldKind,
    pub(super) primary_source: crate::facade::BridgePolicySourceClass,
    pub(super) secondary_source: crate::facade::BridgePolicySourceClass,
    pub(super) digest: String,
}

pub(in crate::harness::adapter::adapter_impl) struct RequestPolicyMatrix {
    branch_local_resolution: Option<crate::facade::BridgeTruthViewPolicyResolution>,
    historical_resolution: Option<crate::facade::BridgeTruthViewPolicyResolution>,
    rows: Vec<RequestPolicyMatrixRow>,
}

impl RequestPolicyMatrix {
    pub(super) fn empty() -> Self {
        Self {
            branch_local_resolution: None,
            historical_resolution: None,
            rows: Vec::new(),
        }
    }

    pub(super) fn new(rows: Vec<RequestPolicyMatrixRow>) -> Self {
        Self {
            branch_local_resolution: None,
            historical_resolution: None,
            rows,
        }
    }

    pub(super) fn with_truth_view_resolutions(
        branch_local_resolution: crate::facade::BridgeTruthViewPolicyResolution,
        historical_resolution: crate::facade::BridgeTruthViewPolicyResolution,
        rows: Vec<RequestPolicyMatrixRow>,
    ) -> Self {
        Self {
            branch_local_resolution: Some(branch_local_resolution),
            historical_resolution: Some(historical_resolution),
            rows,
        }
    }

    pub(super) fn branch_local_resolution(
        &self,
    ) -> Option<&crate::facade::BridgeTruthViewPolicyResolution> {
        self.branch_local_resolution.as_ref()
    }

    pub(super) fn historical_resolution(
        &self,
    ) -> Option<&crate::facade::BridgeTruthViewPolicyResolution> {
        self.historical_resolution.as_ref()
    }

    pub(super) fn rows(&self) -> &[RequestPolicyMatrixRow] {
        &self.rows
    }
}

pub(super) struct RequestPolicyMatrixRow {
    provenance_row: crate::facade::BridgePolicyProvenanceReportRow,
    route_planning_policy_digest: String,
    semantic_route_planning_policy_digest: String,
}

impl RequestPolicyMatrixRow {
    pub(super) fn new(
        provenance_row: crate::facade::BridgePolicyProvenanceReportRow,
        route_planning_policy_digest: &str,
        semantic_route_planning_policy_digest: String,
    ) -> Self {
        Self {
            provenance_row,
            route_planning_policy_digest: route_planning_policy_digest.to_string(),
            semantic_route_planning_policy_digest,
        }
    }

    pub(super) fn provenance_row(&self) -> &crate::facade::BridgePolicyProvenanceReportRow {
        &self.provenance_row
    }

    pub(super) fn route_planning_policy_digest(&self) -> &str {
        &self.route_planning_policy_digest
    }

    pub(super) fn semantic_route_planning_policy_digest(&self) -> &str {
        &self.semantic_route_planning_policy_digest
    }
}

pub(in crate::harness::adapter::adapter_impl) struct RoutePolicyMatrix {
    rows: Vec<RoutePolicyMatrixRow>,
}

impl RoutePolicyMatrix {
    pub(super) fn new(rows: Vec<RoutePolicyMatrixRow>) -> Self {
        Self { rows }
    }

    pub(super) fn rows(&self) -> &[RoutePolicyMatrixRow] {
        &self.rows
    }
}

pub(super) struct RoutePolicyMatrixRow {
    label: String,
    route_planning_policy_digest: String,
    semantic_route_planning_policy_digest: String,
    lowered_policy_identity: BridgeIdentity<LoweredExecutionPolicyIdentityTag>,
    execution_class: crate::facade::BridgeExecutionPolicyClass,
    diagnostics_tier: crate::facade::BridgeDiagnosticsTier,
    route_artifacts: bool,
    replay_artifacts: bool,
}

impl RoutePolicyMatrixRow {
    pub(super) fn from_evidence(evidence: RoutePolicyMatrixRowEvidence) -> Self {
        Self {
            label: evidence.label,
            route_planning_policy_digest: evidence.route_planning_policy_digest,
            semantic_route_planning_policy_digest: evidence.semantic_route_planning_policy_digest,
            lowered_policy_identity: evidence.lowered_policy_identity,
            execution_class: evidence.execution_class,
            diagnostics_tier: evidence.diagnostics_tier,
            route_artifacts: evidence.route_artifacts,
            replay_artifacts: evidence.replay_artifacts,
        }
    }

    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn route_planning_policy_digest(&self) -> &str {
        &self.route_planning_policy_digest
    }

    pub(super) fn semantic_route_planning_policy_digest(&self) -> &str {
        &self.semantic_route_planning_policy_digest
    }

    pub(super) fn lowered_policy_identity(
        &self,
    ) -> &BridgeIdentity<LoweredExecutionPolicyIdentityTag> {
        &self.lowered_policy_identity
    }

    pub(super) fn execution_class(&self) -> crate::facade::BridgeExecutionPolicyClass {
        self.execution_class
    }

    pub(super) fn diagnostics_tier(&self) -> crate::facade::BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub(super) fn route_artifacts(&self) -> bool {
        self.route_artifacts
    }

    pub(super) fn replay_artifacts(&self) -> bool {
        self.replay_artifacts
    }
}

pub(super) struct RoutePolicyMatrixRowEvidence {
    pub(super) label: String,
    pub(super) route_planning_policy_digest: String,
    pub(super) semantic_route_planning_policy_digest: String,
    pub(super) lowered_policy_identity: BridgeIdentity<LoweredExecutionPolicyIdentityTag>,
    pub(super) execution_class: crate::facade::BridgeExecutionPolicyClass,
    pub(super) diagnostics_tier: crate::facade::BridgeDiagnosticsTier,
    pub(super) route_artifacts: bool,
    pub(super) replay_artifacts: bool,
}
