pub struct ForgeQueryIdentityPhaseOneFamily {
    family: &'static str,
    owner_authority: &'static str,
    kind_marker: &'static str,
    allowed_categories: &'static [&'static str],
    phase_two_frontier: &'static str,
}

impl ForgeQueryIdentityPhaseOneFamily {
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

const QUERY_AUTHORITY_CATEGORIES: &[&str] = &[
    "QueryAuthorityIdentity",
    "QueryBoundaryBridgedIdentity",
    "QueryProjectionIdentity",
    "QueryDigestIdentityEvidence",
];

const QUERY_EXTERNAL_CATEGORIES: &[&str] = &["QueryExternalIdentityToken"];

const FORGE_QUERY_IDENTITY_PHASE_ONE_FAMILIES: &[ForgeQueryIdentityPhaseOneFamily] = &[
    ForgeQueryIdentityPhaseOneFamily::new(
        "commit",
        "QueryReceiptAdmissionAuthority",
        "QueryCommitIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "memory workspace commit identity and receipts",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "snapshot",
        "QueryRuntimeBackendAuthority",
        "QuerySnapshotIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "runtime backend current snapshot identity",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "entity",
        "QueryReceiptAdmissionAuthority",
        "QueryEntityIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "memory workspace entity identity storage and ordering",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "evidence",
        "QueryEvidenceAuthority",
        "QueryEvidenceIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "evidence composition and reporting quarantine",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "intent",
        "QueryIntentAuthority",
        "QueryIntentIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "intent admission and receipt lowering",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "session",
        "QuerySubscriptionAuthority",
        "QuerySessionIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "runtime session and subscription lifecycle",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "basis",
        "QueryRuntimeBackendAuthority",
        "QueryBasisIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "basis binding and shared read pinning",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "receipt",
        "QueryReceiptAdmissionAuthority",
        "QueryReceiptIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "mutation/read/write receipt identity fields",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "feeder",
        "QueryFeederAuthority",
        "QueryFeederIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "cross-spine feeder identity flow",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "retained_bridge_mapping",
        "QueryFeederAuthority",
        "QueryRetainedBridgeMappingIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "bridge retained evidence feeder and lookup",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "signal_route",
        "QuerySignalInvalidationAuthority",
        "QuerySignalRouteIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "signal route lookup and invalidation",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "signal_invalidation",
        "QuerySignalInvalidationAuthority",
        "QuerySignalInvalidationIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "signal invalidation evidence and receipt flow",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "workflow",
        "QueryWorkflowAuthority",
        "QueryWorkflowIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "workflow binding, lowering, and inspection",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "domain_capability",
        "QueryDomainCapabilityAuthority",
        "QueryDomainCapabilityIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "domain-capability materialization and canonical runtime",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "materialization",
        "QueryMaterializationAuthority",
        "QueryMaterializationIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "materialized artifact and downstream adapter flow",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "effect_lifecycle",
        "QueryEffectLifecycleAuthority",
        "QueryEffectLifecycleIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "effect lifecycle evidence and bridge observation receipts",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "causal_inspection",
        "QueryCausalInspectionAuthority",
        "QueryCausalInspectionIdentityKind",
        QUERY_AUTHORITY_CATEGORIES,
        "causal inspection request, artifact, and retained bridge evidence",
    ),
    ForgeQueryIdentityPhaseOneFamily::new(
        "subscription",
        "QuerySubscriptionAuthority",
        "QuerySubscriptionIdentityKind",
        QUERY_EXTERNAL_CATEGORIES,
        "live subscription/session feeder flow",
    ),
];

pub const fn forge_query_identity_phase_one_families() -> &'static [ForgeQueryIdentityPhaseOneFamily]
{
    FORGE_QUERY_IDENTITY_PHASE_ONE_FAMILIES
}
