use std::path::Path;

use super::{TestExecutionUnit, TestPlan};
use crate::catalog::TestCatalog;
use crate::classification::CiTestLane;
use crate::product::TestProduct;

#[test]
fn duplicate_identity_names_both_origins() {
    let units = vec![unit("same", "first"), unit("same", "second")];
    let error = TestPlan::new(TestProduct::Smoke, units).unwrap_err();
    assert!(error.contains("first"));
    assert!(error.contains("second"));
}

#[test]
fn sharding_is_delegated_to_nextests_stable_hash_partition() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    let product = TestProduct::Ci {
        lane: CiTestLane::Scenario,
        shard: Some((1, 3)),
    };
    let plan = TestPlan::build(&product, &catalog, workspace_root()).unwrap();

    assert!(plan.units()[0]
        .arguments()
        .windows(2)
        .any(|pair| pair == ["--partition", "hash:2/3"]));
}

#[test]
fn unknown_owner_is_denied_before_execution() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    let product = TestProduct::Owner {
        package: "worth-store-does-not-exist".into(),
    };
    let error = TestPlan::build(&product, &catalog, workspace_root()).unwrap_err();
    assert!(error.contains("worth-store-does-not-exist"));
}

#[test]
fn empty_product_is_never_green() {
    let error = TestPlan::new(TestProduct::Smoke, Vec::new()).unwrap_err();
    assert!(error.contains("selected zero units"));
}

#[test]
fn integration_partitions_name_every_classified_binary_exactly() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    for lane in [CiTestLane::Scenario, CiTestLane::Ui, CiTestLane::Formal] {
        let product = TestProduct::Ci { lane, shard: None };
        let plan = TestPlan::build(&product, &catalog, workspace_root()).unwrap();
        let command = plan.units()[0].arguments().join(" ");
        let expected = catalog
            .targets()
            .iter()
            .filter(|target| target.lane == lane)
            .map(|target| format!("binary_id(={}::{})", target.package, target.name));
        for binary in expected {
            assert!(command.contains(&binary), "{binary} absent from {lane}");
        }
        let selected_names = catalog
            .targets()
            .iter()
            .filter(|target| target.lane == lane)
            .map(|target| target.name.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        for target in selected_names {
            assert!(
                plan.units()[0]
                    .arguments()
                    .windows(2)
                    .any(|pair| pair == ["--test", target]),
                "Cargo was not narrowed to target `{target}` in {lane}"
            );
        }
    }
}

#[test]
fn required_features_are_enabled_for_guarded_targets() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    for product in [
        TestProduct::Ui,
        TestProduct::Ci {
            lane: CiTestLane::Scenario,
            shard: None,
        },
    ] {
        let plan = TestPlan::build(&product, &catalog, workspace_root()).unwrap();
        let guarded = &plan.units()[0];
        assert!(guarded
            .arguments()
            .windows(2)
            .any(|pair| pair == ["--features", "worth-store/certification-test-authority"]));
    }
}

#[test]
fn ci_products_use_ci_output_and_nonincremental_cargo_profiles() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    let product = TestProduct::Ci {
        lane: CiTestLane::Scenario,
        shard: None,
    };
    let plan = TestPlan::build(&product, &catalog, workspace_root()).unwrap();
    let arguments = plan.units()[0].arguments();

    assert!(arguments.windows(2).any(|pair| pair == ["--profile", "ci"]));
    assert!(arguments
        .windows(2)
        .any(|pair| pair == ["--cargo-profile", "ci-test"]));
}

#[test]
fn every_smoke_selector_names_one_exact_binary_test() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    let plan = TestPlan::build(&TestProduct::Smoke, &catalog, workspace_root()).unwrap();

    assert_eq!(plan.units().len(), 1);
    for unit in plan.units() {
        assert_eq!(
            unit.expected_test_count(),
            Some(crate::product::smoke_cases().len())
        );
        let command = unit.arguments().join(" ");
        for case in crate::product::smoke_cases() {
            assert!(command.contains(&format!(
                "binary_id(={}::{}) & test(={})",
                case.package, case.target, case.filter
            )));
        }
    }
}

#[test]
fn owner_product_excludes_integration_scenario_and_ui_targets() {
    let catalog = TestCatalog::load(workspace_root()).unwrap();
    let product = TestProduct::Owner {
        package: "worth-store".into(),
    };
    let plan = TestPlan::build(&product, &catalog, workspace_root()).unwrap();
    let arguments = plan.units()[0].arguments();

    for selector in ["--lib", "--bins", "--examples", "--benches"] {
        assert!(arguments.iter().any(|argument| argument == selector));
    }
    assert!(!arguments.iter().any(|argument| argument == "--test"));
}

fn unit(identity: &str, origin: &str) -> TestExecutionUnit {
    TestExecutionUnit::cargo(
        identity.into(),
        origin.into(),
        Path::new("."),
        vec!["test".into()],
        None,
    )
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
}
