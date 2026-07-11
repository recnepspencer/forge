use forge_store_layout_indexes::layout_customization::S8FutureLayoutCustomizationRequest;
use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;

fn require_request(_: S8FutureLayoutCustomizationRequest) {}

fn main() {
    require_request(S8LayoutStrategyFamily::BaselineBTreeRange);
}
