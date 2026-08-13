//! Exact owner and field topology for the central C7 phase contexts.

use std::collections::BTreeSet;

use syn::{Fields, ImplItem, Item, Type, Visibility};

use super::{query_source, root, syntax};

const OWNER_PRIVATE_CONTEXTS: &[(&str, &str)] = &[
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/application_attempt_affinity.rs",
        "WorthQueryApplicationAttemptBasis",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/application_attempt_affinity.rs",
        "WorthQueryApplicationAttemptAffinity",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared.rs",
        "WorthQueryPreparedApplicationCommit",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running.rs",
        "WorthQueryRunningApplicationCommit",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression.rs",
        "WorthQueryProgressedApplicationCommit",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/registered.rs",
        "WorthQueryRegisteredProviderAttempt",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/fresh.rs",
        "WorthQueryFreshProviderAttempt",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/authorized.rs",
        "WorthQueryAuthorizedProviderCommit",
    ),
    (
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/session_admission.rs",
        "WorthQueryRegisteredProviderSession",
    ),
    (
        "domain_computation/provider_session/protocol/readmission.rs",
        "WorthQueryProviderPlanReadmission",
    ),
    (
        "domain_computation/provider_session/protocol/readmission/prepared.rs",
        "WorthQueryPreparedProviderSession",
    ),
    (
        "domain_computation/provider_session/protocol/readmission/prepared/staged.rs",
        "WorthQuerySessionBoundReadsAndEffects",
    ),
    (
        "domain_computation/provider_session/protocol/readmission/prepared/staged/prepare_outcome.rs",
        "WorthQuerySessionPrepareOutcome",
    ),
    (
        "domain_computation/provider_session/provisional_attempt/provider_port.rs",
        "WorthQueryProvisionalOverlayCleanupBinding",
    ),
    (
        "domain_computation/authorization/operation_admission/capability_admission/preparation/observation/admitted_access/exact_observation.rs",
        "WorthQueryExactCapabilityObservationContext",
    ),
];

fn type_terminal(ty: &Type) -> Option<String> {
    if let Type::Reference(reference) = ty {
        return type_terminal(reference.elem.as_ref());
    }
    let Type::Path(path) = ty else { return None };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn named_struct_has_only_private_fields(source: &str, name: &str) -> bool {
    syntax(source).items.into_iter().any(|item| {
        let Item::Struct(item) = item else {
            return false;
        };
        if item.ident != name {
            return false;
        }
        let fields = match item.fields {
            Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
            Fields::Unnamed(fields) => fields.unnamed.into_iter().collect(),
            Fields::Unit => Vec::new(),
        };
        fields
            .iter()
            .all(|field| matches!(field.vis, Visibility::Inherited))
    })
}

fn observation_seal(source: &str) -> (bool, BTreeSet<String>) {
    let syntax = syntax(source);
    let seal_is_private = syntax.items.iter().any(|item| {
        matches!(item, Item::Mod(module)
            if module.ident == "source_seal"
                && matches!(module.vis, Visibility::Inherited))
    });
    let targets = syntax
        .items
        .into_iter()
        .filter_map(|item| {
            let Item::Impl(item_impl) = item else {
                return None;
            };
            let (_, trait_path, _) = item_impl.trait_?;
            trait_path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "SealedCapabilityObservationSource")
                .then(|| type_terminal(item_impl.self_ty.as_ref()))
                .flatten()
        })
        .collect();
    (seal_is_private, targets)
}

