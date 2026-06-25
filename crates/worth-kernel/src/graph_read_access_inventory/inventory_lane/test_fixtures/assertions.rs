use super::super::seed::WorthGraphReadAccessInventorySeedParts;
use super::super::{
    WorthGraphReadAccessCappedResidueBuilder, WorthGraphReadAccessInventoryErrorKind,
    WorthGraphReadAccessInventoryRowBuilder, WorthGraphReadAccessInventorySeed,
};

pub(crate) fn assert_row_error(
    builder: WorthGraphReadAccessInventoryRowBuilder,
    expected: WorthGraphReadAccessInventoryErrorKind,
) {
    let error = builder
        .build()
        .expect_err("row builder should reject invalid proof shape");
    assert_eq!(error.kind(), expected);
}

pub(crate) fn assert_residue_error(
    builder: WorthGraphReadAccessCappedResidueBuilder,
    expected: WorthGraphReadAccessInventoryErrorKind,
) {
    let error = builder
        .build()
        .expect_err("residue builder should reject invalid proof shape");
    assert_eq!(error.kind(), expected);
}

pub(crate) fn assert_seed_error(
    parts: WorthGraphReadAccessInventorySeedParts,
    expected: WorthGraphReadAccessInventoryErrorKind,
) {
    let error = WorthGraphReadAccessInventorySeed::from_parts_for_tests(parts)
        .expect_err("seed validation should reject invalid Milestone 5 facts");
    assert_eq!(error.kind(), expected);
}

pub(crate) fn assert_no_empty_digest(digests: &[String]) {
    assert!(!digests.is_empty());
    assert!(digests.iter().all(|digest| !digest.is_empty()));
}
