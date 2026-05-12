use forge_proof::TransitionOutcome;

use crate::aspects::{
    AuthoritativeStateAdmissionDenial, ContractValidationDenial, FieldKey,
    StructAspectValueConstructionDenial,
};
use crate::locators::BoundarySourceLocator;
use crate::values::ScalarAspectType;

pub type JsonCompatibilityLoweringOutcome<S> = TransitionOutcome<
    S,
    JsonCompatibilityLoweringDenial,
    JsonCompatibilityLoweringDeferred,
    JsonCompatibilityLoweringStale,
    JsonCompatibilityRebindRequired,
    JsonCompatibilityLoweringFailure,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonCompatibilityLoweringDenial {
    JsonShapeNotAdmitted {
        source: BoundarySourceLocator,
        expected: &'static str,
    },
    InvalidFieldKey {
        source: BoundarySourceLocator,
        field: String,
    },
    UnknownStructField {
        source: BoundarySourceLocator,
        field: FieldKey,
    },
    AmbiguousNumericWidth {
        source: BoundarySourceLocator,
        expected: ScalarAspectType,
    },
    UnsupportedRecursiveDocument {
        source: BoundarySourceLocator,
        expected: ScalarAspectType,
    },
    UnsupportedScalarFamily {
        source: BoundarySourceLocator,
        expected: ScalarAspectType,
    },
    StructConstructionDenied {
        source: BoundarySourceLocator,
        denial: StructAspectValueConstructionDenial,
    },
    ContractValidationDenied {
        source: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    StateAdmissionDenied(AuthoritativeStateAdmissionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonCompatibilityLoweringDeferred {
    source: BoundarySourceLocator,
    reason: &'static str,
}

impl JsonCompatibilityLoweringDeferred {
    pub const fn new(source: BoundarySourceLocator, reason: &'static str) -> Self {
        Self { source, reason }
    }

    pub const fn source(&self) -> &BoundarySourceLocator {
        &self.source
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonCompatibilityLoweringStale {
    source: BoundarySourceLocator,
    reason: &'static str,
}

impl JsonCompatibilityLoweringStale {
    pub const fn new(source: BoundarySourceLocator, reason: &'static str) -> Self {
        Self { source, reason }
    }

    pub const fn source(&self) -> &BoundarySourceLocator {
        &self.source
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonCompatibilityRebindRequired {
    source: BoundarySourceLocator,
    reason: &'static str,
}

impl JsonCompatibilityRebindRequired {
    pub const fn new(source: BoundarySourceLocator, reason: &'static str) -> Self {
        Self { source, reason }
    }

    pub const fn source(&self) -> &BoundarySourceLocator {
        &self.source
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonCompatibilityLoweringFailure {
    source: BoundarySourceLocator,
    reason: &'static str,
}

impl JsonCompatibilityLoweringFailure {
    pub const fn new(source: BoundarySourceLocator, reason: &'static str) -> Self {
        Self { source, reason }
    }

    pub const fn source(&self) -> &BoundarySourceLocator {
        &self.source
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}
