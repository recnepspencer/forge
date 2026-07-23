use worth_ui::facade::{
    registry::{ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource},
};

fn main() {
    let _descriptor = ThemeTokenDescriptor {
        id: ThemeTokenId::new("theme.text.primary").unwrap(),
        family: ThemeTokenFamily::text(),
        source: ThemeTokenSource::application(),
        value: None,
        alias: None,
        raw_color_outside_token_definition: None,
    };
}
