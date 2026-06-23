use worth_ui::facade::{
    WorthUiProjectionDependencySet, WorthUiValidatedProjectionDependencyContract,
};

fn requires_validated(_: WorthUiValidatedProjectionDependencyContract) {}

fn main() {
    requires_validated(WorthUiProjectionDependencySet::empty());
}
