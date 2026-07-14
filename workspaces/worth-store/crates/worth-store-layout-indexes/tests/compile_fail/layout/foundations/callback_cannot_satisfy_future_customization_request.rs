use worth_store_layout_indexes::customization::FutureLayoutCustomizationRequest;

fn require_request(_: FutureLayoutCustomizationRequest) {}

fn callback() {}

fn main() {
    require_request(callback);
}
