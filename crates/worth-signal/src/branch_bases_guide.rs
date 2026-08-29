#[doc = include_str!("../BRANCH_BASES.md")]
pub struct BranchBasesGuide;

// The include stays on line 1. rustdoc reports a guide doctest failure at this
// file's path, offset by the lines that precede the `#[doc]` attribute, so
// holding that offset at zero makes the reported line the `BRANCH_BASES.md`
// line a maintainer must actually open. Anything added above the attribute
// silently shifts every reported guide location. That is why this note sits
// below the item it explains rather than above it.
