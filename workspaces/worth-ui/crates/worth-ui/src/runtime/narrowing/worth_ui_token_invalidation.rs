#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiTokenInvalidation {
    affected_token_count: usize,
}

impl WorthUiTokenInvalidation {
    pub(crate) fn theme_only(affected_token_count: usize) -> Self {
        Self {
            affected_token_count,
        }
    }

    pub fn affected_token_count(&self) -> usize {
        self.affected_token_count
    }
}
