use worth_ui_inspection::{
    UiInspectionMeasurementBasisInput, UiInspectionMeasurementChildIntrinsicSource,
};

use crate::evidence::measurement::MeasurementEvidenceInput;

use super::project_evidence_maps::project_evidence_category;

pub(crate) fn project_basis_input(
    input: &MeasurementEvidenceInput,
) -> UiInspectionMeasurementBasisInput {
    match input {
        MeasurementEvidenceInput::SettledQueryFact(receipt) => {
            UiInspectionMeasurementBasisInput::QueryProjectionFact {
                query_basis_digest: receipt.query_binding_identity().into(),
                projection_contract_digest: receipt.settlement_identity().into(),
                required_fact_family_set_digest: receipt.required_query_fact_family_set_digest(),
                consumed_fact_family_set_digest: receipt.consumed_fact_family_set_digest(),
            }
        }
        MeasurementEvidenceInput::HostCapabilityReport(report) => {
            UiInspectionMeasurementBasisInput::HostCapabilityReport {
                profile_digest: report.profile_identity_digest(),
                observation_generation: report.observation_generation().as_u64(),
            }
        }
        MeasurementEvidenceInput::HostMeasurementResult(result) => {
            UiInspectionMeasurementBasisInput::HostMeasurementResult {
                category: project_evidence_category(result.value().category()),
                identity_digest: input.identity_digest(),
            }
        }
        MeasurementEvidenceInput::ChildIntrinsicMeasurement(evidence) => {
            let source = if evidence.query_projection_fact().is_some() {
                UiInspectionMeasurementChildIntrinsicSource::QueryProjectionFact
            } else {
                let result = evidence
                    .host_measurement_result()
                    .expect("child intrinsic evidence must carry query or host authority");
                UiInspectionMeasurementChildIntrinsicSource::HostMeasurementResult(
                    project_evidence_category(result.value().category()),
                )
            };
            UiInspectionMeasurementBasisInput::ChildIntrinsicMeasurement {
                contributor_graph_node_identity_digest: evidence
                    .contributor_graph_node_identity()
                    .digest(),
                source,
                identity_digest: input.identity_digest(),
            }
        }
        MeasurementEvidenceInput::SiblingResizeSupport(support) => {
            UiInspectionMeasurementBasisInput::SiblingResizeSupport {
                axis_scope: match support.axis_scope() {
                    crate::evidence::UiConstraintAxisScope::Primary => "primary".into(),
                    crate::evidence::UiConstraintAxisScope::Cross => "cross".into(),
                    crate::evidence::UiConstraintAxisScope::Both => "both".into(),
                },
                target_graph_node_identity_digest: support.target_graph_node_identity().digest(),
                sizing_contract_id: support
                    .sizing_contract_id()
                    .map(|contract_id| contract_id.as_str())
                    .unwrap_or("none")
                    .into(),
                source: match support.source() {
                    crate::evidence::UiMeasurementSiblingResizeSupportSource::MosaicSizingCapabilitySnapshot => {
                        "mosaic-sizing-capability-snapshot".into()
                    }
                    crate::evidence::UiMeasurementSiblingResizeSupportSource::RuntimeDurableResizeWitness => {
                        "runtime-durable-resize-witness".into()
                    }
                },
                identity_digest: input.identity_digest(),
            }
        }
    }
}
