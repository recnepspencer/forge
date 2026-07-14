use worth_store_layout_indexes::customization::FutureLayoutCustomizationRequest;
use worth_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;

fn require_request(_: FutureLayoutCustomizationRequest) {}

fn main() {
    require_request(LayoutStrategyFamily::BaselineBTreeRange);
}
