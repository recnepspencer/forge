use worth_ui::facade::{WorthUiPageHostPlan, WorthUiPageHostRequest};
use worth_ui::source::WorthUiParsedSourcePackage;

fn main() {}

fn plan_from_parser_struct(parsed: &WorthUiParsedSourcePackage) {
    let _plan = WorthUiPageHostPlan::from_active_authoring(
        parsed,
        unreachable!(),
        WorthUiPageHostRequest::new("ProductsPage"),
    );
}
