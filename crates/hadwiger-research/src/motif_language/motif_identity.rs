use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MotifVertex {
    label: String,
}

impl MotifVertex {
    pub fn new(label: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            label: require_non_empty(label, "vertex_label")?,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MotifTerminal {
    label: String,
}

impl MotifTerminal {
    pub fn new(label: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            label: require_non_empty(label, "terminal_label")?,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MotifUnitEdge {
    left_label: String,
    right_label: String,
}

impl MotifUnitEdge {
    pub fn new(
        left_label: impl Into<String>,
        right_label: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        normalized_pair(left_label, right_label, "unit_edge").map(|(left_label, right_label)| {
            Self {
                left_label,
                right_label,
            }
        })
    }

    pub fn stable_token(&self) -> String {
        format!("{}:{}", self.left_label, self.right_label)
    }

    pub fn left_label(&self) -> &str {
        &self.left_label
    }

    pub fn right_label(&self) -> &str {
        &self.right_label
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MotifForbiddenSameColorPair {
    left_label: String,
    right_label: String,
}

impl MotifForbiddenSameColorPair {
    pub fn new(
        left_label: impl Into<String>,
        right_label: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        normalized_pair(left_label, right_label, "forbidden_same_color_pair").map(
            |(left_label, right_label)| Self {
                left_label,
                right_label,
            },
        )
    }

    pub fn left_label(&self) -> &str {
        &self.left_label
    }

    pub fn right_label(&self) -> &str {
        &self.right_label
    }

    pub fn stable_token(&self) -> String {
        format!("{}:{}", self.left_label, self.right_label)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MotifParameterBinding {
    name: String,
    value: String,
}

impl MotifParameterBinding {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            name: require_non_empty(name, "parameter_name")?,
            value: require_non_empty(value, "parameter_value")?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stable_token(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

fn normalized_pair(
    left_label: impl Into<String>,
    right_label: impl Into<String>,
    field: &'static str,
) -> Result<(String, String), HadwigerArtifactShapeError> {
    let left_label = require_non_empty(left_label, "left_label")?;
    let right_label = require_non_empty(right_label, "right_label")?;
    if left_label == right_label {
        return Err(HadwigerArtifactShapeError::SelfEdge {
            vertex_label: field.to_string(),
        });
    }
    if left_label <= right_label {
        Ok((left_label, right_label))
    } else {
        Ok((right_label, left_label))
    }
}
