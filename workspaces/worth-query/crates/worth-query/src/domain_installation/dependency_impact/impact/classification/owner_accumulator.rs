use std::collections::BTreeSet;

use super::*;

pub(super) struct OwnerImpactAccumulator {
    roles: BTreeSet<WorthQuerySemanticDependencyRole>,
    role_edges: [usize; WorthQuerySemanticDependencyRole::COUNT],
    floor: WorthQueryImpactClass,
    owner_change_count: usize,
    counters: WorthQueryImpactCounters,
}

impl OwnerImpactAccumulator {
    pub(super) fn new(owner_change_count: usize, mut counters: WorthQueryImpactCounters) -> Self {
        counters.owner_changes_inspected = owner_change_count;
        counters.conditional_outcomes_inspected += 1;
        Self {
            roles: BTreeSet::new(),
            role_edges: [0; WorthQuerySemanticDependencyRole::COUNT],
            floor: WorthQueryImpactClass::UnaffectedOrSuppressed,
            owner_change_count,
            counters,
        }
    }

    pub(super) fn collect_change(
        &mut self,
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        change: &BridgeDeliveredCorrespondenceChange,
    ) {
        let (floor, lookups) =
            collect_affected_roles(closure, change, &mut self.roles, &mut self.role_edges);
        self.counters.index_lookups += lookups;
        self.floor = widen_impact(self.floor, floor);
    }

    pub(super) fn apply_conditional(
        &mut self,
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        conditional: &WorthQueryConditionalProvenance,
        dependency_ordinal: usize,
    ) -> Result<(), WorthQueryImpactAdmissionDenial> {
        if conditional_suppresses_output(conditional.class()) {
            let role =
                WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness;
            self.roles.remove(&role);
            self.role_edges[role.canonical_ordinal()] = 0;
            return Ok(());
        }
        self.counters.index_lookups += 1;
        self.counters.dependency_membership_lookups += 1;
        let Some(roles) =
            closure.conditional_consequence_roles(conditional.location(), dependency_ordinal)
        else {
            return Err(impact_denial(
                WorthQueryImpactAdmissionDenialKind::ForeignConditionalOutcome,
                self.counters,
            ));
        };
        for role in roles {
            self.roles.insert(role);
            self.role_edges[role.canonical_ordinal()] += 1;
        }
        Ok(())
    }

    pub(super) fn finish(
        mut self,
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        delivery: &BridgeCorrespondenceDeliveryReceipt,
        conditional: &WorthQueryConditionalProvenance,
    ) -> WorthQueryImpactDecision {
        let affected_roles = self.roles.into_iter().collect::<Vec<_>>();
        let affected_dependency_count = self.role_edges.into_iter().sum();
        self.counters.affected_edges = affected_dependency_count;
        let role_class = affected_roles
            .iter()
            .copied()
            .map(class_for_role)
            .fold(WorthQueryImpactClass::UnaffectedOrSuppressed, widen_impact);
        let unsupported = affected_roles.is_empty()
            && self.owner_change_count > 0
            && self.floor == WorthQueryImpactClass::UnaffectedOrSuppressed
            && !conditional_suppresses_output(conditional.class());
        WorthQueryImpactDecision {
            class: if unsupported {
                WorthQueryImpactClass::UnsupportedEscalation
            } else {
                widen_impact(role_class, self.floor)
            },
            affected_dependency_count,
            affected_roles,
            owner_change_count: self.owner_change_count,
            counters: self.counters,
            checked_basis: WorthQueryCheckedImpactBasis::owner_conditional(
                closure,
                delivery,
                conditional,
            ),
        }
    }
}
