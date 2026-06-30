#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostKind {
    Headless,
    Egui,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostContract {
    kind: WorthUiHostKind,
}

pub trait WorthUiHostAdapter {
    fn host_contract(self) -> WorthUiHostContract;
}

impl WorthUiHostContract {
    pub fn headless() -> Self {
        Self {
            kind: WorthUiHostKind::Headless,
        }
    }

    pub fn new(kind: WorthUiHostKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> WorthUiHostKind {
        self.kind
    }
}

impl WorthUiHostAdapter for WorthUiHostContract {
    fn host_contract(self) -> WorthUiHostContract {
        self
    }
}
