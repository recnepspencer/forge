/// Display policy for command-owned icon and label metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionIconLabelPolicy {
    PreferCommandIconAndLabel,
    LabelOnly,
    IconOnlyWhenPresent,
}

impl CommandProjectionIconLabelPolicy {
    pub fn digest_basis(self) -> &'static str {
        match self {
            Self::PreferCommandIconAndLabel => "prefer_command_icon_and_label",
            Self::LabelOnly => "label_only",
            Self::IconOnlyWhenPresent => "icon_only_when_present",
        }
    }
}
