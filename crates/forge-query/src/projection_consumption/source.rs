use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::query_context::{QueryContextExecutionArtifact, QueryContextExecutionFamily};
use crate::runtime::{
    ForgeQueryMutationTargetClass, ForgeQueryReadExecutionEngine, ForgeQueryReadReceipt,
    ForgeQueryWriteReceipt,
};
use forge_relational::facade::grouped_truth::{
    RelationalAuthoritativeRowSetArtifact, RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    BridgeGroupedTruthViewArtifact, BridgeMaterializedRowSetArtifact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionSourceFamily {
    QueryReadReceipt,
    QueryWriteReceipt,
    QueryContextExecution,
    RelationalRowSet,
    RelationalGroupedProjection,
    BridgeTruthViewRowSet,
    BridgeGroupedTruthView,
}

impl ProjectionSourceFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryReadReceipt => "query_read_receipt",
            Self::QueryWriteReceipt => "query_write_receipt",
            Self::QueryContextExecution => "query_context_execution",
            Self::RelationalRowSet => "relational_row_set",
            Self::RelationalGroupedProjection => "relational_grouped_projection",
            Self::BridgeTruthViewRowSet => "bridge_truth_view_row_set",
            Self::BridgeGroupedTruthView => "bridge_grouped_truth_view",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionSourceExecutionPosture {
    Current,
    Branch,
    Historical,
    PreviewDerived,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ProjectionWriteReceiptCapabilities {
    has_target_identity: bool,
    has_source_reference: bool,
    has_effect_continuity: bool,
    has_relation_endpoint: bool,
}

impl ProjectionWriteReceiptCapabilities {
    pub(crate) fn has_target_identity(&self) -> bool {
        self.has_target_identity
    }

    pub(crate) fn has_source_reference(&self) -> bool {
        self.has_source_reference
    }

    pub(crate) fn has_effect_continuity(&self) -> bool {
        self.has_effect_continuity
    }

    pub(crate) fn has_relation_endpoint(&self) -> bool {
        self.has_relation_endpoint
    }

    pub(crate) fn synthetic(
        has_target_identity: bool,
        has_source_reference: bool,
        has_effect_continuity: bool,
        has_relation_endpoint: bool,
    ) -> Self {
        Self {
            has_target_identity,
            has_source_reference,
            has_effect_continuity,
            has_relation_endpoint,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        has_target_identity: bool,
        has_source_reference: bool,
        has_effect_continuity: bool,
        has_relation_endpoint: bool,
    ) -> Self {
        Self::synthetic(
            has_target_identity,
            has_source_reference,
            has_effect_continuity,
            has_relation_endpoint,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionSourceCapabilityProfile {
    QueryReadReceipt {
        execution_posture: ProjectionSourceExecutionPosture,
    },
    QueryWriteReceipt {
        capabilities: ProjectionWriteReceiptCapabilities,
    },
    QueryContextExecution {
        execution_posture: ProjectionSourceExecutionPosture,
    },
    RelationalRowSet,
    RelationalGroupedProjection,
    BridgeTruthViewRowSet,
    BridgeGroupedTruthView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceReferenceIdentity {
    label: &'static str,
    identity: String,
}

impl ProjectionSourceReferenceIdentity {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn synthetic(label: &'static str, identity: impl Into<String>) -> Self {
        Self {
            label,
            identity: identity.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(label: &'static str, identity: impl Into<String>) -> Self {
        Self::synthetic(label, identity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSource {
    family: ProjectionSourceFamily,
    capability_profile: ProjectionSourceCapabilityProfile,
    query_digest: Option<String>,
    basis_digest: Option<String>,
    result_digest: Option<String>,
    result_shape_digest: Option<String>,
    source_identity: String,
    source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
}

impl ProjectionConsumptionSource {
    pub fn from_read_receipt(
        receipt: &ForgeQueryReadReceipt,
        result_shape: &CanonicalResultShapeArtifact,
    ) -> Self {
        Self {
            source_reference_identities: Vec::new(),
            family: ProjectionSourceFamily::QueryReadReceipt,
            capability_profile: ProjectionSourceCapabilityProfile::QueryReadReceipt {
                execution_posture: execution_posture_from_read_engine(receipt.execution_engine()),
            },
            query_digest: Some(receipt.query_digest().to_string()),
            basis_digest: Some(receipt.basis_digest().to_string()),
            result_digest: Some(receipt.result_digest().to_string()),
            result_shape_digest: Some(result_shape.digest().as_str().to_string()),
            source_identity: receipt.read_graph_digest().to_string(),
            materialized_fact_posture: receipt.materialized_fact_posture().cloned(),
        }
    }

    pub fn from_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        let resolved_target = receipt.target_evidence().resolved();
        let mut source_reference_identities = Vec::new();
        if let Some(provenance) = receipt.provenance_evidence() {
            source_reference_identities.push(ProjectionSourceReferenceIdentity {
                label: "bridge_provenance_execution_record",
                identity: provenance.execution_record_digest().to_string(),
            });
        }
        if let Some(symbolic_reference) = receipt.symbolic_target_reference_evidence() {
            source_reference_identities.push(ProjectionSourceReferenceIdentity {
                label: "symbolic_target_reference",
                identity: symbolic_reference.symbol().to_string(),
            });
        }
        Self {
            family: ProjectionSourceFamily::QueryWriteReceipt,
            capability_profile: ProjectionSourceCapabilityProfile::QueryWriteReceipt {
                capabilities: ProjectionWriteReceiptCapabilities {
                    has_target_identity: receipt.target_entity_identity().is_some(),
                    has_source_reference: receipt.provenance_evidence().is_some()
                        || receipt.symbolic_target_reference_evidence().is_some(),
                    has_effect_continuity: receipt.continuity_mutation_evidence().is_some(),
                    has_relation_endpoint: matches!(
                        resolved_target.target_class(),
                        ForgeQueryMutationTargetClass::Entity
                    ) && resolved_target.collection().is_some()
                        && resolved_target.entity_identity().is_some(),
                },
            },
            query_digest: None,
            basis_digest: Some(receipt.snapshot_token().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: receipt.commit_identity().to_string(),
            source_reference_identities,
            materialized_fact_posture: None,
        }
    }

    pub fn from_query_context_execution(execution: &QueryContextExecutionArtifact) -> Self {
        let mut source_reference_identities = Vec::new();
        if let Some(materialization_path_identity) = execution.materialization_path_identity() {
            source_reference_identities.push(ProjectionSourceReferenceIdentity {
                label: "query_context_materialization_path",
                identity: materialization_path_identity.to_string(),
            });
        }
        if let Some(preview_provenance_identity) = execution.preview_provenance_identity() {
            source_reference_identities.push(ProjectionSourceReferenceIdentity {
                label: "query_context_preview_provenance",
                identity: preview_provenance_identity.to_string(),
            });
        }
        Self {
            source_reference_identities,
            family: ProjectionSourceFamily::QueryContextExecution,
            capability_profile: ProjectionSourceCapabilityProfile::QueryContextExecution {
                execution_posture: execution_posture_from_query_context_family(execution.family()),
            },
            query_digest: Some(execution.query_digest().to_string()),
            basis_digest: Some(execution.basis_digest().to_string()),
            result_digest: Some(execution.result_digest().to_string()),
            result_shape_digest: Some(execution.result_shape_digest().to_string()),
            source_identity: execution
                .materialization_path_identity()
                .unwrap_or_else(|| execution.family().as_str())
                .to_string(),
            materialized_fact_posture: execution.materialized_fact_posture().cloned(),
        }
    }

    pub fn from_relational_row_set(row_set: &RelationalAuthoritativeRowSetArtifact) -> Self {
        Self {
            source_reference_identities: Vec::new(),
            family: ProjectionSourceFamily::RelationalRowSet,
            capability_profile: ProjectionSourceCapabilityProfile::RelationalRowSet,
            query_digest: None,
            basis_digest: Some(row_set.snapshot_identity().as_str().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: row_set.digest().as_str().to_string(),
            materialized_fact_posture: None,
        }
    }

    pub fn from_relational_grouped_projection(
        grouped_projection: &RelationalGroupedProjectionArtifact,
    ) -> Self {
        Self {
            source_reference_identities: Vec::new(),
            family: ProjectionSourceFamily::RelationalGroupedProjection,
            capability_profile: ProjectionSourceCapabilityProfile::RelationalGroupedProjection,
            query_digest: None,
            basis_digest: Some(grouped_projection.snapshot_identity().as_str().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: grouped_projection.digest().as_str().to_string(),
            materialized_fact_posture: None,
        }
    }

    pub fn from_bridge_truth_view_row_set(row_set: &BridgeMaterializedRowSetArtifact) -> Self {
        Self {
            source_reference_identities: Vec::new(),
            family: ProjectionSourceFamily::BridgeTruthViewRowSet,
            capability_profile: ProjectionSourceCapabilityProfile::BridgeTruthViewRowSet,
            query_digest: None,
            basis_digest: Some(row_set.basis_snapshot_identity().as_str().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: row_set.digest().as_str().to_string(),
            materialized_fact_posture: None,
        }
    }

    pub fn from_bridge_grouped_truth_view(
        grouped_truth_view: &BridgeGroupedTruthViewArtifact,
    ) -> Self {
        Self {
            source_reference_identities: Vec::new(),
            family: ProjectionSourceFamily::BridgeGroupedTruthView,
            capability_profile: ProjectionSourceCapabilityProfile::BridgeGroupedTruthView,
            query_digest: None,
            basis_digest: Some(
                grouped_truth_view
                    .basis_snapshot_identity()
                    .as_str()
                    .to_string(),
            ),
            result_digest: None,
            result_shape_digest: None,
            source_identity: grouped_truth_view.digest().as_str().to_string(),
            materialized_fact_posture: None,
        }
    }

    pub fn family(&self) -> ProjectionSourceFamily {
        self.family
    }

    pub(crate) fn capability_profile(&self) -> &ProjectionSourceCapabilityProfile {
        &self.capability_profile
    }

    pub fn query_digest(&self) -> Option<&str> {
        self.query_digest.as_deref()
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn result_digest(&self) -> Option<&str> {
        self.result_digest.as_deref()
    }

    pub fn result_shape_digest(&self) -> Option<&str> {
        self.result_shape_digest.as_deref()
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn source_reference_identities(&self) -> &[ProjectionSourceReferenceIdentity] {
        &self.source_reference_identities
    }

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub(crate) fn synthetic_for_certification(
        family: ProjectionSourceFamily,
        capability_profile: ProjectionSourceCapabilityProfile,
        source_identity: impl Into<String>,
        source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    ) -> Self {
        Self {
            family,
            capability_profile,
            query_digest: None,
            basis_digest: None,
            result_digest: None,
            result_shape_digest: None,
            source_identity: source_identity.into(),
            source_reference_identities,
            materialized_fact_posture: None,
        }
    }

    pub(crate) fn intent_admission_certification(
        family: ProjectionSourceFamily,
        capability_profile: ProjectionSourceCapabilityProfile,
        query_digest: Option<String>,
        basis_digest: Option<String>,
        result_digest: Option<String>,
        result_shape_digest: Option<String>,
        source_identity: impl Into<String>,
        source_reference_identities: Vec<ProjectionSourceReferenceIdentity>,
    ) -> Self {
        Self {
            family,
            capability_profile,
            query_digest,
            basis_digest,
            result_digest,
            result_shape_digest,
            source_identity: source_identity.into(),
            source_reference_identities,
            materialized_fact_posture: None,
        }
    }
}

fn execution_posture_from_read_engine(
    engine: &ForgeQueryReadExecutionEngine,
) -> ProjectionSourceExecutionPosture {
    match engine {
        ForgeQueryReadExecutionEngine::QueryRuntimeCurrent => {
            ProjectionSourceExecutionPosture::Current
        }
        ForgeQueryReadExecutionEngine::QueryRuntimeBranch => {
            ProjectionSourceExecutionPosture::Branch
        }
        ForgeQueryReadExecutionEngine::QueryRuntimeHistorical => {
            ProjectionSourceExecutionPosture::Historical
        }
        ForgeQueryReadExecutionEngine::QueryRuntimePreviewDerived => {
            ProjectionSourceExecutionPosture::PreviewDerived
        }
    }
}

fn execution_posture_from_query_context_family(
    family: &QueryContextExecutionFamily,
) -> ProjectionSourceExecutionPosture {
    match family {
        QueryContextExecutionFamily::RuntimeCurrent => ProjectionSourceExecutionPosture::Current,
        QueryContextExecutionFamily::RuntimeBranch => ProjectionSourceExecutionPosture::Branch,
        QueryContextExecutionFamily::HistoricalMaterialized => {
            ProjectionSourceExecutionPosture::Historical
        }
        QueryContextExecutionFamily::PreviewDerivedHistorical => {
            ProjectionSourceExecutionPosture::PreviewDerived
        }
    }
}

#[cfg(test)]
#[path = "tests/source_test_support.rs"]
mod test_support;
