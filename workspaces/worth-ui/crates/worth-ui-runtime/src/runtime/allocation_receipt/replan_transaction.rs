use crate::evidence::UiAllocationNeighborhoodIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReplanTransaction {
    frame_ingress_keys: Box<[crate::runtime::UiAllocationFrameIngressKey]>,
    primary_neighborhood: UiAllocationNeighborhoodIdentity,
    ordered_neighborhoods: Box<[UiAllocationNeighborhoodIdentity]>,
    widen_reasons: Box<[Option<crate::graph::UiReplanWidenReason>]>,
    expected_generations: Box<[crate::graph::UiReplanGenerationKey]>,
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    policy: crate::runtime::UiResolvedAllocationStreamPolicy,
    overlap_disposition: crate::graph::UiReplanOverlapDisposition,
    transaction_generation: u64,
    runtime_generation: u64,
    consequences: crate::graph::UiGraphReplanConsequences,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationReplanTransactionDenial {
    EmptyNeighborhoodSet,
    CardinalityMismatch,
    DuplicateNeighborhood,
    PrimaryMismatch,
}

impl UiAllocationReplanTransaction {
    pub(crate) fn for_replacement_activation(
        candidates: &[super::UiAllocationCandidate],
        runtime_generation: u64,
        transaction_generation: u64,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    ) -> Result<Self, UiAllocationReplanTransactionDenial> {
        let Some(primary) = candidates.first() else {
            return Err(UiAllocationReplanTransactionDenial::EmptyNeighborhoodSet);
        };
        let ordered = candidates
            .iter()
            .map(|candidate| candidate.allocation_neighborhood().identity().clone())
            .collect::<Vec<_>>();
        if ordered
            .iter()
            .enumerate()
            .any(|(index, identity)| ordered[..index].contains(identity))
        {
            return Err(UiAllocationReplanTransactionDenial::DuplicateNeighborhood);
        }
        let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
        let crate::runtime::stream_policy::UiAllocationStreamCommitDecision::Commit(receipt) =
            crate::runtime::stream_policy::resolve_stream_families(
                &[crate::runtime::UiAllocationStreamFamily::HostMeasurementReplacement],
                &mut payload,
            )
        else {
            return Err(UiAllocationReplanTransactionDenial::CardinalityMismatch);
        };
        Ok(Self {
            frame_ingress_keys: Box::new([]),
            primary_neighborhood: primary.allocation_neighborhood().identity().clone(),
            widen_reasons: vec![None; ordered.len()].into_boxed_slice(),
            expected_generations: Box::new([]),
            frame_epoch,
            policy: receipt.into_resolution_parts().0,
            overlap_disposition: if ordered.len() == 1 {
                crate::graph::UiReplanOverlapDisposition::Singleton
            } else {
                crate::graph::UiReplanOverlapDisposition::PairwiseDisjoint
            },
            ordered_neighborhoods: ordered.into_boxed_slice(),
            transaction_generation,
            runtime_generation,
            consequences: Default::default(),
        })
    }

    pub(crate) fn from_graph_basis(
        basis: &crate::graph::UiGraphReplanTransactionBasis,
        transaction_generation: u64,
        runtime_generation: u64,
    ) -> Result<Self, UiAllocationReplanTransactionDenial> {
        let primary = basis.primary_neighborhood().clone();
        let ordered = basis.ordered_neighborhoods().cloned().collect::<Vec<_>>();
        if ordered.is_empty() {
            return Err(UiAllocationReplanTransactionDenial::EmptyNeighborhoodSet);
        }
        for (index, item) in ordered.iter().enumerate() {
            if index == 0 && item != &primary {
                return Err(UiAllocationReplanTransactionDenial::PrimaryMismatch);
            }
            if ordered[..index].iter().any(|prior| prior == item) {
                return Err(UiAllocationReplanTransactionDenial::DuplicateNeighborhood);
            }
        }
        Ok(Self {
            frame_ingress_keys: basis.frame_ingress_keys().into(),
            primary_neighborhood: primary,
            ordered_neighborhoods: ordered.into(),
            widen_reasons: basis.widen_reasons().collect::<Vec<_>>().into_boxed_slice(),
            expected_generations: basis
                .expected_generations()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            frame_epoch: basis.frame_epoch(),
            policy: basis.policy(),
            overlap_disposition: basis.overlap_disposition(),
            transaction_generation,
            runtime_generation,
            consequences: basis.consequences().clone(),
        })
    }

    pub fn primary_neighborhood(&self) -> &UiAllocationNeighborhoodIdentity {
        &self.primary_neighborhood
    }
    pub(crate) fn consequences(&self) -> &crate::graph::UiGraphReplanConsequences {
        &self.consequences
    }
    pub fn ordered_neighborhoods(&self) -> &[UiAllocationNeighborhoodIdentity] {
        &self.ordered_neighborhoods
    }
    pub fn widen_reasons(&self) -> &[Option<crate::graph::UiReplanWidenReason>] {
        &self.widen_reasons
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn policy(&self) -> crate::runtime::UiResolvedAllocationStreamPolicy {
        self.policy
    }
    pub fn overlap_disposition(&self) -> crate::graph::UiReplanOverlapDisposition {
        self.overlap_disposition
    }
    pub fn transaction_generation(&self) -> u64 {
        self.transaction_generation
    }
    pub fn runtime_generation(&self) -> u64 {
        self.runtime_generation
    }
    pub(crate) fn same_idempotency_basis(&self, other: &Self) -> bool {
        self.primary_neighborhood == other.primary_neighborhood
            && self.frame_ingress_keys == other.frame_ingress_keys
            && self.ordered_neighborhoods == other.ordered_neighborhoods
            && self.widen_reasons == other.widen_reasons
            && self.expected_generations == other.expected_generations
            && self.frame_epoch == other.frame_epoch
            && self.policy == other.policy
            && self.overlap_disposition == other.overlap_disposition
            && self.runtime_generation == other.runtime_generation
            && self.consequences == other.consequences
    }

    pub(crate) fn idempotency_key(&self) -> u64 {
        let mut digest = 0xcbf29ce484222325u64;
        for key in &self.frame_ingress_keys {
            digest ^= key.ingress_identity().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
            digest ^= key.source_generation().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
            digest ^= key.source_order().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
        }
        for identity in &self.ordered_neighborhoods {
            digest ^= identity.identity_digest();
            digest = digest.wrapping_mul(0x100000001b3);
        }
        digest ^= self.frame_epoch.as_u64();
        digest = digest.wrapping_mul(0x100000001b3);
        digest ^= self.runtime_generation;
        digest = digest.wrapping_mul(0x100000001b3);
        digest = self.policy.mix_canonical_identity(digest);
        digest ^= match self.overlap_disposition {
            crate::graph::UiReplanOverlapDisposition::Singleton => 1,
            crate::graph::UiReplanOverlapDisposition::PairwiseDisjoint => 2,
            crate::graph::UiReplanOverlapDisposition::ContainmentMerged => 3,
            crate::graph::UiReplanOverlapDisposition::ContainmentSuperseded => 4,
        };
        digest = digest.wrapping_mul(0x100000001b3);
        for generation in &self.expected_generations {
            digest ^= generation.identity_digest();
            digest = digest.wrapping_mul(0x100000001b3);
        }
        digest ^= self.consequences.identity_digest();
        digest = digest.wrapping_mul(0x100000001b3);
        digest
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn commit_lane_change_cannot_replay_as_the_same_transaction() {
        let (mut runtime, target, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
        let completion = runtime.execute_framework_turn(|turn| {
            turn.interaction(|source| {
                source
                    .admit_and_submit(
                        target,
                        crate::runtime::WorthUiTransientInteractionState::TextInput,
                    )
                    .expect("ordinary input admits");
            });
        });
        let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
            transaction,
            ..
        } = completion
        else {
            panic!("ordinary input reaches transaction commitment");
        };
        let committed = match transaction {
            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(value) => value,
            outcome => panic!("ordinary input commits: {outcome:?}"),
        };
        let original = committed.transaction();
        let mut lane_changed = original.clone();
        lane_changed.policy = original.policy.with_commit_lane_for_identity_test(
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::ViewportDerived,
        );

        assert_ne!(original.idempotency_key(), lane_changed.idempotency_key());
        assert!(!original.same_idempotency_basis(&lane_changed));
    }
}

#[cfg(any(test, feature = "certification-support"))]
impl UiAllocationReplanTransaction {
    pub(super) fn for_receipt_law_test(
        candidate: &super::UiAllocationCandidate,
        _generation: super::UiAllocationReceiptGeneration,
    ) -> Self {
        let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
        let crate::runtime::stream_policy::UiAllocationStreamCommitDecision::Commit(receipt) =
            crate::runtime::stream_policy::resolve_stream_families(
                &[crate::runtime::UiAllocationStreamFamily::TextInput],
                &mut payload,
            )
        else {
            panic!("single admitted test family resolves to commit policy");
        };
        let identity = candidate.allocation_neighborhood().identity().clone();
        Self {
            frame_ingress_keys: Vec::new().into_boxed_slice(),
            primary_neighborhood: identity.clone(),
            ordered_neighborhoods: vec![identity].into_boxed_slice(),
            widen_reasons: vec![None].into_boxed_slice(),
            expected_generations: Vec::new().into_boxed_slice(),
            frame_epoch: crate::runtime::UiAllocationFrameEpoch::initial(),
            policy: receipt.into_resolution_parts().0,
            overlap_disposition: crate::graph::UiReplanOverlapDisposition::Singleton,
            transaction_generation: 0,
            runtime_generation: 0,
            consequences: Default::default(),
        }
    }
}
