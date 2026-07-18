use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::discovery::{ObservedBuildGraph, PackageSurface, TestSurfaceInventory};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureSemanticAuthority {
    pub schema_version: u32,
    pub declarations: Vec<FeatureSemanticDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureSemanticDeclaration {
    pub package: String,
    pub feature: String,
    pub authority: FeatureAuthorityClass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureAuthorityClass {
    Production,
    TestAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildGraphPolicyViolation {
    pub consumer: String,
    pub provider: String,
    pub feature: String,
    pub dependency_kind: String,
    pub reason: String,
}

impl std::fmt::Display for BuildGraphPolicyViolation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} enables {} feature {} through a {} dependency: {}",
            self.consumer, self.provider, self.feature, self.dependency_kind, self.reason
        )
    }
}

pub fn validate_build_graph_policy(
    graph: &ObservedBuildGraph,
    test_authority_features: &BTreeSet<(String, String)>,
) -> Result<(), Vec<BuildGraphPolicyViolation>> {
    let violations: Vec<_> = graph
        .dependency_edges
        .iter()
        .filter(|edge| edge.dependency_kind == "normal")
        .filter(|edge| !certification_consumer(&edge.consumer))
        .flat_map(|edge| {
            edge.features
                .iter()
                .filter(|feature| {
                    test_authority_features.contains(&(edge.provider.clone(), (*feature).clone()))
                })
                .map(|feature| BuildGraphPolicyViolation {
                    consumer: edge.consumer.clone(),
                    provider: edge.provider.clone(),
                    feature: feature.clone(),
                    dependency_kind: edge.dependency_kind.clone(),
                    reason: "normal production graphs may not activate certification authority"
                        .to_owned(),
                })
        })
        .collect();
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn validate_inventory_build_graph_policy(
    inventory: &TestSurfaceInventory,
) -> Result<(), Vec<BuildGraphPolicyViolation>> {
    let mut authority_violations = Vec::new();
    let test_authority_features =
        validate_feature_semantic_authority(inventory, &mut authority_violations);
    let packages: BTreeMap<_, _> = inventory
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect();
    let edges: BTreeMap<_, Vec<_>> = inventory
        .build_graph
        .dependency_edges
        .iter()
        .filter(|edge| edge.dependency_kind == "normal")
        .fold(BTreeMap::new(), |mut grouped, edge| {
            grouped
                .entry(edge.consumer.as_str())
                .or_insert_with(Vec::new)
                .push(edge);
            grouped
        });
    let mut violations = authority_violations;
    let mut seen_violations = BTreeSet::new();
    for root in inventory
        .packages
        .iter()
        .filter(|package| !certification_consumer(&package.name))
    {
        resolve_production_root(
            root,
            &packages,
            &edges,
            &test_authority_features,
            &mut seen_violations,
            &mut violations,
        );
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn resolve_production_root(
    root: &PackageSurface,
    packages: &BTreeMap<&str, &PackageSurface>,
    edges: &BTreeMap<&str, Vec<&crate::discovery::DependencyEdge>>,
    test_authority_features: &BTreeSet<(String, String)>,
    seen_violations: &mut BTreeSet<String>,
    violations: &mut Vec<BuildGraphPolicyViolation>,
) {
    let mut queue = VecDeque::from([(root.name.clone(), BTreeSet::from(["default".to_owned()]))]);
    let mut activated = BTreeMap::<String, BTreeSet<String>>::new();
    while let Some((package_name, requested)) = queue.pop_front() {
        let Some(package) = packages.get(package_name.as_str()) else {
            continue;
        };
        let expanded = expand_local_features(package, requested);
        let known = activated.entry(package_name.clone()).or_default();
        let delta: BTreeSet<_> = expanded.difference(known).cloned().collect();
        if delta.is_empty() {
            continue;
        }
        known.extend(delta);
        for edge in edges.get(package_name.as_str()).into_iter().flatten() {
            let alias = if edge.manifest_name.is_empty() {
                edge.provider.as_str()
            } else {
                edge.manifest_name.as_str()
            };
            let dependency_features = dependency_features(alias, edge, known);
            if dependency_features.is_none() {
                continue;
            }
            let dependency_features = dependency_features.expect("active dependency has features");
            let resolved_dependency_features = packages.get(edge.provider.as_str()).map_or_else(
                || dependency_features.clone(),
                |provider| expand_local_features(provider, dependency_features.clone()),
            );
            for feature in resolved_dependency_features.iter().filter(|feature| {
                test_authority_features.contains(&(edge.provider.clone(), (*feature).clone()))
            }) {
                record_violation(edge, feature, seen_violations, violations);
            }
            queue.push_back((edge.provider.clone(), resolved_dependency_features));
        }
    }
}

fn validate_feature_semantic_authority(
    inventory: &TestSurfaceInventory,
    violations: &mut Vec<BuildGraphPolicyViolation>,
) -> BTreeSet<(String, String)> {
    if inventory.feature_semantic_authority.schema_version != 1 {
        violations.push(feature_authority_violation(
            "feature-semantic-authority",
            "schema-version",
            "unsupported feature semantic authority schema",
        ));
    }
    let mut declared = BTreeMap::new();
    for declaration in &inventory.feature_semantic_authority.declarations {
        let key = (declaration.package.clone(), declaration.feature.clone());
        if declared
            .insert(key.clone(), declaration.authority)
            .is_some()
        {
            violations.push(feature_authority_violation(
                &key.0,
                &key.1,
                "feature semantic authority contains a duplicate declaration",
            ));
        }
    }
    let known: BTreeSet<_> = inventory
        .packages
        .iter()
        .flat_map(|package| {
            package
                .features
                .iter()
                .filter(|feature| feature.as_str() != "default")
                .map(|feature| (package.name.clone(), feature.clone()))
        })
        .collect();
    for missing in known
        .iter()
        .filter(|feature| !declared.contains_key(*feature))
    {
        violations.push(feature_authority_violation(
            &missing.0,
            &missing.1,
            "workspace feature has no reviewed production/test-authority classification",
        ));
    }
    for phantom in declared.keys().filter(|feature| !known.contains(*feature)) {
        violations.push(feature_authority_violation(
            &phantom.0,
            &phantom.1,
            "feature semantic authority names a feature absent from Cargo metadata",
        ));
    }
    declared
        .into_iter()
        .filter_map(|(identity, authority)| {
            (authority == FeatureAuthorityClass::TestAuthority).then_some(identity)
        })
        .collect()
}

fn feature_authority_violation(
    package: &str,
    feature: &str,
    reason: &str,
) -> BuildGraphPolicyViolation {
    BuildGraphPolicyViolation {
        consumer: "feature-semantic-authority".to_owned(),
        provider: package.to_owned(),
        feature: feature.to_owned(),
        dependency_kind: "classification".to_owned(),
        reason: reason.to_owned(),
    }
}

fn expand_local_features(
    package: &PackageSurface,
    requested: BTreeSet<String>,
) -> BTreeSet<String> {
    let mut expanded = requested;
    let mut pending: Vec<_> = expanded.iter().cloned().collect();
    while let Some(feature) = pending.pop() {
        for member in package
            .feature_definitions
            .get(&feature)
            .into_iter()
            .flatten()
        {
            if expanded.insert(member.clone()) && is_local_feature_member(member, package) {
                pending.push(member.clone());
            }
        }
    }
    expanded
}

fn is_local_feature_member(member: &str, package: &PackageSurface) -> bool {
    !member.starts_with("dep:")
        && !member.contains('/')
        && package.feature_definitions.contains_key(member)
}

fn dependency_features(
    alias: &str,
    edge: &crate::discovery::DependencyEdge,
    enabled_local_features: &BTreeSet<String>,
) -> Option<BTreeSet<String>> {
    let explicitly_activated = enabled_local_features.iter().any(|feature| {
        feature == &format!("dep:{alias}") || feature.starts_with(&format!("{alias}/"))
    });
    if edge.optional && !explicitly_activated {
        return None;
    }
    let mut features: BTreeSet<_> = edge.features.iter().cloned().collect();
    if edge.uses_default_features {
        features.insert("default".to_owned());
    }
    for feature in enabled_local_features {
        if let Some(dependency_feature) = feature.strip_prefix(&format!("{alias}/")) {
            features.insert(dependency_feature.to_owned());
        }
        if let Some(dependency_feature) = feature.strip_prefix(&format!("{alias}?/")) {
            features.insert(dependency_feature.to_owned());
        }
    }
    Some(features)
}

fn record_violation(
    edge: &crate::discovery::DependencyEdge,
    feature: &str,
    seen: &mut BTreeSet<String>,
    violations: &mut Vec<BuildGraphPolicyViolation>,
) {
    let identity = format!("{}::{}::{feature}", edge.consumer, edge.provider);
    if seen.insert(identity) {
        violations.push(BuildGraphPolicyViolation {
            consumer: edge.consumer.clone(),
            provider: edge.provider.clone(),
            feature: feature.to_owned(),
            dependency_kind: edge.dependency_kind.clone(),
            reason: "resolved production feature closure activates certification authority"
                .to_owned(),
        });
    }
}

fn certification_consumer(package: &str) -> bool {
    package == "worth-store-certification"
        || package == "worth-store-physical-certification"
        || package == "worth-store-test-support"
        || package == "store-proof-control"
}
