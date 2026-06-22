pub struct ForgeQueryIdentityPhaseOneRootBreakTarget {
    api: &'static str,
    required_restriction: &'static str,
}

impl ForgeQueryIdentityPhaseOneRootBreakTarget {
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

const FORGE_QUERY_IDENTITY_PHASE_ONE_ROOT_BREAK_TARGETS:
    &[ForgeQueryIdentityPhaseOneRootBreakTarget] = &[
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryCommitIdentity::from_relational_commit_id",
        "admit relational commit authority through owner witness, not string representation",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQuerySnapshotIdentity::from_relational_snapshot",
        "admit relational snapshot authority through owner witness, not string representation",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "crate::memory_workspace::admit_external_commit_label",
        "keep external labels as external tokens until Query admission",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "crate::memory_workspace::admit_external_snapshot_label",
        "keep external labels as external tokens until Query admission",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "crate::memory_workspace::admit_authored_entity_label",
        "keep external labels as external tokens until Query admission",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryCommitIdentity::evidence_identity",
        "prevent evidence/reporting output from feeding authority construction",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQuerySnapshotIdentity::evidence_identity",
        "prevent evidence/reporting output from feeding authority construction",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryRuntimeBackend::current_snapshot_identity",
        "return current snapshot authority or explicit owner revalidation result",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryRuntimeSourceAdapter snapshot identity methods",
        "reject erased source adapter tokens as current Query authority",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "memory_workspace identity storage and ordering",
        "store and compare typed authority/evidence categories, not projected text",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "receipt/intake/storage/feeders identity fields",
        "carry authority-category values through receipts and feeders",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::field_identity",
        "remove the universal AsRef<str> identity sink; identity composition must use typed evidence, bridge evidence, or authority-category inputs",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::field_identity_sequence",
        "remove sequence identity composition from raw strings; evidence sequences must carry typed evidence identities",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::optional_identity",
        "remove optional raw-string identity composition; optional identity material must stay typed or be terminal value/reporting data",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::optional_evidence_identity",
        "keep optional evidence composition typed; never accept optional raw strings in an identity slot",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::field_evidence_identity",
        "must not delegate through raw string ingress; evidence identity entries require typed evidence values",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::field_bridge_identity",
        "must not flatten bridge evidence to terminal reporting projection during composition",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentity::compose",
        "witness-gate or restrict composition to owner admission and typed evidence modules",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentity::as_str",
        "make projection text unavailable outside the crate so downstream authority APIs cannot rebuild identity from reporting labels",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentity Display / AsRef<str>",
        "remove formatting and string-like coercions that let evidence identities re-enter lower-authority string APIs",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentity::bridge_evidence_identity",
        "remove or owner-gate bridge evidence export so digest token text cannot rebuild bridge authority",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentity::bridge_external_identity_evidence",
        "external-token bridge export requires explicit admission and must not imply current authority",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "crate::identity::hash_parts",
        "privatize low-level digest hashing so feeder/certification modules cannot compose authority-adjacent identities from string folklore",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "Canonical*Digest::from_parts",
        "restrict string-part digest construction to owner modules and terminal certification evidence",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "Canonical*Digest::as_str",
        "terminal projection only; digest text must not feed authority, routing, comparison, or compose APIs",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "Canonical*Digest::evidence_identity",
        "remove self-referential digest-string composition; derive digest evidence from owner authority instead",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "crate::memory_workspace::admit_authored_entity_label",
        "external/ad-hoc entity labels must enter as external tokens and require owner admission",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQuery*Identity::preview(ForgeQueryEvidenceIdentity)",
        "preview identity construction requires admitted evidence or owner witness, not bare evidence identity",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "impl Display for ForgeQuery*Identity",
        "remove or make opaque so truth IDs cannot be rebuilt from formatted projection text",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEntityIdentity Ord / Hash via projection text",
        "ordering and hashing must compare typed authority/category values, not projected labels",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentityEvidence::from_external_authority",
        "accept only BridgeTruthExternalIdentityToken; raw/projection/digest text cannot become bridge evidence",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentityEvidence::as_str",
        "terminal reporting module only; bridge evidence projection cannot feed Query authority",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentityEvidence::is_empty",
        "crate-private only; bridge evidence must not expose public string-like predicates",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentityEvidence::from_query_evidence_identity",
        "preserve category-pair evidence and block raw or substitute inputs",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentity::<Tag>::evidence_identity",
        "must not downgrade current truth identity to external-authority evidence",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "BridgeIdentity::<Tag>::bridge_trust_boundary",
        "allowed typed bridge boundary export only when the source tag implements a tag-specific bridge identity family kind",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "ForgeQueryEvidenceIdentityEncoder::field_bridge_authority_identity",
        "typed replacement for field_bridge_identity; must accept only bridge boundary-authority witnesses and never BridgeIdentityEvidence, projection, digest, external token, or raw text",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "*_for_reporting() -> &str on authority-bearing artifacts",
        "return typed projection identity or stay crate-private terminal reporting only",
    ),
    ForgeQueryIdentityPhaseOneRootBreakTarget::new(
        "*_for_reporting: String fields beside typed authority",
        "remove cached projection lanes when typed authority/evidence is already stored",
    ),
];

pub const fn forge_query_identity_phase_one_root_break_targets(
) -> &'static [ForgeQueryIdentityPhaseOneRootBreakTarget] {
    FORGE_QUERY_IDENTITY_PHASE_ONE_ROOT_BREAK_TARGETS
}
