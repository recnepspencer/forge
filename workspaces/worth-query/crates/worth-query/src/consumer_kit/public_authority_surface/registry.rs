use super::model::{
    WorthQueryPublicAuthorityOwner as Owner, WorthQueryPublicAuthoritySurfaceClass as Class,
    WorthQueryPublicAuthoritySurfaceRow as Row,
};
use std::sync::OnceLock;

const FOUNDATION_FACADE: &str = "src/facade/exports_foundation.rs";
const POLICY_FACADE: &str = "src/facade/exports_policy.rs";
const RUNTIME_FACADE: &str = "src/facade/exports_runtime.rs";
const RUNTIME_PRODUCTS_FACADE: &str = "src/facade/exports_runtime_products.rs";

pub fn worth_query_public_authority_surface_rows() -> &'static [Row] {
    static ROWS: OnceLock<Vec<Row>> = OnceLock::new();
    ROWS.get_or_init(|| {
        PUBLIC_AUTHORITY_SURFACE_ROWS
            .iter()
            .chain(super::phase_two_registry::phase_two_authority_surface_rows())
            .chain(super::phase_three_registry::phase_three_authority_surface_rows())
            .copied()
            .collect()
    })
}

#[rustfmt::skip]
const PUBLIC_AUTHORITY_SURFACE_ROWS: &[Row] = &[
    removed(
        "CanonicalQueryDigest::from_domain_parts",
        "src/identity/digest.rs",
        "CanonicalQueryDigest",
        FOUNDATION_FACADE,
        "CanonicalQueryDigest",
        "identity-evolution and canonical-query authority inputs",
        Owner::Identity,
        "Query-minted canonical query authority handle",
    ),
    removed(
        "SchemaBasisDigest::from_domain_parts",
        "src/identity/digest.rs",
        "SchemaBasisDigest",
        FOUNDATION_FACADE,
        "SchemaBasisDigest",
        "schema-basis authority inputs",
        Owner::Identity,
        "non-authoritative external schema token plus Query admission",
    ),
    removed(
        "BasisDigest::from_domain_parts",
        "src/identity/digest.rs",
        "BasisDigest",
        FOUNDATION_FACADE,
        "BasisDigest",
        "basis and identity-evolution authority inputs",
        Owner::Identity,
        "Query-minted scoped basis authority handle",
    ),
    sealed_with_probe(
        "IdentityEvolutionQueryContext::lineage_traversal",
        "src/identity_evolution/request.rs",
        "lineage_traversal",
        FOUNDATION_FACADE,
        "IdentityEvolutionQueryContext",
        "identity-evolution admission",
        Owner::IdentityEvolution,
        "sealed Query lineage traversal context",
    ),
    sealed_with_probe(
        "IdentityEvolutionQueryContext::correspondence_identity_comparison",
        "src/identity_evolution/request.rs",
        "correspondence_identity_comparison",
        FOUNDATION_FACADE,
        "IdentityEvolutionQueryContext",
        "identity-evolution admission",
        Owner::IdentityEvolution,
        "sealed Query correspondence comparison context",
    ),
    removed_basis_request(
        "QueryBasisContextRequest::current_branch_head",
        "current_branch_head",
        "Query-minted current-head declaration",
    ),
    removed_basis_request(
        "QueryBasisContextRequest::branch_head",
        "branch_head",
        "typed branch authority declaration",
    ),
    removed_basis_request(
        "QueryBasisContextRequest::historical_snapshot",
        "historical_snapshot",
        "typed historical snapshot authority declaration",
    ),
    removed_basis_request(
        "QueryBasisContextRequest::historical_commit",
        "historical_commit",
        "typed historical commit authority declaration",
    ),
    removed_basis_request(
        "QueryBasisContextRequest::preview_derived_historical",
        "preview_derived_historical",
        "typed preview-derived historical authority declaration",
    ),
    internalized(
        "bind_legacy_query_basis_context",
        "src/query_context/basis.rs",
        POLICY_FACADE,
        "query basis binding",
        Owner::BasisLifecycle,
        "one scoped basis capability lifecycle",
    ),
    internalized(
        "admit_legacy_query_basis_context",
        "src/query_context/basis.rs",
        POLICY_FACADE,
        "query basis admission",
        Owner::BasisLifecycle,
        "one scoped basis capability lifecycle",
    ),
    internalized(
        "execute_legacy_query_basis_context",
        "src/query_context/execution.rs",
        POLICY_FACADE,
        "query basis execution",
        Owner::BasisLifecycle,
        "execution consuming a scoped basis capability",
    ),
    sealed(
        "admit_query_basis_context",
        "src/query_context/scoped.rs",
        POLICY_FACADE,
        "scoped query basis admission",
        Owner::BasisLifecycle,
    ),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint", "authoritative_runtime_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::effect_runtime_entrypoint", "effect_runtime_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::authoritative_write_entrypoint", "authoritative_write_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::authoritative_write_batch_entrypoint", "authoritative_write_batch_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::basis_observation_lane", "basis_observation_lane"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::read_family_entrypoint", "read_family_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint", "read_family_in_basis_context_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::live_read_entrypoint", "live_read_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::derived_materialization_entrypoint", "derived_materialization_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::derived_inspection_entrypoint", "derived_inspection_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::generic_inspection_entrypoint", "generic_inspection_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint", "existing_truth_probe_entrypoint"),
    delete_raw_intent("WorthQueryRawIntentAdmissionRequest::projection_consumption", "projection_consumption"),
    Row::new(
        "WorthQueryIntentAdmissionEligibility::from_request",
        "src/intent_admission/eligibility/artifact.rs",
        "from_request",
        None,
        None,
        "generic intent eligibility",
        Owner::IntentAdmission,
        Class::InternalAdapter,
        Class::InternalAdapter,
        "internal eligibility derived from ordinary declarations",
    ),
    internalized(
        "admit_runtime_intent_request",
        "src/intent_admission/decisions/mod.rs",
        RUNTIME_PRODUCTS_FACADE,
        "generic intent admission",
        Owner::IntentAdmission,
        "declarative capability-specific admission handoff",
    ),
    projection(
        "QuerySubscriptionBasisPosture",
        "src/subscription/posture.rs",
        RUNTIME_PRODUCTS_FACADE,
        "subscription diagnostics",
        Owner::Subscription,
        "derived posture from scoped subscription basis proof",
    ),
    scoped_subscription_constructor("LiveQueryAdmissionArtifact::from_live_promotion", "from_live_promotion"),
    scoped_subscription_constructor("LiveQueryAdmissionArtifact::from_live_promotion_with_future_selection", "from_live_promotion_with_future_selection"),
    scoped_subscription_constructor("LiveQueryAdmissionArtifact::from_live_promotion_with_view", "from_live_promotion_with_view"),
    scoped_subscription_constructor("LiveQueryAdmissionArtifact::from_live_promotion_with_view_and_future_selection", "from_live_promotion_with_view_and_future_selection"),
    sealed(
        "ScopedSubscriptionDeclarationBasis",
        "src/basis_lifecycle/scoping.rs",
        FOUNDATION_FACADE,
        "subscription declaration",
        Owner::Subscription,
    ),
    sealed(
        "ScopedSubscriptionActivationBasis",
        "src/basis_lifecycle/scoping.rs",
        FOUNDATION_FACADE,
        "subscription activation",
        Owner::Subscription,
    ),
    sealed_with_probe(
        "activate_subscription_basis",
        "src/basis_lifecycle/scoping.rs",
        "activate_subscription_basis",
        FOUNDATION_FACADE,
        "activate_subscription_basis",
        "subscription activation",
        Owner::Subscription,
        "exact activation successor derived from sealed declaration proof",
    ),
    scoped_causal_constructor(
        "CausalInspection::for_observation",
        "for_observation",
    ),
    sealed(
        "ScopedInspectionBasis",
        "src/basis_lifecycle/scoping.rs",
        FOUNDATION_FACADE,
        "causal inspection authorization",
        Owner::CausalInspection,
    ),
    Row::new(
        "PreviewLiveSessionPlanBinding",
        "src/preview/mod.rs",
        "PreviewLiveSessionPlanBinding",
        None,
        None,
        "preview drift and execution",
        Owner::Preview,
        Class::InternalAdapter,
        Class::InternalAdapter,
        "ScopedPreviewLiveSessionPlanBinding",
    ),
    Row::new(
        "assess_preview_live_drift",
        "src/preview/mod.rs",
        "assess_preview_live_drift",
        Some(POLICY_FACADE),
        Some("assess_preview_live_drift"),
        "preview drift assessment",
        Owner::Preview,
        Class::OrdinaryDeclarativeApi,
        Class::OrdinaryDeclarativeApi,
        "drift assessment consuming scoped preview-live binding",
    ),
    sealed(
        "ScopedPreviewLiveSessionPlanBinding",
        "src/preview/scoped.rs",
        POLICY_FACADE,
        "preview drift and execution",
        Owner::Preview,
    ),
    Row::new(
        "facade certification and migration exports",
        "src/facade/exports_certification.rs",
        "worth_query_intent_admission_certification_output_manifest",
        Some("src/facade.rs"),
        Some("pub mod certification"),
        "certification and migration tooling",
        Owner::Facade,
        Class::CertificationOnlyApi,
        Class::CertificationOnlyApi,
        "separate certification/tooling namespace",
    ),
];

