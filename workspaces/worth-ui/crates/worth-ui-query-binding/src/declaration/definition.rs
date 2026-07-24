use super::WorthUiQueryViewIdentity;
use crate::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiQueryViewLifecycle {
    Snapshot,
    Live,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiQueryViewShape {
    Collection,
    Detail,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryViewDefinitionDigest(u64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryViewDefinition {
    identity: WorthUiQueryViewIdentity,
    lifecycle: WorthUiQueryViewLifecycle,
    shape: WorthUiQueryViewShape,
    required_facts: Box<[WorthUiQueryMeasurementFactFamily]>,
    digest: WorthUiQueryViewDefinitionDigest,
}

impl WorthUiQueryViewDefinition {
    pub fn measurement_snapshot(
        identity: impl Into<String>,
    ) -> Result<Self, super::WorthUiQueryViewIdentityError> {
        Ok(Self::measurement(
            WorthUiQueryViewIdentity::new(identity)?,
            WorthUiQueryViewLifecycle::Snapshot,
            WorthUiQueryViewShape::Collection,
        ))
    }

    pub fn measurement_live(
        identity: impl Into<String>,
    ) -> Result<Self, super::WorthUiQueryViewIdentityError> {
        Ok(Self::measurement(
            WorthUiQueryViewIdentity::new(identity)?,
            WorthUiQueryViewLifecycle::Live,
            WorthUiQueryViewShape::Collection,
        ))
    }

    pub(crate) fn measurement(
        identity: WorthUiQueryViewIdentity,
        lifecycle: WorthUiQueryViewLifecycle,
        shape: WorthUiQueryViewShape,
    ) -> Self {
        let required_facts: Box<[WorthUiQueryMeasurementFactFamily]> =
            [WorthUiQueryMeasurementFactFamily::ScrollContentExtent].into();
        let digest = definition_digest(&identity, lifecycle, shape, &required_facts);
        Self {
            identity,
            lifecycle,
            shape,
            required_facts,
            digest,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryViewIdentity {
        &self.identity
    }

    pub fn lifecycle(&self) -> WorthUiQueryViewLifecycle {
        self.lifecycle
    }

    pub fn shape(&self) -> WorthUiQueryViewShape {
        self.shape
    }

    pub fn required_facts(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.required_facts
    }

    pub fn digest(&self) -> WorthUiQueryViewDefinitionDigest {
        self.digest
    }
}

impl WorthUiQueryViewDefinitionDigest {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

fn definition_digest(
    identity: &WorthUiQueryViewIdentity,
    lifecycle: WorthUiQueryViewLifecycle,
    shape: WorthUiQueryViewShape,
    facts: &[WorthUiQueryMeasurementFactFamily],
) -> WorthUiQueryViewDefinitionDigest {
    let mut digest = fold_bytes(0xcbf2_9ce4_8422_2325, identity.as_str().as_bytes());
    digest = fold_bytes(digest, format!("{lifecycle:?}").as_bytes());
    digest = fold_bytes(digest, format!("{shape:?}").as_bytes());
    for fact in facts {
        digest = fold_bytes(digest, format!("{fact:?}").as_bytes());
    }
    WorthUiQueryViewDefinitionDigest(digest)
}

fn fold_bytes(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
