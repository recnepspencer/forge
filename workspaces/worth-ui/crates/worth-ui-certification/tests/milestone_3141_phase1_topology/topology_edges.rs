use std::collections::BTreeSet;

pub(super) fn expected(path: &str) -> Option<BTreeSet<String>> {
    dependencies(path).map(|dependencies| {
        dependencies
            .iter()
            .map(|dependency| (*dependency).to_owned())
            .collect()
    })
}

fn dependencies(path: &str) -> Option<&'static [&'static str]> {
    workspace_dependencies(path)
        .or_else(|| fixture_dependencies(path))
        .or_else(|| crate_dependencies(path))
}

fn workspace_dependencies(path: &str) -> Option<&'static [&'static str]> {
    Some(match path {
        "Cargo.toml" => &[][..],
        "apps/platform-pulse/Cargo.toml" => {
            &["worth-ui", "worth-ui-host-egui", "worth-ui-native-platform"]
        }
        "crates/worth-ui/Cargo.toml" => &[
            "worth-ui-dsl",
            "worth-ui-host-headless",
            "worth-ui-inspection",
            "worth-ui-query-binding",
            "worth-ui-runtime",
        ],
        "crates/worth-ui-certification/Cargo.toml" => &[
            "worth-ui",
            "worth-ui-dsl",
            "worth-ui-host-contract",
            "worth-ui-host-egui",
            "worth-ui-host-headless",
            "worth-ui-host-native",
            "worth-ui-inspection",
            "worth-ui-native-platform",
            "worth-ui-platform-pulse",
            "worth-ui-query-binding",
            "worth-ui-runtime",
            "worth-ui-test-support",
            "worth-ui-text",
        ],
        _ => return None,
    })
}

fn fixture_dependencies(path: &str) -> Option<&'static [&'static str]> {
    Some(match path {
        "crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml" => &[
            "worth-ui",
            "worth-ui-dsl",
            "worth-ui-host-contract",
            "worth-ui-host-egui",
            "worth-ui-host-headless",
            "worth-ui-host-native",
            "worth-ui-inspection",
            "worth-ui-native-platform",
            "worth-ui-query-binding",
            "worth-ui-runtime",
            "worth-ui-test-support",
        ],
        "crates/worth-ui-certification/tests/fixtures/host_contract_only_adapter/Cargo.toml" => {
            &["worth-ui-host-contract"]
        }
        "crates/worth-ui-certification/tests/fixtures/runtime_effect_adapter/Cargo.toml" => {
            &["worth-ui-host-contract", "worth-ui-runtime"]
        }
        "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer-direct-alias/Cargo.toml"
        | "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer-extern-alias/Cargo.toml"
        | "crates/worth-ui-certification/tests/fixtures/topology_negative/admission_facade_bypass_consumer/crates/fake-admission-consumer/Cargo.toml"
        | "crates/worth-ui-certification/tests/fixtures/topology_negative/host_egui_forbidden_runtime_import/crates/worth-ui-host-egui/Cargo.toml"
        | "crates/worth-ui-certification/tests/fixtures/topology_negative/obligation_facade_bypass_consumer/crates/fake-obligation-consumer/Cargo.toml" => {
            &["worth-ui-runtime"]
        }
        "crates/worth-ui-certification/tests/fixtures/topology_negative/inspection_facade_bypass_consumer/crates/fake-inspection-consumer/Cargo.toml" => {
            &["worth-ui-inspection", "worth-ui-runtime"]
        }
        _ => return None,
    })
}

fn crate_dependencies(path: &str) -> Option<&'static [&'static str]> {
    Some(match path {
        "crates/worth-ui-components/Cargo.toml" => &["worth-ui-theme"],
        "crates/worth-ui-dsl/Cargo.toml"
        | "crates/worth-ui-host-contract/Cargo.toml"
        | "crates/worth-ui-inspection/Cargo.toml"
        | "crates/worth-ui-retained-order/Cargo.toml"
        | "crates/worth-ui-theme/Cargo.toml" => &[],
        "crates/worth-ui-query-binding/Cargo.toml" => &["worth-ui-host-contract"],
        "crates/worth-ui-host-egui/Cargo.toml" => {
            &["worth-ui-host-contract", "worth-ui-test-support"]
        }
        "crates/worth-ui-host-headless/Cargo.toml" => &[
            "worth-ui-host-contract",
            "worth-ui-retained-order",
            "worth-ui-test-support",
        ],
        "crates/worth-ui-host-native/Cargo.toml" => {
            &["worth-ui-host-contract", "worth-ui-retained-order"]
        }
        "crates/worth-ui-native-platform/Cargo.toml" => &["worth-ui-runtime"],
        "crates/worth-ui-runtime/Cargo.toml" => &[
            "worth-ui-dsl",
            "worth-ui-host-contract",
            "worth-ui-host-egui",
            "worth-ui-host-headless",
            "worth-ui-host-native",
            "worth-ui-inspection",
            "worth-ui-query-binding",
            "worth-ui-text",
        ],
        "crates/worth-ui-test-support/Cargo.toml" => &["worth-ui-runtime"],
        "crates/worth-ui-text/Cargo.toml" => &["worth-ui-host-contract"],
        _ => return None,
    })
}
