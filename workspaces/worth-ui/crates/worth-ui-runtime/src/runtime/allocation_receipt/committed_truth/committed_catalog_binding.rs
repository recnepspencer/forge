#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationCatalogBinding {
    ordinal: u16,
    planning_identity_digest: u64,
    measurement_basis_identity_digest: u64,
    neighborhood_identity_digest: u64,
    receipt_identity: super::UiAllocationReceiptIdentity,
    receipt_generation: super::UiAllocationReceiptGeneration,
    receipt: super::UiAllocationReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationCatalogBindings {
    rows: Box<[UiCommittedAllocationCatalogBinding]>,
    identity_digest: u64,
}

impl UiCommittedAllocationCatalogBindings {
    pub(super) fn seal(
        candidates: &[super::UiAllocationCandidate],
        receipts: &[super::UiAllocationReceipt],
    ) -> Result<Self, super::UiAllocationReceiptCommitDenial> {
        if candidates.len() != receipts.len() {
            return Err(super::UiAllocationReceiptCommitDenial::CatalogBindingCardinalityMismatch);
        }
        let mut rows = Vec::with_capacity(candidates.len());
        let mut identity = crate::declaration::stable_text_digest(
            "worth-ui.committed-allocation-catalog-bindings",
        );
        for (ordinal, (candidate, receipt)) in candidates.iter().zip(receipts).enumerate() {
            let ordinal = u16::try_from(ordinal).map_err(|_| {
                super::UiAllocationReceiptCommitDenial::CatalogBindingCardinalityMismatch
            })?;
            if candidate.planning_identity_digest()
                != receipt.generation().planning_evidence_digest()
                || candidate.measurement_basis().graph_node_identity()
                    != receipt.identity().graph_node_identity()
                || crate::evidence::UiAllocationNeighborhoodScope::from_neighborhood(
                    candidate.allocation_neighborhood(),
                ) != *receipt.identity().neighborhood_scope()
            {
                return Err(
                    super::UiAllocationReceiptCommitDenial::CatalogBindingIdentityMismatch {
                        ordinal,
                    },
                );
            }
            let row = UiCommittedAllocationCatalogBinding {
                ordinal,
                planning_identity_digest: candidate.planning_identity_digest(),
                measurement_basis_identity_digest: candidate.measurement_basis().identity_digest(),
                neighborhood_identity_digest: candidate
                    .allocation_neighborhood()
                    .identity()
                    .identity_digest(),
                receipt_identity: receipt.identity().clone(),
                receipt_generation: receipt.generation(),
                receipt: receipt.clone(),
            };
            identity = identity.rotate_left(7)
                ^ row.planning_identity_digest
                ^ row.measurement_basis_identity_digest.rotate_left(17)
                ^ row.neighborhood_identity_digest.rotate_left(29)
                ^ row.receipt_identity.identity_digest().rotate_left(37)
                ^ row.receipt_generation.identity_digest().rotate_left(41)
                ^ row
                    .receipt
                    .equivalence_basis()
                    .identity_digest()
                    .rotate_left(53);
            rows.push(row);
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
            identity_digest: identity,
        })
    }
    pub(crate) fn rows(&self) -> &[UiCommittedAllocationCatalogBinding] {
        &self.rows
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

impl UiCommittedAllocationCatalogBinding {
    pub(crate) fn measurement_basis_identity_digest(&self) -> u64 {
        self.measurement_basis_identity_digest
    }
    pub(crate) fn receipt_identity(&self) -> &super::UiAllocationReceiptIdentity {
        &self.receipt_identity
    }
    pub(crate) fn receipt_generation(&self) -> super::UiAllocationReceiptGeneration {
        self.receipt_generation
    }
    pub(crate) fn receipt(&self) -> &super::UiAllocationReceipt {
        &self.receipt
    }
}

/// Unique capability authorizing publication of one committed catalog.
/// Historical committed bindings remain cloneable inspection data; only this
/// move-only wrapper may cross into activation.
#[derive(Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationCatalogActivation {
    rows: Box<[UiCommittedAllocationCatalogActivationRow]>,
    identity_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationCatalogActivationRow {
    binding: UiCommittedAllocationCatalogBinding,
    measurement_basis: std::rc::Rc<crate::evidence::UiMeasurementBasis>,
    neighborhood: std::rc::Rc<crate::evidence::UiAllocationNeighborhood>,
    planning_identity_digest: Option<u64>,
    graph_replan_admission: crate::graph::UiGraphReplanAdmission,
    committed_invalidation_context:
        crate::runtime::invalidation_narrowing::UiCommittedAllocationInvalidationContext,
    scroll_sources: Box<[UiCommittedScrollActivationSource]>,
    portal_source: Option<UiCommittedPortalActivationSource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiCommittedScrollActivationSource {
    Host {
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
        contract: crate::runtime::scroll_owned_allocation::UiAdmittedScrollOwnedContract,
    },
    Query {
        contract: crate::runtime::scroll_owned_allocation::UiAdmittedScrollOwnedContract,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiCommittedPortalActivationSource {
    Host {
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
        contract: crate::runtime::portal_anchored_allocation::UiAdmittedPortalAnchorContract,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiCommittedAllocationCatalogActivationDenial {
    CardinalityMismatch,
    MissingReplanAdmission {
        ordinal: u16,
    },
    ScrollAuthority {
        ordinal: u16,
        denial: crate::runtime::UiScrollContractAdmissionDenial,
    },
    PortalAuthority {
        ordinal: u16,
    },
}

impl UiCommittedAllocationCatalogActivation {
    pub(in crate::runtime) fn seal(
        candidates: &[super::UiAllocationCandidate],
        bindings: &UiCommittedAllocationCatalogBindings,
    ) -> Result<Self, UiCommittedAllocationCatalogActivationDenial> {
        if candidates.len() != bindings.rows().len() {
            return Err(UiCommittedAllocationCatalogActivationDenial::CardinalityMismatch);
        }
        let rows = candidates
            .iter()
            .zip(bindings.rows())
            .enumerate()
            .map(|(ordinal, (candidate, binding))| {
                let ordinal = u16::try_from(ordinal).map_err(|_| {
                    UiCommittedAllocationCatalogActivationDenial::CardinalityMismatch
                })?;
                let admission = candidate.replan_admission_opt().ok_or(
                    UiCommittedAllocationCatalogActivationDenial::MissingReplanAdmission {
                        ordinal,
                    },
                )?;
                let (
                    measurement_basis,
                    neighborhood,
                    planning_identity_digest,
                    graph_replan_admission,
                ) = admission.committed_structural_parts();
                Ok(UiCommittedAllocationCatalogActivationRow {
                    scroll_sources: admission.committed_scroll_sources().map_err(|denial| {
                        UiCommittedAllocationCatalogActivationDenial::ScrollAuthority {
                            ordinal,
                            denial,
                        }
                    })?,
                    portal_source: admission.committed_portal_source().ok_or(
                        UiCommittedAllocationCatalogActivationDenial::PortalAuthority { ordinal },
                    )?,
                    measurement_basis,
                    neighborhood,
                    planning_identity_digest,
                    graph_replan_admission,
                    committed_invalidation_context: admission.commit(),
                    binding: binding.clone(),
                })
            })
            .collect::<Result<Vec<_>, UiCommittedAllocationCatalogActivationDenial>>()?
            .into_boxed_slice();
        Ok(Self {
            rows,
            identity_digest: bindings.identity_digest(),
        })
    }

    pub(crate) fn rows(&self) -> &[UiCommittedAllocationCatalogActivationRow] {
        &self.rows
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub(crate) fn validate_portal_bindings(
        &self,
    ) -> Result<(), crate::runtime::UiPortalActivationBindingDenial> {
        for (ordinal, row) in self.rows.iter().enumerate() {
            let Some(source) = row.portal_source() else {
                continue;
            };
            let ordinal = u16::try_from(ordinal).map_err(|_| {
                crate::runtime::UiPortalActivationBindingDenial::CardinalityExceeded
            })?;
            let UiCommittedPortalActivationSource::Host { witness, contract } = source;
            if contract.neighborhood_identity() != row.neighborhood().identity() {
                return Err(
                    crate::runtime::UiPortalActivationBindingDenial::NeighborhoodMismatch {
                        ordinal,
                    },
                );
            }
            if !matches!(
                row.receipt().geometry_evidence().anchor_posture(),
                crate::runtime::UiAllocationAnchorPosture::PortalAnchored(identity)
                    if identity == contract.identity()
            ) {
                return Err(
                    crate::runtime::UiPortalActivationBindingDenial::AnchorIdentityMismatch {
                        ordinal,
                    },
                );
            }
            let witness_is_retained =
                row.measurement_basis()
                    .evidence_inputs()
                    .iter()
                    .any(|input| {
                        input
                            .as_host_measurement_result()
                            .is_some_and(|result| result.authority_witness() == *witness)
                    });
            if !witness_is_retained {
                return Err(
                    crate::runtime::UiPortalActivationBindingDenial::HostWitnessMismatch {
                        ordinal,
                    },
                );
            }
        }
        Ok(())
    }
}

impl UiCommittedScrollActivationSource {
    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::Host { witness, contract } => {
                crate::declaration::stable_text_digest("worth-ui.activation.scroll.host")
                    ^ witness.identity_digest().rotate_left(17)
                    ^ contract.identity_digest().rotate_left(37)
            }
            Self::Query { contract } => {
                crate::declaration::stable_text_digest("worth-ui.activation.scroll.query")
                    ^ contract.identity_digest().rotate_left(37)
            }
        }
    }
}

impl UiCommittedPortalActivationSource {
    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::Host { witness, contract } => {
                crate::declaration::stable_text_digest("worth-ui.activation.portal.host")
                    ^ witness.identity_digest().rotate_left(17)
                    ^ contract.identity_digest().rotate_left(37)
            }
        }
    }
}

impl UiCommittedAllocationCatalogActivationRow {
    pub(crate) fn scope(&self) -> crate::evidence::UiAllocationNeighborhoodScope {
        crate::evidence::UiAllocationNeighborhoodScope::from_neighborhood(self.neighborhood())
    }
    pub(crate) fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis {
        &self.measurement_basis
    }
    pub(crate) fn neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood {
        &self.neighborhood
    }
    pub(crate) fn planning_identity_digest(&self) -> Option<u64> {
        self.planning_identity_digest
    }
    pub(crate) fn graph_replan_admission(&self) -> crate::graph::UiGraphReplanAdmission {
        self.graph_replan_admission.clone()
    }
    pub(crate) fn committed_invalidation_context(
        &self,
    ) -> &crate::runtime::invalidation_narrowing::UiCommittedAllocationInvalidationContext {
        &self.committed_invalidation_context
    }
    pub(crate) fn measurement_basis_identity_digest(&self) -> u64 {
        self.binding.measurement_basis_identity_digest()
    }
    pub(crate) fn receipt_identity(&self) -> &super::UiAllocationReceiptIdentity {
        self.binding.receipt_identity()
    }
    pub(crate) fn receipt_generation(&self) -> super::UiAllocationReceiptGeneration {
        self.binding.receipt_generation()
    }
    pub(crate) fn receipt(&self) -> &super::UiAllocationReceipt {
        self.binding.receipt()
    }
    pub(crate) fn scroll_sources(&self) -> &[UiCommittedScrollActivationSource] {
        &self.scroll_sources
    }
    pub(crate) fn portal_source(&self) -> Option<&UiCommittedPortalActivationSource> {
        self.portal_source.as_ref()
    }
}
