use super::core::{CanonicalRow, RejectionRow};

pub fn assert_all_equal<T: Eq + std::fmt::Debug>(row: &CanonicalRow<T>) {
    let first = row
        .lanes()
        .first()
        .expect("canonical row should have a lane")
        .payload();
    for lane in &row.lanes()[1..] {
        assert_eq!(
            first,
            lane.payload(),
            "lane {} diverged in row {}",
            lane.lane(),
            row.name()
        );
    }
}

pub fn assert_any_not_equal<T: Eq + std::fmt::Debug>(row: &CanonicalRow<T>) {
    let first = row
        .lanes()
        .first()
        .expect("canonical row should have a lane")
        .payload();
    assert!(
        row.lanes()
            .iter()
            .skip(1)
            .any(|lane| lane.payload() != first),
        "row {} unexpectedly matched across all lanes",
        row.name()
    );
}

pub fn assert_rejection_payloads_present<T>(row: &RejectionRow<T>) {
    assert!(
        !row.lanes().is_empty(),
        "rejection row {} should contain at least one lane result",
        row.name()
    );
}
