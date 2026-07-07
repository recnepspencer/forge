use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteFamily, BatchAdmissionPlannerRouteWitnessKind,
    ConflictIndependencePlannerRouteWitnessKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::WorthTouchedGraphConflictSelectedRoutePacket;

impl WorthTouchedGraphConflictSelectedRoutePacket {
    pub(crate) fn conflict_family_conflict_pre_execution_identity(&self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &self
                .overlap_identity_digests()
                .iter()
                .map(|digest| format!("overlap:{digest}"))
                .chain(
                    self.selected_conflict_plan_digests()
                        .iter()
                        .map(|digest| format!("selected-conflict:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "conflict-family:{}",
                    self.conflict_route_family().as_str()
                )))
                .chain(std::iter::once(format!(
                    "witness-kind:{}",
                    self.conflict_independence_denial_witness_kind()
                        .map(ConflictIndependencePlannerRouteWitnessKind::as_str)
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(format!(
                    "witness:{}",
                    self.conflict_independence_denial_witness_identity()
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-conflict-family:conflict:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        )
    }

    pub(crate) fn conflict_family_independence_pre_execution_identity(&self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &self
                .overlap_identity_digests()
                .iter()
                .map(|digest| format!("overlap:{digest}"))
                .chain(
                    self.independence_proof_digests()
                        .iter()
                        .map(|digest| format!("independence:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "independence-family:{}",
                    self.independence_route_family().as_str()
                )))
                .chain(std::iter::once(format!(
                    "witness-kind:{}",
                    self.conflict_independence_denial_witness_kind()
                        .map(ConflictIndependencePlannerRouteWitnessKind::as_str)
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(format!(
                    "witness:{}",
                    self.conflict_independence_denial_witness_identity()
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-conflict-family:independence:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        )
    }

    pub(crate) fn conflict_family_batch_pre_execution_identity(&self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &self
                .overlap_identity_digests()
                .iter()
                .map(|digest| format!("overlap:{digest}"))
                .chain(
                    self.selected_conflict_plan_digests()
                        .iter()
                        .map(|digest| format!("selected-conflict:{digest}")),
                )
                .chain(
                    self.independence_proof_digests()
                        .iter()
                        .map(|digest| format!("independence:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "selected-batch:{}",
                    self.selected_batch_plan_digest()
                )))
                .chain(
                    self.batch_admission_selected_family_row_digests()
                        .iter()
                        .map(|digest| format!("selected-family-row:{digest}")),
                )
                .chain(std::iter::once(format!(
                    "batch-family:{}",
                    BatchAdmissionPlannerRouteFamily::BatchAdmissionRoute.as_str()
                )))
                .chain(std::iter::once(format!(
                    "witness-kind:{}",
                    self.batch_admission_denial_witness_kind()
                        .map(BatchAdmissionPlannerRouteWitnessKind::as_str)
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(format!(
                    "witness:{}",
                    self.batch_admission_denial_witness_identity()
                        .unwrap_or("not-applicable")
                )))
                .chain(std::iter::once(
                    "worth-kernel:selected-route-conflict-family:batch-admission:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        )
    }
}
