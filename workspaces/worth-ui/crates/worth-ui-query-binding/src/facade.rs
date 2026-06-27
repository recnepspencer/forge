#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingSubsystem {
    _sealed: (),
}

impl WorthUiQueryBindingSubsystem {
    pub fn bootstrap() -> Self {
        Self { _sealed: () }
    }
}
