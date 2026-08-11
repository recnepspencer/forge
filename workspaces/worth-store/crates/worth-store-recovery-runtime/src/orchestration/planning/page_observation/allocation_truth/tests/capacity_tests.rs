use super::*;

#[test]
fn new_segments_fill_the_persisted_capacity_before_spilling() {
    let exact = selected_world("allocation-new-segment-capacity-exact", 2);
    let first_page = next_page(&exact.placements);
    assert_admitted(
        exact,
        vec![
            target(1, first_page, 1, 1, 2, 20),
            target(2, first_page + 1, 1, 2, 1, 21),
            target(2, first_page + 2, 1, 2, 1, 22),
            target(3, first_page + 3, 1, 3, 1, 23),
        ],
    );

    let crossing = selected_world("allocation-new-segment-capacity-crossing", 2);
    let first_page = next_page(&crossing.placements);
    assert_rejected(
        crossing,
        vec![
            target(1, first_page, 1, 1, 2, 24),
            target(2, first_page + 1, 1, 2, 1, 25),
            target(2, first_page + 2, 1, 2, 1, 26),
            target(2, first_page + 3, 1, 2, 1, 27),
        ],
    );
}
