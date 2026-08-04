use super::{
    derive_graph_read_cost_evidence, estimate_graph_read_access_cost,
    match_graph_index_inventory_for_requirements, WorthQueryGraphIndexInventory,
    WorthQueryGraphIndexInventoryMatchReport, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessCostEstimate, WorthQueryGraphReadAccessRequirementKind,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadBudget,
    WorthQueryGraphReadBudgetCheck, WorthQueryGraphReadBudgetClassKind,
    WorthQueryGraphReadPlanReviewDenial, WorthQueryGraphReadPlanReviewDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadPlanReview {
    requirements: WorthQueryGraphReadAccessRequirementSet,
    cost_estimate: WorthQueryGraphReadAccessCostEstimate,
    budget_check: WorthQueryGraphReadBudgetCheck,
    inventory: WorthQueryGraphIndexInventory,
    inventory_match: WorthQueryGraphIndexInventoryMatchReport,
    posture: WorthQueryGraphReadAccessAdmissionPosture,
    denial: Option<WorthQueryGraphReadPlanReviewDenial>,
}

impl WorthQueryGraphReadPlanReview {
    pub fn requirements(&self) -> &WorthQueryGraphReadAccessRequirementSet {
        &self.requirements
    }

    pub fn cost_estimate(&self) -> &WorthQueryGraphReadAccessCostEstimate {
        &self.cost_estimate
    }

    pub fn budget_check(&self) -> &WorthQueryGraphReadBudgetCheck {
        &self.budget_check
    }

    pub fn inventory(&self) -> &WorthQueryGraphIndexInventory {
        &self.inventory
    }

    pub fn inventory_match(&self) -> &WorthQueryGraphIndexInventoryMatchReport {
        &self.inventory_match
    }

    pub fn posture(&self) -> &WorthQueryGraphReadAccessAdmissionPosture {
        &self.posture
    }

    pub fn denial(&self) -> Option<&WorthQueryGraphReadPlanReviewDenial> {
        self.denial.as_ref()
    }

    pub const fn is_admitted(&self) -> bool {
        self.denial.is_none()
    }
}

#[doc(hidden)]
pub fn review_graph_read_access(
    requirements: WorthQueryGraphReadAccessRequirementSet,
    inventory: WorthQueryGraphIndexInventory,
    budget: WorthQueryGraphReadBudget,
) -> WorthQueryGraphReadPlanReview {
    let evidence = derive_graph_read_cost_evidence(&requirements);
    let estimate = estimate_graph_read_access_cost(&requirements, evidence);
    let budget_check = budget.check_supported_cost(&estimate);
    let inventory_match = match_graph_index_inventory_for_requirements(&requirements, &inventory);
    let (posture, denial) = review_posture(&requirements, &budget_check, &inventory_match);
    WorthQueryGraphReadPlanReview {
        requirements,
        cost_estimate: estimate,
        budget_check,
        inventory,
        inventory_match,
        posture,
        denial,
    }
}

fn review_posture(
    requirements: &WorthQueryGraphReadAccessRequirementSet,
    budget: &WorthQueryGraphReadBudgetCheck,
    inventory: &WorthQueryGraphIndexInventoryMatchReport,
) -> (
    WorthQueryGraphReadAccessAdmissionPosture,
    Option<WorthQueryGraphReadPlanReviewDenial>,
) {
    if budget.class().kind() == &WorthQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget {
        if streaming_frontier_is_admissible(requirements) {
            return (
                WorthQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming,
                None,
            );
        }
        if includes(
            inventory,
            WorthQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
        ) {
            return denied(WorthQueryGraphReadPlanReviewDenialKind::RequiredPersistentIndex);
        }
        return denied(WorthQueryGraphReadPlanReviewDenialKind::BudgetExceeded);
    }
    for (posture, denial_kind) in [
        (
            WorthQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired,
            WorthQueryGraphReadPlanReviewDenialKind::RequiredAsyncMaterialization,
        ),
        (
            WorthQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
            WorthQueryGraphReadPlanReviewDenialKind::RequiredAccessCapabilityRegistration,
        ),
        (
            WorthQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
            WorthQueryGraphReadPlanReviewDenialKind::RequiredPersistentIndex,
        ),
        (
            WorthQueryGraphReadAccessAdmissionPosture::Denied,
            WorthQueryGraphReadPlanReviewDenialKind::UnsupportedGraphIndexSupport,
        ),
    ] {
        if includes(inventory, posture) {
            return denied(denial_kind);
        }
    }
    if includes(
        inventory,
        WorthQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
    ) {
        return (
            WorthQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
            None,
        );
    }
    (
        WorthQueryGraphReadAccessAdmissionPosture::InlineIndexed,
        None,
    )
}

fn denied(
    kind: WorthQueryGraphReadPlanReviewDenialKind,
) -> (
    WorthQueryGraphReadAccessAdmissionPosture,
    Option<WorthQueryGraphReadPlanReviewDenial>,
) {
    (
        WorthQueryGraphReadAccessAdmissionPosture::Denied,
        Some(WorthQueryGraphReadPlanReviewDenial::new(kind)),
    )
}

fn includes(
    report: &WorthQueryGraphIndexInventoryMatchReport,
    posture: WorthQueryGraphReadAccessAdmissionPosture,
) -> bool {
    report.includes_admission_posture(&posture)
}

fn streaming_frontier_is_admissible(
    requirements: &WorthQueryGraphReadAccessRequirementSet,
) -> bool {
    requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::TraversalWorkset)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::VisitedSet)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::DedupSet)
        && requirements.requires_kind(WorthQueryGraphReadAccessRequirementKind::ProofSupport)
        && !requirements
            .requires_kind(WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport)
}
