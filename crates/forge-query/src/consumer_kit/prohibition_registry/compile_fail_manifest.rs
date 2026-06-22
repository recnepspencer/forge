use super::ForgeQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryProhibitionCompileFailFixture {
    seam: ForgeQueryProhibitedSeam,
    fixture_path: &'static str,
}

impl ForgeQueryProhibitionCompileFailFixture {
    const fn new(seam: ForgeQueryProhibitedSeam, fixture_path: &'static str) -> Self {
        Self { seam, fixture_path }
    }

    pub fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn forbidden_symbol(&self) -> &'static str {
        self.seam.public_symbol()
    }

    pub fn fixture_path(&self) -> &'static str {
        self.fixture_path
    }
}

static HARD_PROHIBITION_COMPILE_FAIL_FIXTURES: &[ForgeQueryProhibitionCompileFailFixture] = &[
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceDirectWrite,
        "tests/ui/prohibition_registry/workspace_direct_write_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceDirectBatch,
        "tests/ui/prohibition_registry/workspace_direct_batch_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthBindEntity,
        "tests/ui/prohibition_registry/existing_truth_bind_entity_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthBindRelation,
        "tests/ui/prohibition_registry/existing_truth_bind_relation_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthProbe,
        "tests/ui/prohibition_registry/existing_truth_probe_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
        "tests/ui/prohibition_registry/existing_truth_update_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthAssert,
        "tests/ui/prohibition_registry/existing_truth_assert_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthVerify,
        "tests/ui/prohibition_registry/existing_truth_verify_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthUpdateVerified,
        "tests/ui/prohibition_registry/existing_truth_update_verified_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        "tests/ui/prohibition_registry/existing_truth_delete_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDeleteWith,
        "tests/ui/prohibition_registry/existing_truth_delete_with_forbidden.rs",
    ),
    ForgeQueryProhibitionCompileFailFixture::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDeleteVerified,
        "tests/ui/prohibition_registry/existing_truth_delete_verified_forbidden.rs",
    ),
];

pub fn hard_prohibition_compile_fail_fixtures() -> &'static [ForgeQueryProhibitionCompileFailFixture]
{
    HARD_PROHIBITION_COMPILE_FAIL_FIXTURES
}
