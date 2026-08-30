use super::*;

#[test]
fn consecutive_live_ticks_damage_the_previous_and_current_presentations() {
    let world = World::new();
    let mut sampler = UiMountedMotionSampler::default();
    sampler.install(world.receipt(80, 0.0, None)).unwrap();
    let first = commit_tick(&mut sampler, 1, world.presentation);
    let first_geometry = first.samples()[0].geometry().unwrap().components();
    let second = commit_tick(&mut sampler, 20, world.presentation);
    let second_geometry = second.samples()[0].geometry().unwrap().components();
    let clip =
        UiPresentationSampledClipGeometry::from_presented_components([0.0, 0.0, 100.0, 100.0])
            .unwrap();
    let regions = second.samples()[0].damage().clipped_to(clip);

    assert_eq!(regions[0].unwrap().components(), first_geometry);
    assert_eq!(regions[1].unwrap().components(), second_geometry);
    assert_ne!(first_geometry, second_geometry);
}

#[test]
fn live_tick_damage_clips_both_presented_regions() {
    let world = World::new();
    let mut sampler = UiMountedMotionSampler::default();
    sampler.install(world.receipt(81, 0.0, None)).unwrap();
    commit_tick(&mut sampler, 1, world.presentation);
    let second = commit_tick(&mut sampler, 20, world.presentation);
    let clip = UiPresentationSampledClipGeometry::from_presented_components([5.0, 0.0, 10.0, 20.0])
        .unwrap();
    let regions = second.samples()[0].damage().clipped_to(clip);

    assert!(regions.iter().flatten().all(|region| {
        let [x, _, width, _] = region.components();
        x >= 5.0 && x + width <= 15.0
    }));
}

#[test]
fn first_tick_damage_clears_the_published_successor_before_sampling_the_entrance_start() {
    let world = World::new();
    let mut sampler = UiMountedMotionSampler::default();
    sampler.install(world.receipt(82, 0.0, None)).unwrap();

    let first = commit_tick(&mut sampler, 1, world.presentation);
    let clip =
        UiPresentationSampledClipGeometry::from_presented_components([0.0, 0.0, 100.0, 100.0])
            .unwrap();
    let regions = first.samples()[0].damage().clipped_to(clip);

    assert_eq!(regions[0].unwrap().components(), [20.0, 10.0, 24.0, 12.0]);
    assert_eq!(regions[1].unwrap().components(), [20.0, 0.0, 24.0, 12.0]);
}
