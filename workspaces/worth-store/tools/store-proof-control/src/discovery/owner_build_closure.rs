use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};

use super::owner_fixture_policy::{admitted_cross_owner_dependencies, OwnerFixtureDependency};
use super::{DependencyEdge, PackageSurface, TestSurfaceInventory};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerTestBoundary {
    pub owner_package: String,
    pub admitted_direct_production_dependencies: BTreeSet<String>,
    pub observed_direct_test_dependencies: BTreeSet<String>,
    pub admitted_cross_owner_test_dependencies: BTreeSet<OwnerFixtureDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerBuildClosure {
    pub boundary: OwnerTestBoundary,
    pub compiled_workspace_packages: BTreeSet<String>,
    pub activated_features: BTreeMap<String, BTreeSet<String>>,
    pub test_support_authority: Option<TestSupportAuthorityClass>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TestSupportAuthorityClass {
    CompilerBoundaryFixture,
    SharedPhysicalMechanic,
    SharedLayoutMechanic,
    CertificationWorld,
}

pub fn generate_owner_build_closures(inventory: &TestSurfaceInventory) -> Vec<OwnerBuildClosure> {
    let workspace_packages: BTreeSet<_> = inventory
        .packages
        .iter()
        .map(|package| package.name.as_str())
        .collect();
    let edges_by_consumer = edges_by_consumer(&inventory.build_graph.dependency_edges);
    let packages_by_name: BTreeMap<_, _> = inventory
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let mut closures: Vec<_> = inventory
        .packages
        .iter()
        .filter(|package| owner_lane_package(&package.name))
        .map(|package| {
            closure_for_owner(
                &package.name,
                &workspace_packages,
                &edges_by_consumer,
                &packages_by_name,
            )
        })
        .collect();
    closures.sort_by(|left, right| {
        left.boundary
            .owner_package
            .cmp(&right.boundary.owner_package)
    });
    closures
}

pub fn validate_owner_build_closures(closures: &[OwnerBuildClosure]) -> Result<(), Vec<String>> {
    let forbidden = [
        "worth-store-certification",
        "worth-store-physical-certification",
    ];
    let mut violations = Vec::new();
    for closure in closures {
        for package in forbidden {
            if closure.compiled_workspace_packages.contains(package) {
                violations.push(format!(
                    "owner lane {} reaches high-radius test package {}",
                    closure.boundary.owner_package, package
                ));
            }
        }
        if closure
            .activated_features
            .get("worth-store-test-support")
            .is_some_and(|features| features.contains("certification-world"))
        {
            violations.push(format!(
                "owner lane {} activates certification-world test support",
                closure.boundary.owner_package
            ));
        }
        for dependency in &closure.boundary.observed_direct_test_dependencies {
            let owner_local = !dependency.starts_with("worth-store-")
                || closure
                    .boundary
                    .admitted_direct_production_dependencies
                    .contains(dependency)
                || dependency == "worth-store-test-support";
            let explicitly_admitted = closure
                .boundary
                .admitted_cross_owner_test_dependencies
                .iter()
                .any(|admission| &admission.provider == dependency);
            if !owner_local && !explicitly_admitted {
                violations.push(format!(
                    "owner lane {} has an unadmitted cross-owner test dependency on {}",
                    closure.boundary.owner_package, dependency
                ));
            }
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn closure_for_owner(
    owner: &str,
    workspace_packages: &BTreeSet<&str>,
    edges_by_consumer: &BTreeMap<&str, Vec<&DependencyEdge>>,
    packages_by_name: &BTreeMap<&str, &PackageSurface>,
) -> OwnerBuildClosure {
    let owner_edges = edges_by_consumer.get(owner).cloned().unwrap_or_default();
    let direct_production_dependencies = dependencies_of_kind(&owner_edges, "normal");
    let direct_test_dependencies = dependencies_of_kind(&owner_edges, "dev");
    let mut compiled_workspace_packages = BTreeSet::from([owner.to_owned()]);
    let mut activated_features = BTreeMap::<String, BTreeSet<String>>::new();
    let mut pending = VecDeque::from([owner.to_owned()]);
    let mut observed_feature_state = BTreeMap::<String, BTreeSet<String>>::new();
    activate_default_feature(owner, packages_by_name, &mut activated_features);

    while let Some(package) = pending.pop_front() {
        let expanded = expanded_features(
            packages_by_name.get(package.as_str()).copied(),
            activated_features.get(&package),
        );
        if observed_feature_state.get(&package) == Some(&expanded) {
            continue;
        }
        observed_feature_state.insert(package.clone(), expanded.clone());
        for edge in edges_by_consumer
            .get(package.as_str())
            .into_iter()
            .flatten()
        {
            if package != owner && edge.dependency_kind == "dev" {
                continue;
            }
            if edge.optional && !optional_dependency_enabled(edge, &expanded) {
                continue;
            }
            record_edge(
                edge,
                &expanded,
                workspace_packages,
                packages_by_name,
                &mut compiled_workspace_packages,
                &mut activated_features,
                &mut pending,
            );
        }
    }
    let test_support_authority = test_support_authority(&activated_features);
    OwnerBuildClosure {
        boundary: OwnerTestBoundary {
            owner_package: owner.to_owned(),
            admitted_direct_production_dependencies: direct_production_dependencies,
            observed_direct_test_dependencies: direct_test_dependencies,
            admitted_cross_owner_test_dependencies: admitted_cross_owner_dependencies(owner),
        },
        compiled_workspace_packages,
        activated_features,
        test_support_authority,
    }
}

fn test_support_authority(
    activated: &BTreeMap<String, BTreeSet<String>>,
) -> Option<TestSupportAuthorityClass> {
    let features = activated.get("worth-store-test-support")?;
    if features.contains("certification-world") {
        Some(TestSupportAuthorityClass::CertificationWorld)
    } else if features.contains("layout-fixtures") {
        Some(TestSupportAuthorityClass::SharedLayoutMechanic)
    } else if features.contains("physical-isolation-fixtures") {
        Some(TestSupportAuthorityClass::SharedPhysicalMechanic)
    } else {
        Some(TestSupportAuthorityClass::CompilerBoundaryFixture)
    }
}

fn record_edge(
    edge: &DependencyEdge,
    consumer_features: &BTreeSet<String>,
    workspace_packages: &BTreeSet<&str>,
    packages_by_name: &BTreeMap<&str, &PackageSurface>,
    compiled: &mut BTreeSet<String>,
    activated_features: &mut BTreeMap<String, BTreeSet<String>>,
    pending: &mut VecDeque<String>,
) {
    if !workspace_packages.contains(edge.provider.as_str()) {
        return;
    }
    let provider_features = activated_features.entry(edge.provider.clone()).or_default();
    let before = provider_features.len();
    provider_features.extend(edge.features.iter().cloned());
    provider_features.extend(dependency_features(edge, consumer_features));
    if edge.uses_default_features
        && packages_by_name
            .get(edge.provider.as_str())
            .is_some_and(|package| package.feature_definitions.contains_key("default"))
    {
        provider_features.insert("default".to_owned());
    }
    let inserted = compiled.insert(edge.provider.clone());
    if inserted || provider_features.len() != before {
        pending.push_back(edge.provider.clone());
    }
}

fn activate_default_feature(
    package: &str,
    packages: &BTreeMap<&str, &PackageSurface>,
    activated: &mut BTreeMap<String, BTreeSet<String>>,
) {
    if packages
        .get(package)
        .is_some_and(|surface| surface.feature_definitions.contains_key("default"))
    {
        activated
            .entry(package.to_owned())
            .or_default()
            .insert("default".to_owned());
    }
}

fn expanded_features(
    package: Option<&PackageSurface>,
    active: Option<&BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut expanded = active.cloned().unwrap_or_default();
    let Some(package) = package else {
        return expanded;
    };
    let mut pending: VecDeque<_> = expanded.iter().cloned().collect();
    while let Some(feature) = pending.pop_front() {
        for member in package
            .feature_definitions
            .get(&feature)
            .into_iter()
            .flatten()
        {
            if expanded.insert(member.clone()) && package.feature_definitions.contains_key(member) {
                pending.push_back(member.clone());
            }
        }
    }
    expanded
}

fn optional_dependency_enabled(edge: &DependencyEdge, features: &BTreeSet<String>) -> bool {
    let alias = dependency_alias(edge);
    features.contains(&format!("dep:{alias}"))
        || features.contains(alias)
        || features.iter().any(|feature| {
            feature.starts_with(&format!("{alias}/")) || feature.starts_with(&format!("{alias}?/"))
        })
}

fn dependency_features(edge: &DependencyEdge, consumer_features: &BTreeSet<String>) -> Vec<String> {
    let alias = dependency_alias(edge);
    consumer_features
        .iter()
        .filter_map(|feature| {
            let plain = format!("{alias}/");
            let weak = format!("{alias}?/");
            feature
                .strip_prefix(&plain)
                .or_else(|| feature.strip_prefix(&weak))
                .map(str::to_owned)
        })
        .collect()
}

fn dependency_alias(edge: &DependencyEdge) -> &str {
    if edge.manifest_name.is_empty() {
        &edge.provider
    } else {
        &edge.manifest_name
    }
}

fn dependencies_of_kind(edges: &[&DependencyEdge], kind: &str) -> BTreeSet<String> {
    edges
        .iter()
        .filter(|edge| edge.dependency_kind == kind && !edge.optional)
        .map(|edge| edge.provider.clone())
        .collect()
}

fn edges_by_consumer(edges: &[DependencyEdge]) -> BTreeMap<&str, Vec<&DependencyEdge>> {
    let mut by_consumer = BTreeMap::<&str, Vec<&DependencyEdge>>::new();
    for edge in edges {
        by_consumer
            .entry(edge.consumer.as_str())
            .or_default()
            .push(edge);
    }
    by_consumer
}

fn owner_lane_package(package: &str) -> bool {
    !matches!(
        package,
        "worth-store-certification"
            | "worth-store-physical-certification"
            | "worth-store-test-support"
            | "store-proof-control"
    )
}
