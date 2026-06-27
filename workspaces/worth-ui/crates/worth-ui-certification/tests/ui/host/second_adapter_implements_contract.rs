use worth_ui::facade::app::WorthUi;
use worth_ui::facade::dsl::WorthUiDslPackage;
use worth_ui_host_contract::{WorthUiHostAdapter, WorthUiHostContract, WorthUiHostKind};

struct AlternateHost;

impl WorthUiHostAdapter for AlternateHost {
    fn host_contract(self) -> WorthUiHostContract {
        WorthUiHostContract::new(WorthUiHostKind::Headless)
    }
}

fn main() {
    let _ = WorthUi::app()
        .with_dsl_package(WorthUiDslPackage::named("certification.host"))
        .with_host(AlternateHost)
        .freeze();
}
