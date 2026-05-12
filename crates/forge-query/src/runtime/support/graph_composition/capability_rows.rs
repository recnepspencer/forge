use crate::identity::hash_parts;
use crate::runtime::mutation::{
    GRAPH_COMPOSITION_LIFECYCLE_FAMILIES, GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphCompositionCapabilityClass {
    TargetCombination,
    LifecycleStep,
}

impl ForgeQueryGraphCompositionCapabilityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TargetCombination => "target-combination",
            Self::LifecycleStep => "lifecycle-step",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionCapabilitySupportRow {
    capability_family: String,
    capability_class: ForgeQueryGraphCompositionCapabilityClass,
    row_digest: String,
}

impl ForgeQueryGraphCompositionCapabilitySupportRow {
    pub(crate) fn new(
        capability_family: impl Into<String>,
        capability_class: ForgeQueryGraphCompositionCapabilityClass,
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

    pub fn capability_class(&self) -> ForgeQueryGraphCompositionCapabilityClass {
        self.capability_class
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn default_graph_composition_capability_support_rows(
) -> Vec<ForgeQueryGraphCompositionCapabilitySupportRow> {
    let mut rows = Vec::with_capacity(
        GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES.len()
            + GRAPH_COMPOSITION_LIFECYCLE_FAMILIES.len(),
    );
    rows.extend(
        GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES
            .iter()
            .map(|family| {
                ForgeQueryGraphCompositionCapabilitySupportRow::new(
                    *family,
                    ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
                )
            }),
    );
    rows.extend(GRAPH_COMPOSITION_LIFECYCLE_FAMILIES.iter().map(|family| {
        ForgeQueryGraphCompositionCapabilitySupportRow::new(
            *family,
            ForgeQueryGraphCompositionCapabilityClass::LifecycleStep,
        )
    }));
    rows
}
