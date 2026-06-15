use worth_ui::facade::{
    CommandProjectionDescriptor, IconDescriptor, MosaicSizingContractDescriptor,
    RuntimeOutcomeProjectionDescriptor, ThemeTokenDescriptor,
};

use crate::theme::{
    vscode_like_dark_theme_catalog, HarnessDensity, HarnessThemeTokenCatalog,
    HarnessVisualThemeReceipt, HarnessVisualTokenRole,
};

use super::{
    harness_command_visual_projections, harness_icon_descriptors,
    harness_runtime_outcome_projections, HarnessCommandProjectionVisualRole,
    HarnessRuntimeOutcomeVisualRole, HarnessVisualFoundationDenial, HarnessVisualFoundationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessVisualFoundationBundle {
    theme_catalog: HarnessThemeTokenCatalog,
    density: HarnessDensity,
    sizing_contracts: Vec<MosaicSizingContractDescriptor>,
    icons: Vec<IconDescriptor>,
    command_projections: Vec<CommandProjectionDescriptor>,
    runtime_outcome_projections: Vec<RuntimeOutcomeProjectionDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedHarnessVisualFoundation {
    bundle: HarnessVisualFoundationBundle,
    receipt: HarnessVisualFoundationReceipt,
}

pub(crate) struct HarnessVisualFoundationParts {
    pub(crate) theme_tokens: Vec<ThemeTokenDescriptor>,
    pub(crate) sizing_contracts: Vec<MosaicSizingContractDescriptor>,
    pub(crate) icons: Vec<IconDescriptor>,
    pub(crate) command_projections: Vec<CommandProjectionDescriptor>,
    pub(crate) runtime_outcome_projections: Vec<RuntimeOutcomeProjectionDescriptor>,
}

impl HarnessVisualFoundationBundle {
    pub fn vscode_like_dark() -> Self {
        Self::new(HarnessDensity::DEFAULT)
    }

    pub fn with_density(mut self, density: HarnessDensity) -> Self {
        self.density = density;
        self.sizing_contracts = density.sizing_contracts();
        self
    }

    pub fn prepare(self) -> Result<PreparedHarnessVisualFoundation, HarnessVisualFoundationDenial> {
        reject_incomplete_theme_catalog(&self.theme_catalog)?;
        reject_incomplete_density(&self.sizing_contracts)?;
        reject_missing_icons(&self.icons)?;
        reject_incomplete_command_projections(&self.command_projections)?;
        reject_incomplete_runtime_outcome_projections(&self.runtime_outcome_projections)?;
        let receipt = foundation_receipt(&self);
        Ok(PreparedHarnessVisualFoundation {
            bundle: self,
            receipt,
        })
    }

    fn new(density: HarnessDensity) -> Self {
        Self {
            theme_catalog: vscode_like_dark_theme_catalog(),
            density,
            sizing_contracts: density.sizing_contracts(),
            icons: harness_icon_descriptors(),
            command_projections: harness_command_visual_projections(),
            runtime_outcome_projections: harness_runtime_outcome_projections(),
        }
    }
}

impl PreparedHarnessVisualFoundation {
    pub fn receipt(&self) -> &HarnessVisualFoundationReceipt {
        &self.receipt
    }

    pub fn theme_tokens(&self) -> &[ThemeTokenDescriptor] {
        self.bundle.theme_catalog.descriptors()
    }

    pub fn sizing_contracts(&self) -> &[MosaicSizingContractDescriptor] {
        &self.bundle.sizing_contracts
    }

    pub fn icons(&self) -> &[IconDescriptor] {
        &self.bundle.icons
    }

    pub fn command_projections(&self) -> &[CommandProjectionDescriptor] {
        &self.bundle.command_projections
    }

    pub fn runtime_outcome_projections(&self) -> &[RuntimeOutcomeProjectionDescriptor] {
        &self.bundle.runtime_outcome_projections
    }

    pub(crate) fn into_parts(self) -> HarnessVisualFoundationParts {
        HarnessVisualFoundationParts {
            theme_tokens: self.bundle.theme_catalog.descriptors().to_vec(),
            sizing_contracts: self.bundle.sizing_contracts,
            icons: self.bundle.icons,
            command_projections: self.bundle.command_projections,
            runtime_outcome_projections: self.bundle.runtime_outcome_projections,
        }
    }
}

fn reject_incomplete_theme_catalog(
    catalog: &HarnessThemeTokenCatalog,
) -> Result<(), HarnessVisualFoundationDenial> {
    if let Some(role) = catalog.duplicate_role() {
        return Err(HarnessVisualFoundationDenial::DuplicateTokenRole { role });
    }
    for role in HarnessVisualTokenRole::REQUIRED {
        if catalog.token_id_for(role).is_none() {
            return Err(HarnessVisualFoundationDenial::MissingTokenRole { role });
        }
    }
    Ok(())
}

fn reject_incomplete_density(
    sizing_contracts: &[MosaicSizingContractDescriptor],
) -> Result<(), HarnessVisualFoundationDenial> {
    for contract_id in HarnessDensity::REQUIRED_SIZING_CONTRACT_IDS {
        if sizing_contracts
            .iter()
            .all(|contract| contract.id().as_str() != contract_id)
        {
            return Err(HarnessVisualFoundationDenial::MissingDensityMeasurements);
        }
    }
    Ok(())
}

fn reject_missing_icons(icons: &[IconDescriptor]) -> Result<(), HarnessVisualFoundationDenial> {
    if icons.is_empty() {
        Err(HarnessVisualFoundationDenial::MissingIconDescriptors)
    } else {
        Ok(())
    }
}

fn reject_incomplete_command_projections(
    projections: &[CommandProjectionDescriptor],
) -> Result<(), HarnessVisualFoundationDenial> {
    for role in HarnessCommandProjectionVisualRole::REQUIRED {
        if projections
            .iter()
            .all(|projection| projection.id().as_str() != role.projection_id_text())
        {
            return Err(HarnessVisualFoundationDenial::MissingCommandProjection { role });
        }
    }
    Ok(())
}

fn reject_incomplete_runtime_outcome_projections(
    projections: &[RuntimeOutcomeProjectionDescriptor],
) -> Result<(), HarnessVisualFoundationDenial> {
    for role in HarnessRuntimeOutcomeVisualRole::REQUIRED {
        if projections
            .iter()
            .all(|projection| projection.id().as_str() != runtime_outcome_id(role))
        {
            return Err(HarnessVisualFoundationDenial::MissingRuntimeOutcomeProjection { role });
        }
    }
    Ok(())
}

fn foundation_receipt(bundle: &HarnessVisualFoundationBundle) -> HarnessVisualFoundationReceipt {
    HarnessVisualFoundationReceipt::new(
        HarnessVisualThemeReceipt::new(
            bundle.density,
            &bundle.theme_catalog,
            bundle.sizing_contracts.len(),
        ),
        bundle.icons.len(),
        HarnessCommandProjectionVisualRole::REQUIRED.to_vec(),
        HarnessRuntimeOutcomeVisualRole::REQUIRED.to_vec(),
    )
}

fn runtime_outcome_id(role: HarnessRuntimeOutcomeVisualRole) -> &'static str {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => "harness.runtime_outcome.active",
        HarnessRuntimeOutcomeVisualRole::Success => "harness.runtime_outcome.success",
        HarnessRuntimeOutcomeVisualRole::Warning => "harness.runtime_outcome.warning",
        HarnessRuntimeOutcomeVisualRole::Danger => "harness.runtime_outcome.danger",
        HarnessRuntimeOutcomeVisualRole::Disabled => "harness.runtime_outcome.disabled",
    }
}
