#[cfg(not(test))]
use crate::identity::hash_parts;

#[cfg(test)]
mod testing;

#[cfg(test)]
pub use testing::*;

#[cfg(test)]
pub use testing::FrontierSurfaceDigest;

#[cfg(not(test))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrontierSurfaceDigest(String);

#[cfg(not(test))]
impl FrontierSurfaceDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_label(label: &str) -> Self {
        Self(hash_parts(&[format!("frontier_surface:{label}")]))
    }
}
