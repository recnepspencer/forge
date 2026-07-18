use serde::{Deserialize, Serialize};
use worth_store_test_support::structural_preflight::DependencyBoundaryPredicate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "policy", rename_all = "snake_case")]
pub enum BuildGraphPolicyViolation {
    FeatureSemanticAuthority(FeatureSemanticAuthorityViolation),
    DependencyBoundary(DependencyBoundaryViolation),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "subject", rename_all = "snake_case")]
pub enum FeatureSemanticAuthoritySubject {
    Schema { observed_version: u32 },
    Feature { package: String, feature: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureSemanticAuthorityDenial {
    UnsupportedSchema,
    DuplicateDeclaration,
    MissingDeclaration,
    PhantomDeclaration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureSemanticAuthorityViolation {
    pub subject: FeatureSemanticAuthoritySubject,
    pub denial: FeatureSemanticAuthorityDenial,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyBoundaryDenial {
    DirectProductionActivation,
    ResolvedProductionFeatureClosure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencyBoundaryViolation {
    pub predicate: DependencyBoundaryPredicate,
    pub dependency_kind: String,
    pub denial: DependencyBoundaryDenial,
}

impl std::fmt::Display for BuildGraphPolicyViolation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FeatureSemanticAuthority(violation) => violation.fmt(formatter),
            Self::DependencyBoundary(violation) => violation.fmt(formatter),
        }
    }
}

impl std::fmt::Display for FeatureSemanticAuthorityViolation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.subject, self.denial) {
            (
                FeatureSemanticAuthoritySubject::Schema { observed_version },
                FeatureSemanticAuthorityDenial::UnsupportedSchema,
            ) => write!(
                formatter,
                "feature semantic authority schema {observed_version} is unsupported (expected 1)"
            ),
            (FeatureSemanticAuthoritySubject::Feature { package, feature }, denial) => {
                let reason = match denial {
                    FeatureSemanticAuthorityDenial::DuplicateDeclaration => {
                        "has duplicate semantic declarations"
                    }
                    FeatureSemanticAuthorityDenial::MissingDeclaration => {
                        "has no reviewed production/test-authority classification"
                    }
                    FeatureSemanticAuthorityDenial::PhantomDeclaration => {
                        "is absent from Cargo metadata"
                    }
                    FeatureSemanticAuthorityDenial::UnsupportedSchema => {
                        "cannot carry a schema denial"
                    }
                };
                write!(formatter, "feature {package}/{feature} {reason}")
            }
            (FeatureSemanticAuthoritySubject::Schema { .. }, denial) => write!(
                formatter,
                "feature semantic authority schema has invalid denial {denial:?}"
            ),
        }
    }
}

impl std::fmt::Display for DependencyBoundaryViolation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self.denial {
            DependencyBoundaryDenial::DirectProductionActivation => {
                "normal production graphs may not activate certification authority"
            }
            DependencyBoundaryDenial::ResolvedProductionFeatureClosure => {
                "resolved production feature closure activates certification authority"
            }
        };
        write!(
            formatter,
            "{:?} through a {} dependency: {reason}",
            self.predicate, self.dependency_kind
        )
    }
}
