use worth_ui::facade::query_binding::{WorthUiInstalledQueryDomain, WorthUiInstalledQueryView};

fn fabricate(domain: WorthUiInstalledQueryDomain) {
    let _view = WorthUiInstalledQueryView {
        installed_domain: domain,
        definition: worth_ui::facade::query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
            "workspace.measurements",
        )
        .unwrap(),
    };
}

fn main() {}
