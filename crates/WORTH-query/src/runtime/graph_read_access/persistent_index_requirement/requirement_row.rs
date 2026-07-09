use crate::identity::hash_parts;
use crate::runtime::{
    WorthQueryGraphIndexInventoryMatch, WorthQueryGraphIndexInventoryMatchOutcome,
    WorthQueryGraphIndexLifecycleClass, WorthQueryGraphIndexLifecycleOwner,
    WorthQueryGraphIndexSupportState, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessInvalidationBasis,
    WorthQueryGraphReadAccessRebuildBasis, WorthQueryGraphReadAccessRequirementKind,
    WorthQueryGraphReadRequiredCapabilityOwner,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPersistentGraphIndexRequirementRow {
    digest: String,
    requirement_kind: WorthQueryGraphReadAccessRequirementKind,
    requirement_row_digest: String,
    requirement_semantic_slot: String,
    support_row_digest: String,
    match_outcome: WorthQueryGraphIndexInventoryMatchOutcome,
    support_state: WorthQueryGraphIndexSupportState,
    lifecycle_owner: WorthQueryGraphIndexLifecycleOwner,
    lifecycle_class: WorthQueryGraphIndexLifecycleClass,
    rebuild_basis: WorthQueryGraphReadAccessRebuildBasis,
    invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis,
    complexity_contract: WorthQueryGraphReadAccessComplexityContract,
    owning_milestone: Option<String>,
    required_owner: WorthQueryGraphReadRequiredCapabilityOwner,
    required_posture: WorthQueryGraphReadAccessAdmissionPosture,
}

impl WorthQueryPersistentGraphIndexRequirementRow {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn requirement_kind(&self) -> &WorthQueryGraphReadAccessRequirementKind {
        &self.requirement_kind
    }

    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    pub fn requirement_semantic_slot(&self) -> &str {
        &self.requirement_semantic_slot
    }

    pub fn support_row_digest(&self) -> &str {
        &self.support_row_digest
    }

    pub fn match_outcome(&self) -> &WorthQueryGraphIndexInventoryMatchOutcome {
        &self.match_outcome
    }

    pub fn support_state(&self) -> &WorthQueryGraphIndexSupportState {
        &self.support_state
    }

    pub fn lifecycle_owner(&self) -> &WorthQueryGraphIndexLifecycleOwner {
        &self.lifecycle_owner
    }

    pub fn lifecycle_class(&self) -> &WorthQueryGraphIndexLifecycleClass {
        &self.lifecycle_class
    }

    pub fn rebuild_basis(&self) -> &WorthQueryGraphReadAccessRebuildBasis {
        &self.rebuild_basis
    }

    pub fn invalidation_basis(&self) -> &WorthQueryGraphReadAccessInvalidationBasis {
        &self.invalidation_basis
    }

    pub fn complexity_contract(&self) -> &WorthQueryGraphReadAccessComplexityContract {
        &self.complexity_contract
    }

    pub fn owning_milestone(&self) -> Option<&str> {
        self.owning_milestone.as_deref()
    }

    pub fn required_owner(&self) -> &WorthQueryGraphReadRequiredCapabilityOwner {
        &self.required_owner
    }

    pub fn required_posture(&self) -> &WorthQueryGraphReadAccessAdmissionPosture {
        &self.required_posture
    }

    pub(crate) fn from_inventory_match(match_row: &WorthQueryGraphIndexInventoryMatch) -> Self {
        let requirement_kind = match_row.requirement_kind().clone();
        let requirement_row_digest = match_row.requirement_row_digest().to_string();
        let requirement_semantic_slot = match_row.requirement_semantic_slot().to_string();
        let support_row_digest = match_row.support_row_digest().to_string();
        let match_outcome = match_row.outcome().clone();
        let support_state = match_row.support_state().clone();
        let lifecycle_owner = match_row.lifecycle_owner().clone();
        let lifecycle_class = match_row.lifecycle_class().clone();
        let rebuild_basis = match_row.rebuild_basis().clone();
        let invalidation_basis = match_row.invalidation_basis().clone();
        let complexity_contract = match_row.complexity_contract().clone();
        let owning_milestone = match_row.owning_milestone().map(str::to_string);
        let required_owner = match_row.required_capability_owner().clone();
        let required_posture = match_row.resolved_admission_posture().clone();
        let digest = hash_parts(&[
            "worth_query_persistent_graph_index_requirement_row_v1".to_string(),
            format!("kind:{}", requirement_kind.as_str()),
            format!("requirement_row:{requirement_row_digest}"),
            format!("semantic_slot:{requirement_semantic_slot}"),
            format!("support_row:{support_row_digest}"),
            format!("match_outcome:{}", match_outcome.as_str()),
            format!("support_state:{}", support_state.as_str()),
            format!("lifecycle_owner:{}", lifecycle_owner.as_str()),
            format!("lifecycle_class:{}", lifecycle_class.as_str()),
            format!("rebuild:{}", rebuild_basis.as_str()),
            format!("invalidation:{}", invalidation_basis.as_str()),
            format!("complexity:{}", complexity_contract.as_str()),
            format!(
                "owning_milestone:{}",
                owning_milestone.as_deref().unwrap_or("none")
            ),
            format!("owner:{}", required_owner.as_str()),
            format!("posture:{}", required_posture.as_str()),
        ]);
        Self {
            digest,
            requirement_kind,
            requirement_row_digest,
            requirement_semantic_slot,
            support_row_digest,
            match_outcome,
            support_state,
            lifecycle_owner,
            lifecycle_class,
            rebuild_basis,
            invalidation_basis,
            complexity_contract,
            owning_milestone,
            required_owner,
            required_posture,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "persistent_requirement_row:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.requirement_kind.as_str(),
            self.requirement_row_digest,
            self.support_row_digest,
            self.match_outcome.as_str(),
            self.support_state.as_str(),
            self.lifecycle_owner.as_str(),
            self.lifecycle_class.as_str(),
            self.rebuild_basis.as_str(),
            self.invalidation_basis.as_str(),
            self.complexity_contract.as_str(),
            self.owning_milestone.as_deref().unwrap_or("none"),
            self.required_owner.as_str(),
            self.required_posture.as_str()
        )
    }
}
