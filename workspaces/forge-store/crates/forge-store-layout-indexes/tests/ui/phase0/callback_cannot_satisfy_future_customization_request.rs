use forge_store_layout_indexes::S8FutureLayoutCustomizationRequest;

fn require_request(_: S8FutureLayoutCustomizationRequest) {}

fn callback() {}

fn main() {
    require_request(callback);
}
