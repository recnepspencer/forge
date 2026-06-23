use super::{
    ValidationManualFlowDefinition, ValidationManualFlowExpectation,
    ValidationManualFlowExpectationSet, ValidationManualFlowId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowCatalog {
    definitions: Vec<ValidationManualFlowDefinition>,
    expectations: Vec<ValidationManualFlowExpectationSet>,
}

pub fn validation_manual_flow_catalog() -> ValidationManualFlowCatalog {
    ValidationManualFlowCatalog::new(definitions(), expectations())
}

impl ValidationManualFlowCatalog {
    fn new(
        definitions: Vec<ValidationManualFlowDefinition>,
        expectations: Vec<ValidationManualFlowExpectationSet>,
    ) -> Self {
        Self {
            definitions,
            expectations,
        }
    }

    pub fn definitions(&self) -> &[ValidationManualFlowDefinition] {
        &self.definitions
    }

    pub fn expectation_for(
        &self,
        flow_id: ValidationManualFlowId,
    ) -> ValidationManualFlowExpectation {
        self.expectations
            .iter()
            .find(|entry| entry.flow_id() == flow_id)
            .map(|entry| entry.expectation())
            .expect("every manual flow must have an expectation row")
    }
}

fn definitions() -> Vec<ValidationManualFlowDefinition> {
    vec![
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::HeaderText,
            "Header Text",
            "command file edit updates the File/Save label",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::HeaderColor,
            "Header Color",
            "theme file edit changes header panel color",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::HeaderFontSize,
            "Header Font Size",
            "appearance file edit changes header font size",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::DropdownRowPadding,
            "Dropdown Row Padding",
            "density file edit changes dropdown row padding",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::DropdownContainerPadding,
            "Dropdown Container Padding",
            "density file edit changes header container padding",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::DropdownShadow,
            "Dropdown Shadow",
            "appearance file edit changes panel shadow",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::SingleToMultiMode,
            "Single To Multi Mode",
            "projection edit widens File menu from single-select to multi-select",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::MultiToSingleReconciliation,
            "Multi To Single Reconciliation",
            "projection edit narrows File menu back to single-select after multi selection",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::ComponentDescriptor,
            "Component Descriptor",
            "component file edit adds a command binding slot",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::PageSlotReassignment,
            "Page Slot Reassignment",
            "source file edit repoints the proof surface to the alternate registered component",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::LayoutGap,
            "Layout Gap",
            "source file edit changes the authored gap between major Codex-style panels",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::ThreadInset,
            "Thread Inset",
            "source file edit changes the authored inset around the central thread panel",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::InvalidAppearanceDenial,
            "Invalid Appearance Denial",
            "appearance file edit sets font size to an invalid color value",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::EquivalentCanonicalAppearance,
            "Equivalent Canonical Appearance",
            "appearance file rewrites an existing value canonically without changing runtime truth",
        ),
        ValidationManualFlowDefinition::new(
            ValidationManualFlowId::MixedProductStorm,
            "Mixed Product Storm",
            "source, command, projection, component, appearance, density, and denial edits replay as one mixed product storm",
        ),
    ]
}

