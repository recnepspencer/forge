use super::WorthQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryProhibitionCompileFailFixture {
    seam: WorthQueryProhibitedSeam,
    fixture_path: &'static str,
}

impl WorthQueryProhibitionCompileFailFixture {
    const fn new(seam: WorthQueryProhibitedSeam, fixture_path: &'static str) -> Self {
        Self { seam, fixture_path }
    }

    pub fn seam(&self) -> WorthQueryProhibitedSeam {
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

static HARD_PROHIBITION_COMPILE_FAIL_FIXTURES: &[WorthQueryProhibitionCompileFailFixture] = &[
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceDirectWrite,
        "tests/ui/prohibition_registry/workspace_direct_write_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceDirectBatch,
        "tests/ui/prohibition_registry/workspace_direct_batch_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindEntity,
        "tests/ui/prohibition_registry/existing_truth_bind_entity_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindRelation,
        "tests/ui/prohibition_registry/existing_truth_bind_relation_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthProbe,
        "tests/ui/prohibition_registry/existing_truth_probe_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
        "tests/ui/prohibition_registry/existing_truth_update_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthAssert,
        "tests/ui/prohibition_registry/existing_truth_assert_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthVerify,
        "tests/ui/prohibition_registry/existing_truth_verify_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdateVerified,
        "tests/ui/prohibition_registry/existing_truth_update_verified_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        "tests/ui/prohibition_registry/existing_truth_delete_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteWith,
        "tests/ui/prohibition_registry/existing_truth_delete_with_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteVerified,
        "tests/ui/prohibition_registry/existing_truth_delete_verified_forbidden.rs",
    ),
];

pub fn hard_prohibition_compile_fail_fixtures() -> &'static [WorthQueryProhibitionCompileFailFixture]
{
    HARD_PROHIBITION_COMPILE_FAIL_FIXTURES
}
