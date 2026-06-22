use crate::capability::ViewBindingId;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiQueryBindingIdentity {
    view_binding_id: String,
    query_capability_digest: String,
    query_composition_profile_digest: String,
    result_shape_digest: String,
}

impl WorthUiQueryBindingIdentity {
    pub(crate) fn new(
        view_binding_id: &ViewBindingId,
        query_capability_digest: String,
        query_composition_profile_digest: String,
        result_shape_digest: String,
    ) -> Self {
        Self {
            view_binding_id: view_binding_id.as_str().to_owned(),
            query_capability_digest,
            query_composition_profile_digest,
            result_shape_digest,
        }
    }

    pub fn view_binding_id(&self) -> &str {
        &self.view_binding_id
    }

    pub fn query_capability_digest(&self) -> &str {
        &self.query_capability_digest
    }

    pub fn query_composition_profile_digest(&self) -> &str {
        &self.query_composition_profile_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }
}
