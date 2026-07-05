use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanLoopReconstructionSplitConsumption;
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;

pub struct PlanarBooleanLoopReconstructionRequestInput<'a> {
    split_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
    selected_plan_digest: String,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    touched_closure_digest: String,
    overlap_identity_digests: Vec<String>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
}

impl<'a> PlanarBooleanLoopReconstructionRequestInput<'a> {
    pub fn from_split_consumption_and_readiness(
        split_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
        readiness: &TouchedGraphParityReadinessInput,
    ) -> Self {
        let claim = readiness.claim();
        let selected_product_identity = claim
            .selected_product_identity()
            .expect("readiness contract requires selected-product identity");
        let selected_witness_identity = claim
            .witness_identity()
            .expect("readiness contract requires witness identity");

        Self {
            split_consumption,
            selected_plan_digest: readiness.selected_plan_digest().to_string(),
            selected_route_identity_digest: claim
                .selected_route_identity()
                .identity_digest()
                .to_string(),
            selected_family_identity: claim
                .selected_family_identity()
                .selected_family_name()
                .to_string(),
            selected_product_identity_digest: selected_product_identity
                .identity_digest()
                .to_string(),
            selected_witness_identity_digest: Some(
                selected_witness_identity.identity_digest().to_string(),
            ),
            touched_closure_digest: readiness.touched_closure_digest().to_string(),
            overlap_identity_digests: readiness.overlap_identity_digests().to_vec(),
            topology_query_posture_digest: readiness.topology_query_posture_digest().to_string(),
            spatial_query_posture_digest: readiness.spatial_query_posture_digest().to_string(),
            residue_digest: readiness.residue_digest().to_string(),
            source_firewall_digest: readiness.source_firewall_digest().to_string(),
            architecture_claim_digest: readiness.architecture_claim_digest().to_string(),
        }
    }

    pub(crate) fn split_consumption(&self) -> &'a PlanarBooleanLoopReconstructionSplitConsumption {
        self.split_consumption
    }

    pub(crate) fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub(crate) fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub(crate) fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub(crate) fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub(crate) fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub(crate) fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub(crate) fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub(crate) fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }

    pub(crate) fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }

    pub(crate) fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub(crate) fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub(crate) fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }
}
