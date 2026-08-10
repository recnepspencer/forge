mod oracle;
mod world;

use oracle::{adjudicate, expectation, ordered_pixel, OracleDenial};
use world::{produce_maximum_overlap, MountedPresentationWorld};

#[test]
#[ignore = "closure courtroom: compiles and mounts the full 2,048-row public world"]
fn maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work() {
    let recorder = worth_ui_host_headless::WorthUiHeadlessRecorder::with_viewport_extent(
        worth_ui_host_headless::UiHeadlessRecorderCapacity::new(1, 2, 8_192),
        worth_ui::facade::measurement_exchange::UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    let production = produce_maximum_overlap(recorder);
    let world = MountedPresentationWorld::maximum_overlap(
        &production.initial,
        production.authored_instances,
        production.semantic_surface,
    );
    assert_eq!(world.identity(), "mounted-presentation-world");
    assert_eq!(world.version(), 1);
    assert_eq!(world.baseline().len(), 2_048);
    assert_eq!(
        ordered_pixel(world.baseline(), [80, 48]),
        world.baseline().last().unwrap().rgba
    );
    assert_eq!(production.deltas.len(), 3);
    world.assert_unchanged(&production.unchanged);
    for delta in &production.deltas {
        world.assert_removal_delta(delta);
    }
    assert_eq!(production.restorations.len(), 2);
    for restoration in &production.restorations {
        world.assert_restoration(restoration);
    }
    assert_required_oracle_mutations_are_rejected();
    let unchanged_carrier = production.unchanged.cost.delta_rows_carried()
        + production.unchanged.cost.draw_list_mutations()
        + production.unchanged.cost.order_mutations()
        + production.unchanged.cost.logical_damage_regions()
        + production.unchanged.native_work_count as u64;
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P1-HEADLESS-COST-01\":{unchanged_carrier},\"P1-WORLDS-01\":{}}}",
        world.baseline().len()
    );
    println!("WORTH_UI_LEDGER_WORLD=1");
    println!("WORTH_UI_LEDGER_PRESENTATIONS=7");
    let _ = production.session.shutdown();
}

#[test]
fn independent_oracle_rejects_each_required_control_mutation_for_its_exact_cause() {
    assert_required_oracle_mutations_are_rejected();
}

fn assert_required_oracle_mutations_are_rejected() {
    let baseline = [oracle::OracleRect {
        identity: 1,
        bounds: [4, 4, 8, 8],
        rgba: [1, 2, 3, 255],
        order: 1,
    }];
    let delta = oracle::OracleDelta {
        changes: vec![oracle::OracleRectChange {
            identity: 1,
            previous: baseline[0],
            successor: oracle::OracleRect {
                rgba: [9, 8, 7, 255],
                ..baseline[0]
            },
        }],
    };
    let expected = expectation(&baseline, &delta);
    let mut mutant = expected.clone();
    mutant.owner_delta_count = 0;
    assert_eq!(
        adjudicate(&expected, &mutant),
        Err(OracleDenial::OwnerDeltaDropped)
    );
    mutant = expected.clone();
    mutant.damage[0] = [0, 0, 160, 96];
    assert_eq!(
        adjudicate(&expected, &mutant),
        Err(OracleDenial::DamageWidened)
    );
    mutant = expected.clone();
    mutant.ordered_identities.clear();
    assert_eq!(
        adjudicate(&expected, &mutant),
        Err(OracleDenial::PaintOrderChanged)
    );
    mutant = expected.clone();
    mutant.vacated_damage_count = 0;
    assert_eq!(
        adjudicate(&expected, &mutant),
        Err(OracleDenial::VacatedDamageOmitted)
    );
}
