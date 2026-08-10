#[cfg(test)]
use crate::identity::hash_parts;

use super::super::family::LiveViewShapeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeLiveReport {
    digest: String,
    family: LiveViewShapeFamily,
    delivery_digest: String,
    replay_digest: String,
}

impl ViewShapeLiveReport {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn family(&self) -> LiveViewShapeFamily {
        self.family
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
    #[cfg(test)]
    pub(crate) fn new(
        family: LiveViewShapeFamily,
        delivery_digest: impl Into<String>,
        replay_digest: impl Into<String>,
    ) -> Self {
        let delivery_digest = delivery_digest.into();
        let replay_digest = replay_digest.into();
        let digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("delivery:{delivery_digest}"),
            format!("replay:{replay_digest}"),
        ]);
        Self {
            digest,
            family,
            delivery_digest,
            replay_digest,
        }
    }
}
