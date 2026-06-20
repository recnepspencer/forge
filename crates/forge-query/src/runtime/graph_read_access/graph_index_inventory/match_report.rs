use super::match_selection::{
    classify_inventory_match_outcome, select_best_support_row_for_requirement,
};
use super::{
    ForgeQueryGraphIndexInventory, ForgeQueryGraphIndexInventoryCounters,
    ForgeQueryGraphIndexInventoryMatchOutcome, ForgeQueryGraphIndexPosture,
    ForgeQueryGraphIndexSupportRow, ForgeQueryGraphIndexSupportState,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphIndexLifecycleClass, ForgeQueryGraphIndexLifecycleOwner,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessComplexityContract,
    ForgeQueryGraphReadAccessInvalidationBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadRequiredCapabilityOwner,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphIndexInventoryMatch {
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    requirement_row_digest: String,
    requirement_semantic_slot: String,
    support_row_digest: String,
    support_posture: ForgeQueryGraphIndexPosture,
    support_state: ForgeQueryGraphIndexSupportState,
    lifecycle_owner: ForgeQueryGraphIndexLifecycleOwner,
    lifecycle_class: ForgeQueryGraphIndexLifecycleClass,
    rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    owning_milestone: Option<String>,
    outcome: ForgeQueryGraphIndexInventoryMatchOutcome,
    required_capability_owner: ForgeQueryGraphReadRequiredCapabilityOwner,
    resolved_admission_posture: ForgeQueryGraphReadAccessAdmissionPosture,
}

impl ForgeQueryGraphIndexInventoryMatch {
    pub fn requirement_kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
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

    pub fn support_posture(&self) -> &ForgeQueryGraphIndexPosture {
        &self.support_posture
    }

    pub fn support_state(&self) -> &ForgeQueryGraphIndexSupportState {
        &self.support_state
    }

    pub fn lifecycle_owner(&self) -> &ForgeQueryGraphIndexLifecycleOwner {
        &self.lifecycle_owner
    }

    pub fn lifecycle_class(&self) -> &ForgeQueryGraphIndexLifecycleClass {
        &self.lifecycle_class
    }

    pub fn rebuild_basis(&self) -> &ForgeQueryGraphReadAccessRebuildBasis {
        &self.rebuild_basis
    }

    pub fn invalidation_basis(&self) -> &ForgeQueryGraphReadAccessInvalidationBasis {
        &self.invalidation_basis
    }

    pub fn complexity_contract(&self) -> &ForgeQueryGraphReadAccessComplexityContract {
        &self.complexity_contract
    }

    pub fn owning_milestone(&self) -> Option<&str> {
        self.owning_milestone.as_deref()
    }

    pub fn outcome(&self) -> &ForgeQueryGraphIndexInventoryMatchOutcome {
        &self.outcome
    }

    pub fn required_capability_owner(&self) -> &ForgeQueryGraphReadRequiredCapabilityOwner {
        &self.required_capability_owner
    }

    pub fn resolved_admission_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.resolved_admission_posture
    }

    fn from_support_row(
        requirement: &ForgeQueryGraphReadAccessRequirementRow,
        row: &ForgeQueryGraphIndexSupportRow,
    ) -> Self {
        let outcome = classify_inventory_match_outcome(requirement, row);
        let (required_capability_owner, resolved_admission_posture) =
            if outcome == ForgeQueryGraphIndexInventoryMatchOutcome::ExactMatch {
                resolved_exact_match_posture(row)
            } else {
                (
                    ForgeQueryGraphReadRequiredCapabilityOwner::LowerRuntime,
                    ForgeQueryGraphReadAccessAdmissionPosture::Denied,
                )
            };
        Self {
            requirement_kind: requirement.kind().clone(),
            requirement_row_digest: requirement.digest_part(),
            requirement_semantic_slot: requirement.semantic_slot_key(),
            support_row_digest: row.digest().to_string(),
            support_posture: row.posture().clone(),
            support_state: row.support_state().clone(),
            lifecycle_owner: row.lifecycle_owner().clone(),
            lifecycle_class: row.lifecycle_class().clone(),
            rebuild_basis: row.rebuild_basis().clone(),
            invalidation_basis: row.invalidation_basis().clone(),
            complexity_contract: row.complexity_contract().clone(),
            owning_milestone: row.owning_milestone().map(str::to_string),
            outcome,
            required_capability_owner,
            resolved_admission_posture,
        }
    }

    fn missing_support_row(requirement: &ForgeQueryGraphReadAccessRequirementRow) -> Self {
        Self {
            requirement_kind: requirement.kind().clone(),
            requirement_row_digest: requirement.digest_part(),
            requirement_semantic_slot: requirement.semantic_slot_key(),
            support_row_digest: hash_parts(&[
                "forge_query_graph_index_missing_support_row_v1".to_string(),
                requirement.digest_part(),
            ]),
            support_posture: ForgeQueryGraphIndexPosture::Denied,
            support_state: ForgeQueryGraphIndexSupportState::Unsupported,
            lifecycle_owner: ForgeQueryGraphIndexLifecycleOwner::LowerRuntime,
            lifecycle_class: ForgeQueryGraphIndexLifecycleClass::Unsupported,
            rebuild_basis: requirement.rebuild_basis().clone(),
            invalidation_basis: requirement.invalidation_basis().clone(),
            complexity_contract: requirement.complexity_contract().clone(),
            owning_milestone: None,
            outcome: ForgeQueryGraphIndexInventoryMatchOutcome::MissingSupportRow,
            required_capability_owner: ForgeQueryGraphReadRequiredCapabilityOwner::LowerRuntime,
            resolved_admission_posture: ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "match:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.requirement_kind.as_str(),
            self.requirement_row_digest,
            self.requirement_semantic_slot,
            self.support_row_digest,
            self.support_posture.as_str(),
            self.support_state.as_str(),
            self.lifecycle_owner.as_str(),
            self.lifecycle_class.as_str(),
            self.rebuild_basis.as_str(),
            self.invalidation_basis.as_str(),
            self.complexity_contract.as_str(),
            self.owning_milestone.as_deref().unwrap_or("none"),
            self.outcome.as_str(),
            self.required_capability_owner.as_str(),
            self.resolved_admission_posture.as_str()
        )
    }
}

