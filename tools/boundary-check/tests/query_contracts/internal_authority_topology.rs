//! AST-backed C7 fences for Query-internal authority topology.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::{Fields, ImplItem, Item, ReturnType, Type, Visibility};

#[path = "internal_authority_topology/constructor_inventory.rs"]
mod constructor_inventory;
#[path = "internal_authority_topology/phase_contexts.rs"]
mod phase_contexts;
#[path = "internal_authority_topology/producer_inventory.rs"]
mod producer_inventory;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn query_source(relative: &str) -> String {
    let path = root()
        .join("workspaces/worth-query/crates/worth-query-execution/src")
        .join(relative);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn query_api_source(relative: &str) -> String {
    let path = root()
        .join("workspaces/worth-query/crates/worth-query/src")
        .join(relative);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn syntax(source: &str) -> syn::File {
    syn::parse_file(source).unwrap_or_else(|error| panic!("invalid Rust specimen: {error}"))
}

fn field_visibilities(source: &str) -> Vec<(String, bool)> {
    syntax(source)
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Struct(item) => Some(item),
            _ => None,
        })
        .flat_map(|item| match item.fields {
            Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
            Fields::Unnamed(fields) => fields.unnamed.into_iter().collect::<Vec<_>>(),
            Fields::Unit => Vec::new(),
        })
        .map(|field| {
            (
                field
                    .ident
                    .map_or_else(|| "<tuple>".to_owned(), |ident| ident.to_string()),
                matches!(field.vis, Visibility::Inherited),
            )
        })
        .collect()
}

fn all_struct_fields_are_owner_private(source: &str) -> bool {
    field_visibilities(source)
        .iter()
        .all(|(_, is_private)| *is_private)
}

