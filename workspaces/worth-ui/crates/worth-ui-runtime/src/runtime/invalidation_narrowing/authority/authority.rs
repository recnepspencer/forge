use crate::graph::UiGraphNodeIdentity;
use std::rc::Rc;

#[path = "host_target/mapping.rs"]
mod host_target_mapping;

type PersistentScopes = crate::runtime::persistent_index::UiPersistentOrdSet<
    crate::evidence::UiAllocationNeighborhoodScope,
>;
type PersistentWitnessOwners = crate::runtime::persistent_index::UiPersistentOrdMap<
    crate::evidence::UiHostMeasurementAuthorityWitness,
    usize,
>;
/// The currently admitted allocation context. This retains the proof-bearing
/// graph neighborhood and measurement basis themselves; it does not rebuild a
/// second dependency index from their contents.
#[derive(Clone, Debug, Default)]
pub(crate) struct UiAllocationInvalidationAuthority {
    pub(in crate::runtime) catalog: super::UiActiveAllocationCatalog,
    #[cfg(test)]
    pub(super) fixture_contexts: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::evidence::UiAllocationNeighborhoodScope,
        UiCommittedAllocationInvalidationContext,
    >,
    pub(super) query_contexts: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
        PersistentScopes,
    >,
    pub(super) host_targets_by_witness: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::evidence::UiHostMeasurementAuthorityWitness,
        UiHostInvalidationTargetMapping,
    >,
    pub(super) host_scopes_by_witness: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::evidence::UiHostMeasurementAuthorityWitness,
        PersistentScopes,
    >,
    pub(super) scroll_bindings: super::UiScrollInvalidationBindingIndex,
    pub(super) portal_bindings: super::UiPortalInvalidationBindingIndex,
    pub(super) host_witnesses_by_request: crate::runtime::persistent_index::UiPersistentOrdMap<
        (
            worth_ui_host_contract::UiMeasurementRequestIdentity,
            crate::evidence::UiMeasurementEvidenceCategory,
        ),
        PersistentWitnessOwners,
    >,
    pub(super) durable_contexts:
        crate::runtime::persistent_index::UiPersistentOrdMap<u64, PersistentScopes>,
    pub(super) graph_replan: crate::graph::UiGraphReplanAuthority,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationInvalidationContext {
    pub(super) basis: Rc<crate::evidence::UiMeasurementBasis>,
    pub(super) neighborhood: Rc<crate::evidence::UiAllocationNeighborhood>,
    pub(super) allocation_plan: Option<crate::graph::UiAdmittedAllocationPlanReference>,
    pub(super) replacement_impact:
        Option<Rc<crate::runtime::WorthUiReplacementImpactClassification>>,
    pub(super) impact_narrowing: Option<Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>>,
    pub(super) graph_replan_admission: crate::graph::UiGraphReplanAdmission,
    pub(super) scroll_planning:
        Option<crate::runtime::scroll_owned_allocation::UiAdmittedScrollPlanningAuthority>,
    pub(super) scroll_planning_denial: Option<crate::runtime::UiScrollContractAdmissionDenial>,
    pub(super) portal_planning:
        Option<crate::runtime::portal_anchored_allocation::UiAdmittedPortalPlanningAuthority>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiAllocationInvalidationAdmissionContext {
    pub(super) basis: Rc<crate::evidence::UiMeasurementBasis>,
    pub(super) neighborhood: Rc<crate::evidence::UiAllocationNeighborhood>,
    pub(super) allocation_plan: Option<crate::graph::UiAdmittedAllocationPlanReference>,
    pub(super) replacement_impact:
        Option<Rc<crate::runtime::WorthUiReplacementImpactClassification>>,
    pub(super) impact_narrowing: Option<Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>>,
    pub(super) graph_replan_admission: crate::graph::UiGraphReplanAdmission,
    pub(super) scroll_planning:
        Option<crate::runtime::scroll_owned_allocation::UiAdmittedScrollPlanningAuthority>,
    pub(super) scroll_planning_denial: Option<crate::runtime::UiScrollContractAdmissionDenial>,
    pub(super) portal_planning:
        Option<crate::runtime::portal_anchored_allocation::UiAdmittedPortalPlanningAuthority>,
}

pub(crate) struct UiInvalidationAuthorityLookup {
    pub(crate) target: Option<crate::graph::UiAdmittedAllocationInvalidationTargetSet>,
    pub(crate) probes: u16,
}

pub(crate) struct UiHostInvalidationAuthorityLookup {
    target: Option<crate::graph::UiAdmittedAllocationInvalidationTargetSet>,
    pub(crate) probes: u16,
}

#[derive(Clone, Debug, Default)]
pub(super) struct UiHostInvalidationTargetMapping {
    node_owners: crate::runtime::persistent_index::UiPersistentOrdMap<UiGraphNodeIdentity, usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiInvalidationAuthorityLookupDenial {
    QueryAuthorityNotIndexable,
    HostEvidenceGenerationMismatch,
    HostNormalizationAuthorityMismatch,
    AuthorityCounterExhausted,
}

impl UiAllocationInvalidationAuthority {
    pub(in crate::runtime) fn prepare_portal_binding_succession(
        &self,
        permit: &crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        committed: &crate::runtime::UiCommittedAllocationReplan,
    ) -> Result<super::UiPreparedPortalBindingSuccession, super::UiPortalBindingSuccessionDenial>
    {
        if !permit.certifies_committed(committed) {
            return Err(super::UiPortalBindingSuccessionDenial::TransactionPermitMismatch);
        }
        self.portal_bindings.prepare_succession(committed)
    }
    pub(in crate::runtime) fn commit_portal_binding_succession(
        &mut self,
        permit: &mut crate::runtime::allocation_frame_dispatch::UiAllocationTransactionAuthority,
        prepared: super::UiPreparedPortalBindingSuccession,
    ) {
        debug_assert!(
            prepared.predecessor_identity_digest() == self.portal_binding_identity_digest()
        );
        let _ = permit;
        self.portal_bindings = prepared.successor;
    }
    pub(crate) fn portal_binding_identity_digest(&self) -> u64 {
        self.portal_bindings.identity_digest()
    }
    pub(crate) fn portal_movement(
        &self,
        result: &crate::evidence::UiMeasurementResult,
    ) -> Result<
        Option<super::UiAdmittedPortalMovement>,
        super::portal_binding_index::UiPortalMovementLookupDenial,
    > {
        self.portal_bindings.movement(result)
    }
    #[cfg(test)]
    pub(crate) fn scroll_binding_counters(&self) -> crate::runtime::UiScrollBindingCatalogCounters {
        self.scroll_bindings
            .catalog_receipt
            .as_ref()
            .map_or_else(Default::default, |receipt| receipt.counters())
    }
    pub(crate) fn scroll_projection_target(
        &self,
        owner: crate::runtime::UiScrollProjectionOwnerIdentity,
    ) -> Option<crate::runtime::UiActivatedScrollProjectionTarget> {
        self.scroll_bindings.projection(owner)
    }
    pub(crate) fn validate_scroll_projection_receipt(
        &self,
        target: crate::runtime::UiActivatedScrollProjectionTarget,
        key: &crate::runtime::UiScrollReceiptActivationKey,
    ) -> Result<(), crate::runtime::UiScrollOwnerAcquisitionDenial> {
        self.scroll_bindings
            .validate_projection_receipt(target, key)
    }
    pub(crate) fn acquire_host_scroll_projection(
        &self,
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
        receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.scroll_bindings.projection_for_host(witness, receipt)
    }
    pub(crate) fn acquire_query_scroll_projection(
        &self,
        authority: &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.scroll_bindings
            .projection_for_query(authority, allocation_receipt)
    }
    pub(crate) fn acquire_settled_query_scroll_projection(
        &self,
        receipt: &crate::evidence::UiSettledQueryFactReceipt,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.scroll_bindings
            .projection_for_settled_query(receipt, allocation_receipt)
    }
    pub(crate) fn seal_replan_transaction_basis(
        &self,
        plan: &crate::runtime::UiNarrowedAllocationFramePlan,
    ) -> Result<crate::graph::UiAdmittedReplanNeighborhoodSet, crate::graph::UiReplanLocalityDenial>
    {
        self.graph_replan.seal_transaction_basis(plan)
    }

    pub(crate) fn seal_catalog_transition(
        &self,
        catalog: &super::UiAllocationActivationCatalog,
        activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        activation_identity: crate::runtime::UiCommittedAllocationActivationIdentity,
        affected_predecessor_scopes: Option<Box<[crate::evidence::UiAllocationNeighborhoodScope]>>,
    ) -> super::UiAllocationNeighborhoodCatalogTransition {
        super::UiAllocationNeighborhoodCatalogTransition::seal(
            &self.graph_replan,
            catalog,
            activation,
            activation_identity,
            affected_predecessor_scopes,
        )
    }

    pub(crate) fn graph_target(
        &self,
        identity: UiGraphNodeIdentity,
    ) -> Result<UiInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        Ok(UiInvalidationAuthorityLookup {
            target: self.graph_replan.target_set(identity),
            probes: 1,
        })
    }

    pub(crate) fn host_target(
        &self,
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    ) -> Result<UiHostInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        if !self.has_invalidation_contexts() {
            return Ok(UiHostInvalidationAuthorityLookup {
                target: None,
                probes: 0,
            });
        }
        if let Some(mapping) = self.host_targets_by_witness.get(&witness) {
            return Ok(UiHostInvalidationAuthorityLookup {
                target: mapping.materialize(&self.graph_replan),
                probes: 1,
            });
        }
        let Some(admitted) = self
            .host_witnesses_by_request
            .get(&(witness.request_identity(), witness.evidence_category()))
        else {
            return Ok(UiHostInvalidationAuthorityLookup {
                target: None,
                probes: 1,
            });
        };
        if admitted
            .iter()
            .all(|(candidate, _)| candidate.evidence_generation() != witness.evidence_generation())
        {
            return Err(UiInvalidationAuthorityLookupDenial::HostEvidenceGenerationMismatch);
        }
        Err(UiInvalidationAuthorityLookupDenial::HostNormalizationAuthorityMismatch)
    }

    pub(crate) fn query_target(
        &self,
        authority: &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
    ) -> Result<UiInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        if !self.has_invalidation_contexts() {
            return Ok(UiInvalidationAuthorityLookup {
                target: None,
                probes: 0,
            });
        }
        let source_key = crate::evidence::measurement::basis::UiQueryAllocationSourceKey::from_managed_live_compatibility(authority)
            .map_err(|_| UiInvalidationAuthorityLookupDenial::QueryAuthorityNotIndexable)?;
        self.query_target_for_source(&source_key)
    }

