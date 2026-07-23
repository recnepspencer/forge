#[derive(Clone, Debug)]
pub(crate) struct UiPortalInvalidationBindingIndex {
    pub(super) by_request: crate::runtime::persistent_index::UiPersistentOrdMap<
        worth_ui_host_contract::UiMeasurementRequestIdentity,
        super::UiAdmittedPortalInvalidationBinding,
    >,
    pub(super) identity_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalBindingDenial {
    DuplicateRequestIdentity,
    MissingPredecessorBinding,
    MissingGraphTarget,
    ReceiptContextMismatch,
}

impl Default for UiPortalInvalidationBindingIndex {
    fn default() -> Self {
        Self {
            by_request: Default::default(),
            identity_digest: crate::declaration::stable_text_digest(
                "worth-ui.portal-binding-index",
            ),
        }
    }
}

impl UiPortalInvalidationBindingIndex {
    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
    pub(crate) fn seal(
        activation: &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Result<Self, UiPortalBindingDenial> {
        let mut by_request = crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut identity_digest = Self::default().identity_digest;
        for row in activation.rows() {
            let Some(crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::Host {
                witness,
                contract,
            }) = row.portal_source()
            else {
                continue;
            };
            if row.receipt_identity() != row.receipt().identity()
                || row.receipt_generation() != row.receipt().generation()
            {
                return Err(UiPortalBindingDenial::ReceiptContextMismatch);
            }
            let target = graph
                .target_set_for_neighborhood(
                    row.receipt_identity().graph_node_identity(),
                    contract.neighborhood_identity(),
                )
                .ok_or(UiPortalBindingDenial::MissingGraphTarget)?;
            let binding = super::UiAdmittedPortalInvalidationBinding::seal(
                contract.clone(),
                target,
                row.receipt(),
                *witness,
                row.measurement_basis(),
            )
            .ok_or(UiPortalBindingDenial::ReceiptContextMismatch)?;
            let request = witness.request_identity();
            if by_request.get(&request).is_some() {
                return Err(UiPortalBindingDenial::DuplicateRequestIdentity);
            }
            identity_digest ^= binding.identity_digest();
            by_request.insert(request, binding);
        }
        Ok(Self {
            by_request,
            identity_digest,
        })
    }

    pub(crate) fn movement(
        &self,
        result: &crate::evidence::UiMeasurementResult,
    ) -> Result<Option<super::UiAdmittedPortalMovement>, UiPortalMovementLookupDenial> {
        let witness = result.authority_witness();
        let Some(binding) = self.by_request.get(&witness.request_identity()) else {
            return Ok(None);
        };
        if !binding
            .activation_witness()
            .same_normalization_authority(witness)
        {
            return Err(UiPortalMovementLookupDenial::NormalizationAuthorityMismatch);
        }
        super::UiAdmittedPortalMovement::seal(binding.clone(), result, 1)
            .map(Some)
            .map_err(UiPortalMovementLookupDenial::SuccessorBasis)
    }

    pub(crate) fn prepare_succession(
        &self,
        committed: &crate::runtime::UiCommittedAllocationReplan,
    ) -> Result<super::UiPreparedPortalBindingSuccession, super::UiPortalBindingSuccessionDenial>
    {
        let mut successor = self.clone();
        let mut lineage = Vec::new();
        let mut counters = super::UiPortalBindingSuccessionCounters::default();
        for consequence in committed.transaction().consequences().portal_anchors() {
            counters.visit()?;
            let movement = consequence.movement();
            let request = movement
                .observation()
                .authority_witness()
                .request_identity();
            counters.binding_lookup()?;
            let prior = successor.by_request.get(&request).ok_or(
                super::UiPortalBindingSuccessionDenial::MissingRequestBinding {
                    request_identity: request,
                },
            )?;
            if prior.receipt_identity() != movement.receipt_identity()
                || prior.receipt_generation() != movement.receipt_generation()
            {
                return Err(
                    super::UiPortalBindingSuccessionDenial::StalePredecessorReceipt {
                        request_identity: request,
                        expected_identity_digest: movement.receipt_identity().identity_digest(),
                        observed_identity_digest: prior.receipt_identity().identity_digest(),
                        expected_generation_digest: movement.receipt_generation().identity_digest(),
                        observed_generation_digest: prior.receipt_generation().identity_digest(),
                    },
                );
            }
            counters.receipt_lookup()?;
            let receipt = committed
                .receipts()
                .iter()
                .find(|receipt| {
                    receipt
                        .committed_allocation()
                        .allocation_neighborhood()
                        .identity()
                        == movement.target().primary().neighborhood_identity()
                })
                .ok_or(
                    super::UiPortalBindingSuccessionDenial::MissingCommittedReceipt {
                        neighborhood_identity_digest: movement
                            .target()
                            .primary()
                            .neighborhood_identity()
                            .identity_digest(),
                    },
                )?;
            let portal = receipt
                .committed_allocation()
                .planning_basis()
                .portal_allocation_input()
                .ok_or(
                    super::UiPortalBindingSuccessionDenial::MissingCanonicalPortalInput {
                        receipt_identity_digest: receipt.identity().identity_digest(),
                    },
                )?;
            let binding = prior.seal_successor(portal, receipt).ok_or(
                super::UiPortalBindingSuccessionDenial::SuccessorContractDenied {
                    request_identity: request,
                },
            )?;
            lineage.push(super::UiPortalBindingSuccessionLineage::new(
                request, prior, receipt, portal,
            ));
            successor.identity_digest ^= prior.identity_digest() ^ binding.identity_digest();
            successor.by_request.insert(request, binding);
            counters.replacement()?;
        }
        Ok(super::UiPreparedPortalBindingSuccession::new(
            self.identity_digest(),
            successor,
            lineage,
            counters,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPortalMovementLookupDenial {
    NormalizationAuthorityMismatch,
    SuccessorBasis(crate::runtime::UiPortalAnchorSuccessorDenial),
}
