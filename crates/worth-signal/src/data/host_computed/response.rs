use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::output::NodeEvaluationResult;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};

use super::outcome::{HostComputedEvaluationOutcome, HostComputedFailureClass};
use super::prepared::PreparedHostComputedEvaluation;
use super::request::HostComputedEvaluationRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedPreparedResponse {
    prepared: PreparedEvaluation,
}

impl HostComputedPreparedResponse {
    pub(crate) fn new(prepared: PreparedEvaluation) -> Self {
        Self { prepared }
    }

    pub fn prepared(&self) -> &PreparedEvaluation {
        &self.prepared
    }

    pub fn from_result(
        result: NodeEvaluationResult,
        dependencies: impl IntoIterator<Item = DependencyEdge>,
    ) -> Self {
        let mut capture = PreparedDependencyCapture::new();
        for dependency in dependencies {
            capture.record(
                dependency.source(),
                dependency.aspect(),
                dependency.scope_ref().cloned(),
            );
        }
        Self {
            prepared: PreparedEvaluation::from_result(result).with_dependencies(capture),
        }
    }

    pub(crate) fn into_prepared(self) -> PreparedEvaluation {
        self.prepared
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum HostComputedEvaluationResponseKind {
    Prepared(HostComputedPreparedResponse),
    Failed {
        class: HostComputedFailureClass,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedEvaluationResponse {
    inner: HostComputedEvaluationResponseKind,
}

impl HostComputedEvaluationResponse {
    pub fn prepared(prepared: HostComputedPreparedResponse) -> Self {
        Self {
            inner: HostComputedEvaluationResponseKind::Prepared(prepared),
        }
    }

    pub fn failed(class: HostComputedFailureClass, message: impl Into<String>) -> Self {
        Self {
            inner: HostComputedEvaluationResponseKind::Failed {
                class,
                message: message.into(),
            },
        }
    }

    pub(crate) fn from_prepared_evaluation(prepared: PreparedEvaluation) -> Self {
        Self {
            inner: HostComputedEvaluationResponseKind::Prepared(HostComputedPreparedResponse::new(
                prepared,
            )),
        }
    }

    pub(crate) fn admit(
        self,
        request: HostComputedEvaluationRequest,
        telemetry: Option<&mut RuntimeTelemetry>,
    ) -> HostComputedEvaluationOutcome {
        match self.inner {
            HostComputedEvaluationResponseKind::Prepared(prepared) => {
                match PreparedHostComputedEvaluation::admit(
                    request.clone(),
                    prepared.into_prepared(),
                ) {
                    Ok(admitted) => {
                        if let Some(telemetry) = telemetry {
                            telemetry.host_computed.read_set_admission_count += 1;
                            telemetry.host_computed.dependency_patch_count += 1;
                            telemetry.host_computed.committed_artifact_count += 1;
                            telemetry.host_computed.dependency_patch_added_count +=
                                admitted.dependency_patch().added_dependencies().len() as u64;
                            telemetry.host_computed.dependency_patch_removed_count +=
                                admitted.dependency_patch().removed_dependencies().len() as u64;
                            telemetry.host_computed.dependency_patch_retained_count +=
                                admitted.dependency_patch().retained_dependency_count() as u64;
                            telemetry
                                .host_computed
                                .dependency_patch_touched_subscriber_index_count +=
                                admitted.request().previous_dependency_count() as u64
                                    + admitted.next_dependencies().len() as u64;
                        }
                        HostComputedEvaluationOutcome::committed(admitted)
                    }
                    Err(denial) => {
                        if let Some(telemetry) = telemetry {
                            telemetry.host_computed.denied_outcome_count += 1;
                            telemetry.host_computed.evaluation_request_denial_count += 1;
                            if matches!(
                                denial.class(),
                                super::denial::HostComputedDenialClass::SelfRead
                            ) {
                                telemetry.host_computed.self_read_denial_count += 1;
                            }
                        }
                        HostComputedEvaluationOutcome::denied(request, denial)
                    }
                }
            }
            HostComputedEvaluationResponseKind::Failed { class, message } => {
                if let Some(telemetry) = telemetry {
                    telemetry.host_computed.failed_outcome_count += 1;
                }
                HostComputedEvaluationOutcome::failed(request.descriptor().clone(), class, message)
            }
        }
    }

    pub(crate) fn admit_or_error(
        self,
        request: HostComputedEvaluationRequest,
        telemetry: Option<&mut RuntimeTelemetry>,
    ) -> Result<PreparedHostComputedEvaluation, SignalError> {
        match self.admit(request, telemetry) {
            HostComputedEvaluationOutcome::Committed(committed) => {
                Ok(committed.staged().prepared().clone())
            }
            HostComputedEvaluationOutcome::Denied(denied) => {
                Err(SignalError::invalid_input(format!(
                    "host-computed read admission denied for {}: {:?} at {:?}",
                    denied.request().node(),
                    denied.denial().class(),
                    denied.denial().dependency()
                )))
            }
            HostComputedEvaluationOutcome::Failed(failure) => Err(SignalError::internal(format!(
                "host-computed evaluation failed for {} ({}): {}",
                failure.descriptor().node(),
                failure.class().as_str(),
                failure.message()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::aspect::{Aspect, AspectVersion};
    use crate::data::dependency::DependencyEdge;
    use crate::data::host_computed::{HostComputedApiFamily, HostComputedDescriptor};
    use crate::data::output::NodeEvaluationResult;
    use crate::data::telemetry::RuntimeTelemetry;
    use crate::logic::prepared::PreparedDependencyCapture;

    use super::*;

    #[test]
    fn prepared_response_admits_as_committed_outcome() {
        let node = crate::data::handle::NodeId::new(7, 0);
        let source = crate::data::handle::NodeId::new(8, 0);
        let request = HostComputedEvaluationRequest::new(
            HostComputedDescriptor::for_node(node, HostComputedApiFamily::CorePreparedEvaluation),
            &[],
        );
        let mut capture = PreparedDependencyCapture::new();
        capture.record(source, Aspect::new(0), None);
        let prepared = HostComputedPreparedResponse::from_result(
            NodeEvaluationResult::from_version(AspectVersion::zero()),
            [DependencyEdge::new(source, Aspect::new(0))],
        );
        let mut telemetry = RuntimeTelemetry::default();

        let outcome =
            HostComputedEvaluationResponse::prepared(prepared).admit(request, Some(&mut telemetry));

        let HostComputedEvaluationOutcome::Committed(committed) = outcome else {
            panic!("expected committed outcome");
        };
        assert_eq!(
            committed.staged().prepared().next_dependencies(),
            &[DependencyEdge::new(source, Aspect::new(0))]
        );
        assert_eq!(telemetry.host_computed.committed_artifact_count, 1);
    }

    #[test]
    fn failed_response_records_failed_outcome() {
        let node = crate::data::handle::NodeId::new(9, 0);
        let request = HostComputedEvaluationRequest::new(
            HostComputedDescriptor::for_node(node, HostComputedApiFamily::OpaqueHostAdapter),
            &[],
        );
        let mut telemetry = RuntimeTelemetry::default();

        let outcome = HostComputedEvaluationResponse::failed(
            HostComputedFailureClass::HostAdapterRejected,
            "missing callback",
        )
        .admit(request, Some(&mut telemetry));

        let HostComputedEvaluationOutcome::Failed(failure) = outcome else {
            panic!("expected failed outcome");
        };
        assert_eq!(
            failure.class(),
            HostComputedFailureClass::HostAdapterRejected
        );
        assert_eq!(
            failure.diagnostics_summary().failure_class(),
            Some("HostAdapterRejected")
        );
        assert_eq!(telemetry.host_computed.failed_outcome_count, 1);
    }

    #[test]
    fn denial_response_admit_or_error_reports_invalid_input() {
        let node = crate::data::handle::NodeId::new(10, 0);
        let request = HostComputedEvaluationRequest::new(
            HostComputedDescriptor::for_node(node, HostComputedApiFamily::CorePreparedEvaluation),
            &[DependencyEdge::new(node, Aspect::new(0))],
        );
        let mut capture = PreparedDependencyCapture::new();
        capture.record(node, Aspect::new(0), None);
        let prepared = HostComputedPreparedResponse::from_result(
            NodeEvaluationResult::from_version(AspectVersion::zero()),
            [DependencyEdge::new(node, Aspect::new(0))],
        );
        let mut telemetry = RuntimeTelemetry::default();

        let err = HostComputedEvaluationResponse::prepared(prepared)
            .admit_or_error(request, Some(&mut telemetry))
            .unwrap_err();

        assert!(format!("{err}").contains("host-computed read admission denied"));
        assert_eq!(telemetry.host_computed.denied_outcome_count, 1);
        assert_eq!(telemetry.host_computed.self_read_denial_count, 1);
    }
}
