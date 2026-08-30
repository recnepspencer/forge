/// One authored axis of a component whose bounds are resolved directly from
/// the admitted logical viewport.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ComponentViewportAxisPlacement {
    FixedFromStart {
        start_logical_points: u16,
        extent_logical_points: u16,
    },
    StretchBetween {
        start_logical_points: u16,
        end_logical_points: u16,
    },
    FixedFromEnd {
        end_logical_points: u16,
        extent_logical_points: u16,
    },
}

impl ComponentViewportAxisPlacement {
    pub const fn fixed_from_start(
        start_logical_points: u16,
        extent_logical_points: u16,
    ) -> Option<Self> {
        if extent_logical_points == 0 {
            return None;
        }
        Some(Self::FixedFromStart {
            start_logical_points,
            extent_logical_points,
        })
    }

    pub const fn stretch_between(start_logical_points: u16, end_logical_points: u16) -> Self {
        Self::StretchBetween {
            start_logical_points,
            end_logical_points,
        }
    }

    pub const fn fixed_from_end(
        end_logical_points: u16,
        extent_logical_points: u16,
    ) -> Option<Self> {
        if extent_logical_points == 0 {
            return None;
        }
        Some(Self::FixedFromEnd {
            end_logical_points,
            extent_logical_points,
        })
    }

    pub(crate) fn digest_basis(self) -> String {
        match self {
            Self::FixedFromStart {
                start_logical_points,
                extent_logical_points,
            } => format!("fixed-from-start:{start_logical_points}:{extent_logical_points}"),
            Self::StretchBetween {
                start_logical_points,
                end_logical_points,
            } => format!("stretch-between:{start_logical_points}:{end_logical_points}"),
            Self::FixedFromEnd {
                end_logical_points,
                extent_logical_points,
            } => format!("fixed-from-end:{end_logical_points}:{extent_logical_points}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentViewportAxisPlacement;

    #[test]
    fn fixed_axes_reject_empty_extent_and_keep_direction_in_identity() {
        assert!(ComponentViewportAxisPlacement::fixed_from_start(24, 0).is_none());
        assert!(ComponentViewportAxisPlacement::fixed_from_end(24, 0).is_none());
        assert_ne!(
            ComponentViewportAxisPlacement::fixed_from_start(24, 56)
                .unwrap()
                .digest_basis(),
            ComponentViewportAxisPlacement::fixed_from_end(24, 56)
                .unwrap()
                .digest_basis(),
        );
    }
}
