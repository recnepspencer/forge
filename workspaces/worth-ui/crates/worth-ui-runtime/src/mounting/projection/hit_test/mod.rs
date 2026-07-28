mod completion;
mod seed;

pub(in crate::mounting::projection) use completion::{complete_hit_tests, rebind_hit_tests};
pub(in crate::mounting::projection) use seed::{lower_hit_test_seed, UiMountedHitTestSeed};