fn broad_raw_refresh_methods(source: &str) -> Vec<String> {
    syntax(source)
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Impl(item) => Some(item.items),
            _ => None,
        })
        .flatten()
        .filter_map(|item| match item {
            ImplItem::Fn(function) if !matches!(function.vis, Visibility::Inherited) => {
                let inputs = function
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|input| match input {
                        syn::FnArg::Typed(input) => type_terminal(input.ty.as_ref()),
                        syn::FnArg::Receiver(_) => None,
                    })
                    .collect::<BTreeSet<_>>();
                ((inputs.contains("WorthQueryRetainedCapabilityAuthorization")
                    || inputs.contains("WorthQueryRetainedAuthorizationDecisionFacts"))
                    && inputs.contains("WorthQueryGraphWorkSessionIdentity")
                    && inputs.contains("BranchId"))
                .then(|| function.sig.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

fn authorization_sources() -> Vec<String> {
    let directory = root().join(
        "workspaces/worth-query/crates/worth-query-execution/src/domain_computation/authorization",
    );
    let mut paths = Vec::new();
    rust_sources_under(&directory, &mut paths);
    paths
        .into_iter()
        .map(|path| std::fs::read_to_string(&path).expect("read Query authorization source"))
        .collect()
}

fn rust_sources_under(directory: &std::path::Path, sources: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read Relational source directory") {
        let path = entry.expect("Relational source entry").path();
        if path.is_dir() {
            rust_sources_under(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

#[test]
fn relational_runtime_and_every_family_use_semantic_owners() {
    let relational = root().join("crates/worth-relational/src");
    assert!(relational.join("runtime/mod.rs").is_file());
    assert!(relational.join("snapshots/guard.rs").is_file());
    let mut sources = Vec::new();
    rust_sources_under(&relational, &mut sources);
    let logic_sources = sources
        .into_iter()
        .filter(|path| path.components().any(|part| part.as_os_str() == "logic"))
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    assert!(
        logic_sources.is_empty(),
        "generic logic owners remain: {logic_sources:?}"
    );
}

#[test]
fn central_attempt_phase_and_observation_context_fields_are_owner_private() {
    for (path, name) in OWNER_PRIVATE_CONTEXTS {
        assert!(
            named_struct_has_only_private_fields(&query_source(path), name),
            "{name} fields escaped its semantic owner in {path}"
        );
    }
    assert!(!named_struct_has_only_private_fields(
        "struct WorthQueryApplicationAttemptAffinity { pub(super) runtime: u64 }",
        "WorthQueryApplicationAttemptAffinity"
    ));
}

#[test]
fn capability_observation_seal_is_private_with_an_exact_impl_catalog() {
    let source = query_source("domain_computation/authorization/delegation_admission.rs");
    let (private, targets) = observation_seal(&source);
    assert!(private, "observation seal module must be owner-private");
    assert_eq!(
        targets,
        BTreeSet::from([
            "WorthQueryAuthorizationRevalidationObservation".to_owned(),
            "WorthQueryCapabilityRevalidationObservation".to_owned(),
            "WorthQueryCurrentCapabilityObservation".to_owned(),
            "WorthQueryExactCapabilityObservationContext".to_owned(),
        ])
    );
    let (private, _) = observation_seal(
        "pub(super) mod source_seal { trait SealedCapabilityObservationSource {} }",
    );
    assert!(!private, "broadening the seal module must fail the fence");
}

#[test]
fn no_broad_raw_capability_refresh_accepts_independent_session_and_branch_axes() {
    let escaped = authorization_sources()
        .into_iter()
        .flat_map(|source| broad_raw_refresh_methods(&source))
        .collect::<Vec<_>>();
    assert!(
        escaped.is_empty(),
        "raw revalidation axes escaped: {escaped:?}"
    );
    for retained in [
        "WorthQueryRetainedCapabilityAuthorization",
        "WorthQueryRetainedAuthorizationDecisionFacts",
    ] {
        let mutant = format!(
            r#"
                struct Runtime;
                struct {retained};
                struct WorthQueryGraphWorkSessionIdentity;
                struct BranchId;
                impl Runtime {{
                    pub(crate) fn renamed(
                        &self,
                        _: &mut {retained},
                        _: WorthQueryGraphWorkSessionIdentity,
                        _: &BranchId,
                    ) {{}}
                }}
            "#
        );
        assert_eq!(broad_raw_refresh_methods(&mutant), vec!["renamed"]);
    }
}
