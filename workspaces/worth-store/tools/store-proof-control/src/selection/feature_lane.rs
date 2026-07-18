use serde::{Deserialize, Serialize};

use crate::discovery::TestTargetIdentity;
use crate::ValidatedProofInventory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StoreFeatureLane {
    ProductionEquivalent,
    DeclaredProofBoundary {
        cargo_features: Vec<String>,
        dependency_features: Vec<String>,
    },
}

impl StoreFeatureLane {
    pub(crate) fn from_required_features(features: &[String]) -> Self {
        if features.is_empty() {
            Self::ProductionEquivalent
        } else {
            let mut features = features.to_vec();
            features.sort();
            features.dedup();
            Self::DeclaredProofBoundary {
                cargo_features: features,
                dependency_features: Vec::new(),
            }
        }
    }

    pub(crate) fn declared(features: Vec<String>) -> Self {
        Self::from_required_features(&features)
    }

    pub(crate) fn from_target_graph(
        cargo_features: &[String],
        dependency_features: Vec<String>,
    ) -> Self {
        let mut cargo_features = cargo_features.to_vec();
        cargo_features.sort();
        cargo_features.dedup();
        let mut dependency_features = dependency_features;
        dependency_features.sort();
        dependency_features.dedup();
        if cargo_features.is_empty() && dependency_features.is_empty() {
            Self::ProductionEquivalent
        } else {
            Self::DeclaredProofBoundary {
                cargo_features,
                dependency_features,
            }
        }
    }

    pub fn cargo_features(&self) -> &[String] {
        match self {
            Self::ProductionEquivalent => &[],
            Self::DeclaredProofBoundary { cargo_features, .. } => cargo_features,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::ProductionEquivalent => "production-equivalent".to_owned(),
            Self::DeclaredProofBoundary {
                cargo_features,
                dependency_features,
            } => format!(
                "target=[{}]; dependencies=[{}]",
                cargo_features.join(","),
                dependency_features.join(",")
            ),
        }
    }
}

pub(crate) fn feature_lane_for_target(
    inventory: &ValidatedProofInventory,
    target: &TestTargetIdentity,
) -> StoreFeatureLane {
    let dependency_features = inventory
        .inventory()
        .discovered
        .build_graph
        .dependency_edges
        .iter()
        .filter(|edge| edge.consumer == target.package)
        .filter(|edge| {
            matches!(edge.dependency_kind.as_str(), "dev" | "build")
                || edge.features.iter().any(|feature| {
                    feature.contains("certification") || feature.contains("test-authority")
                })
        })
        .flat_map(|edge| {
            edge.features
                .iter()
                .map(|feature| format!("{}:{}/{feature}", edge.dependency_kind, edge.provider))
        })
        .collect();
    StoreFeatureLane::from_target_graph(&target.required_features, dependency_features)
}
