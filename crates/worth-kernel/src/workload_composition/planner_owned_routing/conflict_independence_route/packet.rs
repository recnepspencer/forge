use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictIndependencePlannerRouteFamily, ConflictIndependencePlannerRouteWitness,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::admitted_input::AdmittedConflictIndependencePlannerRouteInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConflictIndependencePlannerRoutePacket {
    conflict_route_family: ConflictIndependencePlannerRouteFamily,
    independence_route_family: ConflictIndependencePlannerRouteFamily,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_identities: Vec<String>,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    denial_witness: Option<ConflictIndependencePlannerRouteWitness>,
    packet_identity: String,
}

pub(crate) fn lower_conflict_independence_planner_route_packet(
    input: AdmittedConflictIndependencePlannerRouteInput,
) -> ConflictIndependencePlannerRoutePacket {
    let receipt = input.receipt();
    let conflict_route_family = input.family_catalog().conflict_route_family();
    let independence_route_family = input.family_catalog().independence_route_family();
    let overlap_identity_digests = receipt.overlap_identity_digests().to_vec();
    let locality_footprint_digests = receipt.locality_footprint_digests().to_vec();
    let selected_conflict_plan_digests = receipt.selected_conflict_plan_digests().to_vec();
    let independence_proof_identities = receipt.independence_proof_identities().to_vec();
    let selected_batch_plan_digest = receipt.selected_batch_plan_digest().to_string();
    let batch_execution_receipt_digest = receipt.execution_receipt_digest().to_string();
    let denial_witness = input.denial_witness().cloned();
    let packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &overlap_identity_digests
            .iter()
            .map(|digest| format!("overlap:{digest}"))
            .chain(
                locality_footprint_digests
                    .iter()
                    .map(|digest| format!("locality:{digest}")),
            )
            .chain(
                selected_conflict_plan_digests
                    .iter()
                    .map(|digest| format!("selected-conflict:{digest}")),
            )
            .chain(
                independence_proof_identities
                    .iter()
                    .map(|digest| format!("independence:{digest}")),
            )
            .chain(std::iter::once(format!(
                "conflict-family:{}",
                conflict_route_family.as_str()
            )))
            .chain(std::iter::once(format!(
                "independence-family:{}",
                independence_route_family.as_str()
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
                    .map(ConflictIndependencePlannerRouteWitness::identity_digest)
                    .unwrap_or("not-applicable")
            )))
            .chain(std::iter::once(
                "worth-kernel:conflict-independence-route-packet:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    ConflictIndependencePlannerRoutePacket {
        conflict_route_family,
        independence_route_family,
        overlap_identity_digests,
        locality_footprint_digests,
        selected_conflict_plan_digests,
        independence_proof_identities,
        selected_batch_plan_digest,
        batch_execution_receipt_digest,
        denial_witness,
        packet_identity,
    }
}

impl ConflictIndependencePlannerRoutePacket {
    pub(crate) const fn conflict_route_family(&self) -> ConflictIndependencePlannerRouteFamily {
        self.conflict_route_family
    }

    pub(crate) const fn independence_route_family(&self) -> ConflictIndependencePlannerRouteFamily {
        self.independence_route_family
    }

    pub(crate) fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub(crate) fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
    }

    pub(crate) fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }

    pub(crate) fn independence_proof_identities(&self) -> &[String] {
        &self.independence_proof_identities
    }

    pub(crate) fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }

    pub(crate) fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }

    pub(crate) fn denial_witness(&self) -> Option<&ConflictIndependencePlannerRouteWitness> {
        self.denial_witness.as_ref()
    }

    pub(crate) fn packet_identity(&self) -> &str {
        &self.packet_identity
    }
}
