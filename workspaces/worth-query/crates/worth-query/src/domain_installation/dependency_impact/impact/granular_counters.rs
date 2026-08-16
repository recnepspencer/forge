use super::super::WorthQuerySemanticDependencyRole;

/// Work performed while narrowing lower-runtime granular deliveries into
/// current Query-owned invalidation impacts.
///
/// These counters are descriptive evidence. They cannot admit an impact or
/// authorize maintenance.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGranularAdmissionCounters {
    delivery_changes_examined: usize,
    locality_entries_examined: usize,
    impact_index_probes: usize,
    candidate_deliveries_returned: usize,
    candidate_roles_returned: usize,
    candidates_rejected_before_admission: usize,
    admitted_impacts: usize,
    admitted_roles: [usize; WorthQuerySemanticDependencyRole::COUNT],
}

impl WorthQueryGranularAdmissionCounters {
    pub const fn delivery_changes_examined(self) -> usize {
        self.delivery_changes_examined
    }

    pub const fn locality_entries_examined(self) -> usize {
        self.locality_entries_examined
    }

    pub const fn impact_index_probes(self) -> usize {
        self.impact_index_probes
    }

    pub const fn candidate_deliveries_returned(self) -> usize {
        self.candidate_deliveries_returned
    }

    pub const fn candidate_roles_returned(self) -> usize {
        self.candidate_roles_returned
    }

    pub const fn candidates_rejected_before_admission(self) -> usize {
        self.candidates_rejected_before_admission
    }

    pub const fn admitted_impacts(self) -> usize {
        self.admitted_impacts
    }

    pub const fn admitted_role_count(self, role: WorthQuerySemanticDependencyRole) -> usize {
        self.admitted_roles[role.canonical_ordinal()]
    }

    pub(super) fn inspect_delivery(
        &mut self,
        delivery: &worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
    ) {
        let changes = delivery.truth().change_set().changes();
        self.delivery_changes_examined += changes.len();
        self.locality_entries_examined += changes
            .iter()
            .filter(|change| change.semantic_change().is_some())
            .count();
    }

    pub(super) fn retain_candidates(
        &mut self,
        index_probes: usize,
        roles: &[WorthQuerySemanticDependencyRole],
    ) {
        self.impact_index_probes += index_probes;
        self.candidate_deliveries_returned += 1;
        self.candidate_roles_returned += roles.len();
    }

    pub(super) fn reject_candidate(&mut self) {
        self.candidates_rejected_before_admission += 1;
    }

    pub(super) fn admit_roles(&mut self, roles: &[WorthQuerySemanticDependencyRole]) {
        self.admitted_impacts += 1;
        for role in roles {
            self.admitted_roles[role.canonical_ordinal()] += 1;
        }
    }
}
