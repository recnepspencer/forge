use crate::identity::hash_parts;
use crate::live::LiveQueryFamily;

use super::super::family::LiveViewShapeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeLiveLowering {
    digest: String,
    family: LiveViewShapeFamily,
    underlying_live_family: LiveQueryFamily,
}

impl ViewShapeLiveLowering {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn underlying_live_family(&self) -> &LiveQueryFamily {
        &self.underlying_live_family
    }

    pub(crate) fn new(family: LiveViewShapeFamily) -> Self {
        let underlying_live_family = family.underlying_live_family();
        let digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("underlying:{}", underlying_live_family.as_str()),
        ]);
        Self {
            digest,
            family,
            underlying_live_family,
        }
    }
}
