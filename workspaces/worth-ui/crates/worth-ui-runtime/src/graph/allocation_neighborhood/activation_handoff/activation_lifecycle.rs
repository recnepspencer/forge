#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiGraphNeighborhoodLifecycleEntry {
    neighborhood: crate::evidence::UiAllocationNeighborhoodIdentity,
    graph_snapshot_authority_digest: u64,
    planning_identity_digest: Option<u64>,
    pub(in crate::graph::allocation_neighborhood) admission: super::UiGraphReplanAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNeighborhoodActivationTransition {
    predecessor: Box<[UiGraphNeighborhoodLifecycleEntry]>,
    successor: Box<[UiGraphNeighborhoodLifecycleEntry]>,
}

impl UiGraphNeighborhoodLifecycleEntry {
    fn admitted(
        neighborhood: crate::evidence::UiAllocationNeighborhoodIdentity,
        graph_snapshot_authority_digest: u64,
        planning_identity_digest: Option<u64>,
        admission: super::UiGraphReplanAdmission,
    ) -> Self {
        Self {
            neighborhood,
            graph_snapshot_authority_digest,
            planning_identity_digest,
            admission,
        }
    }
}

impl super::UiGraphReplanAuthority {
    pub(crate) fn seal_activation_transition(
        &self,
        entries: Vec<(
            crate::evidence::UiAllocationNeighborhoodIdentity,
            u64,
            Option<u64>,
            super::UiGraphReplanAdmission,
        )>,
    ) -> UiGraphNeighborhoodActivationTransition {
        let mut successor = entries
            .into_iter()
            .map(|(neighborhood, snapshot, planning, admission)| {
                UiGraphNeighborhoodLifecycleEntry::admitted(
                    neighborhood,
                    snapshot,
                    planning,
                    admission,
                )
            })
            .collect::<Vec<_>>();
        successor.sort_by_key(|entry| entry.neighborhood.identity_digest());
        UiGraphNeighborhoodActivationTransition {
            predecessor: self.active_neighborhoods.clone().into_boxed_slice(),
            successor: successor.into_boxed_slice(),
        }
    }

    pub(crate) fn certifies_activation_transition(
        &self,
        transition: &UiGraphNeighborhoodActivationTransition,
    ) -> bool {
        self.active_neighborhoods.as_slice() == transition.predecessor.as_ref()
    }

    pub(crate) fn apply_activation_transition(
        &mut self,
        transition: &UiGraphNeighborhoodActivationTransition,
    ) -> bool {
        if !self.certifies_activation_transition(transition) {
            return false;
        }
        self.active_neighborhoods = transition.successor.to_vec();
        self.rebuild_active_targets();
        true
    }
}

impl UiGraphNeighborhoodActivationTransition {
    pub(crate) fn successor_len(&self) -> usize {
        self.successor.len()
    }
    pub(crate) fn predecessor_identity_digest(&self) -> u64 {
        lifecycle_identity("worth-ui.graph-neighborhood-predecessor", &self.predecessor)
    }
    pub(crate) fn successor_identity_digest(&self) -> u64 {
        lifecycle_identity("worth-ui.graph-neighborhood-successor", &self.successor)
    }

    pub(crate) fn successor_ordinal(
        &self,
        neighborhood: &crate::evidence::UiAllocationNeighborhoodIdentity,
        snapshot: u64,
        planning: Option<u64>,
    ) -> Option<usize> {
        self.successor
            .iter()
            .position(|candidate| candidate.matches(neighborhood, snapshot, planning))
    }
}

fn lifecycle_identity(label: &str, rows: &[UiGraphNeighborhoodLifecycleEntry]) -> u64 {
    rows.iter().fold(
        crate::declaration::stable_text_digest(label),
        |digest, row| {
            digest.rotate_left(7)
                ^ row.neighborhood.identity_digest()
                ^ row.graph_snapshot_authority_digest.rotate_left(19)
                ^ row
                    .planning_identity_digest
                    .unwrap_or_default()
                    .rotate_left(37)
        },
    )
}

impl UiGraphNeighborhoodLifecycleEntry {
    fn matches(
        &self,
        neighborhood: &crate::evidence::UiAllocationNeighborhoodIdentity,
        snapshot: u64,
        planning: Option<u64>,
    ) -> bool {
        &self.neighborhood == neighborhood
            && self.graph_snapshot_authority_digest == snapshot
            && self.planning_identity_digest == planning
    }
}
