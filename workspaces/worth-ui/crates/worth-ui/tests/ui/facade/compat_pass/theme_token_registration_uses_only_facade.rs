use worth_ui::facade::{
    ThemeColorValue, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId,
    ThemeTokenSource, ThemeTokenValue, WorthUi,
};

fn main() {
    let primary = ThemeTokenId::new("theme.text.primary").unwrap();
    let alias = ThemeTokenId::new("theme.text.default").unwrap();
    let app = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::define(
            primary.clone(),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#101820").unwrap()),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            alias.clone(),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(primary),
        ))
        .freeze().expect("application preparation should succeed");

    assert_eq!(app.capabilities().theme_tokens().len(), 2);
    assert!(app.capabilities().theme_tokens().get(&alias).is_some());
}
