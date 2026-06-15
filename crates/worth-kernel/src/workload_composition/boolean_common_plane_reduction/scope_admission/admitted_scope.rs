#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneAdmittedOperandScope {
    ClosedPlanarBodyPair,
}

impl PlanarBooleanCommonPlaneAdmittedOperandScope {
    pub fn query_key(self) -> &'static str {
        match self {
            Self::ClosedPlanarBodyPair => {
                "worth.boolean.common_plane.scope.closed_planar_body_pair"
            }
        }
    }

    pub fn human_name(self) -> &'static str {
        match self {
            Self::ClosedPlanarBodyPair => {
                "closed planar body pair admitted for common-plane reduction"
            }
        }
    }
}
