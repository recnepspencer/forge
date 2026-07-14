#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationActivationIdentity {
    committed_binding_identity_digest: u64,
    committed_rows: Box<[UiCommittedAllocationActivationRowIdentity]>,
    ledger_lineage: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerLineage,
}

#[derive(Clone, Debug, PartialEq)]
struct UiCommittedAllocationActivationRowIdentity {
    measurement_basis: crate::evidence::UiMeasurementBasis,
    neighborhood_identity: crate::evidence::UiAllocationNeighborhoodIdentity,
    planning_identity_digest: Option<u64>,
    graph_replan_admission: crate::graph::UiGraphReplanAdmission,
    committed_invalidation_context:
        crate::runtime::invalidation_narrowing::UiCommittedAllocationInvalidationContext,
    receipt: crate::runtime::UiAllocationReceipt,
    scroll_sources: Box<[crate::runtime::allocation_receipt::UiCommittedScrollActivationSource]>,
    portal_source: Option<crate::runtime::allocation_receipt::UiCommittedPortalActivationSource>,
}

impl UiCommittedAllocationActivationIdentity {
    pub(super) fn seal(
        activation: &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        ledger_transition: &crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    ) -> Self {
        let mut committed_rows = Vec::with_capacity(activation.rows().len());
        for row in activation.rows() {
            committed_rows.push(UiCommittedAllocationActivationRowIdentity {
                measurement_basis: row.measurement_basis().clone(),
                neighborhood_identity: row.neighborhood().identity().clone(),
                planning_identity_digest: row.planning_identity_digest(),
                graph_replan_admission: row.graph_replan_admission(),
                committed_invalidation_context: row.committed_invalidation_context().clone(),
                receipt: row.receipt().clone(),
                scroll_sources: row.scroll_sources().into(),
                portal_source: row.portal_source().cloned(),
            });
        }
        Self {
            committed_binding_identity_digest: activation.identity_digest(),
            committed_rows: committed_rows.into_boxed_slice(),
            ledger_lineage: ledger_transition.structural_lineage(),
        }
    }

    pub(in crate::runtime) fn structural_digest(&self) -> u64 {
        self.committed_rows.iter().fold(
            self.committed_binding_identity_digest ^ self.ledger_lineage.identity_digest(),
            |digest, row| {
                let scroll_sources = row.scroll_sources.iter().fold(0_u64, |sources, source| {
                    sources.rotate_left(7) ^ source.identity_digest()
                });
                let portal_source = row
                    .portal_source
                    .as_ref()
                    .map_or(0, crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::identity_digest);
                digest.rotate_left(5)
                    ^ row.measurement_basis.identity_digest()
                    ^ row.neighborhood_identity.identity_digest().rotate_left(11)
                    ^ row
                        .planning_identity_digest
                        .unwrap_or_default()
                        .rotate_left(19)
                    ^ row.receipt.identity().identity_digest().rotate_left(31)
                    ^ row.receipt.generation().identity_digest().rotate_left(47)
                    ^ scroll_sources.rotate_left(53)
                    ^ portal_source.rotate_left(59)
            },
        )
    }

    pub(in crate::runtime) fn committed_row_count(&self) -> usize {
        self.committed_rows.len()
    }
}
