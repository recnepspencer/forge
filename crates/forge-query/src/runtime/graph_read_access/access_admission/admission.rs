use super::case_inventory::inventory_match_digest;
use super::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessCaseRegistry,
    ForgeQueryGraphReadAccessDenial, ForgeQueryGraphReadAccessInventoryMatch,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphIndexInventory, ForgeQueryGraphIndexInventoryMatchReport,
    ForgeQueryGraphReadAccessAuthorityReceipt, ForgeQueryGraphReadAccessCostEstimate,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadBudgetCheck,
    ForgeQueryGraphReadFamilyIndexContract, ForgeQueryPersistentGraphIndexRequirementDeclaration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessAdmission {
    digest: String,
    requirement_set: ForgeQueryGraphReadAccessRequirementSet,
    cost_estimate: ForgeQueryGraphReadAccessCostEstimate,
    budget_check: ForgeQueryGraphReadBudgetCheck,
    case_registry: ForgeQueryGraphReadAccessCaseRegistry,
    graph_index_inventory: ForgeQueryGraphIndexInventory,
    graph_index_inventory_match_report: ForgeQueryGraphIndexInventoryMatchReport,
    graph_read_family_index_contract: ForgeQueryGraphReadFamilyIndexContract,
    authority_receipt: ForgeQueryGraphReadAccessAuthorityReceipt,
    persistent_index_requirement: Option<ForgeQueryPersistentGraphIndexRequirementDeclaration>,
    inventory_matches: Vec<ForgeQueryGraphReadAccessInventoryMatch>,
    posture: ForgeQueryGraphReadAccessAdmissionPosture,
    denial: Option<ForgeQueryGraphReadAccessDenial>,
}

impl ForgeQueryGraphReadAccessAdmission {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn requirement_set(&self) -> &ForgeQueryGraphReadAccessRequirementSet {
        &self.requirement_set
    }

    pub fn cost_estimate(&self) -> &ForgeQueryGraphReadAccessCostEstimate {
        &self.cost_estimate
    }

    pub fn budget_check(&self) -> &ForgeQueryGraphReadBudgetCheck {
        &self.budget_check
    }

    pub fn case_registry(&self) -> &ForgeQueryGraphReadAccessCaseRegistry {
        &self.case_registry
    }

    pub fn graph_index_inventory(&self) -> &ForgeQueryGraphIndexInventory {
        &self.graph_index_inventory
    }

    pub fn graph_index_inventory_match_report(&self) -> &ForgeQueryGraphIndexInventoryMatchReport {
        &self.graph_index_inventory_match_report
    }

    pub fn persistent_index_requirement(
        &self,
    ) -> Option<&ForgeQueryPersistentGraphIndexRequirementDeclaration> {
        self.persistent_index_requirement.as_ref()
    }

    pub fn graph_read_family_index_contract(&self) -> &ForgeQueryGraphReadFamilyIndexContract {
        &self.graph_read_family_index_contract
    }

    pub fn authority_receipt(&self) -> &ForgeQueryGraphReadAccessAuthorityReceipt {
        &self.authority_receipt
    }

    pub fn inventory_matches(&self) -> &[ForgeQueryGraphReadAccessInventoryMatch] {
        &self.inventory_matches
    }

