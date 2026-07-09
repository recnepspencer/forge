use worth_store_layout_indexes::{
    S8FutureLayoutCustomizationRequest, S8LayoutStrategyFamily,
};

fn require_request(_: S8FutureLayoutCustomizationRequest) {}

fn main() {
    require_request(S8LayoutStrategyFamily::BaselineBTreeRange);
}