fn resolved_exact_match_posture(
    row: &ForgeQueryGraphIndexSupportRow,
) -> (
    ForgeQueryGraphReadRequiredCapabilityOwner,
    ForgeQueryGraphReadAccessAdmissionPosture,
) {
    match row.posture() {
        ForgeQueryGraphIndexPosture::Verified
        | ForgeQueryGraphIndexPosture::RuntimeMaintained
        | ForgeQueryGraphIndexPosture::LowerRuntimeOwned => (
            ForgeQueryGraphReadRequiredCapabilityOwner::QueryRuntime,
            ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
        ),
        ForgeQueryGraphIndexPosture::EphemeralAvailable => (
            ForgeQueryGraphReadRequiredCapabilityOwner::QueryRuntime,
            ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
        ),
        ForgeQueryGraphIndexPosture::RequiresAccessCapabilityRegistration => (
            ForgeQueryGraphReadRequiredCapabilityOwner::DomainRegistration,
            ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
        ),
        ForgeQueryGraphIndexPosture::RequiresStoreBackedPersistentIndex => (
            ForgeQueryGraphReadRequiredCapabilityOwner::PersistentStore,
            ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
        ),
        ForgeQueryGraphIndexPosture::TemporarilyUnavailable => (
            ForgeQueryGraphReadRequiredCapabilityOwner::LowerRuntime,
            ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired,
        ),
        ForgeQueryGraphIndexPosture::Denied => (
            ForgeQueryGraphReadRequiredCapabilityOwner::LowerRuntime,
            ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        ),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphIndexInventoryMatchReport {
    digest: String,
    inventory_digest: String,
    requirement_set_digest: String,
    matches: Vec<ForgeQueryGraphIndexInventoryMatch>,
    counters: ForgeQueryGraphIndexInventoryCounters,
}

impl ForgeQueryGraphIndexInventoryMatchReport {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn matches(&self) -> &[ForgeQueryGraphIndexInventoryMatch] {
        &self.matches
    }

    pub fn counters(&self) -> &ForgeQueryGraphIndexInventoryCounters {
        &self.counters
    }

    pub fn includes_admission_posture(
        &self,
        posture: &ForgeQueryGraphReadAccessAdmissionPosture,
    ) -> bool {
        self.matches
            .iter()
            .any(|row| row.resolved_admission_posture() == posture)
    }

    pub(crate) fn match_requirements(
        requirements: &ForgeQueryGraphReadAccessRequirementSet,
        inventory: &ForgeQueryGraphIndexInventory,
    ) -> Self {
        let matches = requirements
            .rows()
            .iter()
            .map(|requirement| {
                select_best_support_row_for_requirement(requirement, inventory).map_or_else(
                    || ForgeQueryGraphIndexInventoryMatch::missing_support_row(requirement),
                    |row| ForgeQueryGraphIndexInventoryMatch::from_support_row(requirement, row),
                )
            })
            .collect::<Vec<_>>();
        let unsupported_requirement_count = matches
            .iter()
            .filter(|row| {
                row.outcome() != &ForgeQueryGraphIndexInventoryMatchOutcome::ExactMatch
                    || !row.support_posture().is_supported()
            })
            .count();
        let counters = ForgeQueryGraphIndexInventoryCounters::new(
            inventory.rows().len(),
            requirements.rows().len(),
            matches.len(),
            unsupported_requirement_count,
            0,
        );
        let digest = hash_parts(
            &[
                "forge_query_graph_index_inventory_match_report_v1".to_string(),
                format!("inventory:{}", inventory.digest()),
                format!("requirements:{}", requirements.digest().as_str()),
                counters.digest_part(),
            ]
            .into_iter()
            .chain(
                matches
                    .iter()
                    .map(ForgeQueryGraphIndexInventoryMatch::digest_part),
            )
            .collect::<Vec<_>>(),
        );
        Self {
            digest,
            inventory_digest: inventory.digest().to_string(),
            requirement_set_digest: requirements.digest().as_str().to_string(),
            matches,
            counters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{
        ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
        ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
        ForgeQueryGraphReadAccessRequirementRow,
    };

    #[test]
    fn missing_inventory_row_remains_localized_in_match_report() {
        let requirement = ForgeQueryGraphReadAccessRequirementRow::new(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
            ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
        );
        let requirements = ForgeQueryGraphReadAccessRequirementSet::new(
            "read-graph",
            "access-shape",
            "selectivity-shape",
            vec![requirement],
        );
        let inventory = ForgeQueryGraphIndexInventory::from_rows(Vec::new());
        let report =
            ForgeQueryGraphIndexInventoryMatchReport::match_requirements(&requirements, &inventory);

        assert_eq!(report.matches().len(), 1);
        assert_eq!(report.counters().matched_requirement_count(), 1);
        assert_eq!(report.counters().unsupported_requirement_count(), 1);
        assert_eq!(
            report.matches()[0].outcome(),
            &ForgeQueryGraphIndexInventoryMatchOutcome::MissingSupportRow
        );
        assert_eq!(
            report.matches()[0].resolved_admission_posture(),
            &ForgeQueryGraphReadAccessAdmissionPosture::Denied
        );
    }
}
