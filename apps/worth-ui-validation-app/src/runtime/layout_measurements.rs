use worth_ui::facade::{
    WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue, WorthUiLayoutTopologyCatalog,
    WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ValidationLayoutMeasurement {
    token_name: &'static str,
    value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ValidationLayoutMeasurementCatalog {
    entries: &'static [ValidationLayoutMeasurement],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ValidationLayoutMeasurementCatalogDenial {
    MissingNamedToken {
        page_name: String,
        token_name: String,
    },
}

const SHOPIFY_ADMIN_LAYOUT_MEASUREMENTS: [ValidationLayoutMeasurement; 5] = [
    ValidationLayoutMeasurement {
        token_name: "rail.md",
        value: 240.0,
    },
    ValidationLayoutMeasurement {
        token_name: "rail.xl",
        value: 360.0,
    },
    ValidationLayoutMeasurement {
        token_name: "inspector.md",
        value: 280.0,
    },
    ValidationLayoutMeasurement {
        token_name: "inspector.xl",
        value: 420.0,
    },
    ValidationLayoutMeasurement {
        token_name: "panel.lg",
        value: 320.0,
    },
];

impl ValidationLayoutMeasurementCatalog {
    pub(crate) fn shopify_admin_defaults() -> Self {
        Self {
            entries: &SHOPIFY_ADMIN_LAYOUT_MEASUREMENTS,
        }
    }

    pub(crate) fn validate_topology(
        &self,
        layout_topology: &WorthUiLayoutTopologyCatalog,
    ) -> Result<(), ValidationLayoutMeasurementCatalogDenial> {
        for page in layout_topology.pages() {
            validate_node_measurements(self, page.page_name(), page.root())?;
        }
        Ok(())
    }

    pub(crate) fn resolve_value(&self, value: &WorthUiLayoutSizingValue) -> Option<f32> {
        match value {
            WorthUiLayoutSizingValue::Number(number) => Some(*number as f32),
            WorthUiLayoutSizingValue::NamedToken(token_name) => self
                .entries
                .iter()
                .find(|entry| entry.token_name == token_name.as_str())
                .map(|entry| entry.value),
        }
    }
}

fn validate_node_measurements(
    catalog: &ValidationLayoutMeasurementCatalog,
    page_name: &str,
    node: &WorthUiLayoutTopologyNode,
) -> Result<(), ValidationLayoutMeasurementCatalogDenial> {
    if let Some(sizing) = node.sizing() {
        validate_sizing_spec(catalog, page_name, sizing)?;
    }

    for child in node.children() {
        if let WorthUiLayoutTopologyChild::Region(region) = child {
            validate_node_measurements(catalog, page_name, region)?;
        }
    }

    Ok(())
}

fn validate_sizing_spec(
    catalog: &ValidationLayoutMeasurementCatalog,
    page_name: &str,
    sizing: &WorthUiLayoutSizingSpec,
) -> Result<(), ValidationLayoutMeasurementCatalogDenial> {
    match sizing {
        WorthUiLayoutSizingSpec::Fit
        | WorthUiLayoutSizingSpec::Fill
        | WorthUiLayoutSizingSpec::Share(_)
        | WorthUiLayoutSizingSpec::Ratio { .. } => Ok(()),
        WorthUiLayoutSizingSpec::Fixed(value) => validate_sizing_value(catalog, page_name, value),
        WorthUiLayoutSizingSpec::Clamp {
            min,
            preferred,
            max,
        } => {
            validate_sizing_value(catalog, page_name, min)?;
            validate_sizing_spec(catalog, page_name, preferred)?;
            validate_sizing_value(catalog, page_name, max)
        }
    }
}

fn validate_sizing_value(
    catalog: &ValidationLayoutMeasurementCatalog,
    page_name: &str,
    value: &WorthUiLayoutSizingValue,
) -> Result<(), ValidationLayoutMeasurementCatalogDenial> {
    match value {
        WorthUiLayoutSizingValue::Number(_) => Ok(()),
        WorthUiLayoutSizingValue::NamedToken(token_name) => {
            if catalog.resolve_value(value).is_some() {
                Ok(())
            } else {
                Err(
                    ValidationLayoutMeasurementCatalogDenial::MissingNamedToken {
                        page_name: page_name.to_owned(),
                        token_name: token_name.clone(),
                    },
                )
            }
        }
    }
}