    pub fn posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.posture
    }

    pub fn denial(&self) -> Option<&ForgeQueryGraphReadAccessDenial> {
        self.denial.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.denial.is_none()
    }

    pub(crate) fn admitted_in_authority(
        requirement_set: ForgeQueryGraphReadAccessRequirementSet,
        cost_estimate: ForgeQueryGraphReadAccessCostEstimate,
        budget_check: ForgeQueryGraphReadBudgetCheck,
        case_registry: ForgeQueryGraphReadAccessCaseRegistry,
        graph_index_inventory: ForgeQueryGraphIndexInventory,
        graph_index_inventory_match_report: ForgeQueryGraphIndexInventoryMatchReport,
        authority_receipt: ForgeQueryGraphReadAccessAuthorityReceipt,
        posture: ForgeQueryGraphReadAccessAdmissionPosture,
    ) -> Self {
        Self::new(
            requirement_set,
            cost_estimate,
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            authority_receipt,
            posture,
            None,
        )
    }

    pub(crate) fn denied_in_authority(
        requirement_set: ForgeQueryGraphReadAccessRequirementSet,
        cost_estimate: ForgeQueryGraphReadAccessCostEstimate,
        budget_check: ForgeQueryGraphReadBudgetCheck,
        case_registry: ForgeQueryGraphReadAccessCaseRegistry,
        graph_index_inventory: ForgeQueryGraphIndexInventory,
        graph_index_inventory_match_report: ForgeQueryGraphIndexInventoryMatchReport,
        authority_receipt: ForgeQueryGraphReadAccessAuthorityReceipt,
        denial: ForgeQueryGraphReadAccessDenial,
    ) -> Self {
        Self::new(
            requirement_set,
            cost_estimate,
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            authority_receipt,
            ForgeQueryGraphReadAccessAdmissionPosture::Denied,
            Some(denial),
        )
    }

    fn new(
        requirement_set: ForgeQueryGraphReadAccessRequirementSet,
        cost_estimate: ForgeQueryGraphReadAccessCostEstimate,
        budget_check: ForgeQueryGraphReadBudgetCheck,
        case_registry: ForgeQueryGraphReadAccessCaseRegistry,
        graph_index_inventory: ForgeQueryGraphIndexInventory,
        graph_index_inventory_match_report: ForgeQueryGraphIndexInventoryMatchReport,
        authority_receipt: ForgeQueryGraphReadAccessAuthorityReceipt,
        posture: ForgeQueryGraphReadAccessAdmissionPosture,
        denial: Option<ForgeQueryGraphReadAccessDenial>,
    ) -> Self {
        let inventory_matches =
            ForgeQueryGraphReadAccessInventoryMatch::from_graph_index_match_report(
                &graph_index_inventory_match_report,
            );
        let persistent_index_requirement_is_required = posture
            == ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired
            || denial.as_ref().is_some_and(|denial| {
                denial.suggested_posture()
                    == &ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired
            });
        let persistent_index_requirement = persistent_index_requirement_is_required
            .then(|| {
                ForgeQueryPersistentGraphIndexRequirementDeclaration::from_admission_parts(
                    &requirement_set,
                    &cost_estimate,
                    &graph_index_inventory_match_report,
                )
            })
            .flatten();
        let graph_read_family_index_contract =
            ForgeQueryGraphReadFamilyIndexContract::from_admission_parts(
                &requirement_set,
                persistent_index_requirement.as_ref(),
            );
        let inventory_match_digest = inventory_match_digest(&inventory_matches);
        let digest = hash_parts(&[
            "forge_query_graph_read_access_admission_v1".to_string(),
            format!("requirements:{}", requirement_set.digest().as_str()),
            format!("estimate:{}", cost_estimate.digest().as_str()),
            format!("budget:{}", budget_check.budget_digest()),
            format!("case_registry:{}", case_registry.digest()),
            format!("graph_index_inventory:{}", graph_index_inventory.digest()),
            format!(
                "graph_index_inventory_match_report:{}",
                graph_index_inventory_match_report.digest()
            ),
            format!("inventory_match:{inventory_match_digest}"),
            format!(
                "family_index_contract:{}",
                graph_read_family_index_contract.digest()
            ),
            format!("authority_receipt:{}", authority_receipt.digest()),
            format!(
                "persistent_index_requirement:{}",
                persistent_index_requirement
                    .as_ref()
                    .map(ForgeQueryPersistentGraphIndexRequirementDeclaration::digest)
                    .unwrap_or("none")
            ),
            posture.digest_part(),
            denial
                .as_ref()
                .map(ForgeQueryGraphReadAccessDenial::digest_part)
                .unwrap_or_else(|| "denial:none".to_string()),
        ]);
        Self {
            digest,
            requirement_set,
            cost_estimate,
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            graph_read_family_index_contract,
            authority_receipt,
            persistent_index_requirement,
            inventory_matches,
            posture,
            denial,
        }
    }
}
