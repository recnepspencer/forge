#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneOperandSide {
    Left,
    Right,
}

impl PlanarBooleanCommonPlaneOperandSide {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::Left => "left operand",
            Self::Right => "right operand",
        }
    }
}
