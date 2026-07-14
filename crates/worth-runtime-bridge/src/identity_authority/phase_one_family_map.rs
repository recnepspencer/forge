pub struct BridgeTruthIdentityPhaseOneFamily {
    family: &'static str,
    owner_authority: &'static str,
    kind_marker: &'static str,
    allowed_categories: &'static [&'static str],
    phase_two_frontier: &'static str,
}

impl BridgeTruthIdentityPhaseOneFamily {
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

const BRIDGE_AUTHORITY_CATEGORIES: &[&str] = &[
    "BridgeTruthAuthorityIdentity",
    "BridgeTruthBoundaryBridgedIdentity",
    "BridgeTruthProjectionIdentity",
    "BridgeTruthDigestIdentityEvidence",
];

const BRIDGE_EXTERNAL_CATEGORIES: &[&str] = &["BridgeTruthExternalIdentityToken"];

const BRIDGE_TRUTH_IDENTITY_PHASE_ONE_FAMILIES: &[BridgeTruthIdentityPhaseOneFamily] = &[
    BridgeTruthIdentityPhaseOneFamily::new(
        "commit",
        "BridgeTruthAuthority",
        "BridgeCommitIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "TruthCommitIdentity construction and evidence export",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "snapshot",
        "BridgeTruthAuthority",
        "BridgeSnapshotIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "TruthSnapshotIdentity construction and evidence export",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "patch",
        "BridgeTruthAuthority",
        "BridgePatchIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "TruthPatchIdentity construction and evidence export",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "branch",
        "BridgeTruthAuthority",
        "BridgeBranchIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "TruthBranchIdentity construction and evidence export",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "evidence_reference",
        "BridgeTruthAuthority",
        "BridgeEvidenceReferenceIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "BridgeIdentityEvidence conversion and retained lookup",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "retained_mapping",
        "BridgeTruthAuthority",
        "BridgeRetainedMappingIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "causal retained mapping evidence",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "causal_envelope",
        "BridgeTruthAuthority",
        "BridgeCausalEnvelopeIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "causal envelope receipt/export",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "causal_reference",
        "BridgeTruthAuthority",
        "BridgeCausalReferenceIdentityKind",
        BRIDGE_AUTHORITY_CATEGORIES,
        "causal reference retention and lookup",
    ),
    BridgeTruthIdentityPhaseOneFamily::new(
        "receipt",
        "BridgeTruthAuthority",
        "BridgeReceiptIdentityKind",
        BRIDGE_EXTERNAL_CATEGORIES,
        "Query intake and receipt lowering",
    ),
];

pub const fn bridge_truth_identity_phase_one_families(
) -> &'static [BridgeTruthIdentityPhaseOneFamily] {
    BRIDGE_TRUTH_IDENTITY_PHASE_ONE_FAMILIES
}
