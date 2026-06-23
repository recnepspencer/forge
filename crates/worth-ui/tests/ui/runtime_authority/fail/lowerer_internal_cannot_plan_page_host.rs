use worth_ui::facade::{WorthUiPageHostPlan, WorthUiPageHostRequest};
use worth_ui::source::lower::file_authored::layout_topology::build_layout_topology_catalog;

fn main() {}

fn plan_from_lowerer_internal() {
    let _plan = WorthUiPageHostPlan::from_active_authoring(
        build_layout_topology_catalog,
        unreachable!(),
        WorthUiPageHostRequest::new("ProductsPage"),
    );
}
