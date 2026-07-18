use worth_store_test_support::structural_preflight::DependencyBoundaryPredicate;

use crate::discovery::ObservedBuildGraph;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyBoundaryViolation {
    pub predicate: DependencyBoundaryPredicate,
    pub source_package: String,
    pub forbidden_dependency: String,
    pub feature: Option<String>,
    pub dependency_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyBoundaryCheckFailure {
    Violation(DependencyBoundaryViolation),
    RequiresSourceBoundaryChecker(DependencyBoundaryPredicate),
}

pub fn evaluate_manifest_predicate(
    graph: &ObservedBuildGraph,
    predicate: &DependencyBoundaryPredicate,
) -> Result<(), DependencyBoundaryCheckFailure> {
    if matches!(predicate, DependencyBoundaryPredicate::SourceBoundary { .. }) {
        return Err(DependencyBoundaryCheckFailure::RequiresSourceBoundaryChecker(
            predicate.clone(),
        ));
    }
    let matched = graph.dependency_edges.iter().find(|edge| match predicate {
        DependencyBoundaryPredicate::ManifestDependencyDirection {
            source_package,
            forbidden_dependency,
        } => edge.consumer == *source_package && edge.provider == *forbidden_dependency,
        DependencyBoundaryPredicate::ForbiddenFeatureEdge {
            source_package,
            feature,
            forbidden_dependency,
        } => {
            edge.consumer == *source_package
                && edge.provider == *forbidden_dependency
                && edge.features.contains(feature)
        }
        DependencyBoundaryPredicate::SourceBoundary { .. } => false,
    });
    let Some(edge) = matched else {
        return Ok(());
    };
    let feature = match predicate {
        DependencyBoundaryPredicate::ForbiddenFeatureEdge { feature, .. } => {
            Some(feature.clone())
        }
        _ => None,
    };
    Err(DependencyBoundaryCheckFailure::Violation(DependencyBoundaryViolation {
        predicate: predicate.clone(),
        source_package: edge.consumer.clone(),
        forbidden_dependency: edge.provider.clone(),
        feature,
        dependency_kind: edge.dependency_kind.clone(),
    }))
}

#[cfg(test)]
mod tests {
    use crate::discovery::{DependencyEdge, ObservedBuildGraph};

    use super::*;

    #[test]
    fn metadata_boundary_localizes_the_manifest_edge_without_a_compile_proxy() {
        let graph = ObservedBuildGraph {
            dependency_edges: vec![DependencyEdge {
                consumer: "worth-store-authority".to_owned(),
                provider: "worth-store-certification".to_owned(),
                manifest_name: "worth-store-certification".to_owned(),
                dependency_kind: "normal".to_owned(),
                features: Vec::new(),
                optional: false,
                uses_default_features: true,
                target: None,
            }],
        };
        let predicate = DependencyBoundaryPredicate::ManifestDependencyDirection {
            source_package: "worth-store-authority".to_owned(),
            forbidden_dependency: "worth-store-certification".to_owned(),
        };

        let failure = evaluate_manifest_predicate(&graph, &predicate).unwrap_err();
        let DependencyBoundaryCheckFailure::Violation(violation) = failure else {
            panic!("manifest predicate was routed to the wrong checker");
        };

        assert_eq!(violation.source_package, "worth-store-authority");
        assert_eq!(violation.forbidden_dependency, "worth-store-certification");
        assert_eq!(violation.dependency_kind, "normal");
        assert_eq!(violation.predicate, predicate);
    }

    #[test]
    fn source_boundary_predicate_cannot_silently_pass_through_metadata_checker() {
        let predicate = DependencyBoundaryPredicate::SourceBoundary {
            source_scope: "worth-store-authority".to_owned(),
            forbidden_import: "worth_store_certification".to_owned(),
        };

        assert_eq!(
            evaluate_manifest_predicate(
                &ObservedBuildGraph {
                    dependency_edges: Vec::new(),
                },
                &predicate,
            ),
            Err(DependencyBoundaryCheckFailure::RequiresSourceBoundaryChecker(
                predicate
            ))
        );
    }
}
