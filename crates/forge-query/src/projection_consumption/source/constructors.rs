use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::query_context::{QueryContextExecutionArtifact, QueryContextExecutionFamily};
use crate::runtime::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryLiveArtifactBinding, ForgeQueryMutationTargetClass,
    ForgeQueryReadExecutionEngine, ForgeQueryReadReceipt, ForgeQueryWriteReceipt,
};
use forge_relational::facade::grouped_truth::{
    RelationalAuthoritativeRowSetArtifact, RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    BridgeGroupedTruthViewArtifact, BridgeMaterializedRowSetArtifact,
};

use super::{
    ProjectionConsumptionSource, ProjectionSourceCapabilityProfile,
    ProjectionSourceExecutionPosture, ProjectionSourceFamily, ProjectionSourceReferenceIdentity,
    ProjectionWriteReceiptCapabilities,
};

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

    pub fn from_retained_derived_artifact_binding(
        binding: &ForgeQueryDerivedArtifactBinding,
    ) -> Self {
        Self {
            source_reference_identities: retained_target_references(
                "retained_target_view",
                binding.target_view_names(),
            ),
            family: ProjectionSourceFamily::RetainedDerivedArtifactBinding,
            capability_profile: ProjectionSourceCapabilityProfile::RetainedDerivedArtifactBinding,
            query_digest: None,
            basis_digest: Some(binding.snapshot_token().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: binding.binding_digest().to_string(),
            materialized_fact_posture: None,
        }
    }

    pub fn from_live_artifact_binding(binding: &ForgeQueryLiveArtifactBinding) -> Self {
        Self {
            source_reference_identities: retained_target_references(
                "live_target_view",
                binding.target_view_names(),
            ),
            family: ProjectionSourceFamily::LiveArtifactBinding,
            capability_profile: ProjectionSourceCapabilityProfile::LiveArtifactBinding,
            query_digest: None,
            basis_digest: Some(binding.snapshot_token().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: binding.binding_digest().to_string(),
            materialized_fact_posture: None,
        }
    }
}

pub(crate) fn execution_posture_from_read_engine(
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

pub(crate) fn execution_posture_from_query_context_family(
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

fn retained_target_references<'a>(
    label: &'static str,
    target_view_names: impl Iterator<Item = &'a str>,
) -> Vec<ProjectionSourceReferenceIdentity> {
    target_view_names
        .map(|view_name| ProjectionSourceReferenceIdentity::synthetic(label, view_name))
        .collect()
}
