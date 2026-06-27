use core::fmt;

/// Structured rejection for invalid capability identity text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityIdError {
    Empty,
    EmptySegment { byte_index: usize },
    InvalidSegmentStart { byte_index: usize, found: char },
    InvalidSegmentCharacter { byte_index: usize, found: char },
}

impl fmt::Display for CapabilityIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("capability id text is empty"),
            Self::EmptySegment { byte_index } => {
                write!(
                    formatter,
                    "capability id has an empty segment at byte {byte_index}"
                )
            }
            Self::InvalidSegmentStart { byte_index, found } => write!(
                formatter,
                "capability id segment starts with invalid character '{found}' at byte {byte_index}"
            ),
            Self::InvalidSegmentCharacter { byte_index, found } => write!(
                formatter,
                "capability id segment contains invalid character '{found}' at byte {byte_index}"
            ),
        }
    }
}

impl std::error::Error for CapabilityIdError {}
