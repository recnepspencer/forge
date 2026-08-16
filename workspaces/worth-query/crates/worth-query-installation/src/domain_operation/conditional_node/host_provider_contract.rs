use worth_foundational::facade::{
    AspectMask, AspectValue, ContractValidatedAspectArtifact, ContractValidatedAspectValueView,
    FieldKey, ProjectionMask,
};

#[derive(Clone, Copy, Debug)]
pub struct WorthQueryConditionalProjectedValue<'a> {
    artifact: &'a ContractValidatedAspectArtifact,
    mask: &'a AspectMask<ProjectionMask>,
}

impl<'a> WorthQueryConditionalProjectedValue<'a> {
    #[doc(hidden)]
    pub fn from_runtime_projection(
        artifact: &'a ContractValidatedAspectArtifact,
        mask: &'a AspectMask<ProjectionMask>,
    ) -> Self {
        Self { artifact, mask }
    }

    pub fn scalar(self) -> Option<&'a AspectValue> {
        if !self.mask.is_whole_aspect() {
            return None;
        }
        match self.artifact.payload().view() {
            ContractValidatedAspectValueView::Scalar(value) => Some(value),
            ContractValidatedAspectValueView::Struct(_) => None,
        }
    }

    pub fn field(self, field: &FieldKey) -> Option<&'a AspectValue> {
        let admitted = self.mask.is_whole_aspect()
            || self
                .mask
                .paths()
                .iter()
                .any(|path| path.fields().len() == 1 && path.fields().first() == Some(field));
        if !admitted {
            return None;
        }
        match self.artifact.payload().view() {
            ContractValidatedAspectValueView::Struct(value) => value.get(field),
            ContractValidatedAspectValueView::Scalar(_) => None,
        }
    }
}

/// One declared dependency value visible to a host conditional provider.
///
/// Absence is explicit so a provider cannot confuse an unavailable dependency
/// with a present value whose payload happens to be empty.
#[derive(Clone, Copy, Debug)]
pub enum WorthQueryConditionalObservedValue<'a> {
    Present(WorthQueryConditionalProjectedValue<'a>),
    Absent,
}

/// Descriptive truth basis shared by every dependency in one evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalObservationTruthBasis<'a> {
    branch_identity: &'a str,
    snapshot_identity: &'a str,
}

impl<'a> WorthQueryConditionalObservationTruthBasis<'a> {
    #[doc(hidden)]
    pub fn from_runtime_truth(branch_identity: &'a str, snapshot_identity: &'a str) -> Self {
        Self {
            branch_identity,
            snapshot_identity,
        }
    }

    pub fn branch_identity(self) -> &'a str {
        self.branch_identity
    }

    pub fn snapshot_identity(self) -> &'a str {
        self.snapshot_identity
    }
}

/// Previous and current values for one dependency declaration ordinal.
#[derive(Clone, Copy, Debug)]
pub struct WorthQueryConditionalDependencyObservation<'a> {
    declaration_ordinal: usize,
    previous: WorthQueryConditionalObservedValue<'a>,
    current: WorthQueryConditionalObservedValue<'a>,
}

impl<'a> WorthQueryConditionalDependencyObservation<'a> {
    #[doc(hidden)]
    pub fn from_runtime_observation(
        declaration_ordinal: usize,
        previous: WorthQueryConditionalObservedValue<'a>,
        current: WorthQueryConditionalObservedValue<'a>,
    ) -> Self {
        Self {
            declaration_ordinal,
            previous,
            current,
        }
    }

    pub fn declaration_ordinal(self) -> usize {
        self.declaration_ordinal
    }

    pub fn previous(self) -> WorthQueryConditionalObservedValue<'a> {
        self.previous
    }

    pub fn current(self) -> WorthQueryConditionalObservedValue<'a> {
        self.current
    }
}

/// Immutable, dependency-indexed view supplied to one admitted host provider.
#[derive(Clone, Copy, Debug)]
pub struct WorthQueryConditionalObservationView<'a> {
    basis: WorthQueryConditionalObservationTruthBasis<'a>,
    dependencies: &'a [WorthQueryConditionalDependencyObservation<'a>],
}

impl<'a> WorthQueryConditionalObservationView<'a> {
    #[doc(hidden)]
    pub fn from_runtime_observations(
        basis: WorthQueryConditionalObservationTruthBasis<'a>,
        dependencies: &'a [WorthQueryConditionalDependencyObservation<'a>],
    ) -> Self {
        Self {
            basis,
            dependencies,
        }
    }

    pub fn basis(self) -> WorthQueryConditionalObservationTruthBasis<'a> {
        self.basis
    }

    pub fn dependencies(self) -> &'a [WorthQueryConditionalDependencyObservation<'a>] {
        self.dependencies
    }

    pub fn dependency(
        self,
        declaration_ordinal: usize,
    ) -> Option<WorthQueryConditionalDependencyObservation<'a>> {
        self.dependencies
            .iter()
            .copied()
            .find(|observation| observation.declaration_ordinal == declaration_ordinal)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryHostPredicateDecision {
    Satisfied,
    Unsatisfied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryHostPredicateFailureKind {
    ObservationUnsupported,
    ProviderUnavailable,
    ProviderFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryHostPredicateFailure {
    kind: WorthQueryHostPredicateFailureKind,
    detail: String,
}

impl WorthQueryHostPredicateFailure {
    pub fn new(kind: WorthQueryHostPredicateFailureKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryHostPredicateFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Host implementation of one typed conditional-node predicate.
///
/// The provider returns domain truth only. Query adapts that truth into the
/// installed Runtime Bridge contract; neither raw Signal eligibility nor wake
/// authority can cross this boundary.
pub trait WorthQueryHostConditionalPredicateProvider<Node>: Send + Sync + 'static {
    const SEMANTIC_IDENTITY: &'static str;

    fn evaluate(
        &self,
        observation: WorthQueryConditionalObservationView<'_>,
    ) -> Result<WorthQueryHostPredicateDecision, WorthQueryHostPredicateFailure>;
}
