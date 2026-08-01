use worth_ui::facade::intent::{
    UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute,
};

fn requires_product(_route: UiResolvedProductIntentRoute) {}

fn requires_confirmation(_route: UiResolvedConfirmationIntentRoute) {}

fn cannot_cross(
    product: UiResolvedProductIntentRoute,
    confirmation: UiResolvedConfirmationIntentRoute,
) {
    requires_product(confirmation);
    requires_confirmation(product);
}

fn main() {}
