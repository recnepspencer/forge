use crate::capability::ThemeTokenId;

use super::{IconColorSupport, IconNativeVectorSupport, IconSizeSupport, IconSourceKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconSourceDescriptor {
    kind: IconSourceKind,
    provider: String,
    source_key: String,
    size_support: IconSizeSupport,
    color_support: IconColorSupport,
    native_vector_support: IconNativeVectorSupport,
    theme_token: Option<ThemeTokenId>,
}

impl IconSourceDescriptor {
    pub fn symbol(source_key: impl Into<String>) -> Self {
        Self::new(IconSourceKind::symbol(), "symbol", source_key)
    }

    pub fn vector_asset(provider: impl Into<String>, source_key: impl Into<String>) -> Self {
        Self::new(IconSourceKind::vector_asset(), provider, source_key)
    }

    pub fn raster_asset(provider: impl Into<String>, source_key: impl Into<String>) -> Self {
        Self::new(IconSourceKind::raster_asset(), provider, source_key)
    }

    pub fn icon_pack(provider: impl Into<String>, source_key: impl Into<String>) -> Self {
        Self::new(IconSourceKind::icon_pack(), provider, source_key)
    }

    pub fn unsupported_for_diagnostics(source_key: impl Into<String>) -> Self {
        Self::new(
            IconSourceKind::unsupported_for_diagnostics(),
            "unsupported",
            source_key,
        )
    }

    fn new(
        kind: IconSourceKind,
        provider: impl Into<String>,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            provider: provider.into(),
            source_key: source_key.into(),
            size_support: IconSizeSupport::scalable(),
            color_support: IconColorSupport::inherits_text_color(),
            native_vector_support: default_native_vector_support(kind),
            theme_token: None,
        }
    }

    pub fn with_size_support(mut self, size_support: IconSizeSupport) -> Self {
        self.size_support = size_support;
        self
    }

    pub fn with_color_support(mut self, color_support: IconColorSupport) -> Self {
        self.color_support = color_support;
        self
    }

    pub fn with_native_vector_support(mut self, support: IconNativeVectorSupport) -> Self {
        self.native_vector_support = support;
        self
    }

    pub fn with_theme_token(mut self, theme_token: ThemeTokenId) -> Self {
        self.theme_token = Some(theme_token);
        self
    }

    pub fn kind(&self) -> IconSourceKind {
        self.kind
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn size_support(&self) -> IconSizeSupport {
        self.size_support
    }

    pub fn color_support(&self) -> IconColorSupport {
        self.color_support
    }

    pub fn native_vector_support(&self) -> IconNativeVectorSupport {
        self.native_vector_support
    }

    pub(crate) fn has_missing_source_metadata(&self) -> bool {
        self.provider.trim().is_empty() || self.source_key.trim().is_empty()
    }

    pub fn theme_token(&self) -> Option<&ThemeTokenId> {
        self.theme_token.as_ref()
    }

    pub(crate) fn declared_theme_token_dependency(&self) -> Option<&ThemeTokenId> {
        self.color_support
            .requires_theme_token()
            .then_some(self.theme_token.as_ref())
            .flatten()
    }

    pub(crate) fn requires_missing_theme_token_reference(&self) -> bool {
        self.color_support.requires_theme_token() && self.theme_token.is_none()
    }

    pub(crate) fn has_unexpected_theme_token_reference(&self) -> bool {
        !self.color_support.is_missing()
            && !self.color_support.requires_theme_token()
            && self.theme_token.is_some()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.kind.digest_basis(),
            length_prefixed(&self.provider),
            length_prefixed(&self.source_key),
            self.size_support.digest_basis(),
            self.color_support.digest_basis(),
            self.native_vector_support.digest_basis(),
            self.theme_token
                .as_ref()
                .map(|token| length_prefixed(token.as_str()))
                .unwrap_or_else(|| "none".to_string())
        )
    }
}

fn default_native_vector_support(kind: IconSourceKind) -> IconNativeVectorSupport {
    match kind {
        IconSourceKind::Symbol | IconSourceKind::VectorAsset | IconSourceKind::IconPack => {
            IconNativeVectorSupport::supported()
        }
        IconSourceKind::RasterAsset | IconSourceKind::Unsupported => {
            IconNativeVectorSupport::unsupported_by_host()
        }
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
