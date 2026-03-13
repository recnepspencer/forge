use super::super::types::{
    ApplyFootprint, DisjointApplyGroup, LoweredTask, ParallelExecutionPolicy,
};

fn conflicts_with(left: &ApplyFootprint, right: &ApplyFootprint) -> bool {
    intersects(
        left.touched_nodes.as_slice(),
        right.touched_nodes.as_slice(),
    ) || intersects(
        left.touched_sources.as_slice(),
        right.touched_sources.as_slice(),
    )
}

fn merge(left: &mut ApplyFootprint, right: &ApplyFootprint) {
    left.partitions =
        merge_sorted_unique(left.partitions.as_slice(), right.partitions.as_slice()).into();
    left.touched_nodes = merge_sorted_unique(
        left.touched_nodes.as_slice(),
        right.touched_nodes.as_slice(),
    )
    .into();
    left.touched_sources = merge_sorted_unique(
        left.touched_sources.as_slice(),
        right.touched_sources.as_slice(),
    )
    .into();
}

fn intersects<T: Ord>(left: &[T], right: &[T]) -> bool {
    let mut left_index = 0usize;
    let mut right_index = 0usize;
    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => left_index += 1,
            std::cmp::Ordering::Greater => right_index += 1,
            std::cmp::Ordering::Equal => return true,
        }
    }
    false
}

fn merge_sorted_unique<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0usize;
    let mut right_index = 0usize;
    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => {
                merged.push(left[left_index].clone());
                left_index += 1;
            }
            std::cmp::Ordering::Greater => {
                merged.push(right[right_index].clone());
                right_index += 1;
            }
            std::cmp::Ordering::Equal => {
                merged.push(left[left_index].clone());
                left_index += 1;
                right_index += 1;
            }
        }
    }
    merged.extend_from_slice(&left[left_index..]);
    merged.extend_from_slice(&right[right_index..]);
    merged
}

pub(super) fn build_stage_apply_groups(
    tasks: &[LoweredTask],
    policy: ParallelExecutionPolicy,
) -> Vec<DisjointApplyGroup> {
    if tasks.is_empty() {
        return Vec::new();
    }

    let max_groups = policy.max_apply_group_count_for(tasks.len());
    let chunk_size = tasks.len().div_ceil(max_groups).max(1);
    let mut groups = Vec::<DisjointApplyGroup>::new();

    for (task_index, task) in tasks.iter().enumerate() {
        let mut placed = false;
        for group in &mut groups {
            if group.task_indices.len() >= chunk_size {
                continue;
            }
            if conflicts_with(&group.footprint, &task.footprint) {
                continue;
            }
            merge(&mut group.footprint, &task.footprint);
            group.task_indices.push(task_index);
            placed = true;
            break;
        }
        if !placed {
            groups.push(DisjointApplyGroup {
                task_indices: vec![task_index],
                footprint: task.footprint.clone(),
            });
        }
    }

    groups
}