const fn removed_basis_request(
    symbol: &'static str,
    method: &'static str,
    replacement: &'static str,
) -> Row {
    Row::new(
        symbol,
        "src/query_context/basis.rs",
        method,
        None,
        None,
        "legacy query-context basis construction",
        Owner::BasisLifecycle,
        Class::RemovedSurface,
        Class::DeleteBeforeCloseout,
        replacement,
    )
}

const fn delete_raw_intent(symbol: &'static str, method: &'static str) -> Row {
    Row::new(
        symbol,
        "src/intent_admission/eligibility/request.rs",
        method,
        None,
        None,
        "raw generic intent admission",
        Owner::IntentAdmission,
        Class::InternalAdapter,
        Class::InternalAdapter,
        "ordinary declaration producing sealed admission outcome",
    )
}

const fn scoped_subscription_constructor(symbol: &'static str, method: &'static str) -> Row {
    Row::new(
        symbol,
        "src/subscription/input.rs",
        method,
        Some(RUNTIME_PRODUCTS_FACADE),
        Some("LiveQueryAdmissionArtifact"),
        "subscription declaration and activation",
        Owner::Subscription,
        Class::OrdinaryDeclarativeApi,
        Class::OrdinaryDeclarativeApi,
        "ordinary constructor consuming sealed scoped subscription declaration proof",
    )
}

