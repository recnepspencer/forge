mod compile_fail_manifest;
mod documentation;
mod enforcement;
mod registry;
mod row;
mod seam;

#[cfg(test)]
mod tests;

pub use compile_fail_manifest::{
    hard_prohibition_compile_fail_fixtures, WorthQueryProhibitionCompileFailFixture,
};
pub use documentation::{
    hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
    render_hard_prohibition_reference, WorthQueryHardProhibitionDocumentationRow,
};
pub use enforcement::WorthQueryProhibitionEnforcementTier;
pub use registry::{hard_prohibition_registry, WorthQueryProhibitionRegistry};
pub use row::WorthQueryProhibitionRegistryRow;
pub use seam::WorthQueryProhibitedSeam;
