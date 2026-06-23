use super::registry::ForgeQueryGraphReadBypassClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadBypassResidueErrorKind {
    MissingRequiredField,
    CountExceedsCap,
    DuplicateClass,
    ResidueGrowth,
    ResidueContractChanged,
    ResidueCoverageShortfall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBypassResidueError {
    kind: ForgeQueryGraphReadBypassResidueErrorKind,
    class: Option<ForgeQueryGraphReadBypassClass>,
    field_name: Option<&'static str>,
    current_count: Option<usize>,
    expected_count: Option<usize>,
    message: String,
}

impl ForgeQueryGraphReadBypassResidueError {
    pub(crate) fn coverage_shortfall(
        class: ForgeQueryGraphReadBypassClass,
        current_count: usize,
        expected_count: usize,
    ) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::ResidueCoverageShortfall,
            class: Some(class),
            field_name: None,
            current_count: Some(current_count),
            expected_count: Some(expected_count),
            message: format!(
                "graph-read bypass residue class `{}` covers {} findings but report requires {}",
                class.as_str(),
                current_count,
                expected_count
            ),
        }
    }

    pub(crate) fn missing_required_field(
        class: ForgeQueryGraphReadBypassClass,
        field_name: &'static str,
    ) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::MissingRequiredField,
            class: Some(class),
            field_name: Some(field_name),
            current_count: None,
            expected_count: None,
            message: format!("graph-read bypass residue field `{field_name}` must not be empty"),
        }
    }

    pub(crate) fn count_exceeds_cap(
        class: ForgeQueryGraphReadBypassClass,
        current_count: usize,
        must_not_exceed_count: usize,
    ) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::CountExceedsCap,
            class: Some(class),
            field_name: None,
            current_count: Some(current_count),
            expected_count: Some(must_not_exceed_count),
            message: format!(
                "graph-read bypass residue class `{}` count {} exceeds cap {}",
                class.as_str(),
                current_count,
                must_not_exceed_count
            ),
        }
    }

    pub(crate) fn duplicate_class(class: ForgeQueryGraphReadBypassClass) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::DuplicateClass,
            class: Some(class),
            field_name: None,
            current_count: None,
            expected_count: None,
            message: format!(
                "duplicate graph-read bypass residue class `{}`",
                class.as_str()
            ),
        }
    }

    pub(crate) fn residue_growth(
        class: ForgeQueryGraphReadBypassClass,
        current_count: usize,
        previous_count: usize,
    ) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::ResidueGrowth,
            class: Some(class),
            field_name: None,
            current_count: Some(current_count),
            expected_count: Some(previous_count),
            message: format!(
                "graph-read bypass residue class `{}` grew from {} to {}",
                class.as_str(),
                previous_count,
                current_count
            ),
        }
    }

    pub(crate) fn contract_changed(class: ForgeQueryGraphReadBypassClass) -> Self {
        Self {
            kind: ForgeQueryGraphReadBypassResidueErrorKind::ResidueContractChanged,
            class: Some(class),
            field_name: None,
            current_count: None,
            expected_count: None,
            message: format!(
                "graph-read bypass residue class `{}` changed owner, introduction, cap, blocker, or removal trigger",
                class.as_str()
            ),
        }
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadBypassResidueErrorKind {
        &self.kind
    }

    pub fn class(&self) -> Option<ForgeQueryGraphReadBypassClass> {
        self.class
    }

    pub fn field_name(&self) -> Option<&'static str> {
        self.field_name
    }

    pub fn current_count(&self) -> Option<usize> {
        self.current_count
    }

    pub fn expected_count(&self) -> Option<usize> {
        self.expected_count
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ForgeQueryGraphReadBypassResidueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQueryGraphReadBypassResidueError {}
