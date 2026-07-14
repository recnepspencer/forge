use super::{UiAdmittedAllocationInvalidationTarget, UiGraphReplanTargetDisposition};

impl super::UiGraphReplanAuthority {
    pub(crate) fn seal_transaction_basis(
        &self,
        plan: &crate::runtime::UiNarrowedAllocationFramePlan,
    ) -> Result<super::UiAdmittedReplanNeighborhoodSet, super::UiReplanLocalityDenial> {
        let mut counters = super::UiReplanNeighborhoodSelectionCounters::default();
        let mut targets = Vec::<&UiAdmittedAllocationInvalidationTarget>::new();
        let mut scroll_consequences = Vec::<super::UiScrollReplanConsequence>::new();
        let mut portal_consequences = Vec::<super::UiPortalReplanConsequence>::new();
        for invalidation in plan.narrowed_invalidations() {
            counters.visit()?;
            let causal_target_sets = target_sets_of(invalidation.target());
            for binding in scroll_bindings_of(invalidation.target()) {
                let consequence = super::UiScrollReplanConsequence::seal(binding)?;
                if let Some(existing) = scroll_consequences.iter().find(|existing| {
                    existing.evidence().contract_identity_digest()
                        == consequence.evidence().contract_identity_digest()
                }) {
                    if existing != &consequence {
                        return Err(super::UiReplanLocalityDenial::ContradictoryScrollConsequence);
                    }
                } else {
                    scroll_consequences.push(consequence);
                }
            }
            if let crate::runtime::UiAllocationInvalidationTarget::PortalAnchor { movement } =
                invalidation.target()
            {
                let consequence = super::UiPortalReplanConsequence::seal(movement);
                if !portal_consequences.contains(&consequence) {
                    portal_consequences.push(consequence);
                }
            }
            for causal_targets in causal_target_sets {
                let primary = causal_targets.primary();
                admit_target(self, &mut targets, primary)?;
                for target in causal_targets.widened() {
                    let key = target
                        .generation_key()
                        .ok_or(super::UiReplanLocalityDenial::MissingAdmittedCandidate)?;
                    if !self.certifies(&key) {
                        return Err(super::UiReplanLocalityDenial::AdmittedGenerationSetChanged);
                    }
                    deny_forbidden_root_fallback(invalidation.family(), primary, target)?;
                    admit_unique_target(&mut targets, target)?;
                }
            }
        }
        if targets.is_empty() {
            return Err(super::UiReplanLocalityDenial::EmptyInvalidationSet);
        }
        let ordered = admit_ordered_neighborhoods(targets, &mut counters)?;
        let (ordered, overlap_disposition) = classify_overlap(ordered, &mut counters)?;
        let root_posture = classify_root_posture(&ordered);
        Ok(super::UiAdmittedReplanNeighborhoodSet::new(
            plan.identity(),
            ordered,
            root_posture,
            overlap_disposition,
            counters,
            super::UiGraphReplanConsequences::seal(scroll_consequences, portal_consequences),
        ))
    }
}

fn scroll_bindings_of(
    target: &crate::runtime::UiAllocationInvalidationTarget,
) -> &[crate::runtime::UiAdmittedScrollInvalidationBinding] {
    match target {
        crate::runtime::UiAllocationInvalidationTarget::ScrollOwnedContentExtent {
            bindings,
            ..
        }
        | crate::runtime::UiAllocationInvalidationTarget::ScrollOwnedExtent { bindings, .. } => {
            bindings
        }
        _ => &[],
    }
}

fn target_sets_of(
    target: &crate::runtime::UiAllocationInvalidationTarget,
) -> Vec<&crate::graph::UiAdmittedAllocationInvalidationTargetSet> {
    let bindings = scroll_bindings_of(target);
    if !bindings.is_empty() {
        return bindings.iter().map(|binding| binding.target()).collect();
    }
    vec![targets_of(target)]
}

fn admit_target<'a>(
    authority: &super::UiGraphReplanAuthority,
    targets: &mut Vec<&'a UiAdmittedAllocationInvalidationTarget>,
    target: &'a UiAdmittedAllocationInvalidationTarget,
) -> Result<(), super::UiReplanLocalityDenial> {
    let key = target
        .generation_key()
        .ok_or(super::UiReplanLocalityDenial::MissingAdmittedCandidate)?;
    if !authority.certifies(&key) {
        return Err(super::UiReplanLocalityDenial::AdmittedGenerationSetChanged);
    }
    admit_unique_target(targets, target)
}

