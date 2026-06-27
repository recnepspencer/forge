use crate::runtime::source_ingress::provider::WorthUiSourceProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceIngressHook {
    id: String,
    provider: WorthUiSourceProvider,
}

impl WorthUiSourceIngressHook {
    pub fn generated_source(id: impl Into<String>, provider: WorthUiSourceProvider) -> Self {
        Self {
            id: id.into(),
            provider,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn provider(&self) -> &WorthUiSourceProvider {
        &self.provider
    }
}
