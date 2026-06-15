#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRepeatedTemplateIdentity {
    template_identity: String,
    item_key: String,
}

impl WorthUiRepeatedTemplateIdentity {
    pub(crate) fn from_identity_basis(identity_basis: &str) -> Option<Self> {
        let template_identity = segment_value(identity_basis, "template")?;
        let item_key = segment_value(identity_basis, "item")?;
        Some(Self {
            template_identity,
            item_key,
        })
    }

    pub(crate) fn is_position_only(identity_basis: &str) -> bool {
        identity_basis.contains("template:")
            && identity_basis.contains("position:")
            && !identity_basis.contains("item:")
    }

    pub fn template_identity(&self) -> &str {
        &self.template_identity
    }

    pub fn item_key(&self) -> &str {
        &self.item_key
    }
}

fn segment_value(identity_basis: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    identity_basis.split('|').find_map(|segment| {
        segment
            .strip_prefix(&prefix)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    })
}