fn classify_overlap(
    mut ordered: Vec<super::UiAdmittedReplanNeighborhood>,
    counters: &mut super::UiReplanNeighborhoodSelectionCounters,
) -> Result<
    (
        Vec<super::UiAdmittedReplanNeighborhood>,
        super::UiReplanOverlapDisposition,
    ),
    super::UiReplanLocalityDenial,
> {
    if ordered.len() == 1 {
        return Ok((ordered, super::UiReplanOverlapDisposition::Singleton));
    }
    let mut merged = false;
    let mut superseded = false;
    let mut left = 0;
    while left < ordered.len() {
        let mut right = left + 1;
        while right < ordered.len() {
            let (relation, probes) = footprint_relation(
                ordered[left].neighborhood_members(),
                ordered[right].neighborhood_members(),
            )?;
            counters.overlap_probe(probes)?;
            match relation {
                FootprintRelation::Disjoint => right += 1,
                FootprintRelation::LeftContained => {
                    if left == 0 {
                        if ordered[left].allocation_candidate()
                            == ordered[right].allocation_candidate()
                        {
                            ordered.remove(right);
                            counters.merged()?;
                            merged = true;
                            continue;
                        }
                        return Err(
                            super::UiReplanLocalityDenial::OverlappingNeighborhoodSupersessionRequired,
                        );
                    }
                    superseded |= admit_containment(&ordered[left], &ordered[right])?;
                    ordered.remove(left);
                    counters.merged()?;
                    merged = true;
                    right = left + 1;
                }
                FootprintRelation::RightContained => {
                    superseded |= admit_containment(&ordered[right], &ordered[left])?;
                    ordered.remove(right);
                    counters.merged()?;
                    merged = true;
                }
                FootprintRelation::PartialOverlap => {
                    let left = u16::try_from(left)
                        .map_err(|_| super::UiReplanLocalityDenial::CounterExhausted)?;
                    let right = u16::try_from(right)
                        .map_err(|_| super::UiReplanLocalityDenial::CounterExhausted)?;
                    return Err(super::UiReplanLocalityDenial::OverlappingNeighborhoods {
                        left,
                        right,
                    });
                }
            }
        }
        left += 1;
    }
    counters.seal(ordered.len())?;
    Ok((
        ordered,
        if superseded {
            super::UiReplanOverlapDisposition::ContainmentSuperseded
        } else if merged {
            super::UiReplanOverlapDisposition::ContainmentMerged
        } else {
            super::UiReplanOverlapDisposition::PairwiseDisjoint
        },
    ))
}