fn struct_names(source: &str) -> BTreeSet<String> {
    syntax(source)
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Struct(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn child_modules(source: &str) -> BTreeSet<String> {
    syntax(source)
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Mod(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn exposes_mutable_binding(source: &str, binding: &str) -> bool {
    syntax(source).items.into_iter().any(|item| {
        let Item::Impl(item_impl) = item else {
            return false;
        };
        item_impl.items.into_iter().any(|member| {
            let ImplItem::Fn(method) = member else {
                return false;
            };
            let ReturnType::Type(_, output) = method.sig.output else {
                return false;
            };
            matches!(
                output.as_ref(),
                Type::Reference(reference)
                    if reference.mutability.is_some()
                        && type_terminal(reference.elem.as_ref()).as_deref() == Some(binding)
            )
        })
    })
}

fn type_terminal(ty: &Type) -> Option<String> {
    if let Type::Reference(reference) = ty {
        return type_terminal(reference.elem.as_ref());
    }
    let Type::Path(path) = ty else {
        return None;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn method_visibility(source: &str, owner: &str, method: &str) -> Option<String> {
    syntax(source).items.into_iter().find_map(|item| {
        let Item::Impl(item_impl) = item else {
            return None;
        };
        if type_terminal(item_impl.self_ty.as_ref()).as_deref() != Some(owner) {
            return None;
        }
        item_impl.items.into_iter().find_map(|member| {
            let ImplItem::Fn(function) = member else {
                return None;
            };
            (function.sig.ident == method).then(|| visibility_kind(&function.vis))
        })
    })
}

fn method_parameter_types(source: &str, owner: &str, method: &str) -> Vec<String> {
    syntax(source)
        .items
        .into_iter()
        .find_map(|item| {
            let Item::Impl(item_impl) = item else {
                return None;
            };
            if type_terminal(item_impl.self_ty.as_ref()).as_deref() != Some(owner) {
                return None;
            }
            item_impl.items.into_iter().find_map(|member| {
                let ImplItem::Fn(function) = member else {
                    return None;
                };
                (function.sig.ident == method).then(|| {
                    function
                        .sig
                        .inputs
                        .into_iter()
                        .filter_map(|input| match input {
                            syn::FnArg::Typed(input) => type_terminal(input.ty.as_ref()),
                            syn::FnArg::Receiver(_) => None,
                        })
                        .collect()
                })
            })
        })
        .unwrap_or_default()
}

fn visibility_kind(visibility: &Visibility) -> String {
    match visibility {
        Visibility::Inherited => "private".to_owned(),
        Visibility::Public(_) => "public".to_owned(),
        Visibility::Restricted(restricted) => format!(
            "restricted:{}",
            restricted
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
        ),
    }
}

#[test]
fn capability_observation_has_no_sibling_mintable_world_or_raw_unpermitted_door() {
    let owner = query_source("domain_computation/authorization/delegation_admission.rs");
    let owner_structs = struct_names(&owner);
    assert!(!owner_structs.contains("WorthQueryCapabilityObservationWorld"));
    assert!(owner_structs.contains("WorthQueryBoundCapabilityObservation"));
    assert!(owner_structs.contains("WorthQueryCapabilityObservationPermit"));
    assert!(all_struct_fields_are_owner_private(&owner));
}

#[test]
fn delegation_observation_owns_discovery_and_transition_associations() {
    let admission = query_source("domain_computation/authorization/delegation_admission.rs");
    let observation =
        query_source("domain_computation/authorization/delegation_admission/observation.rs");
    let admission_modules = child_modules(&admission);
    let observation_modules = child_modules(&observation);
    assert!(!admission_modules.contains("discovery"));
    assert!(!admission_modules.contains("transition"));
    assert!(observation_modules.contains("discovery"));
    assert!(observation_modules.contains("transition"));
    let observed = struct_names(&observation);
    assert!(observed.contains("ObservedDelegationParent"));
    assert!(observed.contains("ObservedDelegationTransition"));
    assert!(all_struct_fields_are_owner_private(&observation));
}

#[test]
fn elevation_binding_exposes_no_domain_wide_mutable_field_bag() {
    let binding =
        query_source("domain_computation/authorization/elevation_progression/request_binding.rs");
    let outcome = query_source(
        "domain_computation/primary_graph/application_attempt/elevation_request_outcome.rs",
    );
    assert!(struct_names(&binding).contains("WorthQueryElevationRequestBinding"));
    assert!(all_struct_fields_are_owner_private(&binding));
    assert!(!exposes_mutable_binding(
        &outcome,
        "WorthQueryElevationRequestBinding"
    ));
}

#[test]
fn registration_and_elevation_transitions_cannot_be_reconstructed_or_split() {
    let registration = query_source(
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/registered.rs",
    );
    assert!(all_struct_fields_are_owner_private(&registration));
    assert_eq!(
        method_visibility(
            &registration,
            "WorthQueryProviderAttemptRegistrationContext",
            "new"
        )
        .as_deref(),
        Some("restricted:super")
    );
    for getter in [
        "provider",
        "admission",
        "idempotency",
        "aftermath_causality",
    ] {
        assert!(method_parameter_types(
            &registration,
            "WorthQueryProviderAttemptRegistrationContext",
            getter
        )
        .iter()
        .any(|parameter| parameter == "WorthQueryProviderRegistrationInspectionPermit"));
    }

    let requested = query_source(
        "domain_computation/primary_graph/application_attempt/elevation_request_outcome.rs",
    );
    assert_eq!(
        method_visibility(&requested, "WorthQueryRequestedElevation", "new").as_deref(),
        Some("private")
    );
    assert!(method_visibility(&requested, "WorthQueryRequestedElevation", "into_parts").is_none());
    assert!(method_parameter_types(
        &requested,
        "WorthQueryRequestedElevation",
        "into_approval_parts"
    )
    .iter()
    .any(|parameter| parameter == "WorthQueryElevationApprovalBindingPermit"));
}

#[test]
fn provider_plan_and_capability_installation_keep_raw_material_owner_private() {
    let contract = query_source("domain_computation/provider_session/protocol/plan_contract.rs");
    let admitted = query_source("domain_computation/provider_session/protocol/execution_plan.rs");
    let lowering = query_source("domain_computation/authorization/capability_lowering.rs");
    let installed =
        query_source("domain_computation/authorization/capability_lowering/installed_plan.rs");
    assert_eq!(
        struct_names(&contract),
        BTreeSet::from(["WorthQueryProviderExecutionPlanContract".to_owned()])
    );
    assert!(all_struct_fields_are_owner_private(&contract));
    assert!(struct_names(&admitted).contains("WorthQueryAdmittedProviderExecutionPlan"));
    assert!(all_struct_fields_are_owner_private(&admitted));
    assert!(struct_names(&lowering).is_empty());
    let installed_structs = struct_names(&installed);
    assert_eq!(
        installed_structs,
        BTreeSet::from(["WorthQueryInstalledCapabilityPlan".to_owned()])
    );
    assert!(all_struct_fields_are_owner_private(&installed));
}

#[test]
fn preview_binding_identity_consumes_the_sealed_binding_not_a_reconstruction_bag() {
    let contract = query_api_source("preview/binding/contract.rs");
    let identity =
        query_api_source("preview/workflow_context_identity/preview_binding_identity.rs");
    let contract_structs = struct_names(&contract);
    assert!(!contract_structs.contains("PreviewSessionBindingParts"));
    assert!(contract_structs.contains("PreviewSessionBindingTuple"));
    assert!(all_struct_fields_are_owner_private(&contract));
    assert!(syntax(&identity).items.into_iter().any(|item| {
        matches!(item, Item::Fn(function) if function.sig.inputs.iter().any(|input| {
            let syn::FnArg::Typed(input) = input else { return false; };
            matches!(input.ty.as_ref(), Type::Reference(reference)
                if type_terminal(reference.elem.as_ref()).as_deref()
                    == Some("PreviewSessionBindingTuple"))
        }))
    }));
}

#[test]
fn visibility_mutants_prove_private_field_fence_is_not_textual() {
    for visibility in ["pub", "pub(crate)", "pub(super)", "pub(in crate)"] {
        let specimen = format!("struct RenamedAxisBag {{ {visibility} authority: u64 }}");
        assert!(!all_struct_fields_are_owner_private(&specimen));
    }
    assert!(all_struct_fields_are_owner_private(
        "struct RenamedSealedContext { authority: u64 }"
    ));
}

#[test]
fn mutable_binding_mutant_proves_axis_replacement_is_detected() {
    let specimen = r#"
        struct WorthQueryElevationRequestBinding;
        struct Sibling;
        impl Sibling {
            pub(crate) fn renamed_axis_door(
                &mut self,
            ) -> &mut WorthQueryElevationRequestBinding { todo!() }
        }
    "#;
    assert!(exposes_mutable_binding(
        specimen,
        "WorthQueryElevationRequestBinding"
    ));
}

#[test]
fn constructor_visibility_mutants_prove_broad_mints_are_detected() {
    for visibility in ["pub", "pub(crate)", "pub(in crate)"] {
        let specimen =
            format!("struct Context; impl Context {{ {visibility} fn new() -> Self {{ Self }} }}");
        assert_ne!(
            method_visibility(&specimen, "Context", "new").as_deref(),
            Some("private")
        );
    }
    assert_eq!(
        method_visibility(
            "struct Context; impl Context { fn new() -> Self { Self } }",
            "Context",
            "new"
        )
        .as_deref(),
        Some("private")
    );
}

#[test]
fn inspection_permit_mutant_proves_raw_registration_getters_are_detected() {
    let unsealed = "struct Context; impl Context { pub(crate) fn provider(&self) -> u64 { 0 } }";
    assert!(method_parameter_types(unsealed, "Context", "provider").is_empty());
    let sealed = r#"
        struct Permit;
        struct Context;
        impl Context {
            pub(crate) fn provider(&self, _permit: &Permit) -> u64 { 0 }
        }
    "#;
    assert_eq!(
        method_parameter_types(sealed, "Context", "provider"),
        vec!["Permit"]
    );
}
