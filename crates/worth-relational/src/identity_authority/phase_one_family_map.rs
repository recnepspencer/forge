pub struct RelationalSourceTruthIdentityPhaseOneFamily {
    family: &'static str,
    owner_authority: &'static str,
    kind_marker: &'static str,
    allowed_categories: &'static [&'static str],
    phase_two_frontier: &'static str,
}

impl RelationalSourceTruthIdentityPhaseOneFamily {
    pub const fn new(
        family: &'static str,
        owner_authority: &'static str,
        kind_marker: &'static str,
        allowed_categories: &'static [&'static str],
        phase_two_frontier: &'static str,
    ) -> Self {
        Self {
            family,
            owner_authority,
            kind_marker,
            allowed_categories,
            phase_two_frontier,
        }
    }

    pub const fn family(&self) -> &'static str {
        self.family
    }

    pub const fn owner_authority(&self) -> &'static str {
        self.owner_authority
    }

    pub const fn kind_marker(&self) -> &'static str {
        self.kind_marker
    }

    pub const fn allowed_categories(&self) -> &'static [&'static str] {
        self.allowed_categories
    }

    pub const fn phase_two_frontier(&self) -> &'static str {
        self.phase_two_frontier
    }
}

const RELATIONAL_AUTHORITY_CATEGORIES: &[&str] = &[
    "RelationalSourceTruthAuthorityIdentity",
    "RelationalSourceTruthBoundaryBridgedIdentity",
    "RelationalSourceTruthProjectionIdentity",
    "RelationalSourceTruthDigestIdentityEvidence",
];

const RELATIONAL_EXTERNAL_CATEGORIES: &[&str] = &["RelationalSourceTruthExternalIdentityToken"];

const RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_FAMILIES:
    &[RelationalSourceTruthIdentityPhaseOneFamily] = &[
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "commit",
        "RelationalSourceTruthAuthority",
        "RelationalCommitIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "presentation::bridge commit export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "entity",
        "RelationalSourceTruthAuthority",
        "RelationalEntityIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "presentation::bridge record export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "relation",
        "RelationalSourceTruthAuthority",
        "RelationalRelationIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "presentation::bridge record export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "snapshot",
        "RelationalSourceTruthAuthority",
        "RelationalSnapshotIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "presentation::bridge snapshot export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "version",
        "RelationalSourceTruthAuthority",
        "RelationalVersionIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "snapshot and commit bridge export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "branch",
        "RelationalSourceTruthAuthority",
        "RelationalBranchIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "branch/workspace bridge export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "workspace",
        "RelationalSourceTruthAuthority",
        "RelationalWorkspaceIdentityKind",
        RELATIONAL_AUTHORITY_CATEGORIES,
        "branch/workspace bridge export",
    ),
    RelationalSourceTruthIdentityPhaseOneFamily::new(
        "bridge_presentation_export",
        "RelationalSourceTruthAuthority",
        "RelationalBridgePresentationExportIdentityKind",
        RELATIONAL_EXTERNAL_CATEGORIES,
        "runtime bridge intake",
    ),
];

pub const fn relational_source_truth_identity_phase_one_families(
) -> &'static [RelationalSourceTruthIdentityPhaseOneFamily] {
    RELATIONAL_SOURCE_TRUTH_IDENTITY_PHASE_ONE_FAMILIES
}
