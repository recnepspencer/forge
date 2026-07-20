use crate::runtime::WorthQueryGraphTouchSelector;

use super::consumer_declaration::WorthQueryGraphObligationConsumerRegistrationDeclaration;
use super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSelectorCoverageRow {
    label: String,
    selector: WorthQueryGraphTouchSelector,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSelectorCoverageDeclaration {
    rows: Vec<WorthQueryGraphObligationSelectorCoverageRow>,
    declaration_digest: String,
}

impl WorthQueryGraphObligationSelectorCoverageDeclaration {
    pub fn required(
        rows: impl IntoIterator<Item = (impl Into<String>, WorthQueryGraphTouchSelector)>,
    ) -> Self {
        let mut rows = rows
            .into_iter()
            .map(|(label, selector)| {
                WorthQueryGraphObligationSelectorCoverageRow::new(label, selector)
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

    pub fn rows(&self) -> &[WorthQueryGraphObligationSelectorCoverageRow] {
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
        registration: &WorthQueryGraphObligationConsumerRegistrationDeclaration,
    ) -> bool {
        registration.registrations().iter().all(|registration| {
            self.rows.iter().any(|row| {
                row.selector.selector_digest() == registration.touch_selector().selector_digest()
            })
        })
    }
}

impl WorthQueryGraphObligationSelectorCoverageRow {
    fn new(label: impl Into<String>, selector: WorthQueryGraphTouchSelector) -> Self {
        let label = label.into();
        let row_digest = kit_digest(
            "graph-obligation-selector-coverage-row",
            [
                label.as_str(),
                selector.terminal_selector_kind_for_boundary(),
                selector
                    .terminal_selector_value_for_boundary()
                    .as_deref()
                    .unwrap_or("<any>"),
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

    pub fn selector(&self) -> &WorthQueryGraphTouchSelector {
        &self.selector
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
