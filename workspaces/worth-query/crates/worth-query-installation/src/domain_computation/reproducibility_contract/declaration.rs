#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactReproducibilityClass {
    ExactDeterministic,
    SeededDeterministic,
    CanonicalReduction,
    DomainComparator,
    IntervalOrErrorBound,
    Distributional,
    NonReplayable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactDeterminismPosture {
    Deterministic,
    SeededDeterministic,
    EnvironmentDependent,
    EntropyDependent,
    Nondeterministic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactComparisonAuthority {
    NotDeclared,
    ExactCanonicalValue,
    CanonicalReduction { family: String },
    RegisteredDomainComparator { family: String },
    RegisteredErrorBoundComparator { family: String },
    RegisteredDistributionTest { family: String },
    NotComparable,
}

impl WorthQueryArtifactComparisonAuthority {
    pub fn registered_family(&self) -> Option<&str> {
        match self {
            Self::CanonicalReduction { family }
            | Self::RegisteredDomainComparator { family }
            | Self::RegisteredErrorBoundComparator { family }
            | Self::RegisteredDistributionTest { family } => Some(family),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactReproducibilityContract {
    class: WorthQueryArtifactReproducibilityClass,
    determinism: WorthQueryArtifactDeterminismPosture,
    comparison: WorthQueryArtifactComparisonAuthority,
    environment_dependencies: Vec<String>,
    entropy_dependencies: Vec<String>,
}

impl WorthQueryArtifactReproducibilityContract {
    pub fn new(
        class: WorthQueryArtifactReproducibilityClass,
        determinism: WorthQueryArtifactDeterminismPosture,
        comparison: WorthQueryArtifactComparisonAuthority,
        environment_dependencies: impl IntoIterator<Item = impl Into<String>>,
        entropy_dependencies: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut contract = Self {
            class,
            determinism,
            comparison,
            environment_dependencies: environment_dependencies
                .into_iter()
                .map(Into::into)
                .collect(),
            entropy_dependencies: entropy_dependencies.into_iter().map(Into::into).collect(),
        };
        contract.canonicalize();
        contract
    }

    pub fn domain_comparator(
        family: impl Into<String>,
        environment_dependencies: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::new(
            WorthQueryArtifactReproducibilityClass::DomainComparator,
            WorthQueryArtifactDeterminismPosture::EnvironmentDependent,
            WorthQueryArtifactComparisonAuthority::RegisteredDomainComparator {
                family: family.into(),
            },
            environment_dependencies,
            std::iter::empty::<String>(),
        )
    }

    pub const fn class(&self) -> WorthQueryArtifactReproducibilityClass {
        self.class
    }

    pub const fn determinism(&self) -> WorthQueryArtifactDeterminismPosture {
        self.determinism
    }

    pub fn comparison(&self) -> &WorthQueryArtifactComparisonAuthority {
        &self.comparison
    }

    pub fn environment_dependencies(&self) -> &[String] {
        &self.environment_dependencies
    }

    pub fn entropy_dependencies(&self) -> &[String] {
        &self.entropy_dependencies
    }

    pub(crate) fn canonicalize(&mut self) {
        self.environment_dependencies.sort();
        self.environment_dependencies.dedup();
        self.entropy_dependencies.sort();
        self.entropy_dependencies.dedup();
    }
}
pub trait WorthQueryArtifactComparatorFamily: 'static {
    const SEMANTIC_FAMILY: &'static str;
}
