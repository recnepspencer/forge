use super::{UiNativeDamageIndex, UiNativeDamageIndexDenial};
use worth_ui_host_contract::{
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
};

fn bounds(x: f32, y: f32, width: f32, height: f32) -> UiMountedCanonicalBox {
    bounds_in(UiMountedCoordinateSpace::HostSurface, [x, y, width, height])
}

fn bounds_in(
    coordinate_space: UiMountedCoordinateSpace,
    [x, y, width, height]: [f32; 4],
) -> UiMountedCanonicalBox {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width,
        height,
        coordinate_space,
    })
    .unwrap()
}

#[test]
fn sparse_and_same_center_adversaries_use_exact_two_dimensional_pruning() {
    for vertical in [false, true] {
        let mut index = UiNativeDamageIndex::new();
        for identity in 0..4_096_u64 {
            let offset = identity as f32 * 32.0;
            let (x, y) = if vertical {
                (0.0, offset)
            } else {
                (offset, 0.0)
            };
            index.insert(identity, bounds(x, y, 8.0, 8.0)).unwrap();
        }
        let offset = 2_048.0 * 32.0;
        let (x, y) = if vertical {
            (0.0, offset)
        } else {
            (offset, 0.0)
        };
        let query = index.intersecting(bounds(x, y, 4.0, 4.0)).unwrap();
        assert_eq!(sorted(query.identities), vec![2_048]);
        assert_eq!(query.branch_aabb_probes, 23);
        assert_eq!(query.leaf_command_bounds_probes, 2);
        assert_eq!(query.hierarchy_height, 12);
        assert_eq!(query.stored_records, 4_096);
        assert_eq!(query.high_water_records, 4_096);
    }
    assert_same_center_zero_result();
}

#[test]
fn disjoint_regions_probe_only_their_local_command_not_a_global_union() {
    let mut index = UiNativeDamageIndex::new();
    for identity in 0..1_024_u64 {
        index
            .insert(identity, bounds(identity as f32 * 32.0, 0.0, 8.0, 8.0))
            .unwrap();
    }
    let totals = (0..1_024_u64)
        .map(|identity| {
            index
                .intersecting(bounds(identity as f32 * 32.0, 0.0, 4.0, 4.0))
                .unwrap()
        })
        .fold((0, 0, 0), |(branches, leaves, selected), query| {
            (
                branches + query.branch_aabb_probes,
                leaves + query.leaf_command_bounds_probes,
                selected + query.identities.len(),
            )
        });
    assert!(totals.0 <= 32 * 1_024, "branch probes={}", totals.0);
    assert!(totals.1 <= 2 * 1_024, "leaf probes={}", totals.1);
    assert_eq!(totals.2, 1_024);
}

fn assert_same_center_zero_result() {
    let mut index = UiNativeDamageIndex::new();
    for identity in 0..4_096_u64 {
        let command = if identity % 2 == 0 {
            bounds(-2_048.0, -1.0, 4_096.0, 2.0)
        } else {
            bounds(-1.0, -2_048.0, 2.0, 4_096.0)
        };
        index.insert(identity, command).unwrap();
    }
    let query = index
        .intersecting(bounds(1_900.0, 1_900.0, 4.0, 4.0))
        .unwrap();
    assert!(query.identities.is_empty());
    assert_eq!(query.leaf_command_bounds_probes, 0);
    assert_eq!(query.branch_aabb_probes, 3);
    assert_eq!(query.hierarchy_height, 16);
}

#[test]
fn maximum_overlap_stores_and_probes_each_command_once() {
    let mut index = UiNativeDamageIndex::new();
    let full = bounds(0.0, 0.0, 16_384.0, 16_384.0);
    for identity in 0..2_048_u64 {
        index.insert(identity, full).unwrap();
    }
    let query = index.intersecting(full).unwrap();
    assert_eq!(query.identities.len(), 2_048);
    assert_eq!(query.branch_aabb_probes, 2_047);
    assert_eq!(query.leaf_command_bounds_probes, 2_048);
    assert_eq!(query.stored_records, 2_048);
    assert_eq!(query.high_water_records, 2_048);
    assert!(query.hierarchy_height <= 16);
}

