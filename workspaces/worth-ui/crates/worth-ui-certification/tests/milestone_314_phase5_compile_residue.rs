use crate::workspace_source_inventory;

const PRODUCTION_ROOTS: [&str; 6] = [
    "crates/worth-ui-runtime/src",
    "crates/worth-ui-host-contract/src",
    "crates/worth-ui-host-egui/src",
    "crates/worth-ui-dsl/src",
    "crates/worth-ui/src",
    "apps/platform-pulse/src",
];

fn identifiers(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|identifier| !identifier.is_empty())
}

fn is_intent_compatibility_identifier(identifier: &str) -> bool {
    let compact = identifier.replace('_', "").to_ascii_lowercase();
    if [
        "commandruntimeintentbinding",
        "commandreadinessbinding",
        "staticintentreadiness",
        "alwaysadmittedintent",
    ]
    .contains(&compact.as_str())
    {
        return true;
    }
    compact.contains("intent")
        && ["callback", "handler", "placeholder"]
            .iter()
            .any(|fragment| compact.contains(fragment))
}

#[test]
fn milestone_314_phase5_renderer_and_adapter_roots_cannot_execute_intent_providers() {
    let inventory = workspace_source_inventory();
    let forbidden_roots = [
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
        "crates/worth-ui-dsl/src",
        "crates/worth-ui-runtime/src/host/adapter",
        "crates/worth-ui-runtime/src/mounting",
        "crates/worth-ui-runtime/src/runtime/execution",
        "apps/platform-pulse/src/intent/product_input",
        "apps/platform-pulse/src/native_frame/intent/native_ingress.rs",
    ];
    let execution_symbols = [
        "UiIntentExecutionProvider",
        "UiIntentExecutionRequest",
        "UiIntentProviderSettlement",
        "dispatch_admitted_intent(",
        "advance_intent_executions(",
        "register_intent_provider(",
    ];
    for root in forbidden_roots {
        for source in inventory.rust_files_under(root) {
            for symbol in execution_symbols {
                assert!(
                    !source.text().contains(symbol),
                    "{} gives renderer or adapter code provider authority through `{symbol}`",
                    source.relative_path().display()
                );
            }
        }
    }
}

#[test]
fn milestone_314_phase5_provider_authority_has_only_canonical_owners() {
    let inventory = workspace_source_inventory();
    for root in PRODUCTION_ROOTS {
        for source in inventory.rust_files_under(root) {
            if !source.text().contains("UiIntentExecutionProvider") {
                continue;
            }
            let path = source.relative_path().to_string_lossy().replace('\\', "/");
            let allowed = path.starts_with("crates/worth-ui-runtime/src/runtime/intent_execution/")
                || path == "crates/worth-ui-runtime/src/facade/intent.rs"
                || path
                    == "crates/worth-ui-runtime/src/facade/entry/app_builder/intent_registration.rs"
                || path == "crates/worth-ui/src/facade/intent.rs"
                || path == "apps/platform-pulse/src/intent/provider.rs";
            assert!(allowed, "{path} gained non-canonical provider authority");
        }
    }
}

#[test]
fn milestone_314_phase5_has_no_intent_compatibility_lane_residue() {
    let inventory = workspace_source_inventory();
    for root in PRODUCTION_ROOTS {
        for source in inventory.rust_files_under(root) {
            let path = source.relative_path().to_string_lossy().replace('\\', "/");
            let path_compact = path.replace(['/', '_', '-'], "").to_ascii_lowercase();
            assert!(
                !is_intent_compatibility_identifier(&path_compact),
                "{path} recreates an intent callback, handler, or placeholder lane"
            );
            for identifier in identifiers(source.text()) {
                assert!(
                    !is_intent_compatibility_identifier(identifier),
                    "{path} retains forbidden intent compatibility identifier `{identifier}`"
                );
            }
        }
    }
}

#[test]
fn milestone_314_phase5_query_mutation_requires_the_product_owner_fact() {
    let inventory = workspace_source_inventory();
    let provider = inventory.text("apps/platform-pulse/src/intent/provider.rs");
    let native_ingress =
        inventory.text("apps/platform-pulse/src/native_frame/intent/native_ingress.rs");
    let product_input = inventory.text("apps/platform-pulse/src/intent/product_input.rs");
    for (owner, text) in [
        ("provider", provider),
        ("native ingress", native_ingress),
        ("product input adapter", product_input),
    ] {
        for forbidden in [
            "query_lifecycle",
            "execute_current_action",
            "UiScalarProjectionFactReceipt",
            "worth_query_",
        ] {
            assert!(
                !text.contains(forbidden),
                "{owner} gained Query authority via `{forbidden}`"
            );
        }
    }

    let product_action =
        inventory.text("apps/platform-pulse/src/native_frame/intent/product_action.rs");
    assert!(product_action.contains(".execute_current_action("));
    let publication = inventory.text("apps/platform-pulse/src/native_frame/intent/execution.rs");
    assert!(publication.contains("release_scalar_projection_predecessor()"));
    assert!(publication.contains(".admit_publication(fact)"));
    let query_owner = inventory.text("apps/platform-pulse/src/query_source/lifecycle.rs");
    assert!(query_owner.contains("fact: UiScalarProjectionFactReceipt"));
    assert!(query_owner.contains("completion.admit_publication(fact)"));
}
