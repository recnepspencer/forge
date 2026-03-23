use smallvec::SmallVec;

use crate::publication::patch::data::AspectKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RecordStructuralChange {
    Created,
    Updated,
    Deleted,
    RetainedForAudit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalAspectSet(SmallVec<[AspectKey; 4]>);

impl CanonicalAspectSet {
    pub fn new(aspects: impl IntoIterator<Item = AspectKey>) -> Self {
        let mut aspects = aspects.into_iter().collect::<SmallVec<[AspectKey; 4]>>();
        if !aspects.windows(2).all(|window| window[0] < window[1]) {
            aspects.sort();
            aspects.dedup();
        }
        Self(aspects)
    }

    pub fn empty() -> Self {
        Self(SmallVec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &AspectKey> {
        self.0.iter()
    }
}

impl Default for CanonicalAspectSet {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<Vec<AspectKey>> for CanonicalAspectSet {
    fn from(value: Vec<AspectKey>) -> Self {
        Self::new(value)
    }
}

impl Serialize for CanonicalAspectSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CanonicalAspectSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let aspects = Vec::<AspectKey>::deserialize(deserializer)?;
        Ok(Self::new(aspects))
    }
}
