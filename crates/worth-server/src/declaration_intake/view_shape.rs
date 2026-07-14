#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectViewShape {
    Detail,
    Table,
    Grouped,
}

impl WorthServerDirectViewShape {
    pub fn detail() -> Self {
        Self::Detail
    }

    pub fn table() -> Self {
        Self::Table
    }

    pub fn grouped() -> Self {
        Self::Grouped
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::Table => "table",
            Self::Grouped => "grouped",
        }
    }
}
