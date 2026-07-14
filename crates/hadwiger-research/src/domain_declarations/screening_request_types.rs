use super::request_types::{
    require_color_count, require_non_empty, HadwigerResearchDeclarationShapeError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FractionalChromaticScreeningDeclaration {
    graph_version_reference: String,
    color_limit: u32,
    screening_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometricFractionalChromaticScreeningDeclaration {
    graph_version_reference: String,
    target_lower_bound: String,
    screening_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LovaszThetaScreeningDeclaration {
    graph_version_reference: String,
    color_limit: u32,
    screening_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutocorrelationZeroScreeningDeclaration {
    subject_reference: String,
    model_reference: String,
    screening_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DensityCapScreeningDeclaration {
    subject_reference: String,
    model_reference: String,
    color_id: String,
    retained_cap_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDensityWindowScreeningDeclaration {
    subject_reference: String,
    model_reference: String,
    window_reference: String,
    color_id: String,
    retained_bound_reference: String,
}

macro_rules! graph_screening_declaration {
    ($type:ident) => {
        impl $type {
            pub fn new(
                graph_version_reference: impl Into<String>,
                color_limit: u32,
                screening_basis: impl Into<String>,
            ) -> Self {
                Self::try_new(graph_version_reference, color_limit, screening_basis).expect(
                    "graph_version_reference and screening_basis must be non-empty and color_limit must be greater than zero",
                )
            }

            pub fn try_new(
                graph_version_reference: impl Into<String>,
                color_limit: u32,
                screening_basis: impl Into<String>,
            ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
                Ok(Self {
                    graph_version_reference: require_non_empty(
                        graph_version_reference,
                        "graph_version_reference",
                    )?,
                    color_limit: require_color_count(color_limit, "color_limit")?,
                    screening_basis: require_non_empty(screening_basis, "screening_basis")?,
                })
            }

            pub(crate) fn graph_version_reference(&self) -> &str {
                &self.graph_version_reference
            }

            pub(crate) fn color_limit(&self) -> u32 {
                self.color_limit
            }

            pub(crate) fn screening_basis(&self) -> &str {
                &self.screening_basis
            }
        }
    };
}

graph_screening_declaration!(FractionalChromaticScreeningDeclaration);
graph_screening_declaration!(LovaszThetaScreeningDeclaration);

impl GeometricFractionalChromaticScreeningDeclaration {
    pub fn new(
        graph_version_reference: impl Into<String>,
        target_lower_bound: impl Into<String>,
        screening_basis: impl Into<String>,
    ) -> Self {
        Self::try_new(graph_version_reference, target_lower_bound, screening_basis).expect(
            "graph_version_reference, target_lower_bound, and screening_basis must be non-empty",
        )
    }

    pub fn try_new(
        graph_version_reference: impl Into<String>,
        target_lower_bound: impl Into<String>,
        screening_basis: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_version_reference: require_non_empty(
                graph_version_reference,
                "graph_version_reference",
            )?,
            target_lower_bound: require_non_empty(target_lower_bound, "target_lower_bound")?,
            screening_basis: require_non_empty(screening_basis, "screening_basis")?,
        })
    }

    pub(crate) fn graph_version_reference(&self) -> &str {
        &self.graph_version_reference
    }

    pub(crate) fn target_lower_bound(&self) -> &str {
        &self.target_lower_bound
    }

    pub(crate) fn screening_basis(&self) -> &str {
        &self.screening_basis
    }
}

impl AutocorrelationZeroScreeningDeclaration {
    pub fn new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        screening_basis: impl Into<String>,
    ) -> Self {
        Self::try_new(subject_reference, model_reference, screening_basis)
            .expect("subject_reference, model_reference, and screening_basis must be non-empty")
    }

    pub fn try_new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        screening_basis: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            subject_reference: require_non_empty(subject_reference, "subject_reference")?,
            model_reference: require_non_empty(model_reference, "model_reference")?,
            screening_basis: require_non_empty(screening_basis, "screening_basis")?,
        })
    }

    pub(crate) fn subject_reference(&self) -> &str {
        &self.subject_reference
    }

    pub(crate) fn model_reference(&self) -> &str {
        &self.model_reference
    }

    pub(crate) fn screening_basis(&self) -> &str {
        &self.screening_basis
    }
}

impl DensityCapScreeningDeclaration {
    pub fn new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        color_id: impl Into<String>,
        retained_cap_reference: impl Into<String>,
    ) -> Self {
        Self::try_new(
            subject_reference,
            model_reference,
            color_id,
            retained_cap_reference,
        )
        .expect("subject_reference, model_reference, color_id, and retained_cap_reference must be non-empty")
    }

    pub fn try_new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        color_id: impl Into<String>,
        retained_cap_reference: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            subject_reference: require_non_empty(subject_reference, "subject_reference")?,
            model_reference: require_non_empty(model_reference, "model_reference")?,
            color_id: require_non_empty(color_id, "color_id")?,
            retained_cap_reference: require_non_empty(
                retained_cap_reference,
                "retained_cap_reference",
            )?,
        })
    }

    pub(crate) fn subject_reference(&self) -> &str {
        &self.subject_reference
    }

    pub(crate) fn model_reference(&self) -> &str {
        &self.model_reference
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn retained_cap_reference(&self) -> &str {
        &self.retained_cap_reference
    }
}

impl LocalDensityWindowScreeningDeclaration {
    pub fn new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        window_reference: impl Into<String>,
        color_id: impl Into<String>,
        retained_bound_reference: impl Into<String>,
    ) -> Self {
        Self::try_new(
            subject_reference,
            model_reference,
            window_reference,
            color_id,
            retained_bound_reference,
        )
        .expect("subject_reference, model_reference, window_reference, color_id, and retained_bound_reference must be non-empty")
    }

    pub fn try_new(
        subject_reference: impl Into<String>,
        model_reference: impl Into<String>,
        window_reference: impl Into<String>,
        color_id: impl Into<String>,
        retained_bound_reference: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            subject_reference: require_non_empty(subject_reference, "subject_reference")?,
            model_reference: require_non_empty(model_reference, "model_reference")?,
            window_reference: require_non_empty(window_reference, "window_reference")?,
            color_id: require_non_empty(color_id, "color_id")?,
            retained_bound_reference: require_non_empty(
                retained_bound_reference,
                "retained_bound_reference",
            )?,
        })
    }

    pub(crate) fn subject_reference(&self) -> &str {
        &self.subject_reference
    }

    pub(crate) fn model_reference(&self) -> &str {
        &self.model_reference
    }

    pub(crate) fn window_reference(&self) -> &str {
        &self.window_reference
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn retained_bound_reference(&self) -> &str {
        &self.retained_bound_reference
    }
}
