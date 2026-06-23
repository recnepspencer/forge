#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ValidationManualFlowId {
    HeaderText,
    HeaderColor,
    HeaderFontSize,
    DropdownRowPadding,
    DropdownContainerPadding,
    DropdownShadow,
    SingleToMultiMode,
    MultiToSingleReconciliation,
    ComponentDescriptor,
    PageSlotReassignment,
    LayoutGap,
    ThreadInset,
    InvalidAppearanceDenial,
    EquivalentCanonicalAppearance,
    MixedProductStorm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowDefinition {
    id: ValidationManualFlowId,
    title: &'static str,
    authored_input: &'static str,
}

impl ValidationManualFlowDefinition {
    pub const fn new(
        id: ValidationManualFlowId,
        title: &'static str,
        authored_input: &'static str,
    ) -> Self {
        Self {
            id,
            title,
            authored_input,
        }
    }

    pub fn id(self) -> ValidationManualFlowId {
        self.id
    }

    pub fn title(self) -> &'static str {
        self.title
    }

    pub fn authored_input(self) -> &'static str {
        self.authored_input
    }
}
