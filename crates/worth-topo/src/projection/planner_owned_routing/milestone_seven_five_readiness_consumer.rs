use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityFamilyKind, TouchedGraphParityReadinessInput,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyMilestoneSevenFiveReadinessErrorKind {
    MissingRepresentativeFamilyProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyMilestoneSevenFiveReadinessError {
    kind: TopologyMilestoneSevenFiveReadinessErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyMilestoneSevenFiveOverlapReadinessConsumer {
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    touched_closure_digest: String,
    selected_plan_digest: String,
    overlap_identity_digests: Vec<String>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
}

pub type TopologyOverlapReadinessRouteConsumer = TopologyMilestoneSevenFiveOverlapReadinessConsumer;
pub type TopologyOverlapReadinessRouteError = TopologyMilestoneSevenFiveReadinessError;
pub type TopologyOverlapReadinessRouteErrorKind = TopologyMilestoneSevenFiveReadinessErrorKind;

pub fn admit_topology_overlap_readiness_route_consumer(
    readiness: &TouchedGraphParityReadinessInput,
) -> Result<TopologyOverlapReadinessRouteConsumer, TopologyOverlapReadinessRouteError> {
    admit_milestone_seven_five_overlap_readiness_consumer(readiness)
}

pub fn admit_milestone_seven_five_overlap_readiness_consumer(
    readiness: &TouchedGraphParityReadinessInput,
) -> Result<
    TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    TopologyMilestoneSevenFiveReadinessError,
> {
    let claim = readiness.claim();
    let selected_product_identity = claim
        .selected_product_identity()
        .expect("readiness contract requires selected-product identity");
    let selected_witness_identity = claim
        .witness_identity()
        .expect("readiness contract requires witness identity");
    let missing = TouchedGraphParityFamilyKind::ALL
        .iter()
        .copied()
        .filter(|family| !readiness.representative_family_coverage().contains(family))
        .map(TouchedGraphParityFamilyKind::as_str)
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(TopologyMilestoneSevenFiveReadinessError::new(
            TopologyMilestoneSevenFiveReadinessErrorKind::MissingRepresentativeFamilyProof,
            format!(
                "Milestone 7.5 overlap readiness requires representative family coverage for {}",
                missing.join(", ")
            ),
        ));
    }

    Ok(TopologyMilestoneSevenFiveOverlapReadinessConsumer {
        selected_route_identity_digest: claim
            .selected_route_identity()
            .identity_digest()
            .to_string(),
        selected_family_identity: claim
            .selected_family_identity()
            .selected_family_name()
            .to_string(),
        selected_product_identity_digest: selected_product_identity.identity_digest().to_string(),
        selected_witness_identity_digest: Some(
            selected_witness_identity.identity_digest().to_string(),
        ),
        touched_closure_digest: readiness.touched_closure_digest().to_string(),
        selected_plan_digest: readiness.selected_plan_digest().to_string(),
        overlap_identity_digests: readiness.overlap_identity_digests().to_vec(),
        topology_query_posture_digest: readiness.topology_query_posture_digest().to_string(),
        spatial_query_posture_digest: readiness.spatial_query_posture_digest().to_string(),
        residue_digest: readiness.residue_digest().to_string(),
        source_firewall_digest: readiness.source_firewall_digest().to_string(),
        architecture_claim_digest: readiness.architecture_claim_digest().to_string(),
    })
}

impl TopologyMilestoneSevenFiveOverlapReadinessConsumer {
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }

    pub fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }
}

impl TopologyMilestoneSevenFiveReadinessError {
    fn new(kind: TopologyMilestoneSevenFiveReadinessErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> TopologyMilestoneSevenFiveReadinessErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
