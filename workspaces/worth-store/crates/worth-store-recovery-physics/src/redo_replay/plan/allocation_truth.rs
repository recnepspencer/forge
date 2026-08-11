use super::*;

impl ImmutablePhysicalRedoPlan {
    pub fn admit_inline_allocation_truth(
        self,
        selected: &[CurrentPhysicalRecordPlacement],
        new_segment_truth: Option<(u64, u32)>,
    ) -> Result<Self, PhysicalRedoPlanningDenial> {
        validate_inline_allocation_truth(&self, selected, new_segment_truth)?;
        Ok(self)
    }
}

fn validate_inline_allocation_truth(
    plan: &ImmutablePhysicalRedoPlan,
    selected: &[CurrentPhysicalRecordPlacement],
    new_segment_truth: Option<(u64, u32)>,
) -> Result<(), PhysicalRedoPlanningDenial> {
    let mut segments = selected_segments(selected)?;
    let pending_groups = plan
        .projections
        .iter()
        .filter(|projection| operation_requires_apply(plan, projection.operation))
        .map(|projection| projection.group.group_identity())
        .collect::<BTreeSet<_>>();
    for group in pending_groups {
        let operations = plan
            .projections
            .iter()
            .filter(|projection| projection.group.group_identity() == group)
            .map(|projection| projection.operation)
            .collect::<BTreeSet<_>>();
        let touched = extend_target_pages(plan, &operations, &mut segments)?;
        validate_group_allocations(plan, group, &segments, &touched, new_segment_truth)?;
    }
    Ok(())
}

fn selected_segments(
    selected: &[CurrentPhysicalRecordPlacement],
) -> Result<BTreeMap<u64, (u32, BTreeSet<u64>)>, PhysicalRedoPlanningDenial> {
    let mut segments = BTreeMap::new();
    for placement in selected {
        let CurrentPhysicalRecordPlacement::Inline(value) = placement else {
            continue;
        };
        let entry = segments
            .entry(value.segment().get())
            .or_insert_with(|| (value.segment_page_capacity(), BTreeSet::new()));
        if entry.0 != value.segment_page_capacity() {
            return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
        }
        entry.1.insert(value.page().get());
    }
    Ok(segments)
}

fn operation_requires_apply(plan: &ImmutablePhysicalRedoPlan, operation: [u8; 32]) -> bool {
    plan.decisions.iter().any(|decision| {
        decision.operation == operation && decision.kind == PhysicalRedoDecisionKind::Apply
    })
}

fn extend_target_pages(
    plan: &ImmutablePhysicalRedoPlan,
    operations: &BTreeSet<[u8; 32]>,
    segments: &mut BTreeMap<u64, (u32, BTreeSet<u64>)>,
) -> Result<BTreeSet<u64>, PhysicalRedoPlanningDenial> {
    let mut touched = BTreeSet::new();
    for decision in &plan.decisions {
        if !operations.contains(&decision.operation) {
            continue;
        }
        let target = plan
            .records
            .get(decision.record_index as usize)
            .and_then(|record| record.targets().get(decision.target_index as usize))
            .ok_or(PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
        let PhysicalRedoTargetIdentity::InlinePage { segment, page, .. } = target.identity() else {
            continue;
        };
        touched.insert(segment);
        segments
            .entry(segment)
            .or_insert_with(|| (0, BTreeSet::new()))
            .1
            .insert(page);
    }
    Ok(touched)
}

fn validate_group_allocations(
    plan: &ImmutablePhysicalRedoPlan,
    group: [u8; 32],
    segments: &BTreeMap<u64, (u32, BTreeSet<u64>)>,
    touched: &BTreeSet<u64>,
    new_segment_truth: Option<(u64, u32)>,
) -> Result<(), PhysicalRedoPlanningDenial> {
    let allocations = plan
        .projections
        .iter()
        .filter(|projection| projection.group.group_identity() == group)
        .flat_map(|projection| projection.materialization.root_state().inline_allocations());
    let mut admitted = BTreeSet::new();
    for allocation in allocations {
        let segment = allocation.segment().segment_id().get();
        if !touched.contains(&segment) || !admitted.insert(segment) {
            return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
        }
        let (selected_capacity, pages) = segments
            .get(&segment)
            .ok_or(PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
        let capacity = if *selected_capacity != 0 {
            *selected_capacity
        } else {
            let (next_segment, capacity) =
                new_segment_truth.ok_or(PhysicalRedoPlanningDenial::InvalidRecoveryProjection)?;
            if segment < next_segment {
                return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
            }
            capacity
        };
        if allocation.page_capacity() != capacity
            || allocation.used_pages() as usize != pages.len()
            || allocation.used_pages() > allocation.page_capacity()
        {
            return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
        }
    }
    if admitted != *touched {
        return Err(PhysicalRedoPlanningDenial::InvalidRecoveryProjection);
    }
    Ok(())
}
