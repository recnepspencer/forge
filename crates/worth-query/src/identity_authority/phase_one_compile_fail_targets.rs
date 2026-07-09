use crate::subscription::WorthQuerySubscriptionPhaseSevenCompileFailTarget;

pub struct WorthQueryIdentityPhaseOneCompileFailTarget {
    path: &'static str,
    forbidden_substitution: &'static str,
}

impl WorthQueryIdentityPhaseOneCompileFailTarget {
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

const WORTH_QUERY_IDENTITY_PHASE_ONE_COMPILE_FAIL_TARGETS:
    &[WorthQueryIdentityPhaseOneCompileFailTarget] = &[
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/projection_cannot_satisfy_query_authority.rs",
        "projection identity cannot satisfy Query authority",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/digest_cannot_satisfy_query_authority.rs",
        "digest evidence cannot satisfy Query authority",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/external_token_cannot_satisfy_query_authority.rs",
        "external token cannot satisfy Query authority",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/bridged_cannot_satisfy_current_query_authority.rs",
        "boundary-bridged identity requires Query readmission",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/wrong_kind_cannot_satisfy_query_family.rs",
        "wrong identity kind cannot satisfy another Query identity family",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/raw_text_cannot_satisfy_query_authority.rs",
        "raw text cannot satisfy Query authority",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/reporting_accessor_cannot_feed_query_authority.rs",
        "reporting accessor output cannot feed Query authority",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/external_label_mint_removed.rs",
        "external label string mint APIs cannot construct truth IDs",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/external_compose_forbidden.rs",
        "evidence compose entry is crate-private",
    ),
    WorthQueryIdentityPhaseOneCompileFailTarget::new(
        "tests/ui/query_identity_authority/external_encoder_forbidden.rs",
        "evidence encoder is crate-private",
    ),
];

pub const fn worth_query_identity_phase_one_compile_fail_targets(
) -> &'static [WorthQueryIdentityPhaseOneCompileFailTarget] {
    WORTH_QUERY_IDENTITY_PHASE_ONE_COMPILE_FAIL_TARGETS
}

pub const fn worth_query_identity_phase_one_subscription_phase_seven_reentry_targets(
) -> &'static [WorthQuerySubscriptionPhaseSevenCompileFailTarget] {
    crate::subscription::worth_query_subscription_phase_seven_compile_fail_targets()
}
