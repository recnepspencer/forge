use worth_ui_host_contract::{WorthUiHostAdapter, WorthUiHostContract, WorthUiHostKind};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHostEgui;

impl WorthUiHostEgui {
    pub fn new() -> Self {
        Self
    }
}

impl WorthUiHostAdapter for WorthUiHostEgui {
    fn host_contract(self) -> WorthUiHostContract {
        WorthUiHostContract::new(WorthUiHostKind::Egui)
    }
}
