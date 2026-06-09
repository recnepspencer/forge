use super::request_types::{require_non_empty, HadwigerResearchDeclarationShapeError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactUnitDistanceConflictScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SameColorSeparationScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileDiameterScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactConflictGraphScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumericalMarginScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinkowskiDifferenceScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenDisplacementScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientGraphScreeningDeclaration {
    subject_reference: String,
    model_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceEmbeddabilityScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RigidityRealizationScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactArithmeticIntervalScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymmetryOrbitReductionScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExhaustiveLocalNeighborhoodScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownObstructionContainmentScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateNoveltyScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryOwnershipScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonodromyColorHolonomyScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationRotationClosureScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubstitutionConsistencyScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinitePatchBoundaryExtensionScreeningDeclaration {
    subject_reference: String,
    certificate_reference: String,
}

macro_rules! subject_certificate_declaration {
    ($type:ident) => {
        impl $type {
            pub fn new(
                subject_reference: impl Into<String>,
                certificate_reference: impl Into<String>,
            ) -> Self {
                Self::try_new(subject_reference, certificate_reference)
                    .expect("subject_reference and certificate_reference must be non-empty")
            }

            pub fn try_new(
                subject_reference: impl Into<String>,
                certificate_reference: impl Into<String>,
            ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
                Ok(Self {
                    subject_reference: require_non_empty(subject_reference, "subject_reference")?,
                    certificate_reference: require_non_empty(
                        certificate_reference,
                        "certificate_reference",
                    )?,
                })
            }

            pub(crate) fn subject_reference(&self) -> &str {
                &self.subject_reference
            }

            pub(crate) fn certificate_reference(&self) -> &str {
                &self.certificate_reference
            }
        }
    };
}

subject_certificate_declaration!(ExactUnitDistanceConflictScreeningDeclaration);
subject_certificate_declaration!(SameColorSeparationScreeningDeclaration);
subject_certificate_declaration!(TileDiameterScreeningDeclaration);
subject_certificate_declaration!(ExactConflictGraphScreeningDeclaration);
subject_certificate_declaration!(NumericalMarginScreeningDeclaration);
subject_certificate_declaration!(MinkowskiDifferenceScreeningDeclaration);
subject_certificate_declaration!(ForbiddenDisplacementScreeningDeclaration);
subject_certificate_declaration!(UnitDistanceEmbeddabilityScreeningDeclaration);
subject_certificate_declaration!(RigidityRealizationScreeningDeclaration);
subject_certificate_declaration!(ExactArithmeticIntervalScreeningDeclaration);
subject_certificate_declaration!(SymmetryOrbitReductionScreeningDeclaration);
subject_certificate_declaration!(ExhaustiveLocalNeighborhoodScreeningDeclaration);
subject_certificate_declaration!(KnownObstructionContainmentScreeningDeclaration);
subject_certificate_declaration!(CandidateNoveltyScreeningDeclaration);
subject_certificate_declaration!(BoundaryOwnershipScreeningDeclaration);
subject_certificate_declaration!(MonodromyColorHolonomyScreeningDeclaration);
subject_certificate_declaration!(TranslationRotationClosureScreeningDeclaration);
subject_certificate_declaration!(SubstitutionConsistencyScreeningDeclaration);
subject_certificate_declaration!(FinitePatchBoundaryExtensionScreeningDeclaration);

impl PeriodicQuotientGraphScreeningDeclaration {
    pub fn new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        certificate_reference: impl Into<String>,
    ) -> Self {
        Self::try_new(subject_reference, model_reference, certificate_reference).expect(
            "subject_reference, model_reference, and certificate_reference must be non-empty",
        )
    }

    pub fn try_new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        certificate_reference: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            subject_reference: require_non_empty(subject_reference, "subject_reference")?,
            model_reference: require_non_empty(model_reference, "model_reference")?,
            certificate_reference: require_non_empty(
                certificate_reference,
                "certificate_reference",
            )?,
        })
    }

    pub(crate) fn subject_reference(&self) -> &str {
        &self.subject_reference
    }

    pub(crate) fn model_reference(&self) -> &str {
        &self.model_reference
    }

    pub(crate) fn certificate_reference(&self) -> &str {
        &self.certificate_reference
    }
}
