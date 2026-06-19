use crate::runtime::ForgeQueryGraphTouchSelector;

use super::consumer_declaration::ForgeQueryGraphObligationConsumerRegistrationDeclaration;
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSelectorCoverageRow {
    label: String,
    selector: ForgeQueryGraphTouchSelector,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSelectorCoverageDeclaration {
    rows: Vec<ForgeQueryGraphObligationSelectorCoverageRow>,
    declaration_digest: String,
}

impl ForgeQueryGraphObligationSelectorCoverageDeclaration {
    pub fn required(
        rows: impl IntoIterator<Item = (impl Into<String>, ForgeQueryGraphTouchSelector)>,
    ) -> Self {
        let mut rows = rows
            .into_iter()
            .map(|(label, selector)| {
                ForgeQueryGraphObligationSelectorCoverageRow::new(label, selector)
            })
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| left.row_digest.cmp(&right.row_digest));
        let declaration_digest = kit_digest(
            "graph-obligation-selector-coverage",
            rows.iter().map(|row| row.row_digest.as_str()),
        );
        Self {
            rows,
            declaration_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryGraphObligationSelectorCoverageRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn covers_registration_declaration(
        &self,
        registration: &ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ) -> bool {
        registration.registrations().iter().all(|registration| {
            self.rows.iter().any(|row| {
                row.selector.selector_digest() == registration.touch_selector().selector_digest()
            })
        })
    }
}

impl ForgeQueryGraphObligationSelectorCoverageRow {
    fn new(label: impl Into<String>, selector: ForgeQueryGraphTouchSelector) -> Self {
        let label = label.into();
        let row_digest = kit_digest(
            "graph-obligation-selector-coverage-row",
            [
                label.as_str(),
                selector.selector_kind(),
                selector.selector_value().as_deref().unwrap_or("<any>"),
                selector.selector_digest(),
            ],
        );
        Self {
            label,
            selector,
            row_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn selector(&self) -> &ForgeQueryGraphTouchSelector {
        &self.selector
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
