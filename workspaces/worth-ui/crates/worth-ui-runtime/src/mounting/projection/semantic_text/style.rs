use worth_ui_host_contract::UiMountedRgba8;

use super::super::UiMountedProjectionDenial;

#[derive(Clone, Copy)]
pub(in crate::mounting::projection) struct UiMountedSemanticTextStyleSeed {
    color: UiMountedRgba8,
    layer_semantic_order: u32,
}

pub(in crate::mounting::projection) fn lower_semantic_text_style(
    plan: super::super::super::UiMountedPlanProjectionSource<'_>,
    plan_index: Option<u32>,
) -> Result<Option<UiMountedSemanticTextStyleSeed>, UiMountedProjectionDenial> {
    let Some(plan_index) = plan_index else {
        return Ok(None);
    };
    let Some(meaning) = plan.ordinary_meaning(plan_index) else {
        return Ok(None);
    };
    let crate::runtime::planning::execution_plan_input::WorthUiPlanOrdinaryMeaning::Component(
        component,
    ) = meaning.as_ref()
    else {
        return Ok(None);
    };
    let Some(layer_semantic_order) = component.semantic_text_layer_order() else {
        return Ok(None);
    };
    let Some((_token_plan_index, token_meaning)) = plan
        .component_semantic_text_token(component)
        .map_err(|_| UiMountedProjectionDenial::AmbiguousSemanticTextToken)?
    else {
        return Err(UiMountedProjectionDenial::MissingSemanticTextToken);
    };
    let crate::runtime::planning::execution_plan_input::WorthUiPlanOrdinaryMeaning::Token(token) =
        token_meaning.as_ref()
    else {
        return Err(UiMountedProjectionDenial::ForeignSemanticTextToken);
    };
    let color = token
        .resolved_color_text()
        .ok_or(UiMountedProjectionDenial::MissingSemanticTextColor)?;
    let color = super::super::static_paint::parse_rgba(color)
        .map_err(|_| UiMountedProjectionDenial::InvalidSemanticTextColor)?;
    Ok(Some(UiMountedSemanticTextStyleSeed {
        color,
        layer_semantic_order,
    }))
}

impl UiMountedSemanticTextStyleSeed {
    pub(in crate::mounting::projection) fn color(self) -> UiMountedRgba8 {
        self.color
    }

    pub(in crate::mounting::projection) fn layer_semantic_order(self) -> u32 {
        self.layer_semantic_order
    }
}
