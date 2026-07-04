use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteFamily, BatchAdmissionPlannerRouteWitness,
    BatchAdmissionPlannerRouteWitnessKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::admitted_input::AdmittedBatchAdmissionPlannerRouteInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BatchAdmissionPlannerRoutePacket {
    family: BatchAdmissionPlannerRouteFamily,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    selected_family_row_digests: Vec<String>,
    denial_witness: Option<BatchAdmissionPlannerRouteWitness>,
    packet_identity: String,
}

pub(crate) fn lower_batch_admission_planner_route_packet(
    input: AdmittedBatchAdmissionPlannerRouteInput,
) -> BatchAdmissionPlannerRoutePacket {
    let receipt = input.receipt();
    let selected_family_row_digests = receipt
        .selected_family_rows()
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.identity().as_str(),
                row.posture().as_str(),
                row.declaration_digest()
            )
        })
        .collect::<Vec<_>>();
    let selected_batch_plan_digest = receipt.selected_batch_plan_digest().to_string();
    let batch_execution_receipt_digest = receipt.execution_receipt_digest().to_string();
    let denial_witness = input.denial_witness().cloned();
    let packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &selected_family_row_digests
            .iter()
            .map(|digest| format!("selected-family-row:{digest}"))
            .chain(std::iter::once(format!(
                "family:{}",
                BatchAdmissionPlannerRouteFamily::BatchAdmissionRoute.as_str()
            )))
            .chain(std::iter::once(format!(
                "selected-batch:{selected_batch_plan_digest}"
            )))
            .chain(std::iter::once(format!(
                "execution:{batch_execution_receipt_digest}"
            )))
            .chain(std::iter::once(format!(
                "denial-witness:{}",
                denial_witness
                    .as_ref()
                    .map(BatchAdmissionPlannerRouteWitness::identity_digest)
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(
                "worth-kernel:batch-admission-route-packet:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    BatchAdmissionPlannerRoutePacket {
        family: BatchAdmissionPlannerRouteFamily::BatchAdmissionRoute,
        selected_batch_plan_digest,
        batch_execution_receipt_digest,
        selected_family_row_digests,
        denial_witness,
        packet_identity,
    }
}

impl BatchAdmissionPlannerRoutePacket {
    pub(crate) fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }

    pub(crate) fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }

    pub(crate) fn selected_family_row_digests(&self) -> &[String] {
        &self.selected_family_row_digests
    }

    pub(crate) fn denial_witness(&self) -> Option<&BatchAdmissionPlannerRouteWitness> {
        self.denial_witness.as_ref()
    }

    pub(crate) fn denial_witness_kind(&self) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.denial_witness.as_ref().map(|witness| witness.kind())
    }

    pub(crate) fn packet_identity(&self) -> &str {
        &self.packet_identity
    }
}
