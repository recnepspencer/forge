use std::collections::{BTreeMap, BTreeSet};

use crate::classification::validate_build_graph_policy;
use crate::discovery::{
    validate_owner_build_closures, DependencyEdge, ObservedBuildGraph, OwnerBuildClosure,
    OwnerTestBoundary,
};

#[test]
fn feature_leaks_and_support_radius_mutants_are_denied() {
    let graph = ObservedBuildGraph {
        dependency_edges: vec![DependencyEdge {
            consumer: "worth-store-blob-chunks".to_owned(),
            provider: "worth-store-io-scheduler".to_owned(),
            manifest_name: "worth-store-io-scheduler".to_owned(),
            dependency_kind: "normal".to_owned(),
            features: vec!["opaque-fixture-surface".to_owned()],
            optional: false,
            uses_default_features: true,
            target: None,
        }],
    };
    let test_authority_features = BTreeSet::from([(
        "worth-store-io-scheduler".to_owned(),
        "opaque-fixture-surface".to_owned(),
    )]);
    let violations = validate_build_graph_policy(&graph, &test_authority_features).unwrap_err();
    assert_eq!(violations[0].consumer, "worth-store-blob-chunks");

    let closure = OwnerBuildClosure {
        boundary: OwnerTestBoundary {
            owner_package: "worth-store-blob-chunks".to_owned(),
            admitted_direct_production_dependencies: BTreeSet::new(),
            observed_direct_test_dependencies: BTreeSet::from([
                "worth-store-certification".to_owned()
            ]),
            admitted_cross_owner_test_dependencies: BTreeSet::new(),
        },
        compiled_workspace_packages: BTreeSet::from([
            "worth-store-blob-chunks".to_owned(),
            "worth-store-certification".to_owned(),
        ]),
        activated_features: BTreeMap::new(),
        test_support_authority: None,
    };
    assert!(validate_owner_build_closures(&[closure]).is_err());
}
