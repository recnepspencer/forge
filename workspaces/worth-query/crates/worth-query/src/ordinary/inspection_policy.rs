#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WorthQueryOrdinaryInspectionPolicy {
    #[default]
    OperationalOnly,
    Rich,
}

impl WorthQueryOrdinaryInspectionPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperationalOnly => "operational-only",
            Self::Rich => "rich",
        }
    }

    pub(crate) fn materializes_rich_inspection(self) -> bool {
        self == Self::Rich
    }
}
