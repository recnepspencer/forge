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
    successor_binding_digest: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiAllocationNeighborhoodCatalogTransition {
    transition: crate::graph::UiGraphNeighborhoodActivationTransition,
    activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    activation_identity: crate::runtime::UiCommittedAllocationActivationIdentity,
    affected_predecessor_scopes: Option<Box<[crate::evidence::UiAllocationNeighborhoodScope]>>,
}

impl UiAllocationActivationCatalog {
    pub(crate) fn empty_successor(
        _authority: crate::runtime::launch::UiAllocationCatalogMintAuthority,
    ) -> Self {
        Self {
            candidates: Box::new([]),
            contexts: Box::new([]),
            successor_binding_digest: None,
        }
    }

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
            successor_binding_digest: None,
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
        if self.successor_binding_digest == Some(planning_identity_digest) {
            return true;
        }
        let primary = self.activation_candidate();
        let primary_projection = primary.planning().projection();
        primary.planning_identity_digest() == planning_identity_digest
            && primary_projection.is_some()
            && self.candidates.iter().all(|candidate| {
                candidate.is_admitted()
                    && candidate
                        .planning()
                        .projection()
                        .zip(primary_projection)
                        .is_some_and(|(candidate, primary)| {
                            candidate.shares_authority_with(primary)
                        })
            })
    }

    pub(crate) fn bind_catalog_successor(&mut self, allocation_identity_digest: u64) {
        self.successor_binding_digest = Some(allocation_identity_digest);
    }
}

impl UiAllocationNeighborhoodCatalogTransition {
    pub(super) fn seal(
        authority: &crate::graph::UiGraphReplanAuthority,
        _catalog: &UiAllocationActivationCatalog,
        activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        activation_identity: crate::runtime::UiCommittedAllocationActivationIdentity,
        affected_predecessor_scopes: Option<Box<[crate::evidence::UiAllocationNeighborhoodScope]>>,
    ) -> Self {
        let entries = activation
            .rows()
            .iter()
            .map(|binding| {
                (
                    binding.scope(),
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
            affected_predecessor_scopes,
        }
    }

    pub(super) fn committed_bindings(
        &self,
    ) -> &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation {
        &self.activation
    }

    pub(super) fn changed_rows(
        &self,
    ) -> &[crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivationRow] {
        self.activation.rows()
    }

    pub(super) fn affected_predecessor_scopes(
        &self,
    ) -> Option<&[crate::evidence::UiAllocationNeighborhoodScope]> {
        self.affected_predecessor_scopes.as_deref()
    }

    pub(super) fn certifies_successor(&self) -> bool {
        if self.affected_predecessor_scopes.is_some() {
            return self.activation.rows().iter().all(|row| {
                row.planning_identity_digest().is_some()
                    && row.receipt_identity() == row.receipt().identity()
                    && row.receipt_generation() == row.receipt().generation()
            });
        }
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
