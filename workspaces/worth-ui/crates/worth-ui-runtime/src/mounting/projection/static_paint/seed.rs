use worth_ui_host_contract::UiMountedRgba8;

use super::super::UiMountedProjectionDenial;

#[derive(Clone, Copy)]
pub(in crate::mounting::projection) struct UiMountedStaticPaintSeed {
    color: UiMountedRgba8,
    layer_semantic_order: u32,
}

pub(in crate::mounting::projection) fn lower_static_paint_seed(
    plan: super::super::super::UiMountedPlanProjectionSource<'_>,
    plan_index: Option<u32>,
) -> Result<Option<UiMountedStaticPaintSeed>, UiMountedProjectionDenial> {
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
    let Some(layer_semantic_order) = lower_layer_semantic_order(component, plan_index) else {
        return Ok(None);
    };
    let Some((_token_plan_index, token_meaning)) = plan
        .component_theme_token(component)
        .map_err(|_| UiMountedProjectionDenial::AmbiguousStaticPaintToken)?
    else {
        return Err(UiMountedProjectionDenial::MissingStaticPaintToken);
    };
    let crate::runtime::planning::execution_plan_input::WorthUiPlanOrdinaryMeaning::Token(token) =
        token_meaning.as_ref()
    else {
        return Err(UiMountedProjectionDenial::ForeignStaticPaintToken);
    };
    let color = token
        .resolved_color_text()
        .ok_or(UiMountedProjectionDenial::MissingStaticPaintColor)?;
    let color =
        parse_rgba(color).map_err(|_| UiMountedProjectionDenial::InvalidStaticPaintColor)?;
    Ok(Some(UiMountedStaticPaintSeed {
        color,
        layer_semantic_order,
    }))
}

fn lower_layer_semantic_order(
    component: &crate::runtime::planning::execution_plan_input::WorthUiComponentPlanMeaning,
    _incidental_plan_index: u32,
) -> Option<u32> {
    Some(component.static_paint_order()?.rank())
}

impl UiMountedStaticPaintSeed {
    pub(in crate::mounting::projection) const fn color(self) -> UiMountedRgba8 {
        self.color
    }

    pub(in crate::mounting::projection) const fn layer_semantic_order(self) -> u32 {
        self.layer_semantic_order
    }

    #[cfg(test)]
    pub(super) const fn for_test(color: UiMountedRgba8) -> Self {
        Self {
            color,
            layer_semantic_order: 0,
        }
    }
}

pub(in crate::mounting::projection) fn parse_rgba(value: &str) -> Result<UiMountedRgba8, ()> {
    match value.as_bytes() {
        [b'#', r0, r1, g0, g1, b0, b1] => Ok(UiMountedRgba8::new(
            hex_pair(*r0, *r1)?,
            hex_pair(*g0, *g1)?,
            hex_pair(*b0, *b1)?,
            255,
        )),
        [b'#', r0, r1, g0, g1, b0, b1, a0, a1] => Ok(UiMountedRgba8::new(
            hex_pair(*r0, *r1)?,
            hex_pair(*g0, *g1)?,
            hex_pair(*b0, *b1)?,
            hex_pair(*a0, *a1)?,
        )),
        _ => Err(()),
    }
}

fn hex_pair(high: u8, low: u8) -> Result<u8, ()> {
    Ok((hex_digit(high)? << 4) | hex_digit(low)?)
}

fn hex_digit(value: u8) -> Result<u8, ()> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::planning::execution_plan_input::WorthUiComponentPlanMeaning;
    use worth_ui_host_contract::UiMountedRgba8;

    use super::{lower_layer_semantic_order, parse_rgba};

    #[test]
    fn admitted_rgb_and_rgba_text_preserve_exact_channels() {
        assert_eq!(
            parse_rgba("#2F81F7"),
            Ok(UiMountedRgba8::new(47, 129, 247, 255))
        );
        assert_eq!(
            parse_rgba("#3fb95080"),
            Ok(UiMountedRgba8::new(63, 185, 80, 128))
        );
    }

    #[test]
    fn malformed_ascii_and_unicode_color_text_deny_without_panicking() {
        for invalid in ["2F81F7", "#2F81FG", "#2F81", "#aéabc"] {
            assert_eq!(parse_rgba(invalid), Err(()));
        }
    }

    #[test]
    fn explicit_paint_order_cannot_be_replaced_by_incidental_plan_index() {
        let meaning = WorthUiComponentPlanMeaning::with_static_paint_order_for_test(7);

        assert_eq!(lower_layer_semantic_order(&meaning, u32::MAX), Some(7));
    }
}
