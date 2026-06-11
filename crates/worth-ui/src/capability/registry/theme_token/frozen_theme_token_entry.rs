use crate::capability::ThemeTokenId;

use super::{ThemeTokenDescriptor, ThemeTokenKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenThemeTokenEntry {
    descriptor: ThemeTokenDescriptor,
    key: ThemeTokenKey,
    resolved_target_id: ThemeTokenId,
}

impl FrozenThemeTokenEntry {
    pub(crate) fn new(
        descriptor: ThemeTokenDescriptor,
        key: ThemeTokenKey,
        resolved_target_id: ThemeTokenId,
    ) -> Self {
        Self {
            descriptor,
            key,
            resolved_target_id,
        }
    }

    pub fn descriptor(&self) -> &ThemeTokenDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &ThemeTokenKey {
        &self.key
    }

    pub fn resolved_target_id(&self) -> &ThemeTokenId {
        &self.resolved_target_id
    }
}
