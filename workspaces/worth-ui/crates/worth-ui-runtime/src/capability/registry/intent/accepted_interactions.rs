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

#[cfg(test)]
mod tests {
    use super::{UiIntentAcceptedInteractions, UiSemanticInteractionFamily};

    #[test]
    fn every_family_is_admitted_in_the_only_canonical_order() {
        let interactions = UiIntentAcceptedInteractions::new(&[
            UiSemanticInteractionFamily::Activate,
            UiSemanticInteractionFamily::EditCommit,
            UiSemanticInteractionFamily::SelectionCommit,
            UiSemanticInteractionFamily::Submit,
        ]);

        assert_eq!(interactions.as_slice().len(), 4);
    }

    #[test]
    #[should_panic(expected = "must accept at least one interaction family")]
    fn empty_family_set_is_rejected() {
        let _ = UiIntentAcceptedInteractions::new(&[]);
    }

    #[test]
    #[should_panic(expected = "must be unique and in canonical order")]
    fn duplicate_family_is_rejected() {
        let _ = UiIntentAcceptedInteractions::new(&[
            UiSemanticInteractionFamily::Activate,
            UiSemanticInteractionFamily::Activate,
        ]);
    }

    #[test]
    #[should_panic(expected = "must be unique and in canonical order")]
    fn reversed_family_order_is_rejected() {
        let _ = UiIntentAcceptedInteractions::new(&[
            UiSemanticInteractionFamily::Submit,
            UiSemanticInteractionFamily::Activate,
        ]);
    }
}