    pub(crate) fn settled_query_target(
        &self,
        source_key: &crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
    ) -> Result<UiInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        self.query_target_for_source(source_key)
    }

    fn query_target_for_source(
        &self,
        source_key: &crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
    ) -> Result<UiInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        if !self.has_invalidation_contexts() {
            return Ok(UiInvalidationAuthorityLookup {
                target: None,
                probes: 0,
            });
        }
        let Some(ordinals) = self.query_contexts.get(source_key) else {
            return Ok(UiInvalidationAuthorityLookup {
                target: None,
                probes: 1,
            });
        };
        let mut probes = 1u16;
        for scope in ordinals.iter() {
            let Some(context) = self.context_for_scope(scope) else {
                continue;
            };
            probes = probes
                .checked_add(1)
                .ok_or(UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted)?;
            for mapping in context
                .basis
                .query_allocation_mappings_for_source(source_key)
            {
                probes = probes
                    .checked_add(1)
                    .ok_or(UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted)?;
                if let Some(target) = self.graph_replan.target_set(mapping.target()) {
                    return Ok(UiInvalidationAuthorityLookup {
                        target: Some(target.with_graph_index_probes(probes)),
                        probes,
                    });
                }
            }
        }
        Ok(UiInvalidationAuthorityLookup {
            target: None,
            probes,
        })
    }

    pub(crate) fn durable_target(
        &self,
        input_identity_digest: u64,
    ) -> Result<UiInvalidationAuthorityLookup, UiInvalidationAuthorityLookupDenial> {
        if !self.has_invalidation_contexts() {
            return Ok(UiInvalidationAuthorityLookup {
                target: None,
                probes: 0,
            });
        }
        let Some(ordinals) = self.durable_contexts.get(&input_identity_digest) else {
            return Ok(UiInvalidationAuthorityLookup {
                target: None,
                probes: 1,
            });
        };
        let mut probes = 1u16;
        for scope in ordinals.iter() {
            let Some(context) = self.context_for_scope(scope) else {
                continue;
            };
            probes = probes
                .checked_add(1)
                .ok_or(UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted)?;
            let support = context.basis.durable_resize_support(input_identity_digest);
            let target_identity = support.map(|support| support.target_graph_node_identity());
            probes = probes
                .checked_add(u16::from(target_identity.is_some()))
                .ok_or(UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted)?;
            if let Some(target) =
                target_identity.and_then(|identity| self.graph_replan.target_set(identity))
            {
                return Ok(UiInvalidationAuthorityLookup {
                    target: Some(target.with_graph_index_probes(probes)),
                    probes,
                });
            }
        }
        Ok(UiInvalidationAuthorityLookup {
            target: None,
            probes,
        })
    }
}
impl UiHostInvalidationAuthorityLookup {
    pub(crate) fn target_count(&self) -> usize {
        self.target
            .as_ref()
            .map_or(0, |target| target.neighborhood_count())
    }

    pub(crate) fn materialize_target(
        self,
    ) -> Option<crate::graph::UiAdmittedAllocationInvalidationTargetSet> {
        self.target
    }
}
