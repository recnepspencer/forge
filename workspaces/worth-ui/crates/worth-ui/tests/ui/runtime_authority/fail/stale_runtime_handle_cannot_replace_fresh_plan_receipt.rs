use worth_ui::facade::{
    WorthUiComponentHandle, WorthUiRuntimeHandleAllocationReceipt, WorthUiRuntimeHandleLocator,
};

fn main() {
    let locator = locator_from_public_option(None);
    let old_handle = WorthUiComponentHandle {
        locator,
    };
    let _receipt = WorthUiRuntimeHandleAllocationReceipt {
        basis_digest: 2,
        arena_identity: old_handle.arena_identity(),
    };
}

fn locator_from_public_option(
    locator: Option<WorthUiRuntimeHandleLocator>,
) -> WorthUiRuntimeHandleLocator {
    locator.expect("test fixture never runs")
}
