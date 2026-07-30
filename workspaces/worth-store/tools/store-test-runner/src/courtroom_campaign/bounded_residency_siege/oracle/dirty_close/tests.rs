use super::verify_dirty;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    parse_dirty, BoundedResidencyDirtyObservation,
};

const VALID: &str = "\
BOUNDED_RESIDENCY_DIRTY 501 502 1 1 3 3 1 601 602 1 1 1 2 1 0 0 1 1 1 0 1 1 1 true true true \
1 true 3 2 1 2 3 2 0 0 0 7 2 2";

#[test]
fn dirty_oracle_accepts_ordinary_append_pressure_and_retry() {
    assert!(verify_dirty(observation(VALID)).is_ok());
}

#[test]
fn dirty_oracle_rejects_each_publication_bypass() {
    assert_denied_at(
        [
            replace(1, "0"),
            replace(2, "0"),
            replace(2, "501"),
            replace(3, "0"),
            replace(4, "0"),
            replace(5, "1"),
            replace(6, "1"),
            replace(7, "0"),
            replace(8, "0"),
            replace(9, "600"),
            replace(10, "2"),
            replace(11, "2"),
        ],
        "Courtroom C ordinary append publications did not reconcile",
    );
}

#[test]
fn dirty_oracle_rejects_each_dirty_frame_bypass() {
    assert_denied_at(
        [
            replace(12, "0"),
            replace(13, "1"),
            replace(14, "0"),
            replace(15, "1"),
            replace(16, "1"),
            replace(17, "0"),
        ],
        "Courtroom C dirty-frame saturation or cleanup did not reconcile",
    );
}

#[test]
fn dirty_oracle_rejects_each_writebehind_bypass() {
    assert_denied_at(
        [
            replace(18, "0"),
            replace(19, "2"),
            replace(20, "1"),
            replace(21, "2"),
            replace(22, "0"),
            replace(23, "2"),
            replace(24, "false"),
            replace(25, "false"),
            replace(26, "false"),
            replace(27, "0"),
            replace(28, "false"),
        ],
        "Courtroom C write-behind saturation did not reconcile",
    );
}

#[test]
fn dirty_oracle_rejects_each_terminal_settlement_bypass() {
    assert_denied_at(
        [
            replace(29, "2"),
            replace(30, "1"),
            replace(31, "0"),
            replace(32, "1"),
            replace(33, "2"),
            replace(34, "1"),
            replace(35, "1"),
            replace(36, "1"),
            replace(37, "1"),
            replace(38, "6"),
            replace(39, "1"),
            replace(40, "1"),
        ],
        "Courtroom C exact writeback settlement did not reconcile",
    );
}

fn assert_denied_at<const N: usize>(markers: [String; N], expected: &str) {
    for marker in markers {
        let hostile = observation(&marker);
        assert_eq!(verify_dirty(hostile).unwrap_err(), expected, "{marker}");
    }
}

fn observation(marker: &str) -> BoundedResidencyDirtyObservation {
    parse_dirty(&[marker.to_owned()]).expect("test marker is structurally valid")
}

fn replace(index: usize, replacement: &str) -> String {
    let mut fields = VALID.split_whitespace().collect::<Vec<_>>();
    fields[index] = replacement;
    fields.join(" ")
}
