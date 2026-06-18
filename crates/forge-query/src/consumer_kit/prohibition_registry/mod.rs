mod compile_fail_manifest;
mod documentation;
mod enforcement;
mod registry;
mod row;
mod seam;

#[cfg(test)]
mod tests;

pub use compile_fail_manifest::{
    hard_prohibition_compile_fail_fixtures, ForgeQueryProhibitionCompileFailFixture,
};
pub use documentation::{
    hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
    render_hard_prohibition_reference, ForgeQueryHardProhibitionDocumentationRow,
};
pub use enforcement::ForgeQueryProhibitionEnforcementTier;
pub use registry::{hard_prohibition_registry, ForgeQueryProhibitionRegistry};
pub use row::ForgeQueryProhibitionRegistryRow;
pub use seam::ForgeQueryProhibitedSeam;
