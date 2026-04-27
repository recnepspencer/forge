mod builders;
mod tests;

use crate::harness::certification::{digest_parts, CertificationMatrix};
use crate::runtime::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

pub const RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "workflow-editor-golden-transcript",
    "geometry-kernel-golden-transcript",
    "table-spreadsheet-golden-transcript",
    "composed-runtime-adversarial-transcript",
];

pub const RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "temporal-basis-deferred-gate",
    "async-resource-deferred-gate",
    "mixed-cause-delivery-deferred-gate",
    "store-backed-parity-deferred-gate",
    "durable-restart-deferred-gate",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RuntimeApiStabilizationPerturbationClass {
    WorkflowEditorGoldenTranscript,
    GeometryKernelGoldenTranscript,
    TableSpreadsheetGoldenTranscript,
    ComposedRuntimeAdversarialTranscript,
    TemporalBasisDeferredGate,
    AsyncResourceDeferredGate,
    MixedCauseDeliveryDeferredGate,
    StoreBackedParityDeferredGate,
    DurableRestartDeferredGate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeApiStabilizationFailureClass {
    DeferredTemporalAsyncGate,
    DeferredStoreDurableGate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeApiStabilizationBundle {
    pub public_api_surface_digest: String,
    pub golden_transcript_digest: String,
    pub handle_contract_digest: String,
    pub state_contract_digest: String,
    pub aspect_contract_digest: String,
    pub authority_lane_digest: String,
    pub inspection_contract_digest: String,
    pub support_matrix_digest: String,
    pub deferred_temporal_async_gate_digest: String,
    pub failure_digest: String,
    pub counter_snapshot: String,
    pub compile_fail_boundary_digest: String,
    pub transcript_family: String,
    pub public_facade_only: bool,
    pub lower_runtime_plumbing_count: usize,
    pub meaningful_assertion_count: usize,
    pub stable_family_count: usize,
    pub deferred_family_count: usize,
    pub unsupported_family_count: usize,
}

impl RuntimeApiStabilizationBundle {
    pub(super) fn has_required_outputs(&self) -> bool {
        !self.public_api_surface_digest.is_empty()
            && !self.golden_transcript_digest.is_empty()
            && !self.handle_contract_digest.is_empty()
            && !self.state_contract_digest.is_empty()
            && !self.aspect_contract_digest.is_empty()
            && !self.authority_lane_digest.is_empty()
            && !self.inspection_contract_digest.is_empty()
            && !self.support_matrix_digest.is_empty()
            && !self.deferred_temporal_async_gate_digest.is_empty()
            && !self.failure_digest.is_empty()
            && !self.counter_snapshot.is_empty()
            && !self.compile_fail_boundary_digest.is_empty()
            && !self.transcript_family.is_empty()
    }

    pub(super) fn semantic_signature(&self) -> String {
        digest_parts(&[
            format!("surface:{}", self.public_api_surface_digest),
            format!("transcript:{}", self.golden_transcript_digest),
            format!("handle:{}", self.handle_contract_digest),
            format!("state:{}", self.state_contract_digest),
            format!("aspect:{}", self.aspect_contract_digest),
            format!("lane:{}", self.authority_lane_digest),
            format!("inspection:{}", self.inspection_contract_digest),
            format!("support:{}", self.support_matrix_digest),
            format!("gate:{}", self.deferred_temporal_async_gate_digest),
            format!("family:{}", self.transcript_family),
            format!("facade_only:{}", self.public_facade_only),
            format!("plumbing:{}", self.lower_runtime_plumbing_count),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeApiStabilizationRejectionBundle {
    pub failure_class: RuntimeApiStabilizationFailureClass,
    pub family: ForgeQueryRuntimeFacadeFamily,
    pub status: ForgeQueryRuntimeFamilySupportStatus,
    pub failure_digest: String,
    pub deferred_temporal_async_gate_digest: String,
    pub counter_snapshot: String,
    pub compile_fail_boundary_digest: String,
}

pub type RuntimeApiStabilizationCertificationMatrix = CertificationMatrix<
    RuntimeApiStabilizationPerturbationClass,
    RuntimeApiStabilizationBundle,
    RuntimeApiStabilizationRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeApiStabilizationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: RuntimeApiStabilizationCertificationMatrix,
}

impl RuntimeApiStabilizationCertificationMatrix {
    pub fn into_runtime_api_stabilization_artifact(self) -> RuntimeApiStabilizationArtifact {
        RuntimeApiStabilizationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest: digest_parts(&builders::bundle_digest_parts(&self)),
            coverage_matrix_digest: digest_parts(&builders::coverage_digest_parts(&self)),
            matrix: self,
        }
    }
}

pub struct RuntimeApiStabilizationAdapter;

impl RuntimeApiStabilizationAdapter {
    pub fn runtime_api_golden_dx_and_async_safe_facade_artifact() -> RuntimeApiStabilizationArtifact
    {
        Self::runtime_api_golden_dx_and_async_safe_facade_test()
            .into_runtime_api_stabilization_artifact()
    }

    pub fn runtime_api_golden_dx_and_async_safe_facade_test(
    ) -> RuntimeApiStabilizationCertificationMatrix {
        RuntimeApiStabilizationCertificationMatrix {
            suite_name: "Runtime API Golden DX And Async-Safe Facade Test",
            rows: builders::canonical_rows(),
            rejection_rows: builders::rejection_rows(),
        }
    }
}
