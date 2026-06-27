#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiInspectionHostContract {
    _sealed: (),
}

impl WorthUiInspectionHostContract {
    pub fn supported() -> Self {
        Self { _sealed: () }
    }
}