#[test]
fn coordinate_spaces_own_disjoint_hierarchy_roots() {
    let mut index = UiNativeDamageIndex::new();
    index
        .insert(
            1_u64,
            bounds_in(UiMountedCoordinateSpace::HostSurface, [0.0, 0.0, 8.0, 8.0]),
        )
        .unwrap();
    index
        .insert(
            2,
            bounds_in(UiMountedCoordinateSpace::Viewport, [0.0, 0.0, 8.0, 8.0]),
        )
        .unwrap();
    let query = index
        .intersecting(bounds_in(
            UiMountedCoordinateSpace::HostSurface,
            [0.0, 0.0, 8.0, 8.0],
        ))
        .unwrap();
    assert_eq!(sorted(query.identities), vec![1]);
    assert_eq!(query.branch_aabb_probes, 0);
    assert_eq!(query.leaf_command_bounds_probes, 1);
}

#[test]
fn replacement_and_removal_preserve_membership_space_and_high_water() {
    let mut index = UiNativeDamageIndex::new();
    for identity in 0..1_024_u64 {
        index
            .insert(identity, bounds(identity as f32 * 32.0, 0.0, 10.0, 10.0))
            .unwrap();
    }
    let moved = bounds_in(
        UiMountedCoordinateSpace::Viewport,
        [128.0, 128.0, 10.0, 10.0],
    );
    index.replace(512, moved).unwrap();
    index.remove(511).unwrap();
    assert!(index
        .intersecting(bounds(512.0 * 32.0, 0.0, 10.0, 10.0))
        .unwrap()
        .identities
        .is_empty());
    assert_eq!(
        sorted(index.intersecting(moved).unwrap().identities),
        vec![512]
    );
    let neighbor = index
        .intersecting(bounds(513.0 * 32.0, 0.0, 10.0, 10.0))
        .unwrap();
    assert_eq!(sorted(neighbor.identities), vec![513]);
    assert!(neighbor.hierarchy_height <= 16);
    index.remove(512).unwrap();
    let query = index.intersecting(moved).unwrap();
    assert!(query.identities.is_empty());
    assert_eq!(query.stored_records, 1_022);
    assert_eq!(query.high_water_records, 1_024);
}

#[test]
fn leaf_capacity_denies_before_mutating_live_or_high_water_state() {
    let mut index = UiNativeDamageIndex::new();
    for identity in 0..4_096_u64 {
        index
            .insert(identity, bounds(identity as f32, 0.0, 0.5, 0.5))
            .unwrap();
    }
    assert_eq!(
        index.insert(4_096, bounds(8_192.0, 0.0, 0.5, 0.5)),
        Err(UiNativeDamageIndexDenial::CapacityExceeded)
    );
    let query = index.intersecting(bounds(8_192.0, 0.0, 0.5, 0.5)).unwrap();
    assert!(query.identities.is_empty());
    assert_eq!(query.stored_records, 4_096);
    assert_eq!(query.high_water_records, 4_096);

    for identity in (0..4_096_u64).step_by(2) {
        index.remove(identity).unwrap();
    }
    for identity in 4_096..6_144_u64 {
        index
            .insert(identity, bounds(identity as f32, 32.0, 0.5, 0.5))
            .unwrap();
    }
    let reused = index.intersecting(bounds(6_143.0, 32.0, 0.5, 0.5)).unwrap();
    assert_eq!(sorted(reused.identities), vec![6_143]);
    assert_eq!(reused.stored_records, 4_096);
    assert_eq!(reused.high_water_records, 4_096);
    assert!(reused.hierarchy_height <= 16);
}

fn sorted(identities: std::collections::HashSet<u64>) -> Vec<u64> {
    let mut identities = identities.into_iter().collect::<Vec<_>>();
    identities.sort_unstable();
    identities
}
