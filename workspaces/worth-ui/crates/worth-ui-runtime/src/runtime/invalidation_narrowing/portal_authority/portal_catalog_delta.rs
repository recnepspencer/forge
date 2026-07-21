type ActivationRow = crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivationRow;

impl super::UiPortalInvalidationBindingIndex {
    pub(super) fn apply_catalog_delta(
        &mut self,
        removed: &[ActivationRow],
        changed: &[ActivationRow],
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Result<(), super::portal_binding_index::UiPortalBindingDenial> {
        for row in removed {
            let Some(crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::Host {
                witness,
                ..
            }) = row.portal_source()
            else {
                continue;
            };
            let request = witness.request_identity();
            let binding = self.by_request.get(&request).cloned().ok_or(
                super::portal_binding_index::UiPortalBindingDenial::MissingPredecessorBinding,
            )?;
            self.identity_digest ^= binding.identity_digest();
            if !self.by_request.remove(&request) {
                return Err(
                    super::portal_binding_index::UiPortalBindingDenial::MissingPredecessorBinding,
                );
            }
        }
        for row in changed {
            self.insert_catalog_row(row, graph)?;
        }
        Ok(())
    }

    fn insert_catalog_row(
        &mut self,
        row: &ActivationRow,
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Result<(), super::portal_binding_index::UiPortalBindingDenial> {
        let Some(crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::Host {
            witness,
            contract,
        }) = row.portal_source()
        else {
            return Ok(());
        };
        if row.receipt_identity() != row.receipt().identity()
            || row.receipt_generation() != row.receipt().generation()
        {
            return Err(super::portal_binding_index::UiPortalBindingDenial::ReceiptContextMismatch);
        }
        let target = graph
            .target_set_for_neighborhood(
                row.receipt_identity().graph_node_identity(),
                contract.neighborhood_identity(),
            )
            .ok_or(super::portal_binding_index::UiPortalBindingDenial::MissingGraphTarget)?;
        let binding = super::UiAdmittedPortalInvalidationBinding::seal(
            contract.clone(),
            target,
            row.receipt(),
            *witness,
            row.measurement_basis(),
        )
        .ok_or(super::portal_binding_index::UiPortalBindingDenial::ReceiptContextMismatch)?;
        let request = witness.request_identity();
        if self.by_request.get(&request).is_some() {
            return Err(
                super::portal_binding_index::UiPortalBindingDenial::DuplicateRequestIdentity,
            );
        }
        self.identity_digest ^= binding.identity_digest();
        self.by_request.insert(request, binding);
        Ok(())
    }
}
