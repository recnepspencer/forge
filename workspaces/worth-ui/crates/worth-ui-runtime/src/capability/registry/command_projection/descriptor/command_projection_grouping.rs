/// Declarative grouping for a command projection lane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandProjectionGrouping {
    group_key: String,
    required: bool,
}

impl CommandProjectionGrouping {
    pub fn optional(group_key: impl Into<String>) -> Self {
        Self {
            group_key: group_key.into(),
            required: false,
        }
    }

    pub fn required(group_key: impl Into<String>) -> Self {
        Self {
            group_key: group_key.into(),
            required: true,
        }
    }

    pub fn group_key(&self) -> &str {
        &self.group_key
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub(crate) fn is_missing_group_key(&self) -> bool {
        self.group_key.trim().is_empty()
    }

    pub(crate) fn conflicts_with(&self, other: &Self) -> bool {
        self.required && other.required && self.group_key != other.group_key
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("{}|{}", self.group_key, self.required)
    }
}
