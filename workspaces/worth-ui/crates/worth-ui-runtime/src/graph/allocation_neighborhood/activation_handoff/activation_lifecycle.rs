#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiGraphNeighborhoodLifecycleEntry {
    scope: crate::evidence::UiAllocationNeighborhoodScope,
    neighborhood: crate::evidence::UiAllocationNeighborhoodIdentity,
    graph_snapshot_authority_digest: u64,
    planning_identity_digest: Option<u64>,
    pub(in crate::graph::allocation_neighborhood) admission: super::UiGraphReplanAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNeighborhoodActivationTransition {
    predecessor: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::evidence::UiAllocationNeighborhoodScope,
        UiGraphNeighborhoodLifecycleEntry,
    >,
    successor: Box<[UiGraphNeighborhoodLifecycleEntry]>,
}

impl UiGraphNeighborhoodLifecycleEntry {
    fn admitted(
        scope: crate::evidence::UiAllocationNeighborhoodScope,
        neighborhood: crate::evidence::UiAllocationNeighborhoodIdentity,
        graph_snapshot_authority_digest: u64,
        planning_identity_digest: Option<u64>,
        admission: super::UiGraphReplanAdmission,
    ) -> Self {
        Self {
            scope,
            neighborhood,
            graph_snapshot_authority_digest,
            planning_identity_digest,
            admission,
        }
    }
}

impl super::UiGraphReplanAuthority {
    pub(crate) fn apply_activation_delta(
        &mut self,
        affected: &[crate::evidence::UiAllocationNeighborhoodScope],
        changed: &[crate::runtime::UiCommittedAllocationCatalogActivationRow],
    ) -> Option<(u64, u64)> {
        let predecessor_identity = self.active_identity_digest;
        for scope in affected {
            let entry = self.active_neighborhoods.get(scope).cloned()?;
            self.remove_admission(&entry.admission);
            self.active_neighborhoods.remove(scope);
        }
        for row in changed {
            let entry = UiGraphNeighborhoodLifecycleEntry::admitted(
                row.scope(),
                row.neighborhood().identity().clone(),
                row.neighborhood().graph_snapshot_authority_digest(),
                row.planning_identity_digest(),
                row.graph_replan_admission(),
            );
            self.insert_admission(&entry.admission);
            self.active_neighborhoods.insert(entry.scope(), entry);
        }
        let successor_identity = changed.iter().fold(
            affected.iter().fold(
                predecessor_identity
                    ^ crate::declaration::stable_text_digest("allocation-catalog-delta"),
                |digest: u64, scope: &crate::evidence::UiAllocationNeighborhoodScope| {
                    digest.rotate_left(7) ^ scope.identity_digest()
                },
            ),
            |digest: u64, row: &crate::runtime::UiCommittedAllocationCatalogActivationRow| {
                digest.rotate_left(11)
                    ^ row.scope().identity_digest()
                    ^ row
                        .neighborhood()
                        .identity()
                        .identity_digest()
                        .rotate_left(23)
            },
        );
        self.active_identity_digest = successor_identity;
        Some((predecessor_identity, successor_identity))
    }

    pub(crate) fn seal_activation_transition(
        &self,
        entries: Vec<(
            crate::evidence::UiAllocationNeighborhoodScope,
            crate::evidence::UiAllocationNeighborhoodIdentity,
            u64,
            Option<u64>,
            super::UiGraphReplanAdmission,
        )>,
    ) -> UiGraphNeighborhoodActivationTransition {
        let mut successor = entries
            .into_iter()
            .map(|(scope, neighborhood, snapshot, planning, admission)| {
                UiGraphNeighborhoodLifecycleEntry::admitted(
                    scope,
                    neighborhood,
                    snapshot,
                    planning,
                    admission,
                )
            })
            .collect::<Vec<_>>();
        successor.sort_by_key(|entry| entry.neighborhood.identity_digest());
        UiGraphNeighborhoodActivationTransition {
            predecessor: self.active_neighborhoods.clone(),
            successor: successor.into_boxed_slice(),
        }
    }

    pub(crate) fn certifies_activation_transition(
        &self,
        transition: &UiGraphNeighborhoodActivationTransition,
    ) -> bool {
        self.active_neighborhoods == transition.predecessor
    }

    pub(crate) fn apply_activation_transition(
        &mut self,
        transition: &UiGraphNeighborhoodActivationTransition,
    ) -> bool {
        if !self.certifies_activation_transition(transition) {
            return false;
        }
        self.active_neighborhoods = Default::default();
        for entry in &transition.successor {
            self.active_neighborhoods
                .insert(entry.scope(), entry.clone());
        }
        self.rebuild_active_targets();
        self.active_identity_digest = transition.successor_identity_digest();
        true
    }
}

impl UiGraphNeighborhoodActivationTransition {
    pub(crate) fn successor_len(&self) -> usize {
        self.successor.len()
    }
    pub(crate) fn predecessor_identity_digest(&self) -> u64 {
        lifecycle_identity(
            "worth-ui.graph-neighborhood-predecessor",
            self.predecessor.iter().map(|(_, row)| row),
        )
    }
    pub(crate) fn successor_identity_digest(&self) -> u64 {
        lifecycle_identity(
            "worth-ui.graph-neighborhood-successor",
            self.successor.iter(),
        )
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

fn lifecycle_identity<'a>(
    label: &str,
    rows: impl Iterator<Item = &'a UiGraphNeighborhoodLifecycleEntry>,
) -> u64 {
    rows.fold(
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
    pub(in crate::graph::allocation_neighborhood) fn scope(
        &self,
    ) -> crate::evidence::UiAllocationNeighborhoodScope {
        self.scope.clone()
    }

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
