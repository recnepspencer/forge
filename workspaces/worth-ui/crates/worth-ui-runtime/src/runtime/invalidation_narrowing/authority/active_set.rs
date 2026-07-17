#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationActivationCatalogDenial {
    EmptyCatalog,
    CandidateNotAdmitted { ordinal: u16 },
    IncompatibleGraphAuthority { ordinal: u16 },
    ReplacementLineageMismatch { ordinal: u16 },
    DuplicateNeighborhood { ordinal: u16 },
}

/// Complete admitted allocation-neighborhood catalog for one active graph lineage.
#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationActivationCatalog {
    candidates: Box<[crate::runtime::UiAllocationCandidate]>,
    contexts: Box<[super::UiAllocationInvalidationAdmissionContext]>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiAllocationNeighborhoodCatalogTransition {
    transition: crate::graph::UiGraphNeighborhoodActivationTransition,
    activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    activation_identity: crate::runtime::UiCommittedAllocationActivationIdentity,
}

impl UiAllocationActivationCatalog {
    pub(crate) fn from_planning(
        mut candidates: Vec<crate::runtime::UiAllocationCandidate>,
        _authority: crate::runtime::launch::UiAllocationCatalogMintAuthority,
    ) -> Result<Self, UiAllocationActivationCatalogDenial> {
        candidates.sort_by_key(|candidate| {
            let neighborhood = candidate.allocation_neighborhood();
            (
                neighborhood.members().len(),
                neighborhood.root_graph_node_identity().digest(),
                neighborhood.identity().identity_digest(),
            )
        });
        let Some(first) = candidates.first() else {
            return Err(UiAllocationActivationCatalogDenial::EmptyCatalog);
        };
        let first_identity = first.allocation_neighborhood().identity().identity_digest();
        candidates.sort_by_key(|candidate| {
            candidate
                .allocation_neighborhood()
                .identity()
                .identity_digest()
                .ne(&first_identity)
        });
        let first = candidates.first().expect("non-empty catalog was proven");
        let world = first
            .allocation_neighborhood()
            .identity()
            .world_identity_digest();
        let generation = first.allocation_neighborhood().graph_generation();
        let snapshot = first
            .allocation_neighborhood()
            .graph_snapshot_authority_digest();
        for (ordinal, candidate) in candidates.iter().enumerate() {
            if !candidate.is_admitted() {
                return Err(UiAllocationActivationCatalogDenial::CandidateNotAdmitted {
                    ordinal: ordinal as u16,
                });
            }
            let neighborhood = candidate.allocation_neighborhood();
            if neighborhood.identity().world_identity_digest() != world
                || neighborhood.graph_generation() != generation
                || neighborhood.graph_snapshot_authority_digest() != snapshot
            {
                return Err(
                    UiAllocationActivationCatalogDenial::IncompatibleGraphAuthority {
                        ordinal: ordinal as u16,
                    },
                );
            }
            if !candidate
                .replan_admission()
                .same_replacement_lineage(first.replan_admission())
            {
                return Err(
                    UiAllocationActivationCatalogDenial::ReplacementLineageMismatch {
                        ordinal: ordinal as u16,
                    },
                );
            }
        }
        for ordinal in 1..candidates.len() {
            if candidates[ordinal - 1].allocation_neighborhood().identity()
                == candidates[ordinal].allocation_neighborhood().identity()
            {
                return Err(UiAllocationActivationCatalogDenial::DuplicateNeighborhood {
                    ordinal: ordinal as u16,
                });
            }
        }
        let contexts = candidates
            .iter()
            .map(|candidate| candidate.replan_admission().clone())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self {
            candidates: candidates.into_boxed_slice(),
            contexts,
        })
    }

    pub(crate) fn activation_candidate(&self) -> &crate::runtime::UiAllocationCandidate {
        &self.candidates[0]
    }

    pub(in crate::runtime) fn candidates_for_commit(
        &self,
    ) -> &[crate::runtime::UiAllocationCandidate] {
        &self.candidates
    }

    pub(crate) fn certifies_activation_binding(&self, planning_identity_digest: u64) -> bool {
        let primary = self.activation_candidate();
        let primary_lowered = primary.planning().lowered_input();
        primary.planning_identity_digest() == planning_identity_digest
            && primary_lowered.is_some()
            && self.candidates.iter().all(|candidate| {
                candidate.is_admitted() && candidate.planning().lowered_input() == primary_lowered
            })
    }
}

impl UiAllocationNeighborhoodCatalogTransition {
    pub(super) fn seal(
        authority: &crate::graph::UiGraphReplanAuthority,
        _catalog: &UiAllocationActivationCatalog,
        activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        activation_identity: crate::runtime::UiCommittedAllocationActivationIdentity,
    ) -> Self {
        let entries = activation
            .rows()
            .iter()
            .map(|binding| {
                (
                    binding.neighborhood().identity().clone(),
                    binding.neighborhood().graph_snapshot_authority_digest(),
                    binding.planning_identity_digest(),
                    binding.graph_replan_admission(),
                )
            })
            .collect();
        Self {
            transition: authority.seal_activation_transition(entries),
            activation,
            activation_identity,
        }
    }

    pub(super) fn committed_bindings(
        &self,
    ) -> &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation {
        &self.activation
    }

    pub(super) fn successor_committed_contexts(
        &self,
    ) -> Vec<super::UiCommittedAllocationInvalidationContext> {
        self.activation
            .rows()
            .iter()
            .map(|row| row.committed_invalidation_context().clone())
            .collect()
    }

    pub(super) fn transition(&self) -> &crate::graph::UiGraphNeighborhoodActivationTransition {
        &self.transition
    }

    pub(super) fn certifies_successor(&self) -> bool {
        let rows = self.activation.rows();
        self.transition.successor_len() == rows.len()
            && rows.iter().all(|row| {
                self.transition
                    .successor_ordinal(
                        row.neighborhood().identity(),
                        row.neighborhood().graph_snapshot_authority_digest(),
                        row.planning_identity_digest(),
                    )
                    .is_some()
            })
    }
}
