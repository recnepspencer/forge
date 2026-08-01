use worth_ui::facade::intent::UiIntentOperabilityProof;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<UiIntentOperabilityProof>();
}
