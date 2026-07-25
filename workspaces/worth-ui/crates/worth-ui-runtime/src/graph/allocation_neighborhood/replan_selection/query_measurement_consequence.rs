#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiQueryMeasurementReplanConsequence {
    neighborhood_identity_digest: u64,
    predecessor_basis_identity_digest: u64,
    receipt: crate::evidence::UiSettledQueryFactReceipt,
}

impl UiQueryMeasurementReplanConsequence {
    pub(in crate::graph::allocation_neighborhood) fn seal(
        target: &crate::graph::UiAdmittedAllocationInvalidationTargetSet,
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Result<Self, super::UiReplanLocalityDenial> {
        let predecessor = target
            .primary()
            .allocation_plan()
            .ok_or(super::UiReplanLocalityDenial::MissingAdmittedCandidate)?
            .candidate()
            .measurement_basis();
        let receipt = crate::evidence::consume_settled_query_measurement_fact(
            predecessor.declaration_identity().clone(),
            predecessor.declaration_support_authority_generation(),
            predecessor.declared_measurement_policy(),
            view_binding_id,
            fact,
        )
        .map_err(|_| super::UiReplanLocalityDenial::QueryMeasurementSuccessorDenied)?;
        predecessor
            .succeed_settled_query_receipt(&receipt)
            .map_err(|_| super::UiReplanLocalityDenial::QueryMeasurementSuccessorDenied)?;
        Ok(Self {
            neighborhood_identity_digest: target
                .primary()
                .neighborhood_identity()
                .identity_digest(),
            predecessor_basis_identity_digest: predecessor.identity_digest(),
            receipt,
        })
    }

    pub(crate) fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub(crate) fn predecessor_basis_identity_digest(&self) -> u64 {
        self.predecessor_basis_identity_digest
    }

    pub(crate) fn receipt(&self) -> &crate::evidence::UiSettledQueryFactReceipt {
        &self.receipt
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        crate::declaration::stable_text_digest("worth-ui.query-measurement-consequence")
            ^ self.neighborhood_identity_digest.rotate_left(11)
            ^ self.predecessor_basis_identity_digest.rotate_left(23)
            ^ self.receipt.observation_identity_digest().rotate_left(37)
            ^ self.receipt.source_generation().rotate_left(47)
            ^ self.receipt.source_order().rotate_left(53)
    }
}
