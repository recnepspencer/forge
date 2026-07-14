use super::registry::WorthQueryGraphReadBypassClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadBypassResidueErrorKind {
    MissingRequiredField,
    CountExceedsCap,
    DuplicateClass,
    ResidueGrowth,
    ResidueContractChanged,
    ResidueCoverageShortfall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassResidueError {
    kind: WorthQueryGraphReadBypassResidueErrorKind,
    class: Option<WorthQueryGraphReadBypassClass>,
    field_name: Option<&'static str>,
    current_count: Option<usize>,
    expected_count: Option<usize>,
    message: String,
}

impl WorthQueryGraphReadBypassResidueError {
    pub(crate) fn coverage_shortfall(
        class: WorthQueryGraphReadBypassClass,
        current_count: usize,
        expected_count: usize,
    ) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::ResidueCoverageShortfall,
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
        class: WorthQueryGraphReadBypassClass,
        field_name: &'static str,
    ) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::MissingRequiredField,
            class: Some(class),
            field_name: Some(field_name),
            current_count: None,
            expected_count: None,
            message: format!("graph-read bypass residue field `{field_name}` must not be empty"),
        }
    }

    pub(crate) fn count_exceeds_cap(
        class: WorthQueryGraphReadBypassClass,
        current_count: usize,
        must_not_exceed_count: usize,
    ) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::CountExceedsCap,
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

    pub(crate) fn duplicate_class(class: WorthQueryGraphReadBypassClass) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::DuplicateClass,
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
        class: WorthQueryGraphReadBypassClass,
        current_count: usize,
        previous_count: usize,
    ) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::ResidueGrowth,
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

    pub(crate) fn contract_changed(class: WorthQueryGraphReadBypassClass) -> Self {
        Self {
            kind: WorthQueryGraphReadBypassResidueErrorKind::ResidueContractChanged,
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

    pub fn kind(&self) -> &WorthQueryGraphReadBypassResidueErrorKind {
        &self.kind
    }

    pub fn class(&self) -> Option<WorthQueryGraphReadBypassClass> {
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

impl std::fmt::Display for WorthQueryGraphReadBypassResidueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for WorthQueryGraphReadBypassResidueError {}
