use worth_ui_host_contract::{WorthUiHostAdapter, WorthUiHostContract, WorthUiHostKind};

struct AlternateHost;

impl WorthUiHostAdapter for AlternateHost {
    fn host_contract(self) -> WorthUiHostContract {
        WorthUiHostContract::new(WorthUiHostKind::Headless)
    }
}

fn main() {
    let _ = AlternateHost.host_contract();
}
