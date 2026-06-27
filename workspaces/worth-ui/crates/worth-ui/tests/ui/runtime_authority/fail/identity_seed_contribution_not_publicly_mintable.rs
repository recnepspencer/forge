use worth_ui::facade::WorthUiIdentitySeedContribution;

fn main() {
    let _ = WorthUiIdentitySeedContribution::from_admitted_seed(
        "workspace.component.special",
        "component:x",
    );
}
