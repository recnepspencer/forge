use worth_ui_host_contract::{
    UiQualifiedTextCaretRecord, UiQualifiedTextLayoutIdentity, UiQualifiedTextLineRecord,
    UiQualifiedTextVisualRunRecord,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UiHeadlessTextAccessibilityGeometry<'mechanic> {
    mechanic: &'mechanic super::UiHeadlessSemanticTextMechanic,
}

impl super::UiHeadlessSemanticTextMechanic {
    pub fn accessibility_geometry(&self) -> UiHeadlessTextAccessibilityGeometry<'_> {
        UiHeadlessTextAccessibilityGeometry { mechanic: self }
    }
}

impl UiHeadlessTextAccessibilityGeometry<'_> {
    pub fn layout_identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.mechanic.layout_identity()
    }
    pub fn lines(&self) -> &[UiQualifiedTextLineRecord] {
        self.mechanic.lines()
    }
    pub fn visual_runs(&self) -> &[UiQualifiedTextVisualRunRecord] {
        self.mechanic.visual_runs()
    }
    pub fn carets(&self) -> &[UiQualifiedTextCaretRecord] {
        self.mechanic.carets()
    }
}
