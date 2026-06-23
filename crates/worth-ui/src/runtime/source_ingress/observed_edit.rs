use crate::runtime::source_ingress::provider::WorthUiSourceProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiObservedAuthoredEdit {
    provider: WorthUiSourceProvider,
    provider_revision_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiObservedAuthoredEditDenial {
    EmptyProvider,
}

impl WorthUiObservedAuthoredEdit {
    pub fn from_source_provider(
        provider: WorthUiSourceProvider,
    ) -> Result<Self, WorthUiObservedAuthoredEditDenial> {
        if provider.is_empty() {
            return Err(WorthUiObservedAuthoredEditDenial::EmptyProvider);
        }
        let provider_revision_id = provider.id().to_owned();
        Ok(Self {
            provider,
            provider_revision_id,
        })
    }

    pub fn provider(&self) -> &WorthUiSourceProvider {
        &self.provider
    }

    pub fn provider_revision_id(&self) -> &str {
        &self.provider_revision_id
    }

    pub(crate) fn into_parts(self) -> (WorthUiSourceProvider, String) {
        (self.provider, self.provider_revision_id)
    }
}
