#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentIconPaintCommand {
    Plus,
    Check,
    Search,
    Info,
    Warning,
    NamedSymbol,
}

impl WorthUiPrimitiveContentIconPaintCommand {
    pub fn from_source_key(source_key: &str) -> Self {
        match source_key {
            "plus" => Self::Plus,
            "check" => Self::Check,
            "search" | "search-icon" => Self::Search,
            "info" => Self::Info,
            "warning" => Self::Warning,
            _ => Self::NamedSymbol,
        }
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::Plus => "plus",
            Self::Check => "check",
            Self::Search => "search",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::NamedSymbol => "named_symbol",
        }
    }
}
