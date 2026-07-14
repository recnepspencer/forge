#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreExternalAuthorityToken {
    external_token_text: String,
    freshness: StoreExternalAuthorityTokenFreshness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreExternalAuthorityTokenFreshness {
    UnverifiedExternal,
    StaleRetained,
}

impl StoreExternalAuthorityToken {
    pub fn imported(external_token_text: impl Into<String>) -> Self {
        Self {
            external_token_text: external_token_text.into(),
            freshness: StoreExternalAuthorityTokenFreshness::UnverifiedExternal,
        }
    }

    pub fn stale_retained(external_token_text: impl Into<String>) -> Self {
        Self {
            external_token_text: external_token_text.into(),
            freshness: StoreExternalAuthorityTokenFreshness::StaleRetained,
        }
    }

    pub fn external_token_text(&self) -> &str {
        &self.external_token_text
    }

    pub const fn freshness(&self) -> StoreExternalAuthorityTokenFreshness {
        self.freshness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAuthorityFilename {
    imported_filename_text: String,
}

impl StoreAuthorityFilename {
    pub fn imported_filename(imported_filename_text: impl Into<String>) -> Self {
        Self {
            imported_filename_text: imported_filename_text.into(),
        }
    }

    pub fn imported_filename_text(&self) -> &str {
        &self.imported_filename_text
    }
}
