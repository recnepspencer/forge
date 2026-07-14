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
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::RawDigestMinting,
        "tests/ui/public_authority_surface/raw_digest_minting_removed.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::RawBasisIdentity,
        "tests/ui/public_authority_surface/historical_string_basis_removed.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::UnscopedQueryContext,
        "tests/ui/public_authority_surface/raw_query_context_surface_removed.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::RawIntentAdmissionRequest,
        "tests/ui/intent_admission/authoring/raw_request_cannot_mint_admitted_plan.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::PostureAuthoredSubscription,
        "tests/ui/subscription_phase_seven/subscription_posture_cannot_author_admission.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::ReceiptOnlyCausalInspection,
        "tests/ui/public_authority_surface/causal_receipt_cannot_author_inspection.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::LegacyPreviewExecution,
        "tests/ui/public_authority_surface/legacy_preview_binding_cannot_execute.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::DeepFacadeToolingImport,
        "tests/ui/public_authority_surface/certification_tooling_not_in_ordinary_facade.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::LegacyQueryBasisLifecycle,
        "tests/ui/prohibition_registry/legacy_query_basis_lifecycle_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::CrateRootPhaseMirror,
        "tests/ui/prohibition_registry/crate_root_phase_mirror_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::DeepPhaseModuleImport,
        "tests/ui/prohibition_registry/deep_phase_module_import_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::OrdinaryFacadePhaseReexport,
        "tests/ui/prohibition_registry/ordinary_facade_phase_reexport_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::PhaseArtifactAlias,
        "tests/ui/prohibition_registry/phase_artifact_alias_forbidden.rs",
    ),
    WorthQueryProhibitionCompileFailFixture::new(
        WorthQueryProhibitedSeam::GenericPhaseConversion,
        "tests/ui/prohibition_registry/generic_phase_conversion_forbidden.rs",
    ),
];

pub fn hard_prohibition_compile_fail_fixtures() -> &'static [WorthQueryProhibitionCompileFailFixture]
{
    HARD_PROHIBITION_COMPILE_FAIL_FIXTURES
}
