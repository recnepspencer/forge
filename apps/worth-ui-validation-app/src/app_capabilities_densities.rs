use worth_ui::facade::{
    DensityTokenId, WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue,
    WorthUiPaddingValue, WorthUiSpacingValue,
};

pub(crate) fn header_density_tokens() -> Vec<WorthUiDensityTokenDescriptor> {
    vec![
        padding_token(
            "validation.density.header.row_padding",
            WorthUiDensityFamily::RowPadding,
            "1px 6px",
            "valid row padding",
        ),
        padding_token(
            "validation.density.header.container_padding",
            WorthUiDensityFamily::ContainerPadding,
            "4px 8px",
            "valid container padding",
        ),
        spacing_token(
            "validation.density.header.control_spacing",
            "8px",
            "valid control spacing",
        ),
        padding_token(
            "validation.density.primitive.padding",
            WorthUiDensityFamily::ContainerPadding,
            "16px 32px",
            "valid primitive padding",
        ),
        padding_token(
            "validation.density.primitive.padding.fat",
            WorthUiDensityFamily::ContainerPadding,
            "22px 56px",
            "valid primitive fat padding",
        ),
        padding_token(
            "validation.density.primitive.padding.wide_shallow",
            WorthUiDensityFamily::ContainerPadding,
            "8px 64px",
            "valid primitive wide shallow padding",
        ),
        spacing_token(
            "validation.density.primitive.radius",
            "8px",
            "valid primitive radius",
        ),
        spacing_token(
            "validation.density.primitive.border.none",
            "0px",
            "valid primitive empty border width",
        ),
        spacing_token(
            "validation.density.primitive.border.default",
            "2px",
            "valid primitive border width",
        ),
        spacing_token(
            "validation.density.primitive.focus.ring",
            "2px",
            "valid primitive focus ring width",
        ),
        spacing_token(
            "validation.density.primitive.motion.fast",
            "120px",
            "valid primitive duration",
        ),
        spacing_token(
            "validation.density.primitive.flow.gap.compact",
            "6px",
            "valid compact primitive flow gap",
        ),
        spacing_token(
            "validation.density.primitive.flow.gap.default",
            "8px",
            "valid primitive flow gap",
        ),
        spacing_token(
            "validation.density.primitive.flow.gap.alias",
            "8px",
            "valid equivalent primitive flow gap",
        ),
        spacing_token(
            "validation.density.primitive.flow.gap.fat",
            "16px",
            "valid fat primitive flow gap",
        ),
        spacing_token(
            "validation.density.primitive.flow.padding.compact",
            "16px",
            "valid compact primitive flow padding",
        ),
        spacing_token(
            "validation.density.primitive.flow.padding.default",
            "32px",
            "valid primitive flow padding",
        ),
        spacing_token(
            "validation.density.primitive.flow.padding.fat",
            "48px",
            "valid fat primitive flow padding",
        ),
        padding_token(
            "validation.density.primitive.flow.padding.wide_shallow",
            WorthUiDensityFamily::ContainerPadding,
            "8px 64px",
            "valid wide shallow primitive flow padding",
        ),
        spacing_token(
            "validation.density.primitive.content.text.default",
            "15px",
            "valid primitive content text size",
        ),
        spacing_token(
            "validation.density.primitive.content.text.large",
            "18px",
            "valid large primitive content text size",
        ),
        spacing_token(
            "validation.density.primitive.content.icon.default",
            "24px",
            "valid primitive content icon size",
        ),
        spacing_token(
            "validation.density.primitive.content.icon.large",
            "32px",
            "valid large primitive content icon size",
        ),
        spacing_token(
            "validation.density.primitive.content.icon.stroke.thin",
            "1px",
            "valid thin primitive content icon stroke",
        ),
        spacing_token(
            "validation.density.primitive.content.icon.stroke.default",
            "2px",
            "valid primitive content icon stroke",
        ),
        spacing_token(
            "validation.density.primitive.content.spacer.default",
            "8px",
            "valid primitive content spacer size",
        ),
        spacing_token(
            "validation.density.primitive.content.divider.default",
            "1px",
            "valid primitive content divider thickness",
        ),
        spacing_token(
            "validation.density.primitive.event.hit_slop.none",
            "0px",
            "valid empty primitive event hit slop",
        ),
        spacing_token(
            "validation.density.primitive.event.hit_slop.compact",
            "4px",
            "valid compact primitive event hit slop",
        ),
        spacing_token(
            "validation.density.primitive.event.hit_slop.default",
            "8px",
            "valid primitive event hit slop",
        ),
        spacing_token(
            "validation.density.primitive.event.hit_slop.comfortable",
            "16px",
            "valid comfortable primitive event hit slop",
        ),
    ]
}

fn padding_token(
    id: &str,
    family: WorthUiDensityFamily,
    value: &str,
    expectation: &str,
) -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new(id).expect("valid density id"),
        family,
        WorthUiDensityValue::Padding(
            WorthUiPaddingValue::from_shorthand_px(value).expect(expectation),
        ),
    )
}

fn spacing_token(id: &str, value: &str, expectation: &str) -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new(id).expect("valid density id"),
        WorthUiDensityFamily::ControlSpacing,
        WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px(value).expect(expectation)),
    )
}