fn admit_containment(
    contained: &super::UiAdmittedReplanNeighborhood,
    containing: &super::UiAdmittedReplanNeighborhood,
) -> Result<bool, super::UiReplanLocalityDenial> {
    if contained.allocation_candidate() == containing.allocation_candidate() {
        return Ok(false);
    }
    let contained_generation = contained.generation_key().measurement_generation().raw();
    let containing_generation = containing.generation_key().measurement_generation().raw();
    if contained_generation < containing_generation {
        Ok(true)
    } else {
        Err(super::UiReplanLocalityDenial::OverlappingNeighborhoodSupersessionRequired)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FootprintRelation {
    Disjoint,
    LeftContained,
    RightContained,
    PartialOverlap,
}

fn footprint_relation<T: Ord>(
    left: &[T],
    right: &[T],
) -> Result<(FootprintRelation, u16), super::UiReplanLocalityDenial> {
    let (mut left_ordinal, mut right_ordinal) = (0, 0);
    let (mut common, mut probes) = (0usize, 0u16);
    while left_ordinal < left.len() && right_ordinal < right.len() {
        probes = probes
            .checked_add(1)
            .ok_or(super::UiReplanLocalityDenial::CounterExhausted)?;
        match left[left_ordinal].cmp(&right[right_ordinal]) {
            std::cmp::Ordering::Less => left_ordinal += 1,
            std::cmp::Ordering::Greater => right_ordinal += 1,
            std::cmp::Ordering::Equal => {
                common = common
                    .checked_add(1)
                    .ok_or(super::UiReplanLocalityDenial::CounterExhausted)?;
                left_ordinal += 1;
                right_ordinal += 1;
            }
        }
    }
    let relation = if common == 0 {
        FootprintRelation::Disjoint
    } else if common == left.len() {
        FootprintRelation::LeftContained
    } else if common == right.len() {
        FootprintRelation::RightContained
    } else {
        FootprintRelation::PartialOverlap
    };
    Ok((relation, probes))
}

fn deny_forbidden_root_fallback(
    family: crate::runtime::UiAllocationInvalidationFamily,
    narrowed_target: &UiAdmittedAllocationInvalidationTarget,
    target: &UiAdmittedAllocationInvalidationTarget,
) -> Result<(), super::UiReplanLocalityDenial> {
    if family == crate::runtime::UiAllocationInvalidationFamily::ViewportExtentChange
        && narrowed_target.disposition() == UiGraphReplanTargetDisposition::LocalPrimaryEligible
        && target.disposition() == UiGraphReplanTargetDisposition::RootPrimaryEligible
    {
        return Err(super::UiReplanLocalityDenial::ForbiddenRootFallback);
    }
    Ok(())
}

fn admit_unique_target<'a>(
    targets: &mut Vec<&'a UiAdmittedAllocationInvalidationTarget>,
    target: &'a UiAdmittedAllocationInvalidationTarget,
) -> Result<(), super::UiReplanLocalityDenial> {
    if let Some(existing) = targets
        .iter()
        .find(|item| item.neighborhood_identity() == target.neighborhood_identity())
    {
        if existing.graph_generation() != target.graph_generation() {
            return Err(super::UiReplanLocalityDenial::ConflictingNeighborhoodForTarget);
        }
    } else {
        targets.push(target);
    }
    Ok(())
}

fn admit_ordered_neighborhoods(
    mut targets: Vec<&UiAdmittedAllocationInvalidationTarget>,
    counters: &mut super::UiReplanNeighborhoodSelectionCounters,
) -> Result<Vec<super::UiAdmittedReplanNeighborhood>, super::UiReplanLocalityDenial> {
    targets.sort_by_key(|target| super::replan_authority::causal_rank(target));
    let mut ordered = Vec::with_capacity(targets.len());
    for (ordinal, target) in targets.into_iter().enumerate() {
        counters.prove()?;
        let admitted = if ordinal == 0 {
            super::UiAdmittedReplanNeighborhood::primary(target)?
        } else {
            if target.disposition() == UiGraphReplanTargetDisposition::RootPrimaryEligible {
                counters.root_widen()?;
            }
            super::UiAdmittedReplanNeighborhood::widened(target, target.graph_consequence())?
        };
        counters.consume_locality(admitted.locality())?;
        ordered.push(admitted);
    }
    counters.seal(ordered.len())?;
    Ok(ordered)
}

fn classify_root_posture(
    ordered: &[super::UiAdmittedReplanNeighborhood],
) -> super::UiReplanRootPosture {
    ordered
        .iter()
        .skip(1)
        .find(|item| item.is_root_target())
        .and_then(super::UiAdmittedReplanNeighborhood::widen_reason)
        .map(|reason| super::UiReplanRootPosture::CountedRootWiden { reason })
        .unwrap_or_else(|| {
            if ordered[0].is_root_target() {
                super::UiReplanRootPosture::RootPrimary
            } else {
                super::UiReplanRootPosture::NotRoot
            }
        })
}

fn targets_of(
    target: &crate::runtime::UiAllocationInvalidationTarget,
) -> &super::UiAdmittedAllocationInvalidationTargetSet {
    match target {
        crate::runtime::UiAllocationInvalidationTarget::Graph(target)
        | crate::runtime::UiAllocationInvalidationTarget::ResizePreview { target, .. }
        | crate::runtime::UiAllocationInvalidationTarget::QueryProjection { target, .. }
        | crate::runtime::UiAllocationInvalidationTarget::HostMeasurement { target, .. }
        | crate::runtime::UiAllocationInvalidationTarget::DurableResize { target, .. } => target,
        crate::runtime::UiAllocationInvalidationTarget::PortalAnchor { movement } => {
            movement.target()
        }
        crate::runtime::UiAllocationInvalidationTarget::ScrollOwnedContentExtent { .. }
        | crate::runtime::UiAllocationInvalidationTarget::ScrollOwnedExtent { .. } => {
            unreachable!("scroll bindings expose their own target sets")
        }
    }
}
