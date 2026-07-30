use super::UiSemanticInteractionFamily;

/// Non-empty canonical interaction family set for one intent definition.
///
/// Construction is const-validated so product definitions cannot defer an
/// empty, duplicate, or order-dependent family set to runtime registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentAcceptedInteractions {
    families: &'static [UiSemanticInteractionFamily],
}

impl UiIntentAcceptedInteractions {
    pub const fn new(families: &'static [UiSemanticInteractionFamily]) -> Self {
        assert!(
            !families.is_empty(),
            "an intent definition must accept at least one interaction family"
        );
        let mut index = 1;
        while index < families.len() {
            assert!(
                families[index - 1].canonical_order() < families[index].canonical_order(),
                "intent interaction families must be unique and in canonical order"
            );
            index += 1;
        }
        Self { families }
    }

    pub const fn as_slice(self) -> &'static [UiSemanticInteractionFamily] {
        self.families
    }
}
