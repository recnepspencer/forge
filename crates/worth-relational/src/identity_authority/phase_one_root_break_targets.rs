pub struct RelationalSourceTruthIdentityPhaseOneRootBreakTarget {
    api: &'static str,
    required_restriction: &'static str,
}

impl RelationalSourceTruthIdentityPhaseOneRootBreakTarget {
    pub const fn new(api: &'static str, required_restriction: &'static str) -> Self {
        Self {
            api,
            required_restriction,
        }
    }

    pub const fn api(&self) -> &'static str {
        self.api
    }

    pub const fn required_restriction(&self) -> &'static str {
        self.required_restriction
    }
}

const RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_ROOT_BREAK_TARGETS:
    &[RelationalSourceTruthIdentityPhaseOneRootBreakTarget] = &[
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::bridge_snapshot_identity_for_commit",
        "mint bridge snapshot identity from relational source-truth authority only",
    ),
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::bridge_snapshot_identity_for_handle",
        "mint bridge snapshot identity from relational source-truth authority only",
    ),
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::commit_envelope_to_bridge_envelope",
        "export commit envelopes with authority-category evidence, not display text",
    ),
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::publication_bundle_to_bridge_envelope",
        "export publication bundles with authority-category evidence, not display text",
    ),
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::publication_patch_to_bridge_envelope",
        "export publication patches with authority-category evidence, not display text",
    ),
    RelationalSourceTruthIdentityPhaseOneRootBreakTarget::new(
        "presentation::bridge::RuntimeBridgeRelationalSource",
        "carry relational source-truth authority through bridge presentation export",
    ),
];

pub const fn relational_source_truth_identity_phase_one_root_break_targets(
) -> &'static [RelationalSourceTruthIdentityPhaseOneRootBreakTarget] {
    RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_ROOT_BREAK_TARGETS
}
