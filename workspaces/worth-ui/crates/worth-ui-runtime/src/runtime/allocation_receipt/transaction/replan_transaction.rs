use crate::evidence::UiAllocationNeighborhoodIdentity;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReplanTransaction {
    frame_ingress_keys: Rc<[crate::runtime::UiAllocationFrameIngressKey]>,
    stream_families: Rc<[crate::runtime::UiAllocationStreamFamily]>,
    invalidation_families: Rc<[crate::runtime::UiAllocationInvalidationFamily]>,
    ingress_policy_verdicts: Rc<[crate::runtime::UiAllocationIngressPolicyVerdict]>,
    primary_neighborhood: UiAllocationNeighborhoodIdentity,
    ordered_neighborhoods: Rc<[UiAllocationNeighborhoodIdentity]>,
    widen_reasons: Rc<[Option<crate::graph::UiReplanWidenReason>]>,
    expected_generations: Rc<[crate::graph::UiReplanGenerationKey]>,
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    policy: crate::runtime::UiResolvedAllocationStreamPolicy,
    overlap_disposition: crate::graph::UiReplanOverlapDisposition,
    root_posture: crate::graph::UiReplanRootPosture,
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
    pub(crate) fn for_catalog_removal_activation(
        removed: Box<[UiAllocationNeighborhoodIdentity]>,
        overlap_disposition: crate::graph::UiReplanOverlapDisposition,
        runtime_generation: u64,
        transaction_generation: u64,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    ) -> Result<Self, UiAllocationReplanTransactionDenial> {
        let Some(primary) = removed.first().cloned() else {
            return Err(UiAllocationReplanTransactionDenial::EmptyNeighborhoodSet);
        };
        if contains_duplicate_neighborhood(&removed) {
            return Err(UiAllocationReplanTransactionDenial::DuplicateNeighborhood);
        }
        let cardinality = u16::try_from(removed.len())
            .map_err(|_| UiAllocationReplanTransactionDenial::CardinalityMismatch)?;
        Ok(Self {
            frame_ingress_keys: Rc::from([]),
            stream_families: vec![
                crate::runtime::UiAllocationStreamFamily::HostMeasurementReplacement,
            ]
            .into(),
            invalidation_families: vec![
                crate::runtime::UiAllocationInvalidationFamily::HostMeasurementResultReplacement,
            ]
            .into(),
            ingress_policy_verdicts: Rc::from([]),
            primary_neighborhood: primary,
            widen_reasons: vec![None; removed.len()].into(),
            ordered_neighborhoods: removed.into(),
            expected_generations: Rc::from([]),
            frame_epoch,
            policy: crate::runtime::stream_policy::replacement_activation_policy(cardinality),
            overlap_disposition,
            root_posture: crate::graph::UiReplanRootPosture::NotRoot,
            transaction_generation,
            runtime_generation,
            consequences: Default::default(),
        })
    }

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
        if contains_duplicate_neighborhood(&ordered) {
            return Err(UiAllocationReplanTransactionDenial::DuplicateNeighborhood);
        }
        let cardinality = u16::try_from(ordered.len())
            .map_err(|_| UiAllocationReplanTransactionDenial::CardinalityMismatch)?;
        Ok(Self {
            frame_ingress_keys: Rc::from([]),
            stream_families: vec![
                crate::runtime::UiAllocationStreamFamily::HostMeasurementReplacement,
            ]
            .into(),
            invalidation_families: vec![
                crate::runtime::UiAllocationInvalidationFamily::HostMeasurementResultReplacement,
            ]
            .into(),
            ingress_policy_verdicts: Rc::from([]),
            primary_neighborhood: primary.allocation_neighborhood().identity().clone(),
            widen_reasons: vec![None; ordered.len()].into(),
            expected_generations: Rc::from([]),
            frame_epoch,
            policy: crate::runtime::stream_policy::replacement_activation_policy(cardinality),
            overlap_disposition: if ordered.len() == 1 {
                crate::graph::UiReplanOverlapDisposition::Singleton
            } else {
                crate::graph::UiReplanOverlapDisposition::PairwiseDisjoint
            },
            root_posture: crate::graph::UiReplanRootPosture::NotRoot,
            ordered_neighborhoods: ordered.into(),
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
            stream_families: basis.stream_families().into(),
            invalidation_families: basis.invalidation_families().collect::<Vec<_>>().into(),
            ingress_policy_verdicts: basis.ingress_policy_verdicts().into(),
            primary_neighborhood: primary,
            ordered_neighborhoods: ordered.into(),
            widen_reasons: basis.widen_reasons().collect::<Vec<_>>().into(),
            expected_generations: basis.expected_generations().collect::<Vec<_>>().into(),
            frame_epoch: basis.frame_epoch(),
            policy: basis.policy(),
            overlap_disposition: basis.overlap_disposition(),
            root_posture: basis.root_posture(),
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
    pub fn stream_families(&self) -> &[crate::runtime::UiAllocationStreamFamily] {
        &self.stream_families
    }
    pub fn invalidation_families(&self) -> &[crate::runtime::UiAllocationInvalidationFamily] {
        &self.invalidation_families
    }
    pub fn ingress_policy_verdicts(&self) -> &[crate::runtime::UiAllocationIngressPolicyVerdict] {
        &self.ingress_policy_verdicts
    }
    pub fn invalidation_count(&self) -> u16 {
        u16::try_from(self.invalidation_families.len()).unwrap_or(u16::MAX)
    }
    pub fn ingress_count(&self) -> u16 {
        u16::try_from(self.frame_ingress_keys.len()).unwrap_or(u16::MAX)
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
    pub fn root_posture(&self) -> crate::graph::UiReplanRootPosture {
        self.root_posture
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
            && self.stream_families == other.stream_families
            && self.invalidation_families == other.invalidation_families
            && self.ingress_policy_verdicts == other.ingress_policy_verdicts
            && self.ordered_neighborhoods == other.ordered_neighborhoods
            && self.widen_reasons == other.widen_reasons
            && self.expected_generations == other.expected_generations
            && self.frame_epoch == other.frame_epoch
            && self.policy == other.policy
            && self.overlap_disposition == other.overlap_disposition
            && self.root_posture == other.root_posture
            && self.runtime_generation == other.runtime_generation
            && self.consequences == other.consequences
    }

    pub(crate) fn idempotency_key(&self) -> u64 {
        let mut digest = 0xcbf29ce484222325u64;
        for key in self.frame_ingress_keys.iter() {
            digest ^= key.ingress_identity().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
            digest ^= key.source_generation().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
            digest ^= key.source_order().as_u64();
            digest = digest.wrapping_mul(0x100000001b3);
        }
        for family in self.stream_families.iter() {
            digest ^= u64::from(family.canonical_order()) + 1;
            digest = digest.wrapping_mul(0x100000001b3);
        }
        for family in self.invalidation_families.iter() {
            digest ^= *family as u64 + 0x40;
            digest = digest.wrapping_mul(0x100000001b3);
        }
        for verdict in self.ingress_policy_verdicts.iter() {
            let word = match verdict {
                crate::runtime::UiAllocationIngressPolicyVerdict::Current => 0x80,
                crate::runtime::UiAllocationIngressPolicyVerdict::PartialQueryStaleButBounded {
                    warnings,
                    max_lag_frames,
                } => 0x81 ^ ((*warnings as u64) << 8) ^ (u64::from(*max_lag_frames) << 16),
            };
            digest ^= word;
            digest = digest.wrapping_mul(0x100000001b3);
        }
        for identity in self.ordered_neighborhoods.iter() {
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
        digest ^= match self.root_posture {
            crate::graph::UiReplanRootPosture::NotRoot => 0,
            crate::graph::UiReplanRootPosture::RootPrimary => 1,
            crate::graph::UiReplanRootPosture::CountedRootWiden { reason } => 2 + reason as u64,
        };
        digest = digest.wrapping_mul(0x100000001b3);
        for generation in self.expected_generations.iter() {
            digest ^= generation.identity_digest();
            digest = digest.wrapping_mul(0x100000001b3);
        }
        digest ^= self.consequences.identity_digest();
        digest = digest.wrapping_mul(0x100000001b3);
        digest
    }
}

fn contains_duplicate_neighborhood(identities: &[UiAllocationNeighborhoodIdentity]) -> bool {
    let mut identities_by_digest = std::collections::BTreeMap::<u64, Vec<&_>>::new();
    for identity in identities {
        let bucket = identities_by_digest
            .entry(identity.identity_digest())
            .or_default();
        if bucket.contains(&identity) {
            return true;
        }
        bucket.push(identity);
    }
    false
}

#[cfg(test)]
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
            frame_ingress_keys: Rc::from([]),
            stream_families: vec![crate::runtime::UiAllocationStreamFamily::TextInput].into(),
            invalidation_families: vec![
                crate::runtime::UiAllocationInvalidationFamily::TextContentChange,
            ]
            .into(),
            ingress_policy_verdicts: Rc::from([]),
            primary_neighborhood: identity.clone(),
            ordered_neighborhoods: vec![identity].into(),
            widen_reasons: vec![None].into(),
            expected_generations: Rc::from([]),
            frame_epoch: crate::runtime::UiAllocationFrameEpoch::initial(),
            policy: receipt.into_resolution_parts().0,
            overlap_disposition: crate::graph::UiReplanOverlapDisposition::Singleton,
            root_posture: crate::graph::UiReplanRootPosture::NotRoot,
            transaction_generation: 0,
            runtime_generation: 0,
            consequences: Default::default(),
        }
    }

    pub(super) fn with_partial_query_policy_for_test(mut self, maximum_lag_frames: u8) -> Self {
        self.ingress_policy_verdicts = vec![
            crate::runtime::UiAllocationIngressPolicyVerdict::PartialQueryStaleButBounded {
                warnings:
                    crate::runtime::UiAllocationFrameQueryWarningPosture::QueryContextRowBound,
                max_lag_frames: maximum_lag_frames,
            },
        ]
        .into();
        self
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
        assert!(std::rc::Rc::ptr_eq(
            &original.ordered_neighborhoods,
            &lane_changed.ordered_neighborhoods,
        ));
        assert!(std::rc::Rc::ptr_eq(
            &original.widen_reasons,
            &lane_changed.widen_reasons,
        ));
        lane_changed.policy = original.policy.with_commit_lane_for_identity_test(
            crate::runtime::stream_policy::UiAllocationResolvedCommitLane::ViewportDerived,
        );

        assert_ne!(original.idempotency_key(), lane_changed.idempotency_key());
        assert!(!original.same_idempotency_basis(&lane_changed));
    }
}
