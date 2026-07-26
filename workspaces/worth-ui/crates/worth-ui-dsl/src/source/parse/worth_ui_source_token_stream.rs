use crate::source::WorthUiSourceToken;

#[derive(Clone, Debug)]
pub(crate) struct WorthUiSourceTokenStream {
    tokens: Vec<WorthUiSourceToken>,
    position: usize,
}

impl WorthUiSourceTokenStream {
    pub(crate) fn new(tokens: Vec<WorthUiSourceToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub(crate) fn peek(&self) -> Option<&WorthUiSourceToken> {
        self.tokens.get(self.position)
    }

    pub(crate) fn next(&mut self) -> Option<WorthUiSourceToken> {
        let next = self.tokens.get(self.position).cloned();
        if next.is_some() {
            self.position += 1;
        }
        next
    }
}
