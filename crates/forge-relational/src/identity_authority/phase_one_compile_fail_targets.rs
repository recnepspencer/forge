pub struct RelationalSourceTruthIdentityPhaseOneCompileFailTarget {
    path: &'static str,
    forbidden_substitution: &'static str,
}

impl RelationalSourceTruthIdentityPhaseOneCompileFailTarget {
    pub const fn new(path: &'static str, forbidden_substitution: &'static str) -> Self {
        Self {
            path,
            forbidden_substitution,
        }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn forbidden_substitution(&self) -> &'static str {
        self.forbidden_substitution
    }
}

const RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_COMPILE_FAIL_TARGETS:
    &[RelationalSourceTruthIdentityPhaseOneCompileFailTarget] = &[
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/projection_cannot_satisfy_source_truth_authority.rs",
        "projection identity cannot satisfy relational source-truth authority",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/digest_cannot_satisfy_source_truth_authority.rs",
        "digest evidence cannot satisfy relational source-truth authority",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/external_token_cannot_satisfy_source_truth_authority.rs",
        "external token cannot satisfy relational source-truth authority",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/bridged_cannot_satisfy_source_truth_authority.rs",
        "boundary-bridged identity requires relational owner readmission",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/wrong_kind_cannot_satisfy_source_truth_family.rs",
        "wrong identity kind cannot satisfy another relational source-truth family",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/raw_text_cannot_satisfy_source_truth_authority.rs",
        "raw text cannot satisfy relational source-truth authority",
    ),
    RelationalSourceTruthIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/relational_truth_identity/bridge_presentation_cannot_reconstruct_authority.rs",
        "bridge presentation export cannot reconstruct relational source-truth authority",
    ),
];

pub const fn relational_source_truth_identity_phase_one_compile_fail_targets(
) -> &'static [RelationalSourceTruthIdentityPhaseOneCompileFailTarget] {
    RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_COMPILE_FAIL_TARGETS
}
