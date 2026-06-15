use crate::runtime::ValidationLayoutMeasurementCatalog;
use worth_ui::facade::{
    WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue, WorthUiLayoutTopologyChild,
};

#[derive(Clone, Copy)]
pub(crate) struct RegionSizing {
    pub(crate) default_size: Option<f32>,
    pub(crate) exact_size: Option<f32>,
    pub(crate) min_size: Option<f32>,
    pub(crate) max_size: Option<f32>,
}

pub(crate) fn child_region_sizing(
    child: &WorthUiLayoutTopologyChild,
    available: f32,
    total_flex: u32,
    layout_measurements: &ValidationLayoutMeasurementCatalog,
) -> Option<RegionSizing> {
    let spec = child_sizing(child)?;
    match spec {
        WorthUiLayoutSizingSpec::Fit => None,
        WorthUiLayoutSizingSpec::Fill => Some(RegionSizing {
            default_size: Some(available * (1.0 / total_flex as f32)),
            exact_size: None,
            min_size: Some(96.0),
            max_size: None,
        }),
        WorthUiLayoutSizingSpec::Fixed(value) => {
            let exact = resolve_sizing_value(value, layout_measurements)
                .expect("prepared launch should validate fixed sizing tokens");
            Some(RegionSizing {
                default_size: Some(exact),
                exact_size: Some(exact),
                min_size: Some(exact),
                max_size: Some(exact),
            })
        }
        WorthUiLayoutSizingSpec::Share(weight) => {
            let size = available * (*weight as f32 / total_flex as f32);
            Some(RegionSizing {
                default_size: Some(size),
                exact_size: None,
                min_size: Some(96.0),
                max_size: None,
            })
        }
        WorthUiLayoutSizingSpec::Ratio {
            numerator,
            denominator,
        } => {
            let size = available * (*numerator as f32 / *denominator as f32);
            Some(RegionSizing {
                default_size: Some(size),
                exact_size: Some(size),
                min_size: Some(96.0),
                max_size: Some(size),
            })
        }
        WorthUiLayoutSizingSpec::Clamp {
            min,
            preferred,
            max,
        } => {
            let min_size = resolve_sizing_value(min, layout_measurements)
                .expect("prepared launch should validate clamp min sizing tokens");
            let max_size = resolve_sizing_value(max, layout_measurements)
                .expect("prepared launch should validate clamp max sizing tokens")
                .max(min_size);
            let preferred_size = child_region_sizing_from_spec(
                preferred,
                available,
                total_flex,
                layout_measurements,
            )
            .unwrap_or((min_size + max_size) * 0.5)
            .clamp(min_size, max_size);
            Some(RegionSizing {
                default_size: Some(preferred_size),
                exact_size: None,
                min_size: Some(min_size),
                max_size: Some(max_size),
            })
        }
    }
}

pub(crate) fn child_resizable(child: &WorthUiLayoutTopologyChild) -> bool {
    match child {
        WorthUiLayoutTopologyChild::Region(node) => node.resizable(),
        WorthUiLayoutTopologyChild::Slot(_) => false,
    }
}

pub(crate) fn total_flex_weight(children: &[WorthUiLayoutTopologyChild]) -> u32 {
    children.iter().map(flex_weight).sum::<u32>().max(1)
}

pub(crate) fn central_child_index(children: &[WorthUiLayoutTopologyChild]) -> usize {
    children
        .iter()
        .position(|child| matches!(child_sizing(child), Some(WorthUiLayoutSizingSpec::Fill)))
        .or_else(|| {
            children.iter().rposition(|child| {
                flex_weight(child) > 0
                    || matches!(
                        child_sizing(child),
                        Some(WorthUiLayoutSizingSpec::Ratio { .. })
                    )
            })
        })
        .unwrap_or(children.len() - 1)
}

fn child_region_sizing_from_spec(
    spec: &WorthUiLayoutSizingSpec,
    available: f32,
    total_flex: u32,
    layout_measurements: &ValidationLayoutMeasurementCatalog,
) -> Option<f32> {
    match spec {
        WorthUiLayoutSizingSpec::Fit => None,
        WorthUiLayoutSizingSpec::Fill => Some(available * (1.0 / total_flex as f32)),
        WorthUiLayoutSizingSpec::Fixed(value) => resolve_sizing_value(value, layout_measurements),
        WorthUiLayoutSizingSpec::Share(weight) => {
            Some(available * (*weight as f32 / total_flex as f32))
        }
        WorthUiLayoutSizingSpec::Ratio {
            numerator,
            denominator,
        } => Some(available * (*numerator as f32 / *denominator as f32)),
        WorthUiLayoutSizingSpec::Clamp { preferred, .. } => {
            child_region_sizing_from_spec(preferred, available, total_flex, layout_measurements)
        }
    }
}

fn child_sizing(child: &WorthUiLayoutTopologyChild) -> Option<&WorthUiLayoutSizingSpec> {
    match child {
        WorthUiLayoutTopologyChild::Region(node) => node.sizing(),
        WorthUiLayoutTopologyChild::Slot(_) => None,
    }
}

fn flex_weight(child: &WorthUiLayoutTopologyChild) -> u32 {
    match child_sizing(child) {
        Some(WorthUiLayoutSizingSpec::Fill) => 1,
        Some(WorthUiLayoutSizingSpec::Share(weight)) => *weight,
        Some(WorthUiLayoutSizingSpec::Clamp { preferred, .. }) => {
            child_region_sizing_weight(preferred)
        }
        _ => 0,
    }
}

fn child_region_sizing_weight(spec: &WorthUiLayoutSizingSpec) -> u32 {
    match spec {
        WorthUiLayoutSizingSpec::Fill => 1,
        WorthUiLayoutSizingSpec::Share(weight) => *weight,
        WorthUiLayoutSizingSpec::Clamp { preferred, .. } => child_region_sizing_weight(preferred),
        _ => 0,
    }
}

fn resolve_sizing_value(
    value: &WorthUiLayoutSizingValue,
    layout_measurements: &ValidationLayoutMeasurementCatalog,
) -> Option<f32> {
    layout_measurements.resolve_value(value)
}
