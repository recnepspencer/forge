use crate::identity::hash_parts;
use crate::runtime::mutation::{
    GRAPH_COMPOSITION_LIFECYCLE_FAMILIES, GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphCompositionCapabilityClass {
    TargetCombination,
    LifecycleStep,
}

impl WorthQueryGraphCompositionCapabilityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TargetCombination => "target-combination",
            Self::LifecycleStep => "lifecycle-step",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCompositionCapabilitySupportRow {
    capability_family: String,
    capability_class: WorthQueryGraphCompositionCapabilityClass,
    row_digest: String,
}

impl WorthQueryGraphCompositionCapabilitySupportRow {
    pub(crate) fn new(
        capability_family: impl Into<String>,
        capability_class: WorthQueryGraphCompositionCapabilityClass,
    ) -> Self {
        let capability_family = capability_family.into();
        let row_digest = hash_parts(&[
            format!("family:{capability_family}"),
            format!("class:{}", capability_class.as_str()),
        ]);
        Self {
            capability_family,
            capability_class,
            row_digest,
        }
    }

    pub fn capability_family(&self) -> &str {
        &self.capability_family
    }

    pub fn capability_class(&self) -> WorthQueryGraphCompositionCapabilityClass {
        self.capability_class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn default_graph_composition_capability_support_rows(
) -> Vec<WorthQueryGraphCompositionCapabilitySupportRow> {
    let mut rows = Vec::with_capacity(
        GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES.len()
            + GRAPH_COMPOSITION_LIFECYCLE_FAMILIES.len(),
    );
    rows.extend(
        GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES
            .iter()
            .map(|family| {
                WorthQueryGraphCompositionCapabilitySupportRow::new(
                    *family,
                    WorthQueryGraphCompositionCapabilityClass::TargetCombination,
                )
            }),
    );
    rows.extend(GRAPH_COMPOSITION_LIFECYCLE_FAMILIES.iter().map(|family| {
        WorthQueryGraphCompositionCapabilitySupportRow::new(
            *family,
            WorthQueryGraphCompositionCapabilityClass::LifecycleStep,
        )
    }));
    rows
}
