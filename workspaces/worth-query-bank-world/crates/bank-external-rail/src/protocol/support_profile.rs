use worth_foundational::facade::{BoundaryProtocolCompatibilityWindow, BoundaryProtocolVersion};

/// Rail-owned compatibility policy selected when the external process starts.
///
/// The producer never selects this profile. It exists so the independent
/// consumer can evolve its accepted interval and retirement policy without
/// changing Query's exact produced-version carriage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RailProtocolSupportProfile {
    Current,
    V2Only,
    V1Retired,
}

impl RailProtocolSupportProfile {
    pub const fn command_line_name(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::V2Only => "v2-only",
            Self::V1Retired => "v1-retired",
        }
    }

    pub fn parse_command_line(value: &str) -> Option<Self> {
        match value {
            "current" => Some(Self::Current),
            "v2-only" => Some(Self::V2Only),
            "v1-retired" => Some(Self::V1Retired),
            _ => None,
        }
    }

    pub(crate) const fn compatibility_window(self) -> BoundaryProtocolCompatibilityWindow {
        match self {
            Self::Current => BoundaryProtocolCompatibilityWindow::inclusive(
                BoundaryProtocolVersion::new(1),
                BoundaryProtocolVersion::new(2),
            ),
            Self::V2Only => BoundaryProtocolCompatibilityWindow::inclusive(
                BoundaryProtocolVersion::new(2),
                BoundaryProtocolVersion::new(2),
            ),
            Self::V1Retired => BoundaryProtocolCompatibilityWindow::inclusive(
                BoundaryProtocolVersion::new(1),
                BoundaryProtocolVersion::new(2),
            )
            .retire_before(BoundaryProtocolVersion::new(2)),
        }
    }
}