const fn scoped_causal_constructor(symbol: &'static str, method: &'static str) -> Row {
    Row::new(
        symbol,
        "src/runtime/inspection/causal/builder.rs",
        method,
        Some(RUNTIME_FACADE),
        Some("CausalInspection"),
        "causal inspection planning",
        Owner::CausalInspection,
        Class::OrdinaryDeclarativeApi,
        Class::OrdinaryDeclarativeApi,
        "constructor consuming observation anchor and sealed ScopedInspectionBasis",
    )
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
const fn removed(symbol: &'static str, source_path: &'static str, source_probe: &'static str, facade_path: &'static str, facade_probe: &'static str, consumer: &'static str, owner: Owner, replacement: &'static str) -> Row {
    let _ = (facade_path, facade_probe);
    Row::new(
        symbol, source_path, source_probe, None, None,
        consumer, owner, Class::RemovedSurface, Class::RemovedSurface, replacement,
    )
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
const fn sealed_with_probe(symbol: &'static str, source_path: &'static str, source_probe: &'static str, facade_path: &'static str, facade_probe: &'static str, consumer: &'static str, owner: Owner, replacement: &'static str) -> Row {
    Row::new(
        symbol, source_path, source_probe, Some(facade_path), Some(facade_probe),
        consumer, owner, Class::SealedPhaseApi, Class::SealedPhaseApi, replacement,
    )
}

const fn internalized(
    symbol: &'static str,
    source_path: &'static str,
    _facade_path: &'static str,
    consumer: &'static str,
    owner: Owner,
    replacement: &'static str,
) -> Row {
    Row::new(
        symbol,
        source_path,
        symbol,
        None,
        None,
        consumer,
        owner,
        Class::InternalAdapter,
        Class::InternalAdapter,
        replacement,
    )
}

const fn sealed(
    symbol: &'static str,
    source_path: &'static str,
    facade_path: &'static str,
    consumer: &'static str,
    owner: Owner,
) -> Row {
    Row::new(
        symbol,
        source_path,
        symbol,
        Some(facade_path),
        Some(symbol),
        consumer,
        owner,
        Class::SealedPhaseApi,
        Class::SealedPhaseApi,
        symbol,
    )
}

#[rustfmt::skip]
const fn projection(symbol: &'static str, source_path: &'static str, facade_path: &'static str, consumer: &'static str, owner: Owner, replacement: &'static str) -> Row {
    Row::new(
        symbol,
        source_path,
        symbol,
        Some(facade_path),
        Some(symbol),
        consumer,
        owner,
        Class::OrdinaryDeclarativeApi,
        Class::ReadOnlyProjection,
        replacement,
    )
}
