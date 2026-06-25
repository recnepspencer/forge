#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExpressionOperatorId(&'static str);

impl WorthUiExpressionOperatorId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
