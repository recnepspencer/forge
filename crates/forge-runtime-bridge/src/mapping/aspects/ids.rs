use crate::identity::{AspectRegistrationIdTag, BridgeIdentity};

pub type BridgeAspectRegistrationId = BridgeIdentity<AspectRegistrationIdTag>;

impl BridgeAspectRegistrationId {
    pub fn from_stable_name(value: impl Into<std::sync::Arc<str>>) -> Self {
        Self::new(value)
    }
}
