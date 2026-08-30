#[doc = include_str!("../BRANCH_BASES.md")]
pub struct BranchBasesGuide;

// The include stays on line 1. rustdoc names each guide doctest by this file's
// path and the line the fence opens on, offset by the lines that precede the
// `#[doc]` attribute, so holding that offset at zero makes the reported line the
// `BRANCH_BASES.md` line a maintainer must actually open. Anything added above
// the attribute silently shifts every reported guide location. That is why this
// note sits below the item it explains rather than above it.
//
// The name line is exact; the rustc diagnostic printed underneath it sits one
// line lower, because rustdoc wraps a snippet that has no `fn main` in one. Read
// the test name for the fence, and subtract one from the inner error.
