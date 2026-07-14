#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiAllocationNeighborhoodActivationDenial {
    StalePredecessor,
    ScrollBinding(super::UiScrollOwnerCatalogDenialReport),
    PortalBinding(crate::runtime::UiPortalActivationBindingDenial),
}

pub(crate) struct UiPreparedInvalidationCatalogTransition {
    successor: super::UiAllocationInvalidationAuthority,
    scroll_evidence: crate::runtime::UiScrollCatalogSwapEvidence,
}

impl UiPreparedInvalidationCatalogTransition {
    pub(crate) fn scroll_catalog_evidence(&self) -> crate::runtime::UiScrollCatalogSwapEvidence {
        self.scroll_evidence.clone()
    }
}

impl super::UiAllocationInvalidationAuthority {
    pub(crate) fn prepare_catalog_transition(
        &self,
        delta: &super::UiAllocationNeighborhoodCatalogTransition,
    ) -> Result<UiPreparedInvalidationCatalogTransition, UiAllocationNeighborhoodActivationDenial>
    {
        let active_contexts = delta
            .certifies_successor()
            .then(|| delta.successor_committed_contexts().into_boxed_slice())
            .ok_or(UiAllocationNeighborhoodActivationDenial::StalePredecessor)?;
        let mut successor = self.clone();
        if !successor
            .graph_replan
            .apply_activation_transition(delta.transition())
        {
            return Err(UiAllocationNeighborhoodActivationDenial::StalePredecessor);
        }
        let committed_bindings = delta.committed_bindings();
        let predecessor_identity = delta.transition().predecessor_identity_digest();
        let successor_identity = delta.transition().successor_identity_digest();
        let scroll_bindings = super::UiScrollInvalidationBindingIndex::seal(
            committed_bindings,
            &successor.graph_replan,
            predecessor_identity,
            successor_identity,
        )
        .map_err(UiAllocationNeighborhoodActivationDenial::ScrollBinding)?;
        let scroll_evidence = crate::runtime::UiScrollCatalogSwapEvidence::Prepared(
            scroll_bindings
                .catalog_receipt
                .clone()
                .expect("sealed scroll bindings carry their receipt"),
        );
        let portal_bindings = super::UiPortalInvalidationBindingIndex::seal(
            committed_bindings,
            &successor.graph_replan,
        )
        .map_err(|denial| {
            let denial = match denial {
                super::portal_binding_index::UiPortalBindingDenial::DuplicateRequestIdentity => {
                    crate::runtime::UiPortalActivationBindingDenial::CardinalityExceeded
                }
                super::portal_binding_index::UiPortalBindingDenial::MissingGraphTarget => {
                    crate::runtime::UiPortalActivationBindingDenial::NeighborhoodMismatch {
                        ordinal: 0,
                    }
                }
                super::portal_binding_index::UiPortalBindingDenial::ReceiptContextMismatch => {
                    crate::runtime::UiPortalActivationBindingDenial::AnchorIdentityMismatch {
                        ordinal: 0,
                    }
                }
            };
            UiAllocationNeighborhoodActivationDenial::PortalBinding(denial)
        })?;
        successor.active_contexts = active_contexts.into_vec();
        successor.scroll_bindings = scroll_bindings;
        successor.portal_bindings = portal_bindings;
        successor.rebuild_indexes();
        Ok(UiPreparedInvalidationCatalogTransition {
            successor,
            scroll_evidence,
        })
    }

    pub(crate) fn commit_catalog_transition(
        &mut self,
        prepared: UiPreparedInvalidationCatalogTransition,
    ) {
        *self = prepared.successor;
    }
}
