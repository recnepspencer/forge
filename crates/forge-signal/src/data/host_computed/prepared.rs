use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::prepared::PreparedEvaluation;

use super::denial::DeniedHostComputedReadSet;
use super::dependency_patch::HostComputedDependencyPatch;
use super::descriptor::{HostComputedApiFamily, HostComputedDescriptor};
use super::diagnostics::HostComputedDiagnosticsSummary;
use super::evaluator::HostComputedEvaluator;
use super::read_set::AdmittedHostComputedReadSet;
use super::request::HostComputedEvaluationRequest;
use super::response::HostComputedEvaluationResponse;

struct PreparedResponseEvaluator {
    prepared: PreparedEvaluation,
}

impl HostComputedEvaluator for PreparedResponseEvaluator {
    type Context = ();

    fn evaluate(
        &mut self,
        _request: &HostComputedEvaluationRequest,
        _ctx: &mut Self::Context,
    ) -> Result<HostComputedEvaluationResponse, SignalError> {
        Ok(HostComputedEvaluationResponse::from_prepared_evaluation(
            self.prepared.clone(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedHostComputedEvaluation {
    request: HostComputedEvaluationRequest,
    evaluation: PreparedEvaluation,
    admitted_reads: AdmittedHostComputedReadSet,
    dependency_patch: HostComputedDependencyPatch,
    diagnostics_summary: HostComputedDiagnosticsSummary,
}

impl PreparedHostComputedEvaluation {
    pub(crate) fn admit(
        request: HostComputedEvaluationRequest,
        prepared: PreparedEvaluation,
    ) -> Result<Self, DeniedHostComputedReadSet> {
        let node = request.node();
        let admitted_reads =
            AdmittedHostComputedReadSet::admit(node, prepared.dependencies.clone())?;
        let dependency_patch = HostComputedDependencyPatch::between(
            node,
            request.previous_dependencies(),
            &admitted_reads,
        );
        let prepared = prepared.with_dependencies(admitted_reads.to_prepared_capture());
        let diagnostics_summary =
            HostComputedDiagnosticsSummary::prepared(&request, &admitted_reads, &dependency_patch);
        Ok(Self::new(
            request,
            prepared,
            admitted_reads,
            dependency_patch,
            diagnostics_summary,
        ))
    }

    fn new(
        request: HostComputedEvaluationRequest,
        evaluation: PreparedEvaluation,
        admitted_reads: AdmittedHostComputedReadSet,
        dependency_patch: HostComputedDependencyPatch,
        diagnostics_summary: HostComputedDiagnosticsSummary,
    ) -> Self {
        Self {
            request,
            evaluation,
            admitted_reads,
            dependency_patch,
            diagnostics_summary,
        }
    }

    pub fn node(&self) -> NodeId {
        self.request.node()
    }

    pub fn request(&self) -> &HostComputedEvaluationRequest {
        &self.request
    }

    pub fn descriptor(&self) -> &HostComputedDescriptor {
        self.request.descriptor()
    }

    pub fn evaluation(&self) -> &PreparedEvaluation {
        &self.evaluation
    }

    pub fn admitted_reads(&self) -> &AdmittedHostComputedReadSet {
        &self.admitted_reads
    }

    pub fn dependency_patch(&self) -> &HostComputedDependencyPatch {
        &self.dependency_patch
    }

    pub fn diagnostics_summary(&self) -> &HostComputedDiagnosticsSummary {
        &self.diagnostics_summary
    }

    pub fn next_dependencies(&self) -> &[DependencyEdge] {
        self.admitted_reads.dependencies()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PreparedEvaluation,
        AdmittedHostComputedReadSet,
        HostComputedDependencyPatch,
    ) {
        (self.evaluation, self.admitted_reads, self.dependency_patch)
    }
}

pub(crate) fn admit_or_error(
    api_family: HostComputedApiFamily,
    node: NodeId,
    previous_dependencies: &[DependencyEdge],
    prepared: PreparedEvaluation,
    telemetry: &mut RuntimeTelemetry,
) -> Result<PreparedHostComputedEvaluation, SignalError> {
    let mut evaluator = PreparedResponseEvaluator { prepared };
    let mut ctx = ();
    evaluate_with_or_error(
        api_family,
        node,
        previous_dependencies,
        &mut evaluator,
        &mut ctx,
        telemetry,
    )
}

pub(crate) fn evaluate_with_or_error<E>(
    api_family: HostComputedApiFamily,
    node: NodeId,
    previous_dependencies: &[DependencyEdge],
    evaluator: &mut E,
    ctx: &mut E::Context,
    telemetry: &mut RuntimeTelemetry,
) -> Result<PreparedHostComputedEvaluation, SignalError>
where
    E: HostComputedEvaluator,
{
    telemetry.host_computed.descriptor_registration_count += 1;
    let request = HostComputedEvaluationRequest::new(
        HostComputedDescriptor::for_node(node, api_family),
        previous_dependencies,
    );
    telemetry.host_computed.evaluation_request_admission_count += 1;
    let response = evaluator.evaluate(&request, ctx).unwrap_or_else(|err| {
        HostComputedEvaluationResponse::failed(
            super::outcome::HostComputedFailureClass::RuntimeInvariantViolation,
            err.to_string(),
        )
    });
    response.admit_or_error(request, telemetry)
}

#[cfg(test)]
mod tests {
    use crate::data::aspect::{Aspect, AspectVersion};
    use crate::data::output::NodeEvaluationResult;
    use crate::data::telemetry::RuntimeTelemetry;
    use crate::logic::prepared::PreparedDependencyCapture;

    use super::*;

    struct FailingEvaluator;

    impl HostComputedEvaluator for FailingEvaluator {
        type Context = ();

        fn evaluate(
            &mut self,
            _request: &HostComputedEvaluationRequest,
            _ctx: &mut Self::Context,
        ) -> Result<HostComputedEvaluationResponse, SignalError> {
            Err(SignalError::internal("evaluator exploded"))
        }
    }

    #[test]
    fn admits_prepared_dependencies() {
        let node = NodeId::new(9, 0);
        let source = NodeId::new(10, 0);
        let mut capture = PreparedDependencyCapture::new();
        capture.record(source, Aspect::new(0), None);
        let prepared = PreparedEvaluation::from_result(NodeEvaluationResult::from_version(
            AspectVersion::zero(),
        ))
        .with_dependencies(capture);
        let request = HostComputedEvaluationRequest::new(
            HostComputedDescriptor::for_node(node, HostComputedApiFamily::CorePreparedEvaluation),
            &[],
        );

        let admitted = PreparedHostComputedEvaluation::admit(request, prepared).unwrap();

        assert_eq!(
            admitted.next_dependencies(),
            &[DependencyEdge::new(source, Aspect::new(0))]
        );
        assert_eq!(
            admitted.dependency_patch().added_dependencies(),
            admitted.next_dependencies()
        );
    }

    #[test]
    fn admit_or_error_records_host_computed_counters() {
        let node = NodeId::new(9, 0);
        let source = NodeId::new(10, 0);
        let mut capture = PreparedDependencyCapture::new();
        capture.record(source, Aspect::new(0), None);
        let prepared = PreparedEvaluation::from_result(NodeEvaluationResult::from_version(
            AspectVersion::zero(),
        ))
        .with_dependencies(capture);
        let mut telemetry = RuntimeTelemetry::default();

        let admitted = admit_or_error(
            HostComputedApiFamily::CorePreparedEvaluation,
            node,
            &[],
            prepared,
            &mut telemetry,
        )
        .unwrap();

        assert_eq!(admitted.request().previous_dependency_count(), 0);
        assert_eq!(telemetry.host_computed.descriptor_registration_count, 1);
        assert_eq!(
            telemetry.host_computed.evaluation_request_admission_count,
            1
        );
        assert_eq!(telemetry.host_computed.read_set_admission_count, 1);
        assert_eq!(telemetry.host_computed.dependency_patch_count, 1);
        assert_eq!(telemetry.host_computed.committed_artifact_count, 1);
        assert_eq!(telemetry.host_computed.dependency_patch_added_count, 1);
    }

    #[test]
    fn evaluate_with_or_error_uses_host_evaluator_boundary() {
        let node = NodeId::new(11, 0);
        let source = NodeId::new(12, 0);
        let mut capture = PreparedDependencyCapture::new();
        capture.record(source, Aspect::new(0), None);
        let prepared = PreparedEvaluation::from_result(NodeEvaluationResult::from_version(
            AspectVersion::zero(),
        ))
        .with_dependencies(capture);
        let mut evaluator = PreparedResponseEvaluator { prepared };
        let mut telemetry = RuntimeTelemetry::default();
        let mut ctx = ();

        let admitted = evaluate_with_or_error(
            HostComputedApiFamily::OpaqueHostAdapter,
            node,
            &[],
            &mut evaluator,
            &mut ctx,
            &mut telemetry,
        )
        .unwrap();

        assert_eq!(
            admitted.descriptor().api_family(),
            HostComputedApiFamily::OpaqueHostAdapter
        );
        assert_eq!(telemetry.host_computed.committed_artifact_count, 1);
    }

    #[test]
    fn evaluate_with_or_error_counts_infrastructure_failure_as_failed_outcome() {
        let node = NodeId::new(13, 0);
        let mut evaluator = FailingEvaluator;
        let mut telemetry = RuntimeTelemetry::default();
        let mut ctx = ();

        let err = evaluate_with_or_error(
            HostComputedApiFamily::OpaqueHostAdapter,
            node,
            &[],
            &mut evaluator,
            &mut ctx,
            &mut telemetry,
        )
        .unwrap_err();

        assert!(format!("{err}").contains("host-computed evaluation failed"));
        assert_eq!(telemetry.host_computed.failed_outcome_count, 1);
    }
}