fn expectations() -> Vec<ValidationManualFlowExpectationSet> {
    vec![
        expectation(
            ValidationManualFlowId::HeaderText,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Save label = Save Everything",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &[
                    "Command(validation.command.edit.copy)",
                    "Command(validation.command.edit.cut)",
                    "Command(validation.command.edit.paste)",
                    "Command(validation.command.edit.redo)",
                    "Command(validation.command.edit.undo)",
                    "Command(validation.command.file.exit)",
                    "Command(validation.command.file.new)",
                    "Command(validation.command.file.open)",
                    "Command(validation.command.file.save)",
                    "Command(validation.command.help.about)",
                    "Command(validation.command.help.docs)",
                    "Command(validation.command.help.palette)",
                    "Command(validation.command.terminal.clear)",
                    "Command(validation.command.terminal.new)",
                    "Command(validation.command.terminal.split)",
                ],
                &["worth-ui.dropdown:validation.header.menu.file"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::HeaderColor,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Header panel fill = #102030ff",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["ThemeToken(validation.theme.header.panel)"],
                &["worth-ui.header.theme"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::HeaderFontSize,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Header font size = 15px",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["Appearance(validation.appearance.header.font_size)"],
                &["worth-ui.header.appearance"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::DropdownRowPadding,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Header row padding = 3px/10px",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["DensityToken(validation.density.header.row_padding)"],
                &["worth-ui.header.appearance"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::DropdownContainerPadding,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Header container padding = top 10 right 14 bottom 10 left 14",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["DensityToken(validation.density.header.container_padding)"],
                &["worth-ui.header.appearance"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::DropdownShadow,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Header shadow = 2px 3px blur 5px spread 1px",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["Appearance(validation.appearance.header.panel_shadow)"],
                &["worth-ui.header.appearance"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::SingleToMultiMode,
            ValidationManualFlowExpectation::new(
                "Activated",
                "File menu selection mode = MultiSelect",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &[
                    "CommandProjection(validation.header.menu.edit)",
                    "CommandProjection(validation.header.menu.file)",
                    "CommandProjection(validation.header.menu.help)",
                    "CommandProjection(validation.header.menu.terminal)",
                    "InteractionPolicy(validation.header.menu.edit)",
                    "InteractionPolicy(validation.header.menu.file)",
                    "InteractionPolicy(validation.header.menu.help)",
                    "InteractionPolicy(validation.header.menu.terminal)",
                ],
                &["worth-ui.dropdown:validation.header.menu.file"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::MultiToSingleReconciliation,
            ValidationManualFlowExpectation::new(
                "Activated",
                "File menu reconciliation = DeniedModeTransition(AmbiguousSingleSelectNarrowing { surviving_command_ids: [\"validation.command.file.new\", \"validation.command.file.open\"] })",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &[
                    "CommandProjection(validation.header.menu.edit)",
                    "CommandProjection(validation.header.menu.file)",
                    "CommandProjection(validation.header.menu.help)",
                    "CommandProjection(validation.header.menu.terminal)",
                    "InteractionPolicy(validation.header.menu.edit)",
                    "InteractionPolicy(validation.header.menu.file)",
                    "InteractionPolicy(validation.header.menu.help)",
                    "InteractionPolicy(validation.header.menu.terminal)",
                ],
                &["worth-ui.dropdown:validation.header.menu.file"],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::ComponentDescriptor,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Component fact changed = validation.component.header.dropdown",
                "Header rebuilt; page-host preserved",
                "not_applicable",
                &["Component(validation.component.header.dropdown)"],
                &[],
                &["worth-ui.page-host.HeaderProofPage"],
            ),
        ),
        expectation(
            ValidationManualFlowId::PageSlotReassignment,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Changed fact = PrimitiveInteraction(worth.surface.preview.primitive.proof)",
                "Header preserved; page-host rebuilt",
                "not_applicable",
                &[
                    "AuthoredSurfaceProps(worth.surface.preview.primitive.proof)",
                    "PrimitiveInteraction(worth.surface.preview.primitive.proof)",
                ],
                &["worth-ui.page-host.HeaderProofPage"],
                &[],
            ),
        ),
        expectation(
            ValidationManualFlowId::LayoutGap,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Changed fact = LayoutGap(HeaderProofPage)",
                "Header preserved; page-host rebuilt",
                "not_applicable",
                &["LayoutGap(HeaderProofPage)"],
                &["worth-ui.page-host.HeaderProofPage"],
                &[],
            ),
        ),
        expectation(
            ValidationManualFlowId::ThreadInset,
            ValidationManualFlowExpectation::new(
                "Activated",
                "Changed fact = LayoutPadding(HeaderProofPage)",
                "Header preserved; page-host rebuilt",
                "not_applicable",
                &["LayoutPadding(HeaderProofPage)"],
                &["worth-ui.page-host.HeaderProofPage"],
                &[],
            ),
        ),
        expectation(
            ValidationManualFlowId::InvalidAppearanceDenial,
            ValidationManualFlowExpectation::new(
                "Denied(AppearanceSourceParse)",
                "Header font size preserved at 13px",
                "Header preserved denied; page-host preserved denied",
                "not_applicable",
                &[],
                &[],
                &[
                    "worth-ui.header.appearance",
                    "worth-ui.page-host.HeaderProofPage",
                ],
            ),
        ),
        expectation(
            ValidationManualFlowId::EquivalentCanonicalAppearance,
            ValidationManualFlowExpectation::new(
                "EquivalentNoOp",
                "Header menu min width = 220px",
                "Header preserved equivalent; no rebuild work",
                "not_applicable",
                &[],
                &[],
                &[],
            ),
        ),
        expectation(
            ValidationManualFlowId::MixedProductStorm,
            ValidationManualFlowExpectation::new(
                "MixedStorm",
                "Storm posture = activated 5 / equivalent 1 / denied 1",
                "Mixed storm counters replay-stable",
                "replay_available",
                &[
                    "PrimitiveInteraction(worth.surface.preview.primitive.proof)",
                    "AuthoredSurfaceProps(worth.surface.preview.primitive.proof)",
                    "Command(validation.command.file.save)",
                    "Component(validation.component.header.dropdown)",
                    "Appearance(validation.appearance.header.menu_min_width)",
                ],
                &[
                    "worth-ui.page-host.HeaderProofPage",
                    "worth-ui.dropdown:validation.header.menu.file",
                ],
                &[],
            ),
        ),
    ]
}

fn expectation(
    flow_id: ValidationManualFlowId,
    expectation: ValidationManualFlowExpectation,
) -> ValidationManualFlowExpectationSet {
    ValidationManualFlowExpectationSet::new(flow_id, expectation)
}
