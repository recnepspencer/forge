use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
};

use crate::topology_operators::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphCounters,
};
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::selection_from_touched_closure::operating_world_lowering::query_operating_world_descriptor_from_topology_world;
use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyValidatorRoutingClosure {
    semantic_family_key: &'static str,
    basis_digest: String,
    touch_descriptor_digest: String,
    operating_world_posture: &'static str,
    operating_world_identity_digest: Option<String>,
    query_operating_world_descriptor: ForgeQueryGraphObligationOperatingWorldDescriptor,
    milestone_eight_seed_digest: String,
    receipt_context_present: bool,
    posture_context_present: bool,
    counters: TopologyTouchedGraphCounters,
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    closure_digest: String,
}

impl WorthTopologyValidatorRoutingClosure {
    pub fn from_declared_touch(
        proof: &TopologyDeclaredTouchedGraphBasisProof,
        milestone_eight_summary: &WorthValidationAuthorityMilestoneEightSeedSummary,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        if milestone_eight_summary.claims_validator_selection()
            || !milestone_eight_summary.posture_context_present()
        {
            return Err(WorthTopologyLegalityCatalogError::MissingMilestoneEightReceiptContext);
        }
        let query_operating_world_descriptor =
            query_operating_world_descriptor_from_topology_world(proof.operating_world());
        let operating_world_identity_digest = proof
            .operating_world()
            .identity_digest()
            .map(str::to_string);
        let touch_descriptor_digest = proof.touch_descriptor().descriptor_digest().to_string();
        let closure_digest = routing_closure_digest(
            proof.semantic_family_key(),
            proof.basis_digest(),
            &touch_descriptor_digest,
            proof.operating_world().as_str(),
            operating_world_identity_digest.as_deref(),
            query_operating_world_descriptor.descriptor_digest(),
            milestone_eight_summary.seed_digest(),
            proof.counters(),
        );
        Ok(Self {
            semantic_family_key: proof.semantic_family_key(),
            basis_digest: proof.basis_digest().to_string(),
            touch_descriptor_digest,
            operating_world_posture: proof.operating_world().as_str(),
            operating_world_identity_digest,
            query_operating_world_descriptor,
            milestone_eight_seed_digest: milestone_eight_summary.seed_digest().to_string(),
            receipt_context_present: milestone_eight_summary.receipt_context_present(),
            posture_context_present: milestone_eight_summary.posture_context_present(),
            counters: proof.counters(),
            touch_descriptor: proof.touch_descriptor().clone(),
            closure_digest,
        })
    }

    pub fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_posture(&self) -> &'static str {
        self.operating_world_posture
    }

    pub fn operating_world_identity_digest(&self) -> Option<&str> {
        self.operating_world_identity_digest.as_deref()
    }

    pub fn query_operating_world_descriptor(
        &self,
    ) -> &ForgeQueryGraphObligationOperatingWorldDescriptor {
        &self.query_operating_world_descriptor
    }

    pub fn milestone_eight_seed_digest(&self) -> &str {
        &self.milestone_eight_seed_digest
    }

    pub const fn receipt_context_present(&self) -> bool {
        self.receipt_context_present
    }

    pub const fn posture_context_present(&self) -> bool {
        self.posture_context_present
    }

    pub const fn counters(&self) -> TopologyTouchedGraphCounters {
        self.counters
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }
}

fn routing_closure_digest(
    semantic_family_key: &str,
    basis_digest: &str,
    touch_descriptor_digest: &str,
    operating_world_posture: &str,
    operating_world_identity_digest: Option<&str>,
    query_operating_world_descriptor_digest: &str,
    milestone_eight_seed_digest: &str,
    counters: TopologyTouchedGraphCounters,
) -> String {
    [
        "worth-topo-validator-routing-closure-v1".to_string(),
        format!("family:{semantic_family_key}"),
        format!("basis:{basis_digest}"),
        format!("touch:{touch_descriptor_digest}"),
        format!("world:{operating_world_posture}"),
        format!(
            "world-identity:{}",
            operating_world_identity_digest.unwrap_or("<mainline>")
        ),
        format!("query-world:{query_operating_world_descriptor_digest}"),
        format!("m8:{milestone_eight_seed_digest}"),
        format!("entities:{}", counters.entity_count()),
        format!("relations:{}", counters.relation_count()),
        format!("relation-kinds:{}", counters.relation_kind_count()),
        format!("aspects:{}", counters.touched_aspect_count()),
        format!("scopes:{}", counters.topology_scope_count()),
    ]
    .join("|")
}
